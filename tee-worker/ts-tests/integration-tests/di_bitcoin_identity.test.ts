import { randomBytes, KeyObject } from 'crypto';
import { step } from 'mocha-steps';
import { assert } from 'chai';
import {
    buildIdentityFromKeypair,
    buildIdentityHelper,
    buildValidations,
    initIntegrationTestContext,
    EthersSigner,
    BitcoinSigner,
    assertIdGraphMutationResult,
    assertIdGraphHash,
} from './common/utils';
import { assertIsInSidechainBlock } from './common/utils/assertion';
import {
    createSignedTrustedCallLinkIdentity,
    createSignedTrustedGetterIdGraph,
    createSignedTrustedCallDeactivateIdentity,
    createSignedTrustedCallActivateIdentity,
    decodeIdGraph,
    getSidechainNonce,
    getTeeShieldingKey,
    sendRequestFromGetter,
    sendRequestFromTrustedCall,
} from './common/di-utils'; // @fixme move to a better place
import type { IntegrationTestContext } from './common/common-types';
import { aesKey } from './common/call';
import { LitentryValidationData, Web3Network, CorePrimitivesIdentity } from 'parachain-api';
import { Bytes, Vec } from '@polkadot/types';
import type { HexString } from '@polkadot/util/types';

import { u8aToHex } from '@polkadot/util';
describe('Test Identity (bitcoin direct invocation)', function () {
    let context: IntegrationTestContext = undefined as any;
    let teeShieldingKey: KeyObject = undefined as any;
    let aliceBitcoinIdentity: CorePrimitivesIdentity = undefined as any;
    let aliceEvmIdentity: CorePrimitivesIdentity;
    let bobBitcoinIdentity: CorePrimitivesIdentity;

    // Alice links:
    // - alice's evm identity
    // - bob's bitcoin identity
    const linkIdentityRequestParams: {
        nonce: number;
        identity: CorePrimitivesIdentity;
        validation: LitentryValidationData;
        networks: Bytes | Vec<Web3Network>;
    }[] = [];

    const deactivateIdentityRequestParams: {
        nonce: number;
        identity: CorePrimitivesIdentity;
    }[] = [];

    const activateIdentityRequestParams: {
        nonce: number;
        identity: CorePrimitivesIdentity;
    }[] = [];

    this.timeout(6000000);

    before(async () => {
        context = await initIntegrationTestContext(
            process.env.WORKER_ENDPOINT!, // @fixme evil assertion; centralize env access
            process.env.NODE_ENDPOINT! // @fixme evil assertion; centralize env access
        );
        teeShieldingKey = await getTeeShieldingKey(context);
        aliceBitcoinIdentity = await buildIdentityHelper(
            u8aToHex(context.bitcoinWallet.alice.publicKey),
            'Bitcoin',
            context
        );
        aliceEvmIdentity = await buildIdentityFromKeypair(new EthersSigner(context.ethersWallet.alice), context);
        bobBitcoinIdentity = await buildIdentityHelper(
            u8aToHex(context.bitcoinWallet.bob.publicKey),
            'Bitcoin',
            context
        );
    });

    step('check idGraph from sidechain storage before linking', async function () {
        const idGraphGetter = await createSignedTrustedGetterIdGraph(
            context.api,
            new BitcoinSigner(context.bitcoinWallet.alice),
            aliceBitcoinIdentity
        );
        const res = await sendRequestFromGetter(context, teeShieldingKey, idGraphGetter);
        const idGraph = decodeIdGraph(context.sidechainRegistry, res.value);
        assert.lengthOf(idGraph, 0);
    });

    step('linking identities (alice bitcoin account)', async function () {
        let currentNonce = (await getSidechainNonce(context, aliceBitcoinIdentity)).toNumber();
        const getNextNonce = () => currentNonce++;

        const aliceEvmNonce = getNextNonce();
        const [aliceEvmValidation] = await buildValidations(
            context,
            [aliceBitcoinIdentity],
            [aliceEvmIdentity],
            aliceEvmNonce,
            'ethereum',
            undefined,
            [context.ethersWallet.alice]
        );
        const aliceEvmNetworks = context.api.createType('Vec<Web3Network>', ['Ethereum', 'Bsc']);
        linkIdentityRequestParams.push({
            nonce: aliceEvmNonce,
            identity: aliceEvmIdentity,
            validation: aliceEvmValidation,
            networks: aliceEvmNetworks,
        });

        // link another bitcoin account with prettified signature
        const bobBitcoinNonce = getNextNonce();
        const [bobBitcoinValidation] = await buildValidations(
            context,
            [aliceBitcoinIdentity],
            [bobBitcoinIdentity],
            bobBitcoinNonce,
            'bitcoinPrettified',
            undefined,
            undefined,
            context.bitcoinWallet.bob
        );
        const bobBitcoinNetowrks = context.api.createType('Vec<Web3Network>', ['BitcoinP2tr']);
        linkIdentityRequestParams.push({
            nonce: bobBitcoinNonce,
            identity: bobBitcoinIdentity,
            validation: bobBitcoinValidation,
            networks: bobBitcoinNetowrks,
        });

        const idGraphHashResults: HexString[] = [];
        let expectedIdGraphs: [CorePrimitivesIdentity, boolean][][] = [
            [
                [aliceBitcoinIdentity, true],
                [aliceEvmIdentity, true],
            ],
            [[bobBitcoinIdentity, true]],
        ];

        for (const { nonce, identity, validation, networks } of linkIdentityRequestParams) {
            const requestIdentifier = `0x${randomBytes(32).toString('hex')}`;
            const linkIdentityCall = await createSignedTrustedCallLinkIdentity(
                context.api,
                context.mrEnclave,
                context.api.createType('Index', nonce),
                new BitcoinSigner(context.bitcoinWallet.alice),
                aliceBitcoinIdentity,
                identity.toHex(),
                validation.toHex(),
                networks.toHex(),
                context.api.createType('Option<RequestAesKey>', aesKey).toHex(),
                requestIdentifier
            );

            const res = await sendRequestFromTrustedCall(context, teeShieldingKey, linkIdentityCall);

            idGraphHashResults.push(
                await assertIdGraphMutationResult(
                    context,
                    teeShieldingKey,
                    aliceBitcoinIdentity,
                    res,
                    'LinkIdentityResult',
                    expectedIdGraphs[0]
                )
            );
            expectedIdGraphs = expectedIdGraphs.slice(1, expectedIdGraphs.length);
            await assertIsInSidechainBlock('linkIdentityCall', res);
        }

        assert.lengthOf(idGraphHashResults, 2);
    });

    step('check user sidechain storage after linking', async function () {
        const idGraphGetter = await createSignedTrustedGetterIdGraph(
            context.api,
            new BitcoinSigner(context.bitcoinWallet.alice),
            aliceBitcoinIdentity
        );
        const res = await sendRequestFromGetter(context, teeShieldingKey, idGraphGetter);
        const idGraph = decodeIdGraph(context.sidechainRegistry, res.value);

        // according to the order of linkIdentityRequestParams
        const expectedWeb3Networks = [['Ethereum', 'Bsc'], ['BitcoinP2tr']];
        let currentIndex = 0;

        for (const { identity } of linkIdentityRequestParams) {
            const identityDump = JSON.stringify(identity.toHuman(), null, 4);
            console.debug(`checking identity: ${identityDump}`);
            const idGraphNode = idGraph.find(([idGraphNodeIdentity]) => idGraphNodeIdentity.eq(identity));
            assert.isDefined(idGraphNode, `identity not found in idGraph: ${identityDump}`);
            const [, idGraphNodeContext] = idGraphNode!;

            const web3networks = idGraphNode![1].web3networks.toHuman();
            assert.deepEqual(web3networks, expectedWeb3Networks[currentIndex]);

            assert.equal(
                idGraphNodeContext.status.toString(),
                'Active',
                `status should be active for identity: ${identityDump}`
            );
            console.debug('active ✅');

            currentIndex++;
        }

        await assertIdGraphHash(context, teeShieldingKey, aliceBitcoinIdentity, idGraph);
    });
    step('deactivating identity(alice bitcoin account)', async function () {
        let currentNonce = (await getSidechainNonce(context, aliceBitcoinIdentity)).toNumber();
        const getNextNonce = () => currentNonce++;

        const aliceEvmNonce = getNextNonce();

        deactivateIdentityRequestParams.push({
            nonce: aliceEvmNonce,
            identity: aliceEvmIdentity,
        });

        const idGraphHashResults: HexString[] = [];
        let expectedIdGraphs: [CorePrimitivesIdentity, boolean][][] = [[[aliceEvmIdentity, false]]];

        for (const { nonce, identity } of deactivateIdentityRequestParams) {
            const requestIdentifier = `0x${randomBytes(32).toString('hex')}`;
            const deactivateIdentityCall = await createSignedTrustedCallDeactivateIdentity(
                context.api,
                context.mrEnclave,
                context.api.createType('Index', nonce),
                new BitcoinSigner(context.bitcoinWallet.alice),
                aliceBitcoinIdentity,
                identity.toHex(),
                context.api.createType('Option<RequestAesKey>', aesKey).toHex(),
                requestIdentifier
            );

            const res = await sendRequestFromTrustedCall(context, teeShieldingKey, deactivateIdentityCall);
            idGraphHashResults.push(
                await assertIdGraphMutationResult(
                    context,
                    teeShieldingKey,
                    aliceBitcoinIdentity,
                    res,
                    'DeactivateIdentityResult',
                    expectedIdGraphs[0]
                )
            );
            expectedIdGraphs = expectedIdGraphs.slice(1, expectedIdGraphs.length);
            await assertIsInSidechainBlock('deactivateIdentityCall', res);
            assert.lengthOf(idGraphHashResults, 1);
        }
    });

    step('check idGraph from sidechain storage after deactivating', async function () {
        const idGraphGetter = await createSignedTrustedGetterIdGraph(
            context.api,
            new BitcoinSigner(context.bitcoinWallet.alice),
            aliceBitcoinIdentity
        );
        const res = await sendRequestFromGetter(context, teeShieldingKey, idGraphGetter);
        const idGraph = decodeIdGraph(context.sidechainRegistry, res.value);

        for (const { identity } of deactivateIdentityRequestParams) {
            const identityDump = JSON.stringify(identity.toHuman(), null, 4);
            console.debug(`checking identity: ${identityDump}`);
            const idGraphNode = idGraph.find(([idGraphNodeIdentity]) => idGraphNodeIdentity.eq(identity));
            assert.isDefined(idGraphNode, `identity not found in idGraph: ${identityDump}`);
            const [, idGraphNodeContext] = idGraphNode!;

            assert.equal(
                idGraphNodeContext.status.toString(),
                'Inactive',
                `status should be Inactive for identity: ${identityDump}`
            );
            console.debug('inactive ✅');
        }

        await assertIdGraphHash(context, teeShieldingKey, aliceBitcoinIdentity, idGraph);
    });

    step('activating identity(alice bitcoin account)', async function () {
        let currentNonce = (await getSidechainNonce(context, aliceBitcoinIdentity)).toNumber();
        const getNextNonce = () => currentNonce++;

        const aliceEvmNonce = getNextNonce();

        activateIdentityRequestParams.push({
            nonce: aliceEvmNonce,
            identity: aliceEvmIdentity,
        });

        const idGraphHashResults: HexString[] = [];
        let expectedIdGraphs: [CorePrimitivesIdentity, boolean][][] = [[[aliceEvmIdentity, true]]];

        for (const { nonce, identity } of activateIdentityRequestParams) {
            const requestIdentifier = `0x${randomBytes(32).toString('hex')}`;
            const activateIdentityCall = await createSignedTrustedCallActivateIdentity(
                context.api,
                context.mrEnclave,
                context.api.createType('Index', nonce),
                new BitcoinSigner(context.bitcoinWallet.alice),
                aliceBitcoinIdentity,
                identity.toHex(),
                context.api.createType('Option<RequestAesKey>', aesKey).toHex(),
                requestIdentifier
            );

            const res = await sendRequestFromTrustedCall(context, teeShieldingKey, activateIdentityCall);
            idGraphHashResults.push(
                await assertIdGraphMutationResult(
                    context,
                    teeShieldingKey,
                    aliceBitcoinIdentity,
                    res,
                    'ActivateIdentityResult',
                    expectedIdGraphs[0]
                )
            );
            expectedIdGraphs = expectedIdGraphs.slice(1, expectedIdGraphs.length);
            await assertIsInSidechainBlock('activateIdentityCall', res);
        }
        assert.lengthOf(idGraphHashResults, 1);
    });

    step('check idGraph from sidechain storage after activating', async function () {
        const idGraphGetter = await createSignedTrustedGetterIdGraph(
            context.api,
            new BitcoinSigner(context.bitcoinWallet.alice),
            aliceBitcoinIdentity
        );
        const res = await sendRequestFromGetter(context, teeShieldingKey, idGraphGetter);
        const idGraph = decodeIdGraph(context.sidechainRegistry, res.value);

        for (const { identity } of linkIdentityRequestParams) {
            const identityDump = JSON.stringify(identity.toHuman(), null, 4);
            console.debug(`checking identity: ${identityDump}`);
            const idGraphNode = idGraph.find(([idGraphNodeIdentity]) => idGraphNodeIdentity.eq(identity));
            assert.isDefined(idGraphNode, `identity not found in idGraph: ${identityDump}`);
            const [, idGraphNodeContext] = idGraphNode!;

            assert.equal(
                idGraphNodeContext.status.toString(),
                'Active',
                `status should be active for identity: ${identityDump}`
            );
            console.debug('active ✅');
        }

        await assertIdGraphHash(context, teeShieldingKey, aliceBitcoinIdentity, idGraph);
    });
});
