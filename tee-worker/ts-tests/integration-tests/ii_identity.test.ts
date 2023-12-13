import {
    describeLitentry,
    generateVerificationMessage,
    checkErrorDetail,
    checkIdGraph,
    buildIdentityHelper,
    buildIdentityTxs,
    buildValidations,
    assertIdentityDeactivated,
    buildIdentityFromKeypair,
    assertIdentityActivated,
    assertIdGraphMutation,
    PolkadotSigner,
} from './common/utils';
import { u8aConcat, u8aToHex, u8aToU8a, stringToU8a } from '@polkadot/util';
import { step } from 'mocha-steps';
import { assert } from 'chai';
import { sendTxsWithUtility } from './common/transactions';
import type { LitentryPrimitivesIdentity } from 'sidechain-api';
import type { LitentryValidationData, Web3Network } from 'parachain-api';
import type { IntegrationTestContext } from './common/common-types';
import type { HexString } from '@polkadot/util/types';
import { ethers } from 'ethers';
import { sendRequest } from './common/call';
import * as base58 from 'micro-base58';
import { decodeRpcBytesAsString } from './common/call';
import { createJsonRpcRequest, nextRequestId } from './common/helpers';

async function getEnclaveSignerPublicKey(context: IntegrationTestContext): Promise<string> {
    const request = createJsonRpcRequest('author_getEnclaveSignerAccount', [], nextRequestId(context));
    const response = await sendRequest(context.tee, request, context.api);
    if (!response.status.isOk) {
        throw new Error('Get author_getEnclaveSignerAccount response error!');
    }
    const enclaveSignerAccount = decodeRpcBytesAsString(response.value);
    console.log('enclaveSignerAccount', enclaveSignerAccount);
    return enclaveSignerAccount;
}

async function getNonce(base58mrEnclave: string, workerAddr: string, context: IntegrationTestContext): Promise<number> {
    const request = createJsonRpcRequest('author_getNextNonce', [base58mrEnclave, workerAddr], nextRequestId(context));
    const res = await sendRequest(context.tee, request, context.api);
    console.log('workerAddr', workerAddr);
    const resHex = res.value.toString();
    let nonce = 0;
    if (resHex) {
        nonce = context.api.createType('Index', '0x' + resHex.slice(2)?.match(/../g)?.reverse().join('')).toNumber();
    }
    console.log('resHex:', resHex);
    console.log('nonce is:', nonce);
    return nonce;
}

