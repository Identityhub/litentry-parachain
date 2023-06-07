import { ApiPromise } from '@polkadot/api';
import { KeyringPair } from '@polkadot/keyring/types';
import { HexString } from '@polkadot/util/types';
import { hexToU8a, u8aToHex } from '@polkadot/util';
import { blake2AsHex } from '@polkadot/util-crypto';
import {
    EvmNetwork,
    IdentityGenericEvent,
    IntegrationTestContext,
    LitentryIdentity,
    LitentryValidationData,
    SubstrateNetwork,
    Web2Network,
} from '../type-definitions';
import { decryptWithAES, encryptWithAES, encryptWithTeeShieldingKey } from './crypto';
import { ApiTypes, SubmittableExtrinsic } from '@polkadot/api/types';
import { assert } from 'chai';
import { ethers } from 'ethers';
import { aesKey, keyNonce } from '../call';

// blake2_256(<sidechain nonce> + shieldingKey.encrypt(<primary account> + <identity-to-be-linked>).ciphertext)
export function generateVerificationMessage(
    context: IntegrationTestContext,
    signerAddress: Uint8Array,
    identity: LitentryIdentity,
    sidechainNonce: number
): HexString {
    const encodedIdentity = context.api.createType('LitentryIdentity', identity).toU8a();
    const payload = Buffer.concat([signerAddress, encodedIdentity]);
    const encryptedPayload = hexToU8a(encryptWithAES(aesKey, hexToU8a(keyNonce), payload));
    const encodedSidechainNonce = context.api.createType('Index', sidechainNonce);
    const msg = Buffer.concat([encodedSidechainNonce.toU8a(), encryptedPayload]);
    return blake2AsHex(msg, 256);
}

export async function buildIdentityHelper(
    address: HexString | string,
    network: SubstrateNetwork | EvmNetwork | Web2Network,
    type: 'Evm' | 'Substrate' | 'Web2'
): Promise<LitentryIdentity> {
    const identity: LitentryIdentity = {
        [type]: {
            address,
            network,
        },
    };
    return identity;
}

// If multiple transactions are built from multiple accounts, pass the signers as an array.
// If multiple transactions are built from a single account, signers cannot be an array.
//
// TODO: enforce `validations` if method is `linkIdentity`
export async function buildIdentityTxs(
    context: IntegrationTestContext,
    signers: KeyringPair[] | KeyringPair,
    identities: LitentryIdentity[],
    method: 'setUserShieldingKey' | 'linkIdentity' | 'removeIdentity',
    validations?: LitentryValidationData[]
): Promise<any[]> {
    const txs: any[] = [];
    const api = context.api;
    const mrEnclave = context.mrEnclave;
    const teeShieldingKey = context.teeShieldingKey;
    const len = Array.isArray(signers) ? signers.length : identities.length;
    for (let k = 0; k < len; k++) {
        const signer = Array.isArray(signers) ? signers[k] : signers;
        const identity = identities[k];
        let tx: SubmittableExtrinsic<ApiTypes>;
        let nonce: number;
        const encod_identity = api.createType('LitentryIdentity', identity).toU8a();
        const ciphertext_identity = encryptWithTeeShieldingKey(teeShieldingKey, encod_identity).toString('hex');
        nonce = (await api.rpc.system.accountNextIndex(signer.address)).toNumber();

        switch (method) {
            case 'setUserShieldingKey':
                const ciphertext = encryptWithTeeShieldingKey(
                    context.teeShieldingKey,
                    hexToU8a('0x22fc82db5b606998ad45099b7978b5b4f9dd4ea6017e57370ac56141caaabd12')
                ).toString('hex');
                tx = context.api.tx.identityManagement.setUserShieldingKey(context.mrEnclave, `0x${ciphertext}`);
                break;
            case 'linkIdentity':
                const data = validations![k];
                const validation = api.createType('LitentryValidationData', data).toU8a();
                const ciphertext_validation = encryptWithTeeShieldingKey(teeShieldingKey, validation).toString('hex');
                tx = api.tx.identityManagement.linkIdentity(
                    mrEnclave,
                    signer.address,
                    `0x${ciphertext_identity}`,
                    `0x${ciphertext_validation}`,
                    keyNonce
                );
                break;
            case 'removeIdentity':
                tx = api.tx.identityManagement.removeIdentity(mrEnclave, `0x${ciphertext_identity}`);
                break;
            default:
                throw new Error(`Invalid method: ${method}`);
        }
        txs.push({ tx, nonce });
    }

    return txs;
}

