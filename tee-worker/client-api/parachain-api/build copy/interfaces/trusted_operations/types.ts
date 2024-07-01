// Auto-generated via `yarn polkadot-types-from-defs`, do not edit
/* eslint-disable */

import type { Bytes, Enum, Option, Struct, Vec, u32 } from "@polkadot/types-codec";
import type { ITuple } from "@polkadot/types-codec/types";
import type { H256 } from "@polkadot/types/interfaces/runtime";
import type { LitentryIdentity, LitentryMultiSignature, LitentryValidationData, Web3Network } from "parachain-api/interfaces/identity";
import type { Getter, RequestAesKey } from "parachain-api/interfaces/sidechain";
import type { Assertion } from "parachain-api/interfaces/vc";

/** @name TrustedCall */
export interface TrustedCall extends Enum {
    readonly isLinkIdentity: boolean;
    readonly asLinkIdentity: ITuple<
        [LitentryIdentity, LitentryIdentity, LitentryIdentity, LitentryValidationData, Vec<Web3Network>, Option<RequestAesKey>, H256]
    >;
    readonly isDeactivateIdentity: boolean;
    readonly asDeactivateIdentity: ITuple<[LitentryIdentity, LitentryIdentity, LitentryIdentity, Option<RequestAesKey>, H256]>;
    readonly isActivateIdentity: boolean;
    readonly asActivateIdentity: ITuple<[LitentryIdentity, LitentryIdentity, LitentryIdentity, Option<RequestAesKey>, H256]>;
    readonly isRequestVc: boolean;
    readonly asRequestVc: ITuple<[LitentryIdentity, LitentryIdentity, Assertion, Option<RequestAesKey>, H256]>;
    readonly isSetIdentityNetworks: boolean;
    readonly asSetIdentityNetworks: ITuple<
        [LitentryIdentity, LitentryIdentity, LitentryIdentity, Vec<Web3Network>, Option<RequestAesKey>, H256]
    >;
    readonly isRequestBatchVc: boolean;
    readonly asRequestBatchVc: ITuple<[LitentryIdentity, LitentryIdentity, Vec<Assertion>, Option<RequestAesKey>, H256]>;
    readonly type: "LinkIdentity" | "DeactivateIdentity" | "ActivateIdentity" | "RequestVc" | "SetIdentityNetworks" | "RequestBatchVc";
}

/** @name TrustedCallSigned */
export interface TrustedCallSigned extends Struct {
    readonly call: TrustedCall;
    readonly index: u32;
    readonly signature: LitentryMultiSignature;
}

/** @name TrustedGetter */
export interface TrustedGetter extends Enum {
    readonly isFreeBalance: boolean;
    readonly asFreeBalance: LitentryIdentity;
    readonly isReservedBalance: boolean;
    readonly asReservedBalance: LitentryIdentity;
    readonly isIdGraph: boolean;
    readonly asIdGraph: LitentryIdentity;
    readonly type: "FreeBalance" | "ReservedBalance" | "IdGraph";
}

/** @name TrustedGetterSigned */
export interface TrustedGetterSigned extends Struct {
    readonly getter: TrustedGetter;
    readonly signature: LitentryMultiSignature;
}

/** @name TrustedOperation */
export interface TrustedOperation extends Enum {
    readonly isIndirectCall: boolean;
    readonly asIndirectCall: TrustedCallSigned;
    readonly isDirectCall: boolean;
    readonly asDirectCall: TrustedCallSigned;
    readonly isGet: boolean;
    readonly asGet: Getter;
    readonly type: "IndirectCall" | "DirectCall" | "Get";
}

/** @name TrustedOperationStatus */
export interface TrustedOperationStatus extends Enum {
    readonly isSubmitted: boolean;
    readonly isFuture: boolean;
    readonly isReady: boolean;
    readonly isBroadcast: boolean;
    readonly isInSidechainBlock: boolean;
    readonly asInSidechainBlock: H256;
    readonly isRetracted: boolean;
    readonly isFinalityTimeout: boolean;
    readonly isFinalized: boolean;
    readonly isUsurped: boolean;
    readonly isDropped: boolean;
    readonly isInvalid: boolean;
    readonly isTopExecuted: boolean;
    readonly asTopExecuted: Bytes;
    readonly type:
        | "Submitted"
        | "Future"
        | "Ready"
        | "Broadcast"
        | "InSidechainBlock"
        | "Retracted"
        | "FinalityTimeout"
        | "Finalized"
        | "Usurped"
        | "Dropped"
        | "Invalid"
        | "TopExecuted";
}

export type PHANTOM_TRUSTED_OPERATIONS = "trusted_operations";
