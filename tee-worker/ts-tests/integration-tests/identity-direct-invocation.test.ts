import { randomBytes, KeyObject } from 'crypto';
import { step } from 'mocha-steps';
import { assert } from 'chai';
import { hexToU8a, u8aToHex } from '@polkadot/util';
import {
    buildIdentityFromKeypair,
    buildIdentityHelper,
    buildValidations,
    checkIdGraph,
    initIntegrationTestContext,
} from './common/utils';
import { assertIdentityLinked, assertInitialIdGraphCreated } from './common/utils/assertion';
import {
    createSignedTrustedCallLinkIdentity,
    createSignedTrustedCallSetUserShieldingKey,
    createSignedTrustedGetterIdGraph,
    createSignedTrustedGetterUserShieldingKey,
    decodeIdGraph,
    getSidechainNonce,
    getTeeShieldingKey,
    sendRequestFromGetter,
    sendRequestFromTrustedCall,
} from './examples/direct-invocation/util'; // @fixme move to a better place
import type { IntegrationTestContext } from './common/type-definitions';
import { aesKey, keyNonce } from './common/call';
import { FrameSystemEventRecord, LitentryValidationData, Web3Network, filterEvents } from 'parachain-api';
import { CorePrimitivesErrorErrorDetail } from 'parachain-api';
import { LitentryPrimitivesIdentity } from 'sidechain-api';
import { Vec } from '@polkadot/types';
import { NodesmithProvider } from '@ethersproject/providers';
import { identity } from 'common/helpers';
import { ethers } from 'ethers';
import type { HexString } from '@polkadot/util/types';
const subscribeToEvents = async (
    requestIdentifier: string,
    context: IntegrationTestContext
): Promise<FrameSystemEventRecord[]> => {
    return new Promise<FrameSystemEventRecord[]>((resolve, reject) => {
        let blocksToScan = 30;

        const unsubscribe = context.api.rpc.chain.subscribeNewHeads(async (blockHeader) => {
            const shiftedApi = await context.api.at(blockHeader.hash);

            const allBlockEvents = await shiftedApi.query.system.events();
            const allExtrinsicEvents = allBlockEvents.filter(({ phase }) => phase.isApplyExtrinsic);

            const matchingEvent = allExtrinsicEvents.find((eventRecord) => {
                const eventData = eventRecord.event.data.toHuman();
                /**
                 * @FIXME I'd love a cleaner way to do this check :P
                 */

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

describe('Test Identity (direct invocation)', function () {
    let context: IntegrationTestContext = undefined as any;
    let teeShieldingKey: KeyObject = undefined as any;
    this.timeout(6000000);

    before(async () => {
        context = await initIntegrationTestContext(
            process.env.WORKER_END_POINT!, // @fixme evil assertion; centralize env access
            process.env.SUBSTRATE_END_POINT!, // @fixme evil assertion; centralize env access
            0
        );
        teeShieldingKey = await getTeeShieldingKey(context.tee, context.api);
    });

    it('needs a lot more work to be complete');
    it('most of the bob cases are missing');

    step('check user sidechain storage before create', async function () {
        const aliceSubject = await buildIdentityFromKeypair(context.substrateWallet.alice, context);
        const shieldingKeyGetter = createSignedTrustedGetterUserShieldingKey(
            context.api,
            context.substrateWallet.alice,
            aliceSubject
        );

        const shieldingKeyGetResult = await sendRequestFromGetter(
            context.tee,
            context.api,
            context.mrEnclave,
            teeShieldingKey,
            shieldingKeyGetter
        );

        const k = context.api.createType('Option<Bytes>', hexToU8a(shieldingKeyGetResult.value.toHex()));
        assert.isTrue(k.isNone, 'shielding key should be empty before set');
    });

    step('Invalid user shielding key', async function () {
        const aliceSubject = await buildIdentityFromKeypair(context.substrateWallet.alice, context);
        const bobSubstrateIdentity = await buildIdentityHelper(
            u8aToHex(context.substrateWallet.bob.addressRaw),
            'Substrate',
            context
        );
        const requestIdentifier = `0x${randomBytes(32).toString('hex')}`;

        const nonce = await getSidechainNonce(
            context.tee,
            context.api,
            context.mrEnclave,
            teeShieldingKey,
            aliceSubject
        );
        const [bobValidationData] = await buildValidations(
            context,
            [aliceSubject],
            [bobSubstrateIdentity],
            nonce.toNumber(),
            'substrate',
            context.substrateWallet.bob
        );
        const eventsPromise = subscribeToEvents(requestIdentifier, context);

        const linkIdentityCall = createSignedTrustedCallLinkIdentity(
            context.api,
            context.mrEnclave,
            nonce,
            context.substrateWallet.alice,
            aliceSubject,
            context.sidechainRegistry.createType('LitentryPrimitivesIdentity', bobSubstrateIdentity).toHex(),
            context.api.createType('LitentryValidationData', bobValidationData).toHex(),
            context.api.createType('Vec<Web3Network>', ['Polkadot', 'Litentry']).toHex(),
            keyNonce,
            requestIdentifier
        );

        const res = await sendRequestFromTrustedCall(
            context.tee,
            context.api,
            context.mrEnclave,
            teeShieldingKey,
            linkIdentityCall
        );
        assert.isTrue(
            res.status.isTrustedOperationStatus,
            `linkIdentityCall should be trusted operation status, but is ${res.status.type}`
        );
        const status = res.status.asTrustedOperationStatus;
        assert.isTrue(
            status.isSubmitted || status.isInSidechainBlock,
            `linkIdentityCall should be submitted or in sidechain block, but is ${status.type}`
        );

        const events = await eventsPromise;

        const linkIdentityFailed = context.api.events.identityManagement.LinkIdentityFailed;

        const isLinkIdentityFailed = linkIdentityFailed.is.bind(linkIdentityFailed);
        type EventLike = Parameters<typeof isLinkIdentityFailed>[0];
        const ievents: EventLike[] = events.map(({ event }) => event);
        const linkIdentityFailedEvents = ievents.filter(isLinkIdentityFailed);

        assert.lengthOf(linkIdentityFailedEvents, 1);
        /**
         * @fixme tsc is STILL not seeing the correct type for these events, WTF!?!?!?!?
         */
        assert.equal(
            (linkIdentityFailedEvents[0].data[1] as CorePrimitivesErrorErrorDetail).type,
            'UserShieldingKeyNotFound',
            'check linkIdentityFailedEvent detail is UserShieldingKeyNotFound, but is not'
        );
    });

    ['alice', 'bob'].forEach((name) => {
        step(`set user shielding key (${name})`, async function () {
            const wallet = context.substrateWallet[name]; // @FIXME: support EVM!!!
            const subject = await buildIdentityFromKeypair(wallet, context);
            const nonce = await getSidechainNonce(
                context.tee,
                context.api,
                context.mrEnclave,
                teeShieldingKey,
                subject
            );

            const requestIdentifier = `0x${randomBytes(32).toString('hex')}`;

            const setUserShieldingKeyCall = createSignedTrustedCallSetUserShieldingKey(
                context.api,
                context.mrEnclave,
                nonce,
                wallet,
                subject,
                aesKey,
                requestIdentifier
            );

            const eventsPromise = subscribeToEvents(requestIdentifier, context);
            const res = await sendRequestFromTrustedCall(
                context.tee,
                context.api,
                context.mrEnclave,
                teeShieldingKey,
                setUserShieldingKeyCall
            );

            assert.isTrue(
                res.status.isTrustedOperationStatus,
                `setUserShieldingKeyCall should be trusted operation status, but is ${res.status.type}`
            );
            const status = res.status.asTrustedOperationStatus;
            assert.isTrue(
                status.isSubmitted || status.isInSidechainBlock,
                `setUserShieldingKeyCall should be submitted or in sidechain block, but is ${status.type}`
            );

            const events = await eventsPromise;
            const userShieldingKeySetEvents = events
                .map(({ event }) => event)
                .filter(({ section, method }) => section === 'identityManagement' && method === 'UserShieldingKeySet');

            await assertInitialIdGraphCreated(context, [wallet], userShieldingKeySetEvents);
        });
    });

    step('check user shielding key from sidechain storage after setUserShieldingKey', async function () {
        const aliceSubject = await buildIdentityFromKeypair(context.substrateWallet.alice, context);
        const shieldingKeyGetter = createSignedTrustedGetterUserShieldingKey(
            context.api,
            context.substrateWallet.alice,
            aliceSubject
        );

        const shieldingKeyGetResult = await sendRequestFromGetter(
            context.tee,
            context.api,
            context.mrEnclave,
            teeShieldingKey,
            shieldingKeyGetter
        );

        const k = context.api.createType('Option<Bytes>', hexToU8a(shieldingKeyGetResult.value.toHex()));
        assert.equal(k.value.toString(), aesKey, 'respShieldingKey should be equal aesKey after set');
    });

    step('check idgraph from sidechain storage before linking', async function () {
        const aliceSubject = await buildIdentityFromKeypair(context.substrateWallet.alice, context);

        const idgraphGetter = createSignedTrustedGetterIdGraph(
            context.api,
            context.substrateWallet.alice,
            aliceSubject
        );
        const res = await sendRequestFromGetter(
            context.tee,
            context.api,
            context.mrEnclave,
            teeShieldingKey,
            idgraphGetter
        );

        const idGraph = decodeIdGraph(context.sidechainRegistry, res.value);
        assert.lengthOf(idGraph, 1);
        const [idGraphNodeIdentity, idGraphNodeContext] = idGraph[0];
        assert.deepEqual(idGraphNodeIdentity.toHuman(), aliceSubject.toHuman(), 'idGraph should include main address');
        assert.equal(idGraphNodeContext.status.toString(), 'Active', 'status should be active for main address');
    });

    step('link identities (alice)', async function () {
        const aliceSubject = await buildIdentityFromKeypair(context.substrateWallet.alice, context);

        let currentNonce = (
            await getSidechainNonce(context.tee, context.api, context.mrEnclave, teeShieldingKey, aliceSubject)
        ).toNumber();
        const getNextNonce = () => currentNonce++;

        // Alice links:
        // - a `mock_user` twitter
        // - alice's evm identity
        // - eve's substrate identity (as alice can't link her own substrate again)
        const linkIdentityRequestParams: {
            nonce: number;
            identity: LitentryPrimitivesIdentity;
            validation: LitentryValidationData;
            networks: Vec<Web3Network>;
        }[] = [];

        const twitterNonce = getNextNonce();
        const twitterIdentity = await buildIdentityHelper('mock_user', 'Twitter', context);
        const [twitterValidation] = await buildValidations(
            context,
            [aliceSubject],
            [twitterIdentity],
            twitterNonce,
            'twitter'
        );
        const twitterNetworks = context.api.createType('Vec<Web3Network>', []);
        linkIdentityRequestParams.push({
            nonce: twitterNonce,
            identity: twitterIdentity,
            validation: twitterValidation,
            networks: twitterNetworks,
        });

        const evmNonce = getNextNonce();
        const evmIdentity = await buildIdentityHelper(context.ethersWallet.alice.address, 'Evm', context);
        const [evmValidation] = await buildValidations(
            context,
            [aliceSubject],
            [evmIdentity],
            evmNonce,
            'ethereum',
            undefined,
            [context.ethersWallet.alice]
        );
        const evmNetworks = context.api.createType('Vec<Web3Network>', ['Ethereum', 'Bsc']);
        linkIdentityRequestParams.push({
            nonce: evmNonce,
            identity: evmIdentity,
            validation: evmValidation,
            networks: evmNetworks,
        });

        const eveSubstrateNonce = getNextNonce();
        const eveSubstrateIdentity = await buildIdentityHelper(
            u8aToHex(context.substrateWallet.eve.addressRaw),
            'Substrate',
            context
        );
        const [eveSubstrateValidation] = await buildValidations(
            context,
            [aliceSubject],
            [eveSubstrateIdentity],
            eveSubstrateNonce,
            'substrate',
            context.substrateWallet.eve
        );
        const eveSubstrateNetworks = context.api.createType('Vec<Web3Network>', ['Litentry', 'Polkadot']);
        linkIdentityRequestParams.push({
            nonce: eveSubstrateNonce,
            identity: eveSubstrateIdentity,
            validation: eveSubstrateValidation,
            networks: eveSubstrateNetworks,
        });

        for (const { nonce, identity, validation, networks } of linkIdentityRequestParams) {
            const requestIdentifier = `0x${randomBytes(32).toString('hex')}`;
            const eventsPromise = subscribeToEvents(requestIdentifier, context);
            const linkIdentityCall = createSignedTrustedCallLinkIdentity(
                context.api,
                context.mrEnclave,
                context.api.createType('Index', nonce),
                context.substrateWallet.alice,
                aliceSubject,
                identity.toHex(),
                validation.toHex(),
                networks.toHex(),
                keyNonce,
                requestIdentifier
            );

            const res = await sendRequestFromTrustedCall(
                context.tee,
                context.api,
                context.mrEnclave,
                teeShieldingKey,
                linkIdentityCall
            );
            assert.isTrue(
                res.status.isTrustedOperationStatus,
                `linkIdentityCall should be trusted operation status, but is ${res.status.type}`
            );
            const status = res.status.asTrustedOperationStatus;
            assert.isTrue(
                status.isSubmitted || status.isInSidechainBlock,
                `linkIdentityCall should be submitted or in sidechain block, but is ${status.type}`
            );

            const events = (await eventsPromise).map(({ event }) => event);
            let isIdentityLinked = false;
            events.forEach((event) => {
                if (context.api.events.identityManagement.LinkIdentityFailed.is(event)) {
                    assert.fail(JSON.stringify(event.toHuman(), null, 4));
                }
                if (context.api.events.identityManagement.IdentityLinked.is(event)) {
                    isIdentityLinked = true;
                }
            });
            assert.isTrue(isIdentityLinked); // @FIXME: dump events if no success or failure reported
        }

        /**
         * @FIXME
         * merging the next step with this one for now,
         * to have the identities available without having to repeat all of the above :P
         * There *has* to be a better way to organize all this tho.
         */
        const idgraphGetter = createSignedTrustedGetterIdGraph(
            context.api,
            context.substrateWallet.alice,
            aliceSubject
        );
        const res = await sendRequestFromGetter(
            context.tee,
            context.api,
            context.mrEnclave,
            teeShieldingKey,
            idgraphGetter
        );

        const idGraph = decodeIdGraph(context.sidechainRegistry, res.value);

        // for (const { identity } of linkIdentityRequestParams) {
        //     const identityDump = JSON.stringify(identity.toHuman(), null, 4);
        //     console.debug(`checking identity: ${identityDump}`);
        //     const idGraphNode = idGraph.find(([idGraphNodeIdentity]) => idGraphNodeIdentity.eq(identity)
        //     );
        //     assert.isDefined(idGraphNode, `identity not found in idGraph: ${identityDump}`);
        //     const [, idGraphNodeContext] = idGraphNode!;
        //     assert.equal(
        //         idGraphNodeContext.status.toString(),
        //         'Active',
        //         `status should be active for identity: ${identityDump}`
        //     );
        //     console.debug('active ✅');
        // }
    });

    /**
     * @FIXME: the logic for this currently lives in the previous step :P
     */
    it('check idgraph from sidechain storage after linking');

    step('linking invalid identity', async function () {

        const aliceSubject = await buildIdentityFromKeypair(context.substrateWallet.bob, context);

        let currentNonce = (
            await getSidechainNonce(context.tee, context.api, context.mrEnclave, teeShieldingKey, aliceSubject)
        ).toNumber();

        const getNextNonce = () => currentNonce++;

        const twitterIdentity = await buildIdentityHelper('mock_user', 'Twitter', context);
        const twitterNonce = getNextNonce();
        const evmNonce = getNextNonce();
        const evmIdentity = await buildIdentityHelper(context.ethersWallet.alice.address, 'Evm', context);
        const [evmValidation] = await buildValidations(
            context,
            [aliceSubject],
            [evmIdentity],
            evmNonce,
            'ethereum',
            undefined,
            [context.ethersWallet.bob]
        );

        const evmNetworks = context.api.createType('Vec<Web3Network>', ['Ethereum', 'Bsc']);
        const requestIdentifier = `0x${randomBytes(32).toString('hex')}`;
        const eventsPromise = subscribeToEvents(requestIdentifier, context);
        const linkIdentityCall = createSignedTrustedCallLinkIdentity(
            context.api,
            context.mrEnclave,
            context.api.createType('Index', twitterNonce),
            context.substrateWallet.bob,
            aliceSubject,
            twitterIdentity.toHex(),
            evmValidation.toHex(),
            evmNetworks.toHex(),
            keyNonce,
            requestIdentifier
        );

        const res = await sendRequestFromTrustedCall(
            context.tee,
            context.api,
            context.mrEnclave,
            teeShieldingKey,
            linkIdentityCall
        );

        assert.isTrue(
            res.status.isTrustedOperationStatus,
            `linkIdentityCall should be trusted operation status, but is ${res.status.type}`
        );
        const status = res.status.asTrustedOperationStatus;
        assert.isTrue(
            status.isSubmitted || status.isInSidechainBlock,
            `linkIdentityCall should be submitted or in sidechain block, but is ${status.type}`
        );

        const events = (await eventsPromise).map(({ event }) => event);


        // todo check events

    });

    step('linking identity with wrong signature', async function () {

        const aliceSubject = await buildIdentityFromKeypair(context.substrateWallet.alice, context);

        let currentNonce = (
            await getSidechainNonce(context.tee, context.api, context.mrEnclave, teeShieldingKey, aliceSubject)
        ).toNumber();
        const getNextNonce = () => currentNonce++;
        const evmIdentity = await buildIdentityHelper(context.ethersWallet.alice.address, 'Evm', context);
        const evmNetworks = context.api.createType('Vec<Web3Network>', ['Ethereum', 'Bsc']);

        const evmNonce = getNextNonce();
        // random wrong msg
        const wrongMsg = '0x693d9131808e7a8574c7ea5eb7813bdf356223263e61fa8fe2ee8e434508bc75';
        const ethereumSignature = (await context.ethersWallet.alice.signMessage(
            ethers.utils.arrayify(wrongMsg)
        )) as HexString;

        const ethereumValidationData = {
            Web3Validation: {
                Evm: {
                    message: wrongMsg as HexString,
                    signature: {
                        Ethereum: ethereumSignature as HexString,
                    },
                },
            },
        };
        const encodedVerifyIdentityValidation = context.api.createType(
            'LitentryValidationData',
            ethereumValidationData
        )
        const requestIdentifier = `0x${randomBytes(32).toString('hex')}`;
        const eventsPromise = subscribeToEvents(requestIdentifier, context);

        const linkIdentityCall = createSignedTrustedCallLinkIdentity(
            context.api,
            context.mrEnclave,
            context.api.createType('Index', evmNonce),
            context.substrateWallet.alice,
            aliceSubject,
            evmIdentity.toHex(),
            encodedVerifyIdentityValidation.toHex(),
            evmNetworks.toHex(),
            keyNonce,
            requestIdentifier
        );
        const res = await sendRequestFromTrustedCall(
            context.tee,
            context.api,
            context.mrEnclave,
            teeShieldingKey,
            linkIdentityCall
        );

        assert.isTrue(
            res.status.isTrustedOperationStatus,
            `linkIdentityCall should be trusted operation status, but is ${res.status.type}`
        );
        const status = res.status.asTrustedOperationStatus;
        assert.isTrue(
            status.isSubmitted || status.isInSidechainBlock,
            `linkIdentityCall should be submitted or in sidechain block, but is ${status.type}`
        );

        const events = (await eventsPromise).map(({ event }) => event);

        // todo check events

    })
    step('linking aleady linked identity', async function () {
        const aliceSubject = await buildIdentityFromKeypair(context.substrateWallet.alice, context);

        let currentNonce = (
            await getSidechainNonce(context.tee, context.api, context.mrEnclave, teeShieldingKey, aliceSubject)
        ).toNumber();
        const getNextNonce = () => currentNonce++;

        const twitterNonce = getNextNonce();
        const twitterIdentity = await buildIdentityHelper('mock_user', 'Twitter', context);
        const [twitterValidation] = await buildValidations(
            context,
            [aliceSubject],
            [twitterIdentity],
            twitterNonce,
            'twitter'
        );
        const twitterNetworks = context.api.createType('Vec<Web3Network>', []);

        const requestIdentifier = `0x${randomBytes(32).toString('hex')}`;
        const eventsPromise = subscribeToEvents(requestIdentifier, context);
        const linkIdentityCall = createSignedTrustedCallLinkIdentity(
            context.api,
            context.mrEnclave,
            context.api.createType('Index', twitterNonce),
            context.substrateWallet.alice,
            aliceSubject,
            twitterIdentity.toHex(),
            twitterValidation.toHex(),
            twitterNetworks.toHex(),
            keyNonce,
            requestIdentifier
        );
        const res = await sendRequestFromTrustedCall(
            context.tee,
            context.api,
            context.mrEnclave,
            teeShieldingKey,
            linkIdentityCall
        );

        assert.isTrue(
            res.status.isTrustedOperationStatus,
            `linkIdentityCall should be trusted operation status, but is ${res.status.type}`
        );
        const status = res.status.asTrustedOperationStatus;
        assert.isTrue(
            status.isSubmitted || status.isInSidechainBlock,
            `linkIdentityCall should be submitted or in sidechain block, but is ${status.type}`
        );

        const events = (await eventsPromise).map(({ event }) => event);

        // todo check events

    });
});
