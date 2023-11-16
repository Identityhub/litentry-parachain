import { ApiPromise, SubmittableResult } from '@polkadot/api';
import { KeyringPair } from '@polkadot/keyring/types';
import { defaultListenTimeoutInBlockNumber } from './utils';
import { EventRecord, Event } from '@polkadot/types/interfaces';
import { expect } from 'chai';
import colors from 'colors';
import type { HexString } from '@polkadot/util/types';
import type { Codec } from '@polkadot/types/types';
import type { IntegrationTestContext, TransactionSubmit } from './type-definitions';
import type { ApiTypes, SubmittableExtrinsic } from '@polkadot/api/types';
import { RequestEvent } from './type-definitions';

import { u8aToHex } from '@polkadot/util';
import { FrameSystemEventRecord } from '@litentry/parachain-api';

// transaction utils
export async function sendTxUntilInBlock(tx: SubmittableExtrinsic<ApiTypes>, signer: KeyringPair) {
    return new Promise<SubmittableResult>((resolve, reject) => {
        tx.signAndSend(signer, (result) => {
            if (result.status.isInBlock) {
                console.log(`Transaction included at blockHash ${result.status.asInBlock}`);
                resolve(result);
            } else if (result.status.isInvalid) {
                reject(`Transaction is ${result.status}`);
            }
        });
    });
}

export async function sendTxUntilInBlockList(
    api: ApiPromise,
    txs: TransactionSubmit[],
    signer: KeyringPair | KeyringPair[]
) {
    const signers = Array.isArray(signer) ? signer : [signer];
    return Promise.all(
        txs.map(async ({ tx, nonce }, index) => {
            const s = signers[index % signers.length];
            // The purpose of paymentInfo is to check whether the version of polkadot/api is suitable for the current test and to determine whether the transaction is successful.
            await tx.paymentInfo(s);
            const result = await new Promise((resolve, reject) => {
                tx.signAndSend(s, { nonce }, (result) => {
                    if (result.status.isInBlock) {
                        //catch error
                        if (result.dispatchError) {
                            if (result.dispatchError.isModule) {
                                const decoded = api.registry.findMetaError(result.dispatchError.asModule);
                                const { docs, name, section } = decoded;

                                console.log(`${section}.${name}: ${docs.join(' ')}`);
                                resolve(`${section}.${name}`);
                            } else {
                                console.log(result.dispatchError.toString());
                                resolve(result.dispatchError.toString());
                            }
                        } else {
                            console.log(`Transaction included at blockHash ${result.status.asInBlock}`);
                            resolve(result);
                        }
                    } else if (result.status.isInvalid) {
                        reject(`Transaction is ${result.status}`);
                    }
                });
            });

            return result;
        })
    );
}

// Subscribe to the chain until we get the first specified event with given `section` and `methods`.
// We can listen to multiple `methods` as long as they are emitted in the same block.
// The event consumer should do the decryption optionaly as it's event specific
export async function listenEvent(
    api: ApiPromise,
    section: string,
    methods: string[],
    txsLength: number,
    signers: HexString[],
    listenTimeoutInBlockNumber: number = defaultListenTimeoutInBlockNumber
) {
    // eslint-disable-next-line no-async-promise-executor
    return new Promise<Event[]>(async (resolve, reject) => {
        let startBlock = 0;
        const events: EventRecord[] = [];
        const unsubscribe = await api.rpc.chain.subscribeNewHeads(async (header) => {
            const currentBlockNumber = header.number.toNumber();
            if (startBlock == 0) startBlock = currentBlockNumber;
            if (currentBlockNumber > startBlock + listenTimeoutInBlockNumber) {
                reject('Timeout: No event received, please check the worker logs for more details');
                return;
            }
            console.log(`\n--------- block #${header.number}, hash ${header.hash} ---------\n`);
            const [, apiAt] = await Promise.all([api.rpc.chain.getBlock(header.hash), api.at(header.hash)]);

            const records: EventRecord[] = (await apiAt.query.system.events()) as any;
            const signerToIndexMap: Record<string, number> = {};
            for (let i = 0; i < signers.length; i++) {
                signerToIndexMap[signers[i]] = i;
            }
            const signerMatches = (d: Codec) => {
                if (Array.isArray(d)) {
                    return d.find((v) => signers.includes(v.toHex()));
                } else {
                    return signers.includes(d.toHex());
                }
            };
            records.forEach((e, i) => {
                const s = e.event.section;
                const m = e.event.method;
                const d = e.event.data;
                const eventString = `Event[${i}]: ${s}.${m} ${d}`;
                console.log(section === s ? colors.green(eventString) : eventString);
            });
            const eventsInExtrinsic = records.filter(({ event, phase }) => {
                if (
                    phase.isApplyExtrinsic &&
                    section === event.section &&
                    !methods.includes(event.method) &&
                    !(event.method in RequestEvent)
                ) {
                    reject(
                        `listenEvent error----Expect event:${methods} but received unexpected event :${event.method}`
                    );
                }
                return phase.isApplyExtrinsic && section === event.section && methods.includes(event.method);
            });
            // We're going to have to filter by signer, because multiple txs is going to mix
            const filteredEventsWithSigner = eventsInExtrinsic
                .filter((event) => {
                    const signerDatas = event.event.data.find(signerMatches);
                    return !!signerDatas;
                })
                .sort((a, b) => {
                    // We need sort by signers order
                    // First convert the signers array into an object signerToIndexMap, where the keys are each element in the signers array and the values are the index of that element in the array.
                    // Then, for each of the filtered events that match the given section and methods, the function uses the find function to locate the index of a specific parameter in the signers array.
                    // Then, it sorts the events based on this index so that the resulting event array is sorted according to the order of the signers array.
                    const signerIndexA = signerToIndexMap[a.event.data.find(signerMatches)!.toHex()];
                    const signerIndexB = signerToIndexMap[b.event.data.find(signerMatches)!.toHex()];
                    return signerIndexA - signerIndexB;
                });

            // There is no good compatibility method here.Only successful and failed events can be filtered normally, but it cannot filter error + successful events, which may need further optimization
            const eventsToUse = filteredEventsWithSigner.length > 0 ? filteredEventsWithSigner : eventsInExtrinsic;

            // we should append the new events, as the desired events can be emitted in different blocks
            events.push(...eventsToUse);

            if (events.length === txsLength) {
                resolve(events.map((e) => e.event));
                unsubscribe();
                return;
            }
        });
    });
}

