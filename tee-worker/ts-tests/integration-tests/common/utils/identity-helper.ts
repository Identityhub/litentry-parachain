import { u8aToHex } from '@polkadot/util';
import { blake2AsHex } from '@polkadot/util-crypto';
import type { IntegrationTestContext } from '../common-types';
import { AesOutput } from 'parachain-api';
import { decryptWithAes, encryptWithTeeShieldingKey, Signer } from './crypto';
import { ethers } from 'ethers';
import type { Bytes, TypeRegistry, Vec } from '@polkadot/types';
import type { PalletIdentityManagementTeeIdentityContext } from 'sidechain-api';
import type { LitentryValidationData, Web3Network, CorePrimitivesIdentity } from 'parachain-api';
import type { ApiTypes, SubmittableExtrinsic } from '@polkadot/api/types';
import type { KeyringPair } from '@polkadot/keyring/types';
import type { HexString } from '@polkadot/util/types';
import { ECPairInterface } from 'ecpair';
import * as bitcoinMessage from 'bitcoinjs-message';

// blake2_256(<sidechain nonce> + <primary AccountId> + <identity-to-be-linked>)
export function generateVerificationMessage(
    context: IntegrationTestContext,
    signer: CorePrimitivesIdentity,
    identity: CorePrimitivesIdentity,
    sidechainNonce: number
): HexString {
    const encodedIdentity = context.api.createType('CorePrimitivesIdentity', identity).toU8a();
    const encodedWho = context.api.createType('CorePrimitivesIdentity', signer).toU8a();
    const encodedSidechainNonce = context.api.createType('Index', sidechainNonce);
    const msg = Buffer.concat([encodedSidechainNonce.toU8a(), encodedWho, encodedIdentity]);
    return blake2AsHex(msg, 256);
}

export async function buildIdentityHelper(
    address: HexString | string,
    type: CorePrimitivesIdentity['type'],
    context: IntegrationTestContext
): Promise<CorePrimitivesIdentity> {
    const identity = {
        [type]: address,
    };
    return context.api.createType('CorePrimitivesIdentity', identity);
}

export async function buildIdentityFromKeypair(
    signer: Signer,
    context: IntegrationTestContext
): Promise<CorePrimitivesIdentity> {
    const type: string = (() => {
        switch (signer.type()) {
            case 'ethereum':
                return 'Evm';
            case 'sr25519':
                return 'Substrate';
            case 'ed25519':
                return 'Substrate';
            case 'ecdsa':
                return 'Substrate';
            case 'bitcoin':
                return 'Bitcoin';
            default:
                return 'Substrate';
        }
    })();

    const address = signer.getAddressRaw();
    const identity = {
        [type]: address,
    };

    return context.api.createType('CorePrimitivesIdentity', identity);
}

// If multiple transactions are built from multiple accounts, pass the signers as an array.
// If multiple transactions are built from a single account, signers cannot be an array.
//
// TODO: enforce `validations` if method is `linkIdentity`
export async function buildIdentityTxs(
    context: IntegrationTestContext,
    signers: KeyringPair[] | KeyringPair,
    identities: CorePrimitivesIdentity[],
    method: 'linkIdentity' | 'deactivateIdentity' | 'activateIdentity',
    validations?: LitentryValidationData[],
    web3networks?: (Bytes | Vec<Web3Network>)[]
): Promise<any[]> {
    const txs: {
        tx: SubmittableExtrinsic<ApiTypes>;
        nonce: number;
    }[] = [];
    const api = context.api;
    const mrEnclave = context.mrEnclave;
    const teeShieldingKey = context.teeShieldingKey;
    const len = Array.isArray(signers) ? signers.length : identities.length;
    for (let k = 0; k < len; k++) {
        const signer = Array.isArray(signers) ? signers[k] : signers;
        const identity = identities[k];
        let tx: SubmittableExtrinsic<ApiTypes>;
        const ciphertextIdentity =
            identity && encryptWithTeeShieldingKey(teeShieldingKey, identity.toU8a()).toString('hex');
        const nonce = (await api.rpc.system.accountNextIndex(signer.address)).toNumber();

        switch (method) {
            case 'linkIdentity': {
                const validation = api.createType('LitentryValidationData', validations![k]).toU8a();
                const networks = api.createType('Vec<Web3Network>', web3networks![k]).toU8a();
                const ciphertextValidation = encryptWithTeeShieldingKey(teeShieldingKey, validation).toString('hex');
                const ciphertextNetworks = encryptWithTeeShieldingKey(teeShieldingKey, networks).toString('hex');

                tx = api.tx.identityManagement.linkIdentity(
                    mrEnclave,
                    signer.address,
                    `0x${ciphertextIdentity}`,
                    `0x${ciphertextValidation}`,
                    `0x${ciphertextNetworks}`
                );
                break;
            }
            case 'deactivateIdentity': {
                tx = api.tx.identityManagement.deactivateIdentity(mrEnclave, `0x${ciphertextIdentity}`);
                break;
            }
            case 'activateIdentity': {
                tx = api.tx.identityManagement.activateIdentity(mrEnclave, `0x${ciphertextIdentity}`);
                break;
            }
            default:
                throw new Error(`Invalid method: ${method}`);
        }
        txs.push({ tx, nonce });
    }

    return txs;
}

export function parseIdGraph(
    sidechainRegistry: TypeRegistry,
    idGraphOutput: AesOutput,
    aesKey: HexString
): [CorePrimitivesIdentity, PalletIdentityManagementTeeIdentityContext][] {
    const decryptedIdGraph = decryptWithAes(aesKey, idGraphOutput, 'hex');
    const idGraph: [CorePrimitivesIdentity, PalletIdentityManagementTeeIdentityContext][] =
        sidechainRegistry.createType(
            'Vec<(CorePrimitivesIdentity, PalletIdentityManagementTeeIdentityContext)>',
            decryptedIdGraph
        );

    return idGraph;
}

