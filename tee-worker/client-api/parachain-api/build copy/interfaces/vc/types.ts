// Auto-generated via `yarn polkadot-types-from-defs`, do not edit
/* eslint-disable */

import type { Bytes, Enum, Option, Struct, U8aFixed, Vec, bool, u8 } from "@polkadot/types-codec";
import type { ITuple } from "@polkadot/types-codec/types";
import type { AccountId, AccountId32, H256 } from "@polkadot/types/interfaces/runtime";
import type { Web3Network } from "parachain-api/interfaces/identity";
import type { AesOutput, ShardIdentifier } from "parachain-api/interfaces/sidechain";

/** @name AchainableAmount */
export interface AchainableAmount extends Struct {
    readonly name: Bytes;
    readonly chain: Vec<Web3Network>;
    readonly amount: Bytes;
}

/** @name AchainableAmountHolding */
export interface AchainableAmountHolding extends Struct {
    readonly name: Bytes;
    readonly chain: Vec<Web3Network>;
    readonly amount: Bytes;
    readonly date: Bytes;
    readonly token: Option<Bytes>;
}

/** @name AchainableAmounts */
export interface AchainableAmounts extends Struct {
    readonly name: Bytes;
    readonly chain: Vec<Web3Network>;
    readonly amount1: Bytes;
    readonly amount2: Bytes;
}

/** @name AchainableAmountToken */
export interface AchainableAmountToken extends Struct {
    readonly name: Bytes;
    readonly chain: Vec<Web3Network>;
    readonly amount: Bytes;
    readonly token: Option<Bytes>;
}

/** @name AchainableBasic */
export interface AchainableBasic extends Struct {
    readonly name: Bytes;
    readonly chain: Vec<Web3Network>;
}

/** @name AchainableBetweenPercents */
export interface AchainableBetweenPercents extends Struct {
    readonly name: Bytes;
    readonly chain: Vec<Web3Network>;
    readonly greaterThanOrEqualTo: Bytes;
    readonly lessThanOrEqualTo: Bytes;
}

/** @name AchainableClassOfYear */
export interface AchainableClassOfYear extends Struct {
    readonly name: Bytes;
    readonly chain: Vec<Web3Network>;
}

/** @name AchainableDate */
export interface AchainableDate extends Struct {
    readonly name: Bytes;
    readonly chain: Vec<Web3Network>;
    readonly date: Bytes;
}

/** @name AchainableDateInterval */
export interface AchainableDateInterval extends Struct {
    readonly name: Bytes;
    readonly chain: Vec<Web3Network>;
    readonly startDate: Bytes;
    readonly endDate: Bytes;
}

/** @name AchainableDatePercent */
export interface AchainableDatePercent extends Struct {
    readonly name: Bytes;
    readonly chain: Vec<Web3Network>;
    readonly token: Bytes;
    readonly date: Bytes;
    readonly percent: Bytes;
}

/** @name AchainableParams */
export interface AchainableParams extends Enum {
    readonly isAmountHolding: boolean;
    readonly asAmountHolding: AchainableAmountHolding;
    readonly isAmountToken: boolean;
    readonly asAmountToken: AchainableAmountToken;
    readonly isAmount: boolean;
    readonly asAmount: AchainableAmount;
    readonly isAmounts: boolean;
    readonly asAmounts: AchainableAmounts;
    readonly isBasic: boolean;
    readonly asBasic: AchainableBasic;
    readonly isBetweenPercents: boolean;
    readonly asBetweenPercents: AchainableBetweenPercents;
    readonly isClassOfYear: boolean;
    readonly asClassOfYear: AchainableClassOfYear;
    readonly isDateInterval: boolean;
    readonly asDateInterval: AchainableDateInterval;
    readonly isDatePercent: boolean;
    readonly asDatePercent: AchainableDatePercent;
    readonly isDate: boolean;
    readonly asDate: AchainableDate;
    readonly isToken: boolean;
    readonly asToken: AchainableToken;
    readonly type:
        | "AmountHolding"
        | "AmountToken"
        | "Amount"
        | "Amounts"
        | "Basic"
        | "BetweenPercents"
        | "ClassOfYear"
        | "DateInterval"
        | "DatePercent"
        | "Date"
        | "Token";
}

/** @name AchainableToken */
export interface AchainableToken extends Struct {
    readonly name: Bytes;
    readonly chain: Vec<Web3Network>;
    readonly token: Bytes;
}

