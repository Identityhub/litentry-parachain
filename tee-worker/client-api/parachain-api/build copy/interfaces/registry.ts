// Auto-generated via `yarn polkadot-types-from-defs`, do not edit
/* eslint-disable */

// import type lookup before we augment - in some environments
// this is required to allow for ambient/previous definitions
import "@polkadot/types/types/registry";

import type {
    CorePrimitivesAssertion,
    CorePrimitivesAssertionAchainableAchainableAmount,
    CorePrimitivesAssertionAchainableAchainableAmountHolding,
    CorePrimitivesAssertionAchainableAchainableAmountToken,
    CorePrimitivesAssertionAchainableAchainableAmounts,
    CorePrimitivesAssertionAchainableAchainableBasic,
    CorePrimitivesAssertionAchainableAchainableBetweenPercents,
    CorePrimitivesAssertionAchainableAchainableClassOfYear,
    CorePrimitivesAssertionAchainableAchainableDate,
    CorePrimitivesAssertionAchainableAchainableDateInterval,
    CorePrimitivesAssertionAchainableAchainableDatePercent,
    CorePrimitivesAssertionAchainableAchainableMirror,
    CorePrimitivesAssertionAchainableAchainableParams,
    CorePrimitivesAssertionAchainableAchainableToken,
    CorePrimitivesAssertionBnbDomainBnbDigitDomainType,
    CorePrimitivesAssertionContestContestType,
    CorePrimitivesAssertionEvmAmountHoldingEvmTokenType,
    CorePrimitivesAssertionGenericDiscordRoleGenericDiscordRoleType,
    CorePrimitivesAssertionNetworkWeb3Network,
    CorePrimitivesAssertionOneblockOneBlockCourseType,
    CorePrimitivesAssertionPlatformUserPlatformUserType,
    CorePrimitivesAssertionSoraquizSoraQuizType,
    CorePrimitivesAssertionVip3Vip3MembershipCardLevel,
    CorePrimitivesAssertionWeb3NftWeb3NftType,
    CorePrimitivesAssertionWeb3TokenWeb3TokenType,
    CorePrimitivesErrorErrorDetail,
    CorePrimitivesErrorImpError,
    CorePrimitivesErrorVcmpError,
    CorePrimitivesIdentity,
    CorePrimitivesIdentityAddress20,
    CorePrimitivesIdentityAddress32,
    CorePrimitivesIdentityAddress33,
    CumulusPalletDmpQueueCall,
    CumulusPalletDmpQueueConfigData,
    CumulusPalletDmpQueueError,
    CumulusPalletDmpQueueEvent,
    CumulusPalletDmpQueuePageIndexData,
    CumulusPalletParachainSystemCall,
    CumulusPalletParachainSystemCodeUpgradeAuthorization,
    CumulusPalletParachainSystemError,
    CumulusPalletParachainSystemEvent,
    CumulusPalletParachainSystemRelayStateSnapshotMessagingStateSnapshot,
    CumulusPalletXcmCall,
    CumulusPalletXcmError,
    CumulusPalletXcmEvent,
    CumulusPalletXcmOrigin,
    CumulusPalletXcmpQueueCall,
    CumulusPalletXcmpQueueError,
    CumulusPalletXcmpQueueEvent,
    CumulusPalletXcmpQueueInboundChannelDetails,
    CumulusPalletXcmpQueueInboundState,
    CumulusPalletXcmpQueueOutboundChannelDetails,
    CumulusPalletXcmpQueueOutboundState,
    CumulusPalletXcmpQueueQueueConfigData,
    CumulusPrimitivesParachainInherentParachainInherentData,
    EthbloomBloom,
    EthereumBlock,
    EthereumHeader,
    EthereumLog,
    EthereumReceiptEip658ReceiptData,
    EthereumReceiptReceiptV3,
    EthereumTransactionAccessListItem,
    EthereumTransactionEip1559Transaction,
    EthereumTransactionEip2930Transaction,
    EthereumTransactionLegacyTransaction,
    EthereumTransactionTransactionAction,
    EthereumTransactionTransactionSignature,
    EthereumTransactionTransactionV2,
    EthereumTypesHashH64,
    EvmCoreErrorExitError,
    EvmCoreErrorExitFatal,
    EvmCoreErrorExitReason,
    EvmCoreErrorExitRevert,
    EvmCoreErrorExitSucceed,
    FpRpcTransactionStatus,
    FrameSupportDispatchDispatchClass,
    FrameSupportDispatchDispatchInfo,
    FrameSupportDispatchPays,
    FrameSupportDispatchPerDispatchClassU32,
    FrameSupportDispatchPerDispatchClassWeight,
    FrameSupportDispatchPerDispatchClassWeightsPerClass,
    FrameSupportDispatchRawOrigin,
    FrameSupportPalletId,
    FrameSupportPreimagesBounded,
    FrameSupportTokensMiscBalanceStatus,
    FrameSystemAccountInfo,
    FrameSystemCall,
    FrameSystemError,
    FrameSystemEvent,
    FrameSystemEventRecord,
    FrameSystemExtensionsCheckGenesis,
    FrameSystemExtensionsCheckNonZeroSender,
    FrameSystemExtensionsCheckNonce,
    FrameSystemExtensionsCheckSpecVersion,
    FrameSystemExtensionsCheckTxVersion,
    FrameSystemExtensionsCheckWeight,
    FrameSystemLastRuntimeUpgradeInfo,
    FrameSystemLimitsBlockLength,
    FrameSystemLimitsBlockWeights,
    FrameSystemLimitsWeightsPerClass,
    FrameSystemPhase,
    OrmlTokensAccountData,
    OrmlTokensBalanceLock,
    OrmlTokensModuleCall,
    OrmlTokensModuleError,
    OrmlTokensModuleEvent,
    OrmlTokensReserveData,
    OrmlXtokensModuleCall,
    OrmlXtokensModuleError,
    OrmlXtokensModuleEvent,
    PalletAccountFixCall,
    PalletAssetManagerAssetMetadata,
    PalletAssetManagerCall,
    PalletAssetManagerError,
    PalletAssetManagerEvent,
    PalletBalancesAccountData,
    PalletBalancesBalanceLock,
    PalletBalancesCall,
    PalletBalancesError,
    PalletBalancesEvent,
    PalletBalancesIdAmount,
    PalletBalancesReasons,
    PalletBalancesReserveData,
    PalletBitacrossCall,
    PalletBitacrossCustodialWallet,
    PalletBitacrossError,
    PalletBitacrossEvent,
    PalletBitacrossMimicCall,
    PalletBitacrossMimicCustodialTypeBtcToEth,
    PalletBitacrossMimicCustodialTypeEthToBtc,
    PalletBitacrossMimicEvent,
    PalletBountiesBounty,
    PalletBountiesBountyStatus,
    PalletBountiesCall,
    PalletBountiesError,
    PalletBountiesEvent,
    PalletBridgeBridgeEvent,
    PalletBridgeCall,
    PalletBridgeError,
    PalletBridgeEvent,
    PalletBridgeProposalStatus,
    PalletBridgeProposalVotes,
    PalletBridgeTransferCall,
    PalletBridgeTransferError,
    PalletBridgeTransferEvent,
    PalletCollectiveCall,
    PalletCollectiveError,
    PalletCollectiveEvent,
    PalletCollectiveRawOrigin,
    PalletCollectiveVotes,
    PalletDemocracyCall,
    PalletDemocracyConviction,
    PalletDemocracyDelegations,
    PalletDemocracyError,
    PalletDemocracyEvent,
    PalletDemocracyMetadataOwner,
    PalletDemocracyReferendumInfo,
    PalletDemocracyReferendumStatus,
    PalletDemocracyTally,
    PalletDemocracyVoteAccountVote,
    PalletDemocracyVotePriorLock,
    PalletDemocracyVoteThreshold,
    PalletDemocracyVoteVoting,
    PalletEthereumCall,
    PalletEthereumError,
    PalletEthereumEvent,
    PalletEthereumRawOrigin,
    PalletEvmAssertionsAssertion,
    PalletEvmAssertionsCall,
    PalletEvmAssertionsError,
    PalletEvmAssertionsEvent,
    PalletEvmCall,
    PalletEvmCodeMetadata,
    PalletEvmError,
    PalletEvmEvent,
    PalletExtrinsicFilterCall,
    PalletExtrinsicFilterError,
    PalletExtrinsicFilterEvent,
    PalletExtrinsicFilterOperationalMode,
    PalletGroupCall,
    PalletGroupError,
    PalletGroupEvent,
    PalletIdentityBitFlags,
    PalletIdentityCall,
    PalletIdentityError,
    PalletIdentityEvent,
    PalletIdentityIdentityField,
    PalletIdentityIdentityInfo,
    PalletIdentityJudgement,
    PalletIdentityManagementCall,
    PalletIdentityManagementError,
    PalletIdentityManagementEvent,
    PalletIdentityRegistrarInfo,
    PalletIdentityRegistration,
    PalletMembershipCall,
    PalletMembershipError,
    PalletMembershipEvent,
    PalletMultisigCall,
    PalletMultisigError,
    PalletMultisigEvent,
    PalletMultisigMultisig,
    PalletMultisigTimepoint,
    PalletParachainStakingAutoCompoundAutoCompoundConfig,
    PalletParachainStakingBond,
    PalletParachainStakingBondWithAutoCompound,
    PalletParachainStakingCall,
    PalletParachainStakingCandidateBondLessRequest,
    PalletParachainStakingCandidateMetadata,
    PalletParachainStakingCapacityStatus,
    PalletParachainStakingCollatorSnapshot,
    PalletParachainStakingCollatorStatus,
    PalletParachainStakingDelayedPayout,
    PalletParachainStakingDelegationRequestsCancelledScheduledRequest,
    PalletParachainStakingDelegationRequestsDelegationAction,
    PalletParachainStakingDelegationRequestsScheduledRequest,
    PalletParachainStakingDelegations,
    PalletParachainStakingDelegator,
    PalletParachainStakingDelegatorAdded,
    PalletParachainStakingDelegatorStatus,
    PalletParachainStakingError,
    PalletParachainStakingEvent,
    PalletParachainStakingInflationInflationInfo,
    PalletParachainStakingParachainBondConfig,
    PalletParachainStakingRoundInfo,
    PalletParachainStakingSetOrderedSet,
    PalletPreimageCall,
    PalletPreimageError,
    PalletPreimageEvent,
    PalletPreimageRequestStatus,
    PalletProxyAnnouncement,
    PalletProxyCall,
    PalletProxyError,
    PalletProxyEvent,
    PalletProxyProxyDefinition,
    PalletSchedulerCall,
    PalletSchedulerError,
    PalletSchedulerEvent,
    PalletSchedulerScheduled,
    PalletSessionCall,
    PalletSessionError,
    PalletSessionEvent,
    PalletSudoCall,
    PalletSudoError,
    PalletSudoEvent,
    PalletTeebagAttestationType,
    PalletTeebagCall,
    PalletTeebagDcapProvider,
    PalletTeebagEnclave,
    PalletTeebagError,
    PalletTeebagEvent,
    PalletTeebagOperationalMode,
    PalletTeebagQuotingEnclave,
    PalletTeebagRsaRequest,
    PalletTeebagSgxBuildMode,
    PalletTeebagSidechainBlockConfirmation,
    PalletTeebagTcbQeTcb,
    PalletTeebagTcbTcbInfoOnChain,
    PalletTeebagTcbTcbVersionStatus,
    PalletTeebagWorkerMode,
    PalletTeebagWorkerType,
    PalletTimestampCall,
    PalletTipsCall,
    PalletTipsError,
    PalletTipsEvent,
    PalletTipsOpenTip,
    PalletTransactionPaymentChargeTransactionPayment,
    PalletTransactionPaymentEvent,
    PalletTransactionPaymentReleases,
    PalletTreasuryCall,
    PalletTreasuryError,
    PalletTreasuryEvent,
    PalletTreasuryProposal,
    PalletUtilityCall,
    PalletUtilityError,
    PalletUtilityEvent,
    PalletVcManagementCall,
    PalletVcManagementError,
    PalletVcManagementEvent,
    PalletVcManagementSchemaStatus,
    PalletVcManagementSchemaVcSchema,
    PalletVestingCall,
    PalletVestingError,
    PalletVestingEvent,
    PalletVestingReleases,
    PalletVestingVestingInfo,
    PalletXcmCall,
    PalletXcmError,
    PalletXcmEvent,
    PalletXcmOrigin,
    PalletXcmQueryStatus,
    PalletXcmRemoteLockedFungibleRecord,
    PalletXcmVersionMigrationStage,
    ParachainInfoCall,
    PolkadotCorePrimitivesInboundDownwardMessage,
    PolkadotCorePrimitivesInboundHrmpMessage,
    PolkadotCorePrimitivesOutboundHrmpMessage,
    PolkadotParachainPrimitivesXcmpMessageFormat,
    PolkadotPrimitivesV4AbridgedHostConfiguration,
    PolkadotPrimitivesV4AbridgedHrmpChannel,
    PolkadotPrimitivesV4PersistedValidationData,
    PolkadotPrimitivesV4UpgradeRestriction,
    RococoParachainRuntimeOriginCaller,
    RococoParachainRuntimeProxyType,
    RococoParachainRuntimeRuntime,
    RococoParachainRuntimeSessionKeys,
    RuntimeCommonXcmImplCurrencyId,
    SpArithmeticArithmeticError,
    SpConsensusAuraSr25519AppSr25519Public,
    SpCoreCryptoKeyTypeId,
    SpCoreEcdsaSignature,
    SpCoreEd25519Public,
    SpCoreEd25519Signature,
    SpCoreSr25519Public,
    SpCoreSr25519Signature,
    SpCoreVoid,
    SpRuntimeDigest,
    SpRuntimeDigestDigestItem,
    SpRuntimeDispatchError,
    SpRuntimeModuleError,
    SpRuntimeMultiSignature,
    SpRuntimeTokenError,
    SpRuntimeTransactionalError,
    SpTrieStorageProof,
    SpVersionRuntimeVersion,
    SpWeightsRuntimeDbWeight,
    SpWeightsWeightV2Weight,
    XcmDoubleEncoded,
    XcmV2BodyId,
    XcmV2BodyPart,
    XcmV2Instruction,
    XcmV2Junction,
    XcmV2MultiAsset,
    XcmV2MultiLocation,
    XcmV2MultiassetAssetId,
    XcmV2MultiassetAssetInstance,
    XcmV2MultiassetFungibility,
    XcmV2MultiassetMultiAssetFilter,
    XcmV2MultiassetMultiAssets,
    XcmV2MultiassetWildFungibility,
    XcmV2MultiassetWildMultiAsset,
    XcmV2MultilocationJunctions,
    XcmV2NetworkId,
    XcmV2OriginKind,
    XcmV2Response,
    XcmV2TraitsError,
    XcmV2WeightLimit,
    XcmV2Xcm,
    XcmV3Instruction,
    XcmV3Junction,
    XcmV3JunctionBodyId,
    XcmV3JunctionBodyPart,
    XcmV3JunctionNetworkId,
    XcmV3Junctions,
    XcmV3MaybeErrorCode,
    XcmV3MultiAsset,
    XcmV3MultiLocation,
    XcmV3MultiassetAssetId,
    XcmV3MultiassetAssetInstance,
    XcmV3MultiassetFungibility,
    XcmV3MultiassetMultiAssetFilter,
    XcmV3MultiassetMultiAssets,
    XcmV3MultiassetWildFungibility,
    XcmV3MultiassetWildMultiAsset,
    XcmV3PalletInfo,
    XcmV3QueryResponseInfo,
    XcmV3Response,
    XcmV3TraitsError,
    XcmV3TraitsOutcome,
    XcmV3WeightLimit,
    XcmV3Xcm,
    XcmVersionedAssetId,
    XcmVersionedMultiAsset,
    XcmVersionedMultiAssets,
    XcmVersionedMultiLocation,
    XcmVersionedResponse,
    XcmVersionedXcm,
} from "@polkadot/types/lookup";