export async function sendTxsWithUtility(
    context: IntegrationTestContext,
    signer: KeyringPair,
    txs: TransactionSubmit[],
    pallet: string,
    events: string[],
    listenTimeoutInBlockNumber?: number
): Promise<Event[]> {
    // ensure the tx is in block
    const isInBlockPromise = new Promise((resolve) => {
        context.api.tx.utility.batchAll(txs.map(({ tx }) => tx)).signAndSend(signer, async (result) => {
            if (result.status.isInBlock) {
                console.log(`Transaction included at blockHash ${result.status.asInBlock}`);
                resolve(result.status);
            } else if (result.status.isInvalid) {
                console.log(`Transaction is ${result.status}`);
            }
        });
    });

    await isInBlockPromise;

    const eventResults = await listenEvent(
        context.api,
        pallet,
        events,
        txs.length,
        [u8aToHex(signer.addressRaw)],
        listenTimeoutInBlockNumber
    );

    expect(eventResults.length).to.be.equal(txs.length);
    return eventResults;
}

export async function multiAccountTxSender(
    context: IntegrationTestContext,
    txs: TransactionSubmit[],
    signers: KeyringPair | KeyringPair[],
    pallet: string,
    events: string[],
    listenTimeoutInBlockNumber?: number
): Promise<Event[]> {
    const signersHex: HexString[] = [];
    if (Array.isArray(signers)) {
        for (let index = 0; index < signers.length; index++) {
            signersHex.push(u8aToHex(signers[index].addressRaw));
        }
    } else {
        signersHex.push(u8aToHex(signers.addressRaw));
    }

    await sendTxUntilInBlockList(context.api, txs, signers);
    const eventsResp = await listenEvent(
        context.api,
        pallet,
        events,
        txs.length,
        signersHex,
        listenTimeoutInBlockNumber
    );
    expect(eventsResp.length).to.be.equal(txs.length);
    return eventsResp;
}

// for DI-test

export const subscribeToEventsWithExtHash = async (
    requestIdentifier: string,
    context: IntegrationTestContext
): Promise<FrameSystemEventRecord[]> => {
    return new Promise<FrameSystemEventRecord[]>((resolve, reject) => {
        let blocksToScan = 30;
        /* 
        WARNING:The unsubscribe function is called inside the Promise callback, which is executed each time a new blockHeader is received. 
               `unsubscribe` is intended to unsubscribe a blockHeader if certain conditions are met. 
                If you use await, you will actually wait for this function to finish executing. 
                However, since it doesn't return a Promise, using await doesn't make sense and can lead to problematic code behaviour.
                soooooo, don't use await here
        */
        const unsubscribe = context.api.rpc.chain.subscribeNewHeads(async (blockHeader) => {
            const shiftedApi = await context.api.at(blockHeader.hash);

            const allBlockEvents = await shiftedApi.query.system.events();
            const allExtrinsicEvents = allBlockEvents.filter(({ phase }) => phase.isApplyExtrinsic);

            const matchingEvent = allExtrinsicEvents.find((eventRecord) => {
                const eventData = eventRecord.event.data.toHuman();
                return (
                    eventData != undefined &&
                    typeof eventData === 'object' &&
                    'reqExtHash' in eventData &&
                    eventData.reqExtHash === requestIdentifier
                );
            });

            if (matchingEvent == undefined) {
                blocksToScan -= 1;
                if (blocksToScan < 1) {
                    reject(new Error(`timed out listening for reqExtHash: ${requestIdentifier} in parachain events`));
                    (await unsubscribe)();
                }
                return;
            }

            const extrinsicIndex = matchingEvent.phase.asApplyExtrinsic;
            const requestEvents = allExtrinsicEvents.filter((eventRecord) =>
                eventRecord.phase.asApplyExtrinsic.eq(extrinsicIndex)
            );

            resolve(requestEvents);
            (await unsubscribe)();
        });
    });
};