export async function buildValidations(
    context: IntegrationTestContext,
    signerIdentities: CorePrimitivesIdentity[],
    identities: CorePrimitivesIdentity[],
    startingSidechainNonce: number,
    network: 'ethereum' | 'substrate' | 'twitter' | 'bitcoin' | 'bitcoinPrettified',
    substrateSigners?: KeyringPair[] | KeyringPair,
    evmSigners?: ethers.Wallet[],
    bitcoinSigners?: ECPairInterface[] | ECPairInterface
): Promise<LitentryValidationData[]> {
    let evmSignature: HexString;
    let substrateSignature: Uint8Array;
    let bitcoinSignature: Buffer;
    const validations: LitentryValidationData[] = [];

    for (let index = 0; index < identities.length; index++) {
        const validationNonce = startingSidechainNonce + index;

        const msg = generateVerificationMessage(context, signerIdentities[index], identities[index], validationNonce);
        if (network === 'ethereum') {
            const evmValidationData = {
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
            evmValidationData.Web3Validation.Evm.message = msg;
            const msgHash = ethers.utils.arrayify(msg);
            const evmSigner = evmSigners![index];
            evmSignature = (await evmSigner.signMessage(msgHash)) as HexString;
            console.log('evmSignature', evmSigner.address, evmSignature);

            evmValidationData!.Web3Validation.Evm.signature.Ethereum = evmSignature;
            console.log('evmValidationData', evmValidationData);
            const encodedVerifyIdentityValidation = context.api.createType('LitentryValidationData', evmValidationData);

            validations.push(encodedVerifyIdentityValidation);
        } else if (network === 'substrate') {
            const substrateValidationData = {
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
            substrateValidationData.Web3Validation.Substrate.message = msg;
            const substrateSigner = Array.isArray(substrateSigners!) ? substrateSigners![index] : substrateSigners!;
            substrateSignature = substrateSigner.sign(msg) as Uint8Array;
            substrateValidationData!.Web3Validation.Substrate.signature.Sr25519 = u8aToHex(substrateSignature);
            const encodedVerifyIdentityValidation: LitentryValidationData = context.api.createType(
                'LitentryValidationData',
                substrateValidationData
            );
            validations.push(encodedVerifyIdentityValidation);
        } else if (network === 'bitcoin') {
            const bitcoinValidationData = {
                Web3Validation: {
                    Bitcoin: {
                        message: '' as HexString,
                        signature: {
                            Bitcoin: '' as HexString,
                        },
                    },
                },
            };
            console.log('post verification msg to bitcoin: ', msg);
            bitcoinValidationData.Web3Validation.Bitcoin.message = msg;
            const bitcoinSigner = Array.isArray(bitcoinSigners!) ? bitcoinSigners![index] : bitcoinSigners!;
            // we need to sign the hex string without `0x` prefix, the signature is base64-encoded string
            bitcoinSignature = bitcoinMessage.sign(
                msg.substring(2),
                bitcoinSigner.privateKey!,
                bitcoinSigner.compressed
            );
            bitcoinValidationData!.Web3Validation.Bitcoin.signature.Bitcoin = u8aToHex(bitcoinSignature);
            console.log('bitcoin pubkey: ', `0x${bitcoinSigner.publicKey.toString('hex')}`);
            console.log('bitcoin sig (base64): ', bitcoinSignature.toString('base64'));
            console.log('bitcoin sig (hex): ', u8aToHex(bitcoinSignature));
            const encodedVerifyIdentityValidation: LitentryValidationData = context.api.createType(
                'LitentryValidationData',
                bitcoinValidationData
            );
            validations.push(encodedVerifyIdentityValidation);
        } else if (network === 'bitcoinPrettified') {
            const bitcoinValidationData = {
                Web3Validation: {
                    Bitcoin: {
                        message: '' as HexString,
                        signature: {
                            BitcoinPrettified: '' as HexString,
                        },
                    },
                },
            };
            console.log('post verification msg to bitcoin: ', msg);
            bitcoinValidationData.Web3Validation.Bitcoin.message = msg;
            const bitcoinSigner = Array.isArray(bitcoinSigners!) ? bitcoinSigners![index] : bitcoinSigners!;
            bitcoinSignature = bitcoinMessage.sign(
                'Litentry authorization token: ' + msg,
                bitcoinSigner.privateKey!,
                bitcoinSigner.compressed
            );

            bitcoinValidationData!.Web3Validation.Bitcoin.signature.BitcoinPrettified = u8aToHex(bitcoinSignature);
            console.log('bitcoin pubkey: ', `0x${bitcoinSigner.publicKey.toString('hex')}`);

            console.log('bitcoin sig (base64): ', bitcoinSignature.toString('base64'));
            console.log('bitcoin sig (hex): ', u8aToHex(bitcoinSignature));
            const encodedVerifyIdentityValidation: LitentryValidationData = context.api.createType(
                'LitentryValidationData',
                bitcoinValidationData
            );
            validations.push(encodedVerifyIdentityValidation);
        } else if (network === 'twitter') {
            console.log('post verification msg to twitter: ', msg);
            const twitterValidationData = {
                Web2Validation: {
                    Twitter: {
                        tweet_id: `0x${Buffer.from(validationNonce.toString(), 'utf8').toString('hex')}`,
                    },
                },
            };

            const encodedVerifyIdentityValidation = context.api.createType(
                'LitentryValidationData',
                twitterValidationData
            );
            validations.push(encodedVerifyIdentityValidation);
        }
    }
    return validations;
}