/** @name Assertion */
export interface Assertion extends Enum {
    readonly isA1: boolean;
    readonly isA2: boolean;
    readonly asA2: Bytes;
    readonly isA3: boolean;
    readonly asA3: ITuple<[Bytes, Bytes, Bytes]>;
    readonly isA4: boolean;
    readonly asA4: Bytes;
    readonly isA6: boolean;
    readonly isA7: boolean;
    readonly asA7: Bytes;
    readonly isA8: boolean;
    readonly asA8: Vec<AssertionSupportedNetwork>;
    readonly isA10: boolean;
    readonly asA10: Bytes;
    readonly isA11: boolean;
    readonly asA11: Bytes;
    readonly isA13: boolean;
    readonly asA13: AccountId32;
    readonly isA14: boolean;
    readonly isAchainable: boolean;
    readonly asAchainable: AchainableParams;
    readonly isA20: boolean;
    readonly isOneBlock: boolean;
    readonly asOneBlock: OneBlockCourseType;
    readonly isGenericDiscordRole: boolean;
    readonly asGenericDiscordRole: GenericDiscordRoleType;
    readonly isBnbDomainHolding: boolean;
    readonly isBnbDigitDomainClub: boolean;
    readonly asBnbDigitDomainClub: BnbDigitDomainType;
    readonly isVip3MembershipCard: boolean;
    readonly asVip3MembershipCard: VIP3MembershipCardLevel;
    readonly isWeirdoGhostGangHolder: boolean;
    readonly isLitStaking: boolean;
    readonly isEvmAmountHolding: boolean;
    readonly asEvmAmountHolding: EVMTokenType;
    readonly isBrc20AmountHolder: boolean;
    readonly isCryptoSummary: boolean;
    readonly isTokenHoldingAmount: boolean;
    readonly asTokenHoldingAmount: Web3TokenType;
    readonly isPlatformUser: boolean;
    readonly asPlatformUser: PlatformUserType;
    readonly isNftHolder: boolean;
    readonly asNftHolder: Web3NftType;
    readonly isDynamic: boolean;
    readonly asDynamic: ITuple<[U8aFixed, Bytes]>;
    readonly type:
        | "A1"
        | "A2"
        | "A3"
        | "A4"
        | "A6"
        | "A7"
        | "A8"
        | "A10"
        | "A11"
        | "A13"
        | "A14"
        | "Achainable"
        | "A20"
        | "OneBlock"
        | "GenericDiscordRole"
        | "BnbDomainHolding"
        | "BnbDigitDomainClub"
        | "Vip3MembershipCard"
        | "WeirdoGhostGangHolder"
        | "LitStaking"
        | "EvmAmountHolding"
        | "Brc20AmountHolder"
        | "CryptoSummary"
        | "TokenHoldingAmount"
        | "PlatformUser"
        | "NftHolder"
        | "Dynamic";
}

/** @name AssertionSupportedNetwork */
export interface AssertionSupportedNetwork extends Enum {
    readonly isLitentry: boolean;
    readonly isLitmus: boolean;
    readonly isLitentryRococo: boolean;
    readonly isPolkadot: boolean;
    readonly isKusama: boolean;
    readonly isKhala: boolean;
    readonly isEthereum: boolean;
    readonly isTestNet: boolean;
    readonly type: "Litentry" | "Litmus" | "LitentryRococo" | "Polkadot" | "Kusama" | "Khala" | "Ethereum" | "TestNet";
}

/** @name BnbDigitDomainType */
export interface BnbDigitDomainType extends Enum {
    readonly isBnb999ClubMember: boolean;
    readonly isBnb10kClubMember: boolean;
    readonly type: "Bnb999ClubMember" | "Bnb10kClubMember";
}

/** @name EVMTokenType */
export interface EVMTokenType extends Enum {
    readonly isTon: boolean;
    readonly isTrx: boolean;
    readonly type: "Ton" | "Trx";
}

/** @name GenericDiscordRoleContestType */
export interface GenericDiscordRoleContestType extends Enum {
    readonly isLegend: boolean;
    readonly isPopularity: boolean;
    readonly isParticipant: boolean;
    readonly type: "Legend" | "Popularity" | "Participant";
}

/** @name GenericDiscordRoleSoraQuizType */
export interface GenericDiscordRoleSoraQuizType extends Enum {
    readonly isAttendee: boolean;
    readonly isMaster: boolean;
    readonly type: "Attendee" | "Master";
}