describeLitentry('Test Identity', 0, (context) => {
    // random wrong msg
    const wrongMsg = '0x693d9131808e7a8574c7ea5eb7813bdf356223263e61fa8fe2ee8e434508bc75';
    let signatureSubstrate;
    let eveIdentities: LitentryPrimitivesIdentity[] = [];
    let charlieIdentities: LitentryPrimitivesIdentity[] = [];
    let eveValidations: LitentryValidationData[] = [];
    let bobValidations: LitentryValidationData[] = [];
    let web3networks: Web3Network[][] = [];
    let base58mrEnclave: string;
    let workerAddress: string;
    let identityLinkedEvents;

    step('init', async () => {
        base58mrEnclave = base58.encode(Buffer.from(context.mrEnclave.slice(2), 'hex'));
        workerAddress = await getEnclaveSignerPublicKey(context);
    });

    step('link identities', async function () {
        // Alice links:
        // - a `mock_user` twitter
        // - alice's evm identity
        // - eve's substrate identity (as she can't link her own substrate again)
        let nonce = await getNonce(base58mrEnclave, workerAddress, context);
        const evmIdentity = await buildIdentityHelper(context.ethersWallet.alice.address, 'Evm', context);
        const eveSubstrateIdentity = await buildIdentityHelper(
            u8aToHex(context.substrateWallet.eve.addressRaw),
            'Substrate',
            context
        );
        const twitterIdentity = await buildIdentityHelper('mock_user', 'Twitter', context);

        // Bob links:
        // - charlie's substrate identity
        const charlieSubstrateIdentity = await buildIdentityHelper(
            u8aToHex(context.substrateWallet.charlie.addressRaw),
            'Substrate',
            context
        );

        eveIdentities = [evmIdentity, eveSubstrateIdentity, twitterIdentity];
        charlieIdentities = [charlieSubstrateIdentity];

        const aliceSubject = await buildIdentityFromKeypair(new PolkadotSigner(context.substrateWallet.alice), context);

        const evmValidations = await buildValidations(
            context,
            [aliceSubject],
            [evmIdentity],
            nonce,
            'ethereum',
            undefined,
            [context.ethersWallet.alice]
        );

        const eveSubstrateValidations = await buildValidations(
            context,
            [aliceSubject],
            [eveSubstrateIdentity],
            nonce + 1,
            'substrate',
            context.substrateWallet.eve
        );

        const twitterValidations = await buildValidations(
            context,
            [aliceSubject],
            [twitterIdentity],
            nonce + 2,
            'twitter'
        );

        eveValidations = [...evmValidations, ...eveSubstrateValidations, ...twitterValidations];

        const twitterNetworks = context.api.createType('Vec<Web3Network>', []) as unknown as Web3Network[];
        const evmNetworks = context.api.createType('Vec<Web3Network>', ['Ethereum', 'Bsc']) as unknown as Web3Network[];
        const eveSubstrateNetworks = context.api.createType('Vec<Web3Network>', [
            'Litentry',
            'Polkadot',
        ]) as unknown as Web3Network[];

        web3networks = [evmNetworks, eveSubstrateNetworks, twitterNetworks];

        const aliceTxs = await buildIdentityTxs(
            context,
            context.substrateWallet.alice,
            eveIdentities,
            'linkIdentity',
            eveValidations,
            web3networks
        );

        const aliceRespEvents = await sendTxsWithUtility(
            context,
            context.substrateWallet.alice,
            aliceTxs,
            'identityManagement',
            ['IdentityLinked']
        );

        identityLinkedEvents = aliceRespEvents.filter((e) =>
            context.api.events.identityManagement.IdentityLinked.is(e)
        );

        await assertIdGraphMutation(
            new PolkadotSigner(context.substrateWallet.alice),
            identityLinkedEvents,
            undefined,
            aliceTxs.length
        );

        // Bob check extension substrate identity
        // https://github.com/litentry/litentry-parachain/issues/1137
        const substrateExtensionValidationData = {
            Web3Validation: {
                Substrate: {
                    message: `0x${Buffer.from('mock_message', 'utf8').toString('hex')}`,
                    signature: {
                        Sr25519: '' as HexString,
                    },
                },
            },
        };
        const bobSubject = await buildIdentityFromKeypair(new PolkadotSigner(context.substrateWallet.bob), context);
        nonce = await getNonce(base58mrEnclave, workerAddress, context);
        console.log('nonce for step link identities', nonce);
        const msg = generateVerificationMessage(context, bobSubject, charlieSubstrateIdentity, nonce);
        console.log('post verification msg to substrate: ', msg);
        substrateExtensionValidationData.Web3Validation.Substrate.message = msg;
        // sign the wrapped version as in polkadot-extension
        signatureSubstrate = context.substrateWallet.charlie.sign(
            u8aConcat(stringToU8a('<Bytes>'), u8aToU8a(msg), stringToU8a('</Bytes>'))
        );
        substrateExtensionValidationData!.Web3Validation.Substrate.signature.Sr25519 = u8aToHex(signatureSubstrate);
        const bobSubstrateValidation = context.api.createType(
            'LitentryValidationData',
            substrateExtensionValidationData
        ) as unknown as LitentryValidationData;
        bobValidations = [bobSubstrateValidation];

        const bobSubstrateNetworks = context.api.createType('Vec<Web3Network>', [
            'Litentry',
            'Polkadot',
        ]) as unknown as Web3Network[];

        const bobTxs = await buildIdentityTxs(
            context,
            context.substrateWallet.bob,
            charlieIdentities,
            'linkIdentity',
            bobValidations,
            [bobSubstrateNetworks]
        );

        const bobRespEvents = await sendTxsWithUtility(
            context,
            context.substrateWallet.bob,
            bobTxs,
            'identityManagement',
            ['IdentityLinked']
        );

        identityLinkedEvents = bobRespEvents.filter((e) => context.api.events.identityManagement.IdentityLinked.is(e));
        await assertIdGraphMutation(
            new PolkadotSigner(context.substrateWallet.bob),
            identityLinkedEvents,
            undefined,
            bobTxs.length
        );
    });

    step('check IDGraph after LinkIdentity', async function () {
        const twitterIdentity = await buildIdentityHelper('mock_user', 'Twitter', context);
        const identityHex = context.sidechainRegistry.createType('LitentryPrimitivesIdentity', twitterIdentity).toHex();
        const aliceSubject = await buildIdentityFromKeypair(new PolkadotSigner(context.substrateWallet.alice), context);

        const respIdGraph = await checkIdGraph(context, 'IdentityManagement', 'IDGraphs', aliceSubject, identityHex);
        assert.isTrue(respIdGraph.linkBlock.toNumber() > 0, 'linkBlock should be greater than 0');
        assert.isTrue(respIdGraph.status.isActive, 'status should be active');
    });

    step('link invalid identities', async function () {
        const twitterIdentity = eveIdentities[2];
        const evmValidation = eveValidations[0];

        // link twitter identity with ethereum validation data
        // the `InvalidIdentity` error should be emitted prior to `AlreadyLinked` error
        const aliceTxs = await buildIdentityTxs(
            context,
            context.substrateWallet.alice,
            [twitterIdentity],
            'linkIdentity',
            [evmValidation],
            []
        );
        const aliceRespEvents = await sendTxsWithUtility(
            context,
            context.substrateWallet.alice,
            aliceTxs,
            'identityManagement',
            ['LinkIdentityFailed']
        );
        await checkErrorDetail(aliceRespEvents, 'InvalidIdentity');
    });

    step('link identities with wrong signature', async function () {
        const evmIdentity = eveIdentities[0];

        // link evm identity with wrong validation data(raw message)
        const evmSignature = (await context.ethersWallet.alice.signMessage(
            ethers.utils.arrayify(wrongMsg)
        )) as HexString;

        const validation = {
            Web3Validation: {
                Evm: {
                    message: wrongMsg as HexString,
                    signature: {
                        Ethereum: evmSignature as HexString,
                    },
                },
            },
        };
        const evmValidationData: LitentryValidationData = context.api.createType(
            'LitentryValidationData',
            validation
        ) as unknown as LitentryValidationData;
        const aliceTxs = await buildIdentityTxs(
            context,
            context.substrateWallet.alice,
            [evmIdentity],
            'linkIdentity',
            [evmValidationData],
            []
        );
        const aliceRespEvents = await sendTxsWithUtility(
            context,
            context.substrateWallet.alice,
            aliceTxs,
            'identityManagement',
            ['LinkIdentityFailed']
        );

        await checkErrorDetail(aliceRespEvents, 'UnexpectedMessage');
    });

    step('link already linked identity', async function () {
        const twitterIdentity = await buildIdentityHelper('mock_user', 'Twitter', context);
        const aliceSubject = await buildIdentityFromKeypair(new PolkadotSigner(context.substrateWallet.alice), context);

        const aliceIdentities = [twitterIdentity];

        const nonce = await getNonce(base58mrEnclave, workerAddress, context);
        console.log('nonce for step link already linked identity', nonce);
        const aliceTwitterValidations = await buildValidations(
            context,
            [aliceSubject],
            [twitterIdentity],
            nonce,
            'twitter',
            context.substrateWallet.alice,
            []
        );

        const aliceValidations = [...aliceTwitterValidations];

        const aliceTxs = await buildIdentityTxs(
            context,
            context.substrateWallet.alice,
            aliceIdentities,
            'linkIdentity',
            aliceValidations,
            []
        );

        const aliceRespEvents = await sendTxsWithUtility(
            context,
            context.substrateWallet.alice,
            aliceTxs,
            'identityManagement',
            ['LinkIdentityFailed']
        );

        await checkErrorDetail(aliceRespEvents, 'IdentityAlreadyLinked');
    });

    // TODO: testcase for linking prime address

    step('deactivate identities', async function () {
        // Alice deactivate all identities
        const aliceTxs = await buildIdentityTxs(
            context,
            context.substrateWallet.alice,
            eveIdentities,
            'deactivateIdentity'
        );
        const aliceDeactivatedEvents = await sendTxsWithUtility(
            context,
            context.substrateWallet.alice,
            aliceTxs,
            'identityManagement',
            ['IdentityDeactivated']
        );

        // Bob deactivate substrate identities
        const bobTxs = await buildIdentityTxs(
            context,
            context.substrateWallet.bob,
            charlieIdentities,
            'deactivateIdentity'
        );
        const bobDeactivatedEvents = await sendTxsWithUtility(
            context,
            context.substrateWallet.bob,
            bobTxs,
            'identityManagement',
            ['IdentityDeactivated']
        );

        // Alice check identity
        assertIdentityDeactivated(context.substrateWallet.alice, aliceDeactivatedEvents);

        // Bob check identity
        assertIdentityDeactivated(context.substrateWallet.bob, bobDeactivatedEvents);
    });

    step('check IDGraph after deactivateIdentity', async function () {
        // TODO: we should verify the IDGraph is empty
    });

    step('activate identity', async () => {
        const evmIdentity = await buildIdentityHelper(context.ethersWallet.alice.address, 'Evm', context);
        // Alice activate all identities
        const aliceTxs = await buildIdentityTxs(
            context,
            context.substrateWallet.alice,
            [evmIdentity],
            'activateIdentity'
        );
        const aliceActivatedEvents = await sendTxsWithUtility(
            context,
            context.substrateWallet.alice,
            aliceTxs,
            'identityManagement',
            ['IdentityActivated']
        );
        // Alice check identity
        await assertIdentityActivated(context, context.substrateWallet.alice, aliceActivatedEvents);
    });

    step('deactivate prime identity is disallowed', async function () {
        // deactivate prime identity
        const substratePrimeIdentity = await buildIdentityHelper(
            u8aToHex(context.substrateWallet.alice.addressRaw),
            'Substrate',
            context
        );

        const primeTxs = await buildIdentityTxs(
            context,
            context.substrateWallet.alice,
            [substratePrimeIdentity],
            'deactivateIdentity'
        );
        const primeEvents = await sendTxsWithUtility(
            context,
            context.substrateWallet.alice,
            primeTxs,
            'identityManagement',
            ['DeactivateIdentityFailed']
        );

        await checkErrorDetail(primeEvents, 'DeactivatePrimeIdentityDisallowed');
    });

    step('deactivate error identities', async function () {
        // Deactivate a nonexistent identity
        // context.substrateWallet.alice has already deactivated all identities in step('deactivate identities')
        const notExistingIdentity = await buildIdentityHelper('new_mock_user', 'Twitter', context);
        const aliceDeactivateTxs = await buildIdentityTxs(
            context,
            context.substrateWallet.alice,
            [notExistingIdentity],
            'deactivateIdentity'
        );
        const aliceDeactivatedEvents = await sendTxsWithUtility(
            context,
            context.substrateWallet.alice,
            aliceDeactivateTxs,
            'identityManagement',
            ['DeactivateIdentityFailed']
        );

        await checkErrorDetail(aliceDeactivatedEvents, 'IdentityNotExist');

        // deactivate a wrong identity (alice) for charlie
        const charlieDeactivateTxs = await buildIdentityTxs(
            context,
            context.substrateWallet.charlie,
            eveIdentities,
            'deactivateIdentity'
        );
        const charlieDeactivateEvents = await sendTxsWithUtility(
            context,
            context.substrateWallet.charlie,
            charlieDeactivateTxs,
            'identityManagement',
            ['DeactivateIdentityFailed']
        );

        await checkErrorDetail(charlieDeactivateEvents, 'IdentityNotExist');
    });

    step('exceeding IDGraph limit not allowed', async function () {
        // TODO: this needs to be reworked
        //       we have to provide validation data when linking
    });
});
