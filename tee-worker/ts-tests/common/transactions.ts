import { ApiPromise } from '@polkadot/api';
import { ApiTypes, SubmittableExtrinsic } from '@polkadot/api/types';
import { TransactionSubmit } from './type-definitions';
import { KeyringPair } from '@polkadot/keyring/types';
import { getListenTimeoutInBlocks } from './utils';
import { EventRecord, Event } from '@polkadot/types/interfaces';
import { HexString } from '@polkadot/util/types';

//transactions utils
export async function sendTxUntilInBlock(api: ApiPromise, tx: SubmittableExtrinsic<ApiTypes>, signer: KeyringPair) {
    return new Promise<{ block: string }>(async (resolve, reject) => {
        // The purpose of paymentInfo is to check whether the version of polkadot/api is suitable for the current test and to determine whether the transaction is successful.
        await tx.paymentInfo(signer);
        const nonce = await api.rpc.system.accountNextIndex(signer.address);
        await tx.signAndSend(signer, { nonce }, (result) => {
            if (result.status.isInBlock) {
                console.log(`Transaction included at blockHash ${result.status.asInBlock}`);
                resolve({
                    block: result.status.asInBlock.toString(),
                });
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
            const result = await new Promise((resolve, reject) => {
                const s = signers[index % signers.length];
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
                            resolve({
                                block: result.status.asInBlock.toString(),
                                txHash: result.status.hash.toString(),
                            });
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
export async function listenEvent(api: ApiPromise, section: string, methods: string[], txsLength: number, signers: HexString[]) {
    return new Promise<Event[]>(async (resolve, reject) => {
        let startBlock = 0;
        let events: EventRecord[] = [];
        const unsubscribe = await api.rpc.chain.subscribeNewHeads(async (header) => {

            const currentBlockNumber = header.number.toNumber();
            if (startBlock == 0) startBlock = currentBlockNumber;
            const timeout = await getListenTimeoutInBlocks(api);
            if (currentBlockNumber > startBlock + timeout) {
                reject('timeout');
                return;
            }
            console.log(`\n--------- block #${header.number}, hash ${header.hash} ---------\n`);
            const apiAt = await api.at(header.hash);

            const records: EventRecord[] = (await apiAt.query.system.events()) as any;
            records.forEach((e, i) => {
                const s = e.event.section;
                const m = e.event.method;
                const d = e.event.data;
                console.log(`Event[${i}]: ${s}.${m} ${d}`);
            });

            const filtered_events = records.filter(({ phase, event }) => {

                return phase.isApplyExtrinsic && section === event.section && methods.includes(event.method);
            });

            const filtered_events_with_signer = filtered_events.filter(event => {
                const signerDatas = event.event.data.find(d => {
                    if (Array.isArray(d)) {

                        return d.find(v => signers.includes(v.toHex()));
                    } else {


                        return signers.includes(d.toHex());
                    }
                });
                return !!signerDatas
            });
            const eventsToUse = filtered_events_with_signer.length > 0 ? filtered_events_with_signer : filtered_events;

            events = [...eventsToUse]


            if (events.length === txsLength) {
                resolve(events.map(e => e.event));

                unsubscribe();

                return;
            }

        });
    });
}