/** @name GenericDiscordRoleType */
export interface GenericDiscordRoleType extends Enum {
    readonly isContest: boolean;
    readonly asContest: GenericDiscordRoleContestType;
    readonly isSoraQuiz: boolean;
    readonly asSoraQuiz: GenericDiscordRoleSoraQuizType;
    readonly type: "Contest" | "SoraQuiz";
}

/** @name OneBlockCourseType */
export interface OneBlockCourseType extends Enum {
    readonly isCourseCompletion: boolean;
    readonly isCourseOutstanding: boolean;
    readonly isCourseParticipation: boolean;
    readonly type: "CourseCompletion" | "CourseOutstanding" | "CourseParticipation";
}

/** @name PlatformUserType */
export interface PlatformUserType extends Enum {
    readonly isKaratDaoUser: boolean;
    readonly isMagicCraftStakingUser: boolean;
    readonly type: "KaratDaoUser" | "MagicCraftStakingUser";
}

/** @name RequestVCResult */
export interface RequestVCResult extends Struct {
    readonly vc_payload: AesOutput;
    readonly vc_logs: AesOutput;
    readonly pre_mutated_id_graph: AesOutput;
    readonly pre_id_graph_hash: H256;
}

/** @name RequestVcResultOrError */
export interface RequestVcResultOrError extends Struct {
    readonly payload: Bytes;
    readonly is_error: bool;
    readonly idx: u8;
    readonly len: u8;
}

/** @name VCRequested */
export interface VCRequested extends Struct {
    readonly account: AccountId;
    readonly mrEnclave: ShardIdentifier;
    readonly assertion: Assertion;
}

/** @name VIP3MembershipCardLevel */
export interface VIP3MembershipCardLevel extends Enum {
    readonly isGold: boolean;
    readonly isSilver: boolean;
    readonly type: "Gold" | "Silver";
}

/** @name Web3NftType */
export interface Web3NftType extends Enum {
    readonly isWeirdoGhostGang: boolean;
    readonly isClub3Sbt: boolean;
    readonly type: "WeirdoGhostGang" | "Club3Sbt";
}

/** @name Web3TokenType */
export interface Web3TokenType extends Enum {
    readonly isBnb: boolean;
    readonly isEth: boolean;
    readonly isSpaceId: boolean;
    readonly isLit: boolean;
    readonly isWbtc: boolean;
    readonly isUsdc: boolean;
    readonly isUsdt: boolean;
    readonly isCrv: boolean;
    readonly isMatic: boolean;
    readonly isDydx: boolean;
    readonly isAmp: boolean;
    readonly isCvx: boolean;
    readonly isTusd: boolean;
    readonly isUsdd: boolean;
    readonly isGusd: boolean;
    readonly isLink: boolean;
    readonly isGrt: boolean;
    readonly isComp: boolean;
    readonly isPeople: boolean;
    readonly isGtc: boolean;
    readonly isTon: boolean;
    readonly isTrx: boolean;
    readonly isNfp: boolean;
    readonly isSol: boolean;
    readonly isMcrt: boolean;
    readonly isBtc: boolean;
    readonly isAda: boolean;
    readonly isDoge: boolean;
    readonly isShib: boolean;
    readonly isUni: boolean;
    readonly isBch: boolean;
    readonly isEtc: boolean;
    readonly isAtom: boolean;
    readonly isDai: boolean;
    readonly isLeo: boolean;
    readonly isFil: boolean;
    readonly isImx: boolean;
    readonly isCro: boolean;
    readonly isInj: boolean;
    readonly isBean: boolean;
    readonly type:
        | "Bnb"
        | "Eth"
        | "SpaceId"
        | "Lit"
        | "Wbtc"
        | "Usdc"
        | "Usdt"
        | "Crv"
        | "Matic"
        | "Dydx"
        | "Amp"
        | "Cvx"
        | "Tusd"
        | "Usdd"
        | "Gusd"
        | "Link"
        | "Grt"
        | "Comp"
        | "People"
        | "Gtc"
        | "Ton"
        | "Trx"
        | "Nfp"
        | "Sol"
        | "Mcrt"
        | "Btc"
        | "Ada"
        | "Doge"
        | "Shib"
        | "Uni"
        | "Bch"
        | "Etc"
        | "Atom"
        | "Dai"
        | "Leo"
        | "Fil"
        | "Imx"
        | "Cro"
        | "Inj"
        | "Bean";
}

export type PHANTOM_VC = "vc";