export async function handleIdentityEvents(
    context: IntegrationTestContext,
    aesKey: HexString,
    events: any[],
    type: 'UserShieldingKeySet' | 'IdentityLinked' | 'IdentityRemoved' | 'Failed' | 'LinkIdentityFailed'
): Promise<any[]> {
    let results: IdentityGenericEvent[] = [];

    for (let index = 0; index < events.length; index++) {
        switch (type) {
            case 'UserShieldingKeySet':
                results.push(
                    createIdentityEvent(
                        context.api,
                        events[index].data.account.toHex(),
                        undefined,
                        decryptWithAES(aesKey, events[index].data.idGraph, 'hex')
                    )
                );
                break;
            case 'IdentityLinked':
                results.push(
                    createIdentityEvent(
                        context.api,
                        events[index].data.account.toHex(),
                        decryptWithAES(aesKey, events[index].data.identity, 'hex'),
                        decryptWithAES(aesKey, events[index].data.idGraph, 'hex')
                    )
                );
                break;
            case 'IdentityRemoved':
                results.push(
                    createIdentityEvent(
                        context.api,
                        events[index].data.account.toHex(),
                        decryptWithAES(aesKey, events[index].data.identity, 'hex')
                    )
                );
                break;
            case 'Failed':
            case 'LinkIdentityFailed':
                results.push(events[index].data.detail.toHuman());
                break;
        }
    }
    console.log(`${type} event data:`, results);

    return [...results];
}

export function createIdentityEvent(
    api: ApiPromise,
    who: HexString,
    identityString?: HexString,
    idGraphString?: HexString
): IdentityGenericEvent {
    let identity = identityString ? api.createType('LitentryIdentity', identityString).toJSON() : undefined;
    let idGraph = idGraphString
        ? api.createType('Vec<(LitentryIdentity, IdentityContext)>', idGraphString).toJSON()
        : undefined;
    return <IdentityGenericEvent>{
        who,
        identity,
        idGraph,
    };
}

export async function buildValidations(
    context: IntegrationTestContext,
    identities: any[],
    startingSidechainNonce: number,
    network: 'ethereum' | 'substrate' | 'twitter',
    substrateSigners: KeyringPair[] | KeyringPair,
    ethereumSigners?: ethers.Wallet[]
): Promise<LitentryValidationData[]> {
    let signature_ethereum: HexString;
    let signature_substrate: Uint8Array;
    let validations: LitentryValidationData[] = [];

    for (let index = 0; index < identities.length; index++) {
        const substrateSigner = Array.isArray(substrateSigners) ? substrateSigners[index] : substrateSigners;
        const ethereumSigner = network === 'ethereum' ? ethereumSigners![index] : undefined;

        const msg = generateVerificationMessage(
            context,
            substrateSigner.addressRaw,
            identities[index],
            startingSidechainNonce + index
        );
        if (network === 'ethereum') {
            const ethereumValidationData: LitentryValidationData = {
                Web3Validation: {
                    Evm: {
                        message: '' as HexString,
                        signature: {
                            Ethereum: '' as HexString,
                        },
                    },
                },
            };
            console.log('post verification msg to ethereum: ', msg);
            ethereumValidationData!.Web3Validation!.Evm!.message = msg;
            const msgHash = ethers.utils.arrayify(msg);
            signature_ethereum = (await ethereumSigner!.signMessage(msgHash)) as HexString;
            console.log('signature_ethereum', ethereumSigners![index].address, signature_ethereum);

            ethereumValidationData!.Web3Validation!.Evm!.signature!.Ethereum = signature_ethereum;
            console.log('ethereumValidationData', ethereumValidationData);

            validations.push(ethereumValidationData);
        } else if (network === 'substrate') {
            const substrateValidationData: LitentryValidationData = {
                Web3Validation: {
                    Substrate: {
                        message: '' as HexString,
                        signature: {
                            Sr25519: '' as HexString,
                        },
                    },
                },
            };
            console.log('post verification msg to substrate: ', msg);
            substrateValidationData!.Web3Validation!.Substrate!.message = msg;
            signature_substrate = substrateSigner.sign(msg) as Uint8Array;
            substrateValidationData!.Web3Validation!.Substrate!.signature!.Sr25519 = u8aToHex(signature_substrate);
            validations.push(substrateValidationData);
        } else if (network === 'twitter') {
            console.log('post verification msg to twitter', msg);
            const twitterValidationData: LitentryValidationData = {
                Web2Validation: {
                    Twitter: {
                        tweet_id: `0x${Buffer.from('100', 'utf8').toString('hex')}`,
                    },
                },
            };
            validations.push(twitterValidationData);
        }
    }
    return validations;
}
