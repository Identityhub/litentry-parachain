import {
    IdentityGenericEvent,
    IntegrationTestContext,
    LitentryIdentity,
    LitentryValidationData,
} from './type-definitions';
import {
    encryptWithTeeShieldingKey,
    listenEncryptedEvents,
    sendTxUntilInBlock,
    listenCreatedIdentityEvents,
} from './utils';
import { KeyringPair } from '@polkadot/keyring/types';
import { HexString } from '@polkadot/util/types';
import { generateChallengeCode } from './web3/setup';
import { ApiPromise } from '@polkadot/api';
import { Assertion } from './type-definitions';


export async function setUserShieldingKey(
    context: IntegrationTestContext,
    signer: KeyringPair,
    aesKey: HexString,
    listening: boolean
): Promise<HexString | undefined> {
    const ciphertext = encryptWithTeeShieldingKey(context.teeShieldingKey, aesKey).toString('hex');

    await context.substrate.tx.identityManagement
        .setUserShieldingKey(context.shard, `0x${ciphertext}`)
        .paymentInfo(signer);

    const tx =context.substrate.tx.identityManagement
        .setUserShieldingKey(context.shard, `0x${ciphertext}`)
    
    //The purpose of paymentInfo is to check whether the version of polkadot/api is suitable for the current test and to determine whether the transaction is successful.
    await tx.paymentInfo(signer);

    await sendTxUntilInBlock(context.substrate, tx, signer);
    
    if (listening) {
        const event = await listenEncryptedEvents(context, aesKey, {
            module: 'identityManagement',
            method: 'userShieldingKeySet',
            event: 'UserShieldingKeySet',
        });
        const [who] = event.eventData;
        return who;
    }
    return undefined;
}

export async function createIdentity(
    context: IntegrationTestContext,
    signer: KeyringPair,
    aesKey: HexString,
    listening: boolean,
    identity: LitentryIdentity
): Promise<IdentityGenericEvent | undefined> {
    const encode = context.substrate.createType('LitentryIdentity', identity).toHex();
    const ciphertext = encryptWithTeeShieldingKey(context.teeShieldingKey, encode).toString('hex');

    const tx =context.substrate.tx.identityManagement
        .createIdentity(context.shard, signer.address, `0x${ciphertext}`, null)
    
    //The purpose of paymentInfo is to check whether the version of polkadot/api is suitable for the current test and to determine whether the transaction is successful.
    await tx.paymentInfo(signer);

    await sendTxUntilInBlock(context.substrate, tx, signer);
    
    if (listening) {
        const event = await listenCreatedIdentityEvents(context, aesKey);
        const [who, _identity, idGraph, challengeCode] = event.eventData;
        return decodeIdentityEvent(context.substrate, who, _identity, idGraph, challengeCode);
    }
    return undefined;
}

export async function removeIdentity(
    context: IntegrationTestContext,
    signer: KeyringPair,
    aesKey: HexString,
    listening: boolean,
    identity: LitentryIdentity
): Promise<IdentityGenericEvent | undefined> {
    const encode = context.substrate.createType('LitentryIdentity', identity).toHex();
    const ciphertext = encryptWithTeeShieldingKey(context.teeShieldingKey, encode).toString('hex');

    const tx = context.substrate.tx.identityManagement.removeIdentity(context.shard, `0x${ciphertext}`);

    //The purpose of paymentInfo is to check whether the version of polkadot/api is suitable for the current test and to determine whether the transaction is successful.
    await tx.paymentInfo(signer);
    await sendTxUntilInBlock(context.substrate, tx, signer);

    if (listening) {
        const event = await listenEncryptedEvents(context, aesKey, {
            module: 'identityManagement',
            method: 'identityRemoved',
            event: 'IdentityRemoved',
        });
        const [who, identity, idGraph] = event.eventData;
        return decodeIdentityEvent(context.substrate, who, identity, idGraph);
    }
    return undefined;
}

export async function verifyIdentity(
    context: IntegrationTestContext,
    signer: KeyringPair,
    aesKey: HexString,
    listening: boolean,
    identity: LitentryIdentity,
    data: LitentryValidationData
): Promise<IdentityGenericEvent | undefined> {
    const identity_encode = context.substrate.createType('LitentryIdentity', identity).toHex();
    const validation_encode = context.substrate.createType('LitentryValidationData', data).toHex();
    const identity_ciphertext = encryptWithTeeShieldingKey(context.teeShieldingKey, identity_encode).toString('hex');
    const validation_ciphertext = encryptWithTeeShieldingKey(context.teeShieldingKey, validation_encode).toString(
        'hex'
    );
 

    const tx = context.substrate.tx.identityManagement.verifyIdentity(
        context.shard,
        `0x${identity_ciphertext}`,
        `0x${validation_ciphertext}`
    );

    //The purpose of paymentInfo is to check whether the version of polkadot/api is suitable for the current test and to determine whether the transaction is successful.
    await tx.paymentInfo(signer);
    
    await sendTxUntilInBlock(context.substrate, tx, signer);

    if (listening) {
        const event = await listenEncryptedEvents(context, aesKey, {
            module: 'identityManagement',
            method: 'identityVerified',
            event: 'IdentityVerified',
        });
        const [who, identity, idGraph] = event.eventData;

        return decodeIdentityEvent(context.substrate, who, identity, idGraph);
    }
    return undefined;
}
export async function requestVC(
    context: IntegrationTestContext,
    signer: KeyringPair,
    aesKey: HexString,
    listening: boolean,
    shard: HexString,
    assertion: Assertion
): Promise<HexString | undefined> {
    const tx = context.substrate.tx.vcManagement.requestVc(shard, assertion);
     //The purpose of paymentInfo is to check whether the version of polkadot/api is suitable for the current test and to determine whether the transaction is successful.
    await tx.paymentInfo(signer);
    
    await sendTxUntilInBlock(context.substrate, tx, signer);
    if (listening) {
        const event = await listenEncryptedEvents(context, aesKey, {
            module: 'vcManagement',
            method: 'vcIssued',
            event: 'VCIssued',
        });

        const [who, hash, vc] = event.eventData;
        return vc;
    }
    return undefined;
}
function decodeIdentityEvent(
    api: ApiPromise,
    who: HexString,
    identityString: HexString,
    idGraphString: HexString,
    challengeCode?: HexString
): IdentityGenericEvent {
    let identity = api.createType('LitentryIdentity', identityString).toJSON();
    let idGraph = api.createType('Vec<(LitentryIdentity, IdentityContext)>', idGraphString).toJSON();
    return <IdentityGenericEvent>{
        who,
        identity,
        idGraph,
        challengeCode,
    };
}