declare module "@polkadot/types/types/registry" {
    interface InterfaceTypes {
        CorePrimitivesAssertion: CorePrimitivesAssertion;
        CorePrimitivesAssertionAchainableAchainableAmount: CorePrimitivesAssertionAchainableAchainableAmount;
        CorePrimitivesAssertionAchainableAchainableAmountHolding: CorePrimitivesAssertionAchainableAchainableAmountHolding;
        CorePrimitivesAssertionAchainableAchainableAmountToken: CorePrimitivesAssertionAchainableAchainableAmountToken;
        CorePrimitivesAssertionAchainableAchainableAmounts: CorePrimitivesAssertionAchainableAchainableAmounts;
        CorePrimitivesAssertionAchainableAchainableBasic: CorePrimitivesAssertionAchainableAchainableBasic;
        CorePrimitivesAssertionAchainableAchainableBetweenPercents: CorePrimitivesAssertionAchainableAchainableBetweenPercents;
        CorePrimitivesAssertionAchainableAchainableClassOfYear: CorePrimitivesAssertionAchainableAchainableClassOfYear;
        CorePrimitivesAssertionAchainableAchainableDate: CorePrimitivesAssertionAchainableAchainableDate;
        CorePrimitivesAssertionAchainableAchainableDateInterval: CorePrimitivesAssertionAchainableAchainableDateInterval;
        CorePrimitivesAssertionAchainableAchainableDatePercent: CorePrimitivesAssertionAchainableAchainableDatePercent;
        CorePrimitivesAssertionAchainableAchainableMirror: CorePrimitivesAssertionAchainableAchainableMirror;
        CorePrimitivesAssertionAchainableAchainableParams: CorePrimitivesAssertionAchainableAchainableParams;
        CorePrimitivesAssertionAchainableAchainableToken: CorePrimitivesAssertionAchainableAchainableToken;
        CorePrimitivesAssertionBnbDomainBnbDigitDomainType: CorePrimitivesAssertionBnbDomainBnbDigitDomainType;
        CorePrimitivesAssertionContestContestType: CorePrimitivesAssertionContestContestType;
        CorePrimitivesAssertionEvmAmountHoldingEvmTokenType: CorePrimitivesAssertionEvmAmountHoldingEvmTokenType;
        CorePrimitivesAssertionGenericDiscordRoleGenericDiscordRoleType: CorePrimitivesAssertionGenericDiscordRoleGenericDiscordRoleType;
        CorePrimitivesAssertionNetworkWeb3Network: CorePrimitivesAssertionNetworkWeb3Network;
        CorePrimitivesAssertionOneblockOneBlockCourseType: CorePrimitivesAssertionOneblockOneBlockCourseType;
        CorePrimitivesAssertionPlatformUserPlatformUserType: CorePrimitivesAssertionPlatformUserPlatformUserType;
        CorePrimitivesAssertionSoraquizSoraQuizType: CorePrimitivesAssertionSoraquizSoraQuizType;
        CorePrimitivesAssertionVip3Vip3MembershipCardLevel: CorePrimitivesAssertionVip3Vip3MembershipCardLevel;
        CorePrimitivesAssertionWeb3NftWeb3NftType: CorePrimitivesAssertionWeb3NftWeb3NftType;
        CorePrimitivesAssertionWeb3TokenWeb3TokenType: CorePrimitivesAssertionWeb3TokenWeb3TokenType;
        CorePrimitivesErrorErrorDetail: CorePrimitivesErrorErrorDetail;
        CorePrimitivesErrorImpError: CorePrimitivesErrorImpError;
        CorePrimitivesErrorVcmpError: CorePrimitivesErrorVcmpError;
        CorePrimitivesIdentity: CorePrimitivesIdentity;
        CorePrimitivesIdentityAddress20: CorePrimitivesIdentityAddress20;
        CorePrimitivesIdentityAddress32: CorePrimitivesIdentityAddress32;
        CorePrimitivesIdentityAddress33: CorePrimitivesIdentityAddress33;
        CumulusPalletDmpQueueCall: CumulusPalletDmpQueueCall;
        CumulusPalletDmpQueueConfigData: CumulusPalletDmpQueueConfigData;
        CumulusPalletDmpQueueError: CumulusPalletDmpQueueError;
        CumulusPalletDmpQueueEvent: CumulusPalletDmpQueueEvent;
        CumulusPalletDmpQueuePageIndexData: CumulusPalletDmpQueuePageIndexData;
        CumulusPalletParachainSystemCall: CumulusPalletParachainSystemCall;
        CumulusPalletParachainSystemCodeUpgradeAuthorization: CumulusPalletParachainSystemCodeUpgradeAuthorization;
        CumulusPalletParachainSystemError: CumulusPalletParachainSystemError;
        CumulusPalletParachainSystemEvent: CumulusPalletParachainSystemEvent;
        CumulusPalletParachainSystemRelayStateSnapshotMessagingStateSnapshot: CumulusPalletParachainSystemRelayStateSnapshotMessagingStateSnapshot;
        CumulusPalletXcmCall: CumulusPalletXcmCall;
        CumulusPalletXcmError: CumulusPalletXcmError;
        CumulusPalletXcmEvent: CumulusPalletXcmEvent;
        CumulusPalletXcmOrigin: CumulusPalletXcmOrigin;
        CumulusPalletXcmpQueueCall: CumulusPalletXcmpQueueCall;
        CumulusPalletXcmpQueueError: CumulusPalletXcmpQueueError;
        CumulusPalletXcmpQueueEvent: CumulusPalletXcmpQueueEvent;
        CumulusPalletXcmpQueueInboundChannelDetails: CumulusPalletXcmpQueueInboundChannelDetails;
        CumulusPalletXcmpQueueInboundState: CumulusPalletXcmpQueueInboundState;
        CumulusPalletXcmpQueueOutboundChannelDetails: CumulusPalletXcmpQueueOutboundChannelDetails;
        CumulusPalletXcmpQueueOutboundState: CumulusPalletXcmpQueueOutboundState;
        CumulusPalletXcmpQueueQueueConfigData: CumulusPalletXcmpQueueQueueConfigData;
        CumulusPrimitivesParachainInherentParachainInherentData: CumulusPrimitivesParachainInherentParachainInherentData;
        EthbloomBloom: EthbloomBloom;
        EthereumBlock: EthereumBlock;
        EthereumHeader: EthereumHeader;
        EthereumLog: EthereumLog;
        EthereumReceiptEip658ReceiptData: EthereumReceiptEip658ReceiptData;
        EthereumReceiptReceiptV3: EthereumReceiptReceiptV3;
        EthereumTransactionAccessListItem: EthereumTransactionAccessListItem;
        EthereumTransactionEip1559Transaction: EthereumTransactionEip1559Transaction;
        EthereumTransactionEip2930Transaction: EthereumTransactionEip2930Transaction;
        EthereumTransactionLegacyTransaction: EthereumTransactionLegacyTransaction;
        EthereumTransactionTransactionAction: EthereumTransactionTransactionAction;
        EthereumTransactionTransactionSignature: EthereumTransactionTransactionSignature;
        EthereumTransactionTransactionV2: EthereumTransactionTransactionV2;
        EthereumTypesHashH64: EthereumTypesHashH64;
        EvmCoreErrorExitError: EvmCoreErrorExitError;
        EvmCoreErrorExitFatal: EvmCoreErrorExitFatal;
        EvmCoreErrorExitReason: EvmCoreErrorExitReason;
        EvmCoreErrorExitRevert: EvmCoreErrorExitRevert;
        EvmCoreErrorExitSucceed: EvmCoreErrorExitSucceed;
        FpRpcTransactionStatus: FpRpcTransactionStatus;
        FrameSupportDispatchDispatchClass: FrameSupportDispatchDispatchClass;
        FrameSupportDispatchDispatchInfo: FrameSupportDispatchDispatchInfo;
        FrameSupportDispatchPays: FrameSupportDispatchPays;
        FrameSupportDispatchPerDispatchClassU32: FrameSupportDispatchPerDispatchClassU32;
        FrameSupportDispatchPerDispatchClassWeight: FrameSupportDispatchPerDispatchClassWeight;
        FrameSupportDispatchPerDispatchClassWeightsPerClass: FrameSupportDispatchPerDispatchClassWeightsPerClass;
        FrameSupportDispatchRawOrigin: FrameSupportDispatchRawOrigin;
        FrameSupportPalletId: FrameSupportPalletId;
        FrameSupportPreimagesBounded: FrameSupportPreimagesBounded;
        FrameSupportTokensMiscBalanceStatus: FrameSupportTokensMiscBalanceStatus;
        FrameSystemAccountInfo: FrameSystemAccountInfo;
        FrameSystemCall: FrameSystemCall;
        FrameSystemError: FrameSystemError;
        FrameSystemEvent: FrameSystemEvent;
        FrameSystemEventRecord: FrameSystemEventRecord;
        FrameSystemExtensionsCheckGenesis: FrameSystemExtensionsCheckGenesis;
        FrameSystemExtensionsCheckNonZeroSender: FrameSystemExtensionsCheckNonZeroSender;
        FrameSystemExtensionsCheckNonce: FrameSystemExtensionsCheckNonce;
        FrameSystemExtensionsCheckSpecVersion: FrameSystemExtensionsCheckSpecVersion;
        FrameSystemExtensionsCheckTxVersion: FrameSystemExtensionsCheckTxVersion;
        FrameSystemExtensionsCheckWeight: FrameSystemExtensionsCheckWeight;
        FrameSystemLastRuntimeUpgradeInfo: FrameSystemLastRuntimeUpgradeInfo;
        FrameSystemLimitsBlockLength: FrameSystemLimitsBlockLength;
        FrameSystemLimitsBlockWeights: FrameSystemLimitsBlockWeights;
        FrameSystemLimitsWeightsPerClass: FrameSystemLimitsWeightsPerClass;
        FrameSystemPhase: FrameSystemPhase;
        OrmlTokensAccountData: OrmlTokensAccountData;
        OrmlTokensBalanceLock: OrmlTokensBalanceLock;
        OrmlTokensModuleCall: OrmlTokensModuleCall;
        OrmlTokensModuleError: OrmlTokensModuleError;
        OrmlTokensModuleEvent: OrmlTokensModuleEvent;
        OrmlTokensReserveData: OrmlTokensReserveData;
        OrmlXtokensModuleCall: OrmlXtokensModuleCall;
        OrmlXtokensModuleError: OrmlXtokensModuleError;
        OrmlXtokensModuleEvent: OrmlXtokensModuleEvent;
        PalletAccountFixCall: PalletAccountFixCall;
        PalletAssetManagerAssetMetadata: PalletAssetManagerAssetMetadata;
        PalletAssetManagerCall: PalletAssetManagerCall;
        PalletAssetManagerError: PalletAssetManagerError;
        PalletAssetManagerEvent: PalletAssetManagerEvent;
        PalletBalancesAccountData: PalletBalancesAccountData;
        PalletBalancesBalanceLock: PalletBalancesBalanceLock;
        PalletBalancesCall: PalletBalancesCall;
        PalletBalancesError: PalletBalancesError;
        PalletBalancesEvent: PalletBalancesEvent;
        PalletBalancesIdAmount: PalletBalancesIdAmount;
        PalletBalancesReasons: PalletBalancesReasons;
        PalletBalancesReserveData: PalletBalancesReserveData;
        PalletBitacrossCall: PalletBitacrossCall;
        PalletBitacrossCustodialWallet: PalletBitacrossCustodialWallet;
        PalletBitacrossError: PalletBitacrossError;
        PalletBitacrossEvent: PalletBitacrossEvent;
        PalletBitacrossMimicCall: PalletBitacrossMimicCall;
        PalletBitacrossMimicCustodialTypeBtcToEth: PalletBitacrossMimicCustodialTypeBtcToEth;
        PalletBitacrossMimicCustodialTypeEthToBtc: PalletBitacrossMimicCustodialTypeEthToBtc;
        PalletBitacrossMimicEvent: PalletBitacrossMimicEvent;
        PalletBountiesBounty: PalletBountiesBounty;
        PalletBountiesBountyStatus: PalletBountiesBountyStatus;
        PalletBountiesCall: PalletBountiesCall;
        PalletBountiesError: PalletBountiesError;
        PalletBountiesEvent: PalletBountiesEvent;
        PalletBridgeBridgeEvent: PalletBridgeBridgeEvent;
        PalletBridgeCall: PalletBridgeCall;
        PalletBridgeError: PalletBridgeError;
        PalletBridgeEvent: PalletBridgeEvent;
        PalletBridgeProposalStatus: PalletBridgeProposalStatus;
        PalletBridgeProposalVotes: PalletBridgeProposalVotes;
        PalletBridgeTransferCall: PalletBridgeTransferCall;
        PalletBridgeTransferError: PalletBridgeTransferError;
        PalletBridgeTransferEvent: PalletBridgeTransferEvent;
        PalletCollectiveCall: PalletCollectiveCall;
        PalletCollectiveError: PalletCollectiveError;
        PalletCollectiveEvent: PalletCollectiveEvent;
        PalletCollectiveRawOrigin: PalletCollectiveRawOrigin;
        PalletCollectiveVotes: PalletCollectiveVotes;
        PalletDemocracyCall: PalletDemocracyCall;
        PalletDemocracyConviction: PalletDemocracyConviction;
        PalletDemocracyDelegations: PalletDemocracyDelegations;
        PalletDemocracyError: PalletDemocracyError;
        PalletDemocracyEvent: PalletDemocracyEvent;
        PalletDemocracyMetadataOwner: PalletDemocracyMetadataOwner;
        PalletDemocracyReferendumInfo: PalletDemocracyReferendumInfo;
        PalletDemocracyReferendumStatus: PalletDemocracyReferendumStatus;
        PalletDemocracyTally: PalletDemocracyTally;
        PalletDemocracyVoteAccountVote: PalletDemocracyVoteAccountVote;
        PalletDemocracyVotePriorLock: PalletDemocracyVotePriorLock;
        PalletDemocracyVoteThreshold: PalletDemocracyVoteThreshold;
        PalletDemocracyVoteVoting: PalletDemocracyVoteVoting;
        PalletEthereumCall: PalletEthereumCall;
        PalletEthereumError: PalletEthereumError;
        PalletEthereumEvent: PalletEthereumEvent;
        PalletEthereumRawOrigin: PalletEthereumRawOrigin;
        PalletEvmAssertionsAssertion: PalletEvmAssertionsAssertion;
        PalletEvmAssertionsCall: PalletEvmAssertionsCall;
        PalletEvmAssertionsError: PalletEvmAssertionsError;
        PalletEvmAssertionsEvent: PalletEvmAssertionsEvent;
        PalletEvmCall: PalletEvmCall;
        PalletEvmCodeMetadata: PalletEvmCodeMetadata;
        PalletEvmError: PalletEvmError;
        PalletEvmEvent: PalletEvmEvent;
        PalletExtrinsicFilterCall: PalletExtrinsicFilterCall;
        PalletExtrinsicFilterError: PalletExtrinsicFilterError;
        PalletExtrinsicFilterEvent: PalletExtrinsicFilterEvent;
        PalletExtrinsicFilterOperationalMode: PalletExtrinsicFilterOperationalMode;
        PalletGroupCall: PalletGroupCall;
        PalletGroupError: PalletGroupError;
        PalletGroupEvent: PalletGroupEvent;
        PalletIdentityBitFlags: PalletIdentityBitFlags;
        PalletIdentityCall: PalletIdentityCall;
        PalletIdentityError: PalletIdentityError;
        PalletIdentityEvent: PalletIdentityEvent;
        PalletIdentityIdentityField: PalletIdentityIdentityField;
        PalletIdentityIdentityInfo: PalletIdentityIdentityInfo;
        PalletIdentityJudgement: PalletIdentityJudgement;
        PalletIdentityManagementCall: PalletIdentityManagementCall;
        PalletIdentityManagementError: PalletIdentityManagementError;
        PalletIdentityManagementEvent: PalletIdentityManagementEvent;
        PalletIdentityRegistrarInfo: PalletIdentityRegistrarInfo;
        PalletIdentityRegistration: PalletIdentityRegistration;
        PalletMembershipCall: PalletMembershipCall;
        PalletMembershipError: PalletMembershipError;
        PalletMembershipEvent: PalletMembershipEvent;
        PalletMultisigCall: PalletMultisigCall;
        PalletMultisigError: PalletMultisigError;
        PalletMultisigEvent: PalletMultisigEvent;
        PalletMultisigMultisig: PalletMultisigMultisig;
        PalletMultisigTimepoint: PalletMultisigTimepoint;
        PalletParachainStakingAutoCompoundAutoCompoundConfig: PalletParachainStakingAutoCompoundAutoCompoundConfig;
        PalletParachainStakingBond: PalletParachainStakingBond;
        PalletParachainStakingBondWithAutoCompound: PalletParachainStakingBondWithAutoCompound;
        PalletParachainStakingCall: PalletParachainStakingCall;
        PalletParachainStakingCandidateBondLessRequest: PalletParachainStakingCandidateBondLessRequest;
        PalletParachainStakingCandidateMetadata: PalletParachainStakingCandidateMetadata;
        PalletParachainStakingCapacityStatus: PalletParachainStakingCapacityStatus;
        PalletParachainStakingCollatorSnapshot: PalletParachainStakingCollatorSnapshot;
        PalletParachainStakingCollatorStatus: PalletParachainStakingCollatorStatus;
        PalletParachainStakingDelayedPayout: PalletParachainStakingDelayedPayout;
        PalletParachainStakingDelegationRequestsCancelledScheduledRequest: PalletParachainStakingDelegationRequestsCancelledScheduledRequest;
        PalletParachainStakingDelegationRequestsDelegationAction: PalletParachainStakingDelegationRequestsDelegationAction;
        PalletParachainStakingDelegationRequestsScheduledRequest: PalletParachainStakingDelegationRequestsScheduledRequest;
        PalletParachainStakingDelegations: PalletParachainStakingDelegations;
        PalletParachainStakingDelegator: PalletParachainStakingDelegator;
        PalletParachainStakingDelegatorAdded: PalletParachainStakingDelegatorAdded;
        PalletParachainStakingDelegatorStatus: PalletParachainStakingDelegatorStatus;
        PalletParachainStakingError: PalletParachainStakingError;
        PalletParachainStakingEvent: PalletParachainStakingEvent;
        PalletParachainStakingInflationInflationInfo: PalletParachainStakingInflationInflationInfo;
        PalletParachainStakingParachainBondConfig: PalletParachainStakingParachainBondConfig;
        PalletParachainStakingRoundInfo: PalletParachainStakingRoundInfo;
        PalletParachainStakingSetOrderedSet: PalletParachainStakingSetOrderedSet;
        PalletPreimageCall: PalletPreimageCall;
        PalletPreimageError: PalletPreimageError;
        PalletPreimageEvent: PalletPreimageEvent;
        PalletPreimageRequestStatus: PalletPreimageRequestStatus;
        PalletProxyAnnouncement: PalletProxyAnnouncement;
        PalletProxyCall: PalletProxyCall;
        PalletProxyError: PalletProxyError;
        PalletProxyEvent: PalletProxyEvent;
        PalletProxyProxyDefinition: PalletProxyProxyDefinition;
        PalletSchedulerCall: PalletSchedulerCall;
        PalletSchedulerError: PalletSchedulerError;
        PalletSchedulerEvent: PalletSchedulerEvent;
        PalletSchedulerScheduled: PalletSchedulerScheduled;
        PalletSessionCall: PalletSessionCall;
        PalletSessionError: PalletSessionError;
        PalletSessionEvent: PalletSessionEvent;
        PalletSudoCall: PalletSudoCall;
        PalletSudoError: PalletSudoError;
        PalletSudoEvent: PalletSudoEvent;
        PalletTeebagAttestationType: PalletTeebagAttestationType;
        PalletTeebagCall: PalletTeebagCall;
        PalletTeebagDcapProvider: PalletTeebagDcapProvider;
        PalletTeebagEnclave: PalletTeebagEnclave;
        PalletTeebagError: PalletTeebagError;
        PalletTeebagEvent: PalletTeebagEvent;
        PalletTeebagOperationalMode: PalletTeebagOperationalMode;
        PalletTeebagQuotingEnclave: PalletTeebagQuotingEnclave;
        PalletTeebagRsaRequest: PalletTeebagRsaRequest;
        PalletTeebagSgxBuildMode: PalletTeebagSgxBuildMode;
        PalletTeebagSidechainBlockConfirmation: PalletTeebagSidechainBlockConfirmation;
        PalletTeebagTcbQeTcb: PalletTeebagTcbQeTcb;
        PalletTeebagTcbTcbInfoOnChain: PalletTeebagTcbTcbInfoOnChain;
        PalletTeebagTcbTcbVersionStatus: PalletTeebagTcbTcbVersionStatus;
        PalletTeebagWorkerMode: PalletTeebagWorkerMode;
        PalletTeebagWorkerType: PalletTeebagWorkerType;
        PalletTimestampCall: PalletTimestampCall;
        PalletTipsCall: PalletTipsCall;
        PalletTipsError: PalletTipsError;
        PalletTipsEvent: PalletTipsEvent;
        PalletTipsOpenTip: PalletTipsOpenTip;
        PalletTransactionPaymentChargeTransactionPayment: PalletTransactionPaymentChargeTransactionPayment;
        PalletTransactionPaymentEvent: PalletTransactionPaymentEvent;
        PalletTransactionPaymentReleases: PalletTransactionPaymentReleases;
        PalletTreasuryCall: PalletTreasuryCall;
        PalletTreasuryError: PalletTreasuryError;
        PalletTreasuryEvent: PalletTreasuryEvent;
        PalletTreasuryProposal: PalletTreasuryProposal;
        PalletUtilityCall: PalletUtilityCall;
        PalletUtilityError: PalletUtilityError;
        PalletUtilityEvent: PalletUtilityEvent;
        PalletVcManagementCall: PalletVcManagementCall;
        PalletVcManagementError: PalletVcManagementError;
        PalletVcManagementEvent: PalletVcManagementEvent;
        PalletVcManagementSchemaStatus: PalletVcManagementSchemaStatus;
        PalletVcManagementSchemaVcSchema: PalletVcManagementSchemaVcSchema;
        PalletVestingCall: PalletVestingCall;
        PalletVestingError: PalletVestingError;
        PalletVestingEvent: PalletVestingEvent;
        PalletVestingReleases: PalletVestingReleases;
        PalletVestingVestingInfo: PalletVestingVestingInfo;
        PalletXcmCall: PalletXcmCall;
        PalletXcmError: PalletXcmError;
        PalletXcmEvent: PalletXcmEvent;
        PalletXcmOrigin: PalletXcmOrigin;
        PalletXcmQueryStatus: PalletXcmQueryStatus;
        PalletXcmRemoteLockedFungibleRecord: PalletXcmRemoteLockedFungibleRecord;
        PalletXcmVersionMigrationStage: PalletXcmVersionMigrationStage;
        ParachainInfoCall: ParachainInfoCall;
        PolkadotCorePrimitivesInboundDownwardMessage: PolkadotCorePrimitivesInboundDownwardMessage;
        PolkadotCorePrimitivesInboundHrmpMessage: PolkadotCorePrimitivesInboundHrmpMessage;
        PolkadotCorePrimitivesOutboundHrmpMessage: PolkadotCorePrimitivesOutboundHrmpMessage;
        PolkadotParachainPrimitivesXcmpMessageFormat: PolkadotParachainPrimitivesXcmpMessageFormat;
        PolkadotPrimitivesV4AbridgedHostConfiguration: PolkadotPrimitivesV4AbridgedHostConfiguration;
        PolkadotPrimitivesV4AbridgedHrmpChannel: PolkadotPrimitivesV4AbridgedHrmpChannel;
        PolkadotPrimitivesV4PersistedValidationData: PolkadotPrimitivesV4PersistedValidationData;
        PolkadotPrimitivesV4UpgradeRestriction: PolkadotPrimitivesV4UpgradeRestriction;
        RococoParachainRuntimeOriginCaller: RococoParachainRuntimeOriginCaller;
        RococoParachainRuntimeProxyType: RococoParachainRuntimeProxyType;
        RococoParachainRuntimeRuntime: RococoParachainRuntimeRuntime;
        RococoParachainRuntimeSessionKeys: RococoParachainRuntimeSessionKeys;
        RuntimeCommonXcmImplCurrencyId: RuntimeCommonXcmImplCurrencyId;
        SpArithmeticArithmeticError: SpArithmeticArithmeticError;
        SpConsensusAuraSr25519AppSr25519Public: SpConsensusAuraSr25519AppSr25519Public;
        SpCoreCryptoKeyTypeId: SpCoreCryptoKeyTypeId;
        SpCoreEcdsaSignature: SpCoreEcdsaSignature;
        SpCoreEd25519Public: SpCoreEd25519Public;
        SpCoreEd25519Signature: SpCoreEd25519Signature;
        SpCoreSr25519Public: SpCoreSr25519Public;
        SpCoreSr25519Signature: SpCoreSr25519Signature;
        SpCoreVoid: SpCoreVoid;
        SpRuntimeDigest: SpRuntimeDigest;
        SpRuntimeDigestDigestItem: SpRuntimeDigestDigestItem;
        SpRuntimeDispatchError: SpRuntimeDispatchError;
        SpRuntimeModuleError: SpRuntimeModuleError;
        SpRuntimeMultiSignature: SpRuntimeMultiSignature;
        SpRuntimeTokenError: SpRuntimeTokenError;
        SpRuntimeTransactionalError: SpRuntimeTransactionalError;
        SpTrieStorageProof: SpTrieStorageProof;
        SpVersionRuntimeVersion: SpVersionRuntimeVersion;
        SpWeightsRuntimeDbWeight: SpWeightsRuntimeDbWeight;
        SpWeightsWeightV2Weight: SpWeightsWeightV2Weight;
        XcmDoubleEncoded: XcmDoubleEncoded;
        XcmV2BodyId: XcmV2BodyId;
        XcmV2BodyPart: XcmV2BodyPart;
        XcmV2Instruction: XcmV2Instruction;
        XcmV2Junction: XcmV2Junction;
        XcmV2MultiAsset: XcmV2MultiAsset;
        XcmV2MultiLocation: XcmV2MultiLocation;
        XcmV2MultiassetAssetId: XcmV2MultiassetAssetId;
        XcmV2MultiassetAssetInstance: XcmV2MultiassetAssetInstance;
        XcmV2MultiassetFungibility: XcmV2MultiassetFungibility;
        XcmV2MultiassetMultiAssetFilter: XcmV2MultiassetMultiAssetFilter;
        XcmV2MultiassetMultiAssets: XcmV2MultiassetMultiAssets;
        XcmV2MultiassetWildFungibility: XcmV2MultiassetWildFungibility;
        XcmV2MultiassetWildMultiAsset: XcmV2MultiassetWildMultiAsset;
        XcmV2MultilocationJunctions: XcmV2MultilocationJunctions;
        XcmV2NetworkId: XcmV2NetworkId;
        XcmV2OriginKind: XcmV2OriginKind;
        XcmV2Response: XcmV2Response;
        XcmV2TraitsError: XcmV2TraitsError;
        XcmV2WeightLimit: XcmV2WeightLimit;
        XcmV2Xcm: XcmV2Xcm;
        XcmV3Instruction: XcmV3Instruction;
        XcmV3Junction: XcmV3Junction;
        XcmV3JunctionBodyId: XcmV3JunctionBodyId;
        XcmV3JunctionBodyPart: XcmV3JunctionBodyPart;
        XcmV3JunctionNetworkId: XcmV3JunctionNetworkId;
        XcmV3Junctions: XcmV3Junctions;
        XcmV3MaybeErrorCode: XcmV3MaybeErrorCode;
        XcmV3MultiAsset: XcmV3MultiAsset;
        XcmV3MultiLocation: XcmV3MultiLocation;
        XcmV3MultiassetAssetId: XcmV3MultiassetAssetId;
        XcmV3MultiassetAssetInstance: XcmV3MultiassetAssetInstance;
        XcmV3MultiassetFungibility: XcmV3MultiassetFungibility;
        XcmV3MultiassetMultiAssetFilter: XcmV3MultiassetMultiAssetFilter;
        XcmV3MultiassetMultiAssets: XcmV3MultiassetMultiAssets;
        XcmV3MultiassetWildFungibility: XcmV3MultiassetWildFungibility;
        XcmV3MultiassetWildMultiAsset: XcmV3MultiassetWildMultiAsset;
        XcmV3PalletInfo: XcmV3PalletInfo;
        XcmV3QueryResponseInfo: XcmV3QueryResponseInfo;
        XcmV3Response: XcmV3Response;
        XcmV3TraitsError: XcmV3TraitsError;
        XcmV3TraitsOutcome: XcmV3TraitsOutcome;
        XcmV3WeightLimit: XcmV3WeightLimit;
        XcmV3Xcm: XcmV3Xcm;
        XcmVersionedAssetId: XcmVersionedAssetId;
        XcmVersionedMultiAsset: XcmVersionedMultiAsset;
        XcmVersionedMultiAssets: XcmVersionedMultiAssets;
        XcmVersionedMultiLocation: XcmVersionedMultiLocation;
        XcmVersionedResponse: XcmVersionedResponse;
        XcmVersionedXcm: XcmVersionedXcm;
    } // InterfaceTypes
} // declare module
