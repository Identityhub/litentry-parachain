// Auto-generated via `yarn polkadot-types-from-defs`, do not edit
/* eslint-disable */

import type { Bytes, Enum, Struct, Text, U8aFixed, bool, u32 } from "@polkadot/types-codec";
import type { ITuple } from "@polkadot/types-codec/types";
import type { H256, Index } from "@polkadot/types/interfaces/runtime";
import type { LitentryIdentity } from "parachain-api/interfaces/identity";
import type { TrustedGetterSigned, TrustedOperationStatus } from "parachain-api/interfaces/trusted_operations";
import type { Assertion } from "parachain-api/interfaces/vc";

/** @name AesOutput */
export interface AesOutput extends Struct {
    readonly ciphertext: Bytes;
    readonly aad: Bytes;
    readonly nonce: U8aFixed;
}

/** @name AesRequest */
export interface AesRequest extends Struct {
    readonly shard: ShardIdentifier;
    readonly key: Bytes;
    readonly payload: AesOutput;
}

/** @name DirectRequestStatus */
export interface DirectRequestStatus extends Enum {
    readonly isOk: boolean;
    readonly isTrustedOperationStatus: boolean;
    readonly asTrustedOperationStatus: ITuple<[TrustedOperationStatus, H256]>;
    readonly isError: boolean;
    readonly type: "Ok" | "TrustedOperationStatus" | "Error";
}

/** @name ErrorDetail */
export interface ErrorDetail extends Enum {
    readonly isImportError: boolean;
    readonly isUnauthorizedSigner: boolean;
    readonly isStfError: boolean;
    readonly asStfError: Bytes;
    readonly isSendStfRequestFailed: boolean;
    readonly isParseError: boolean;
    readonly isDataProviderError: boolean;
    readonly asDataProviderError: Bytes;
    readonly isInvalidIdentity: boolean;
    readonly isWrongWeb2Handle: boolean;
    readonly isUnexpectedMessage: boolean;
    readonly isVerifyWeb3SignatureFailed: boolean;
    readonly isNoEligibleIdentity: boolean;
    readonly type:
        | "ImportError"
        | "UnauthorizedSigner"
        | "StfError"
        | "SendStfRequestFailed"
        | "ParseError"
        | "DataProviderError"
        | "InvalidIdentity"
        | "WrongWeb2Handle"
        | "UnexpectedMessage"
        | "VerifyWeb3SignatureFailed"
        | "NoEligibleIdentity";
}

/** @name Getter */
export interface Getter extends Enum {
    readonly isPublic: boolean;
    readonly asPublic: PublicGetter;
    readonly isTrusted: boolean;
    readonly asTrusted: TrustedGetterSigned;
    readonly type: "Public" | "Trusted";
}

/** @name PublicGetter */
export interface PublicGetter extends Enum {
    readonly isSomeValue: boolean;
    readonly asSomeValue: u32;
    readonly isNonce: boolean;
    readonly asNonce: LitentryIdentity;
    readonly isIdGraphHash: boolean;
    readonly asIdGraphHash: LitentryIdentity;
    readonly type: "SomeValue" | "Nonce" | "IdGraphHash";
}

/** @name RequestAesKey */
export interface RequestAesKey extends U8aFixed {}

/** @name RsaRequest */
export interface RsaRequest extends Struct {
    readonly shard: ShardIdentifier;
    readonly payload: Bytes;
}

/** @name ShardIdentifier */
export interface ShardIdentifier extends H256 {}

/** @name StfError */
export interface StfError extends Enum {
    readonly isLinkIdentityFailed: boolean;
    readonly asLinkIdentityFailed: ErrorDetail;
    readonly isDeactivateIdentityFailed: boolean;
    readonly asDeactivateIdentityFailed: ErrorDetail;
    readonly isActivateIdentityFailed: boolean;
    readonly asActivateIdentityFailed: ErrorDetail;
    readonly isRequestVCFailed: boolean;
    readonly asRequestVCFailed: ITuple<[Assertion, ErrorDetail]>;
    readonly isSetScheduledMrEnclaveFailed: boolean;
    readonly isSetIdentityNetworksFailed: boolean;
    readonly asSetIdentityNetworksFailed: ErrorDetail;
    readonly isInvalidAccount: boolean;
    readonly isUnclassifiedError: boolean;
    readonly isRemoveIdentityFailed: boolean;
    readonly asRemoveIdentityFailed: ErrorDetail;
    readonly isEmptyIDGraph: boolean;
    readonly isMissingPrivileges: boolean;
    readonly asMissingPrivileges: LitentryIdentity;
    readonly isRequireEnclaveSignerAccount: boolean;
    readonly isDispatch: boolean;
    readonly asDispatch: Text;
    readonly isMissingFunds: boolean;
    readonly isInvalidNonce: boolean;
    readonly asInvalidNonce: ITuple<[Index, Index]>;
    readonly isStorageHashMismatch: boolean;
    readonly isInvalidStorageDiff: boolean;
    readonly isInvalidMetadata: boolean;
    readonly type:
        | "LinkIdentityFailed"
        | "DeactivateIdentityFailed"
        | "ActivateIdentityFailed"
        | "RequestVCFailed"
        | "SetScheduledMrEnclaveFailed"
        | "SetIdentityNetworksFailed"
        | "InvalidAccount"
        | "UnclassifiedError"
        | "RemoveIdentityFailed"
        | "EmptyIDGraph"
        | "MissingPrivileges"
        | "RequireEnclaveSignerAccount"
        | "Dispatch"
        | "MissingFunds"
        | "InvalidNonce"
        | "StorageHashMismatch"
        | "InvalidStorageDiff"
        | "InvalidMetadata";
}

/** @name WorkerRpcReturnValue */
export interface WorkerRpcReturnValue extends Struct {
    readonly value: Bytes;
    readonly do_watch: bool;
    readonly status: DirectRequestStatus;
}

export type PHANTOM_SIDECHAIN = "sidechain";
