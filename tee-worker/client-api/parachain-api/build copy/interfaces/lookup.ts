// Auto-generated via `yarn polkadot-types-from-defs`, do not edit
/* eslint-disable */

/* eslint-disable sort-keys */

export default {
    /**
     * Lookup3: frame_system::AccountInfo<Index, pallet_balances::types::AccountData<Balance>>
     **/
    FrameSystemAccountInfo: {
        nonce: "u32",
        consumers: "u32",
        providers: "u32",
        sufficients: "u32",
        data: "PalletBalancesAccountData",
    },
    /**
     * Lookup5: pallet_balances::types::AccountData<Balance>
     **/
    PalletBalancesAccountData: {
        free: "u128",
        reserved: "u128",
        frozen: "u128",
        flags: "u128",
    },
    /**
     * Lookup8: frame_support::dispatch::PerDispatchClass<sp_weights::weight_v2::Weight>
     **/
    FrameSupportDispatchPerDispatchClassWeight: {
        normal: "SpWeightsWeightV2Weight",
        operational: "SpWeightsWeightV2Weight",
        mandatory: "SpWeightsWeightV2Weight",
    },
    /**
     * Lookup9: sp_weights::weight_v2::Weight
     **/
    SpWeightsWeightV2Weight: {
        refTime: "Compact<u64>",
        proofSize: "Compact<u64>",
    },
    /**
     * Lookup14: sp_runtime::generic::digest::Digest
     **/
    SpRuntimeDigest: {
        logs: "Vec<SpRuntimeDigestDigestItem>",
    },
    /**
     * Lookup16: sp_runtime::generic::digest::DigestItem
     **/
    SpRuntimeDigestDigestItem: {
        _enum: {
            Other: "Bytes",
            __Unused1: "Null",
            __Unused2: "Null",
            __Unused3: "Null",
            Consensus: "([u8;4],Bytes)",
            Seal: "([u8;4],Bytes)",
            PreRuntime: "([u8;4],Bytes)",
            __Unused7: "Null",
            RuntimeEnvironmentUpdated: "Null",
        },
    },
    /**
     * Lookup19: frame_system::EventRecord<rococo_parachain_runtime::RuntimeEvent, primitive_types::H256>
     **/
    FrameSystemEventRecord: {
        phase: "FrameSystemPhase",
        event: "Event",
        topics: "Vec<H256>",
    },
    /**
     * Lookup21: frame_system::pallet::Event<T>
     **/
    FrameSystemEvent: {
        _enum: {
            ExtrinsicSuccess: {
                dispatchInfo: "FrameSupportDispatchDispatchInfo",
            },
            ExtrinsicFailed: {
                dispatchError: "SpRuntimeDispatchError",
                dispatchInfo: "FrameSupportDispatchDispatchInfo",
            },
            CodeUpdated: "Null",
            NewAccount: {
                account: "AccountId32",
            },
            KilledAccount: {
                account: "AccountId32",
            },
            Remarked: {
                _alias: {
                    hash_: "hash",
                },
                sender: "AccountId32",
                hash_: "H256",
            },
        },
    },
    /**
     * Lookup22: frame_support::dispatch::DispatchInfo
     **/
    FrameSupportDispatchDispatchInfo: {
        weight: "SpWeightsWeightV2Weight",
        class: "FrameSupportDispatchDispatchClass",
        paysFee: "FrameSupportDispatchPays",
    },
    /**
     * Lookup23: frame_support::dispatch::DispatchClass
     **/
    FrameSupportDispatchDispatchClass: {
        _enum: ["Normal", "Operational", "Mandatory"],
    },
    /**
     * Lookup24: frame_support::dispatch::Pays
     **/
    FrameSupportDispatchPays: {
        _enum: ["Yes", "No"],
    },
    /**
     * Lookup25: sp_runtime::DispatchError
     **/
    SpRuntimeDispatchError: {
        _enum: {
            Other: "Null",
            CannotLookup: "Null",
            BadOrigin: "Null",
            Module: "SpRuntimeModuleError",
            ConsumerRemaining: "Null",
            NoProviders: "Null",
            TooManyConsumers: "Null",
            Token: "SpRuntimeTokenError",
            Arithmetic: "SpArithmeticArithmeticError",
            Transactional: "SpRuntimeTransactionalError",
            Exhausted: "Null",
            Corruption: "Null",
            Unavailable: "Null",
        },
    },
    /**
     * Lookup26: sp_runtime::ModuleError
     **/
    SpRuntimeModuleError: {
        index: "u8",
        error: "[u8;4]",
    },
    /**
     * Lookup27: sp_runtime::TokenError
     **/
    SpRuntimeTokenError: {
        _enum: [
            "FundsUnavailable",
            "OnlyProvider",
            "BelowMinimum",
            "CannotCreate",
            "UnknownAsset",
            "Frozen",
            "Unsupported",
            "CannotCreateHold",
            "NotExpendable",
        ],
    },
    /**
     * Lookup28: sp_arithmetic::ArithmeticError
     **/
    SpArithmeticArithmeticError: {
        _enum: ["Underflow", "Overflow", "DivisionByZero"],
    },
    /**
     * Lookup29: sp_runtime::TransactionalError
     **/
    SpRuntimeTransactionalError: {
        _enum: ["LimitReached", "NoLayer"],
    },
    /**
     * Lookup30: pallet_scheduler::pallet::Event<T>
     **/
    PalletSchedulerEvent: {
        _enum: {
            Scheduled: {
                when: "u32",
                index: "u32",
            },
            Canceled: {
                when: "u32",
                index: "u32",
            },
            Dispatched: {
                task: "(u32,u32)",
                id: "Option<[u8;32]>",
                result: "Result<Null, SpRuntimeDispatchError>",
            },
            CallUnavailable: {
                task: "(u32,u32)",
                id: "Option<[u8;32]>",
            },
            PeriodicFailed: {
                task: "(u32,u32)",
                id: "Option<[u8;32]>",
            },
            PermanentlyOverweight: {
                task: "(u32,u32)",
                id: "Option<[u8;32]>",
            },
        },
    },
    /**
     * Lookup35: pallet_utility::pallet::Event
     **/
    PalletUtilityEvent: {
        _enum: {
            BatchInterrupted: {
                index: "u32",
                error: "SpRuntimeDispatchError",
            },
            BatchCompleted: "Null",
            BatchCompletedWithErrors: "Null",
            ItemCompleted: "Null",
            ItemFailed: {
                error: "SpRuntimeDispatchError",
            },
            DispatchedAs: {
                result: "Result<Null, SpRuntimeDispatchError>",
            },
        },
    },
    /**
     * Lookup36: pallet_multisig::pallet::Event<T>
     **/
    PalletMultisigEvent: {
        _enum: {
            NewMultisig: {
                approving: "AccountId32",
                multisig: "AccountId32",
                callHash: "[u8;32]",
            },
            MultisigApproval: {
                approving: "AccountId32",
                timepoint: "PalletMultisigTimepoint",
                multisig: "AccountId32",
                callHash: "[u8;32]",
            },
            MultisigExecuted: {
                approving: "AccountId32",
                timepoint: "PalletMultisigTimepoint",
                multisig: "AccountId32",
                callHash: "[u8;32]",
                result: "Result<Null, SpRuntimeDispatchError>",
            },
            MultisigCancelled: {
                cancelling: "AccountId32",
                timepoint: "PalletMultisigTimepoint",
                multisig: "AccountId32",
                callHash: "[u8;32]",
            },
        },
    },
    /**
     * Lookup37: pallet_multisig::Timepoint<BlockNumber>
     **/
    PalletMultisigTimepoint: {
        height: "u32",
        index: "u32",
    },
    /**
     * Lookup38: pallet_proxy::pallet::Event<T>
     **/
    PalletProxyEvent: {
        _enum: {
            ProxyExecuted: {
                result: "Result<Null, SpRuntimeDispatchError>",
            },
            PureCreated: {
                pure: "AccountId32",
                who: "AccountId32",
                proxyType: "RococoParachainRuntimeProxyType",
                disambiguationIndex: "u16",
            },
            Announced: {
                real: "AccountId32",
                proxy: "AccountId32",
                callHash: "H256",
            },
            ProxyAdded: {
                delegator: "AccountId32",
                delegatee: "AccountId32",
                proxyType: "RococoParachainRuntimeProxyType",
                delay: "u32",
            },
            ProxyRemoved: {
                delegator: "AccountId32",
                delegatee: "AccountId32",
                proxyType: "RococoParachainRuntimeProxyType",
                delay: "u32",
            },
        },
    },
    /**
     * Lookup39: rococo_parachain_runtime::ProxyType
     **/
    RococoParachainRuntimeProxyType: {
        _enum: ["Any", "NonTransfer", "CancelProxy", "Collator", "Governance"],
    },
    /**
     * Lookup41: pallet_preimage::pallet::Event<T>
     **/
    PalletPreimageEvent: {
        _enum: {
            Noted: {
                _alias: {
                    hash_: "hash",
                },
                hash_: "H256",
            },
            Requested: {
                _alias: {
                    hash_: "hash",
                },
                hash_: "H256",
            },
            Cleared: {
                _alias: {
                    hash_: "hash",
                },
                hash_: "H256",
            },
        },
    },
    /**
     * Lookup42: pallet_balances::pallet::Event<T, I>
     **/
    PalletBalancesEvent: {
        _enum: {
            Endowed: {
                account: "AccountId32",
                freeBalance: "u128",
            },
            DustLost: {
                account: "AccountId32",
                amount: "u128",
            },
            Transfer: {
                from: "AccountId32",
                to: "AccountId32",
                amount: "u128",
            },
            BalanceSet: {
                who: "AccountId32",
                free: "u128",
            },
            Reserved: {
                who: "AccountId32",
                amount: "u128",
            },
            Unreserved: {
                who: "AccountId32",
                amount: "u128",
            },
            ReserveRepatriated: {
                from: "AccountId32",
                to: "AccountId32",
                amount: "u128",
                destinationStatus: "FrameSupportTokensMiscBalanceStatus",
            },
            Deposit: {
                who: "AccountId32",
                amount: "u128",
            },
            Withdraw: {
                who: "AccountId32",
                amount: "u128",
            },
            Slashed: {
                who: "AccountId32",
                amount: "u128",
            },
            Minted: {
                who: "AccountId32",
                amount: "u128",
            },
            Burned: {
                who: "AccountId32",
                amount: "u128",
            },
            Suspended: {
                who: "AccountId32",
                amount: "u128",
            },
            Restored: {
                who: "AccountId32",
                amount: "u128",
            },
            Upgraded: {
                who: "AccountId32",
            },
            Issued: {
                amount: "u128",
            },
            Rescinded: {
                amount: "u128",
            },
            Locked: {
                who: "AccountId32",
                amount: "u128",
            },
            Unlocked: {
                who: "AccountId32",
                amount: "u128",
            },
            Frozen: {
                who: "AccountId32",
                amount: "u128",
            },
            Thawed: {
                who: "AccountId32",
                amount: "u128",
            },
        },
    },
    /**
     * Lookup43: frame_support::traits::tokens::misc::BalanceStatus
     **/
    FrameSupportTokensMiscBalanceStatus: {
        _enum: ["Free", "Reserved"],
    },
    /**
     * Lookup44: pallet_vesting::pallet::Event<T>
     **/
    PalletVestingEvent: {
        _enum: {
            VestingUpdated: {
                account: "AccountId32",
                unvested: "u128",
            },
            VestingCompleted: {
                account: "AccountId32",
            },
        },
    },
    /**
     * Lookup45: pallet_transaction_payment::pallet::Event<T>
     **/
    PalletTransactionPaymentEvent: {
        _enum: {
            TransactionFeePaid: {
                who: "AccountId32",
                actualFee: "u128",
                tip: "u128",
            },
        },
    },
    /**
     * Lookup46: pallet_treasury::pallet::Event<T, I>
     **/
    PalletTreasuryEvent: {
        _enum: {
            Proposed: {
                proposalIndex: "u32",
            },
            Spending: {
                budgetRemaining: "u128",
            },
            Awarded: {
                proposalIndex: "u32",
                award: "u128",
                account: "AccountId32",
            },
            Rejected: {
                proposalIndex: "u32",
                slashed: "u128",
            },
            Burnt: {
                burntFunds: "u128",
            },
            Rollover: {
                rolloverBalance: "u128",
            },
            Deposit: {
                value: "u128",
            },
            SpendApproved: {
                proposalIndex: "u32",
                amount: "u128",
                beneficiary: "AccountId32",
            },
            UpdatedInactive: {
                reactivated: "u128",
                deactivated: "u128",
            },
        },
    },
    /**
     * Lookup47: pallet_democracy::pallet::Event<T>
     **/
    PalletDemocracyEvent: {
        _enum: {
            Proposed: {
                proposalIndex: "u32",
                deposit: "u128",
            },
            Tabled: {
                proposalIndex: "u32",
                deposit: "u128",
            },
            ExternalTabled: "Null",
            Started: {
                refIndex: "u32",
                threshold: "PalletDemocracyVoteThreshold",
            },
            Passed: {
                refIndex: "u32",
            },
            NotPassed: {
                refIndex: "u32",
            },
            Cancelled: {
                refIndex: "u32",
            },
            Delegated: {
                who: "AccountId32",
                target: "AccountId32",
            },
            Undelegated: {
                account: "AccountId32",
            },
            Vetoed: {
                who: "AccountId32",
                proposalHash: "H256",
                until: "u32",
            },
            Blacklisted: {
                proposalHash: "H256",
            },
            Voted: {
                voter: "AccountId32",
                refIndex: "u32",
                vote: "PalletDemocracyVoteAccountVote",
            },
            Seconded: {
                seconder: "AccountId32",
                propIndex: "u32",
            },
            ProposalCanceled: {
                propIndex: "u32",
            },
            MetadataSet: {
                _alias: {
                    hash_: "hash",
                },
                owner: "PalletDemocracyMetadataOwner",
                hash_: "H256",
            },
            MetadataCleared: {
                _alias: {
                    hash_: "hash",
                },
                owner: "PalletDemocracyMetadataOwner",
                hash_: "H256",
            },
            MetadataTransferred: {
                _alias: {
                    hash_: "hash",
                },
                prevOwner: "PalletDemocracyMetadataOwner",
                owner: "PalletDemocracyMetadataOwner",
                hash_: "H256",
            },
        },
    },
    /**
     * Lookup48: pallet_democracy::vote_threshold::VoteThreshold
     **/
    PalletDemocracyVoteThreshold: {
        _enum: ["SuperMajorityApprove", "SuperMajorityAgainst", "SimpleMajority"],
    },
    /**
     * Lookup49: pallet_democracy::vote::AccountVote<Balance>
     **/
    PalletDemocracyVoteAccountVote: {
        _enum: {
            Standard: {
                vote: "Vote",
                balance: "u128",
            },
            Split: {
                aye: "u128",
                nay: "u128",
            },
        },
    },
    /**
     * Lookup51: pallet_democracy::types::MetadataOwner
     **/
    PalletDemocracyMetadataOwner: {
        _enum: {
            External: "Null",
            Proposal: "u32",
            Referendum: "u32",
        },
    },
    /**
     * Lookup52: pallet_collective::pallet::Event<T, I>
     **/
    PalletCollectiveEvent: {
        _enum: {
            Proposed: {
                account: "AccountId32",
                proposalIndex: "u32",
                proposalHash: "H256",
                threshold: "u32",
            },
            Voted: {
                account: "AccountId32",
                proposalHash: "H256",
                voted: "bool",
                yes: "u32",
                no: "u32",
            },
            Approved: {
                proposalHash: "H256",
            },
            Disapproved: {
                proposalHash: "H256",
            },
            Executed: {
                proposalHash: "H256",
                result: "Result<Null, SpRuntimeDispatchError>",
            },
            MemberExecuted: {
                proposalHash: "H256",
                result: "Result<Null, SpRuntimeDispatchError>",
            },
            Closed: {
                proposalHash: "H256",
                yes: "u32",
                no: "u32",
            },
        },
    },
    /**
     * Lookup54: pallet_membership::pallet::Event<T, I>
     **/
    PalletMembershipEvent: {
        _enum: ["MemberAdded", "MemberRemoved", "MembersSwapped", "MembersReset", "KeyChanged", "Dummy"],
    },
    /**
     * Lookup57: pallet_bounties::pallet::Event<T, I>
     **/
    PalletBountiesEvent: {
        _enum: {
            BountyProposed: {
                index: "u32",
            },
            BountyRejected: {
                index: "u32",
                bond: "u128",
            },
            BountyBecameActive: {
                index: "u32",
            },
            BountyAwarded: {
                index: "u32",
                beneficiary: "AccountId32",
            },
            BountyClaimed: {
                index: "u32",
                payout: "u128",
                beneficiary: "AccountId32",
            },
            BountyCanceled: {
                index: "u32",
            },
            BountyExtended: {
                index: "u32",
            },
        },
    },
    /**
     * Lookup58: pallet_tips::pallet::Event<T, I>
     **/
    PalletTipsEvent: {
        _enum: {
            NewTip: {
                tipHash: "H256",
            },
            TipClosing: {
                tipHash: "H256",
            },
            TipClosed: {
                tipHash: "H256",
                who: "AccountId32",
                payout: "u128",
            },
            TipRetracted: {
                tipHash: "H256",
            },
            TipSlashed: {
                tipHash: "H256",
                finder: "AccountId32",
                deposit: "u128",
            },
        },
    },
    /**
     * Lookup59: pallet_identity::pallet::Event<T>
     **/
    PalletIdentityEvent: {
        _enum: {
            IdentitySet: {
                who: "AccountId32",
            },
            IdentityCleared: {
                who: "AccountId32",
                deposit: "u128",
            },
            IdentityKilled: {
                who: "AccountId32",
                deposit: "u128",
            },
            JudgementRequested: {
                who: "AccountId32",
                registrarIndex: "u32",
            },
            JudgementUnrequested: {
                who: "AccountId32",
                registrarIndex: "u32",
            },
            JudgementGiven: {
                target: "AccountId32",
                registrarIndex: "u32",
            },
            RegistrarAdded: {
                registrarIndex: "u32",
            },
            SubIdentityAdded: {
                sub: "AccountId32",
                main: "AccountId32",
                deposit: "u128",
            },
            SubIdentityRemoved: {
                sub: "AccountId32",
                main: "AccountId32",
                deposit: "u128",
            },
            SubIdentityRevoked: {
                sub: "AccountId32",
                main: "AccountId32",
                deposit: "u128",
            },
        },
    },
    /**
     * Lookup60: cumulus_pallet_parachain_system::pallet::Event<T>
     **/
    CumulusPalletParachainSystemEvent: {
        _enum: {
            ValidationFunctionStored: "Null",
            ValidationFunctionApplied: {
                relayChainBlockNum: "u32",
            },
            ValidationFunctionDiscarded: "Null",
            UpgradeAuthorized: {
                codeHash: "H256",
            },
            DownwardMessagesReceived: {
                count: "u32",
            },
            DownwardMessagesProcessed: {
                weightUsed: "SpWeightsWeightV2Weight",
                dmqHead: "H256",
            },
            UpwardMessageSent: {
                messageHash: "Option<[u8;32]>",
            },
        },
    },
    /**
     * Lookup61: pallet_session::pallet::Event
     **/
    PalletSessionEvent: {
        _enum: {
            NewSession: {
                sessionIndex: "u32",
            },
        },
    },
    /**
     * Lookup62: pallet_parachain_staking::pallet::Event<T>
     **/
    PalletParachainStakingEvent: {
        _enum: {
            NewRound: {
                startingBlock: "u32",
                round: "u32",
                selectedCollatorsNumber: "u32",
                totalBalance: "u128",
            },
            JoinedCollatorCandidates: {
                account: "AccountId32",
                amountLocked: "u128",
                newTotalAmtLocked: "u128",
            },
            CollatorChosen: {
                round: "u32",
                collatorAccount: "AccountId32",
                totalExposedAmount: "u128",
            },
            CandidateBondLessRequested: {
                candidate: "AccountId32",
                amountToDecrease: "u128",
                executeRound: "u32",
            },
            CandidateBondedMore: {
                candidate: "AccountId32",
                amount: "u128",
                newTotalBond: "u128",
            },
            CandidateBondedLess: {
                candidate: "AccountId32",
                amount: "u128",
                newBond: "u128",
            },
            CandidateWentOffline: {
                candidate: "AccountId32",
            },
            CandidateBackOnline: {
                candidate: "AccountId32",
            },
            CandidateScheduledExit: {
                exitAllowedRound: "u32",
                candidate: "AccountId32",
                scheduledExit: "u32",
            },
            CancelledCandidateExit: {
                candidate: "AccountId32",
            },
            CancelledCandidateBondLess: {
                candidate: "AccountId32",
                amount: "u128",
                executeRound: "u32",
            },
            CandidateLeft: {
                exCandidate: "AccountId32",
                unlockedAmount: "u128",
                newTotalAmtLocked: "u128",
            },
            DelegationDecreaseScheduled: {
                delegator: "AccountId32",
                candidate: "AccountId32",
                amountToDecrease: "u128",
                executeRound: "u32",
            },
            DelegationIncreased: {
                delegator: "AccountId32",
                candidate: "AccountId32",
                amount: "u128",
                inTop: "bool",
            },
            DelegationDecreased: {
                delegator: "AccountId32",
                candidate: "AccountId32",
                amount: "u128",
                inTop: "bool",
            },
            DelegatorExitScheduled: {
                round: "u32",
                delegator: "AccountId32",
                scheduledExit: "u32",
            },
            DelegationRevocationScheduled: {
                round: "u32",
                delegator: "AccountId32",
                candidate: "AccountId32",
                scheduledExit: "u32",
            },
            DelegatorLeft: {
                delegator: "AccountId32",
                unstakedAmount: "u128",
            },
            DelegationRevoked: {
                delegator: "AccountId32",
                candidate: "AccountId32",
                unstakedAmount: "u128",
            },
            DelegationKicked: {
                delegator: "AccountId32",
                candidate: "AccountId32",
                unstakedAmount: "u128",
            },
            DelegatorExitCancelled: {
                delegator: "AccountId32",
            },
            CancelledDelegationRequest: {
                delegator: "AccountId32",
                cancelledRequest: "PalletParachainStakingDelegationRequestsCancelledScheduledRequest",
                collator: "AccountId32",
            },
            Delegation: {
                delegator: "AccountId32",
                lockedAmount: "u128",
                candidate: "AccountId32",
                delegatorPosition: "PalletParachainStakingDelegatorAdded",
                autoCompound: "Percent",
            },
            DelegatorLeftCandidate: {
                delegator: "AccountId32",
                candidate: "AccountId32",
                unstakedAmount: "u128",
                totalCandidateStaked: "u128",
            },
            Rewarded: {
                account: "AccountId32",
                rewards: "u128",
            },
            ReservedForParachainBond: {
                account: "AccountId32",
                value: "u128",
            },
            ParachainBondAccountSet: {
                _alias: {
                    new_: "new",
                },
                old: "AccountId32",
                new_: "AccountId32",
            },
            ParachainBondReservePercentSet: {
                _alias: {
                    new_: "new",
                },
                old: "Percent",
                new_: "Percent",
            },
            InflationSet: {
                annualMin: "Perbill",
                annualIdeal: "Perbill",
                annualMax: "Perbill",
                roundMin: "Perbill",
                roundIdeal: "Perbill",
                roundMax: "Perbill",
            },
            StakeExpectationsSet: {
                expectMin: "u128",
                expectIdeal: "u128",
                expectMax: "u128",
            },
            TotalSelectedSet: {
                _alias: {
                    new_: "new",
                },
                old: "u32",
                new_: "u32",
            },
            CollatorCommissionSet: {
                _alias: {
                    new_: "new",
                },
                old: "Perbill",
                new_: "Perbill",
            },
            BlocksPerRoundSet: {
                _alias: {
                    new_: "new",
                },
                currentRound: "u32",
                firstBlock: "u32",
                old: "u32",
                new_: "u32",
                newPerRoundInflationMin: "Perbill",
                newPerRoundInflationIdeal: "Perbill",
                newPerRoundInflationMax: "Perbill",
            },
            CandidateWhiteListAdded: {
                candidate: "AccountId32",
            },
            CandidateWhiteListRemoved: {
                candidate: "AccountId32",
            },
            AutoCompoundSet: {
                candidate: "AccountId32",
                delegator: "AccountId32",
                value: "Percent",
            },
            Compounded: {
                candidate: "AccountId32",
                delegator: "AccountId32",
                amount: "u128",
            },
        },
    },
    /**
     * Lookup63: pallet_parachain_staking::delegation_requests::CancelledScheduledRequest<Balance>
     **/
    PalletParachainStakingDelegationRequestsCancelledScheduledRequest: {
        whenExecutable: "u32",
        action: "PalletParachainStakingDelegationRequestsDelegationAction",
    },
    /**
     * Lookup64: pallet_parachain_staking::delegation_requests::DelegationAction<Balance>
     **/
    PalletParachainStakingDelegationRequestsDelegationAction: {
        _enum: {
            Revoke: "u128",
            Decrease: "u128",
        },
    },
    /**
     * Lookup65: pallet_parachain_staking::types::DelegatorAdded<B>
     **/
    PalletParachainStakingDelegatorAdded: {
        _enum: {
            AddedToTop: {
                newTotal: "u128",
            },
            AddedToBottom: "Null",
        },
    },
    /**
     * Lookup68: cumulus_pallet_xcmp_queue::pallet::Event<T>
     **/
    CumulusPalletXcmpQueueEvent: {
        _enum: {
            Success: {
                messageHash: "Option<[u8;32]>",
                weight: "SpWeightsWeightV2Weight",
            },
            Fail: {
                messageHash: "Option<[u8;32]>",
                error: "XcmV3TraitsError",
                weight: "SpWeightsWeightV2Weight",
            },
            BadVersion: {
                messageHash: "Option<[u8;32]>",
            },
            BadFormat: {
                messageHash: "Option<[u8;32]>",
            },
            XcmpMessageSent: {
                messageHash: "Option<[u8;32]>",
            },
            OverweightEnqueued: {
                sender: "u32",
                sentAt: "u32",
                index: "u64",
                required: "SpWeightsWeightV2Weight",
            },
            OverweightServiced: {
                index: "u64",
                used: "SpWeightsWeightV2Weight",
            },
        },
    },
    /**
     * Lookup69: xcm::v3::traits::Error
     **/
    XcmV3TraitsError: {
        _enum: {
            Overflow: "Null",
            Unimplemented: "Null",
            UntrustedReserveLocation: "Null",
            UntrustedTeleportLocation: "Null",
            LocationFull: "Null",
            LocationNotInvertible: "Null",
            BadOrigin: "Null",
            InvalidLocation: "Null",
            AssetNotFound: "Null",
            FailedToTransactAsset: "Null",
            NotWithdrawable: "Null",
            LocationCannotHold: "Null",
            ExceedsMaxMessageSize: "Null",
            DestinationUnsupported: "Null",
            Transport: "Null",
            Unroutable: "Null",
            UnknownClaim: "Null",
            FailedToDecode: "Null",
            MaxWeightInvalid: "Null",
            NotHoldingFees: "Null",
            TooExpensive: "Null",
            Trap: "u64",
            ExpectationFalse: "Null",
            PalletNotFound: "Null",
            NameMismatch: "Null",
            VersionIncompatible: "Null",
            HoldingWouldOverflow: "Null",
            ExportError: "Null",
            ReanchorFailed: "Null",
            NoDeal: "Null",
            FeesNotMet: "Null",
            LockError: "Null",
            NoPermission: "Null",
            Unanchored: "Null",
            NotDepositable: "Null",
            UnhandledXcmVersion: "Null",
            WeightLimitReached: "SpWeightsWeightV2Weight",
            Barrier: "Null",
            WeightNotComputable: "Null",
            ExceedsStackLimit: "Null",
        },
    },
    /**
     * Lookup71: pallet_xcm::pallet::Event<T>
     **/
    PalletXcmEvent: {
        _enum: {
            Attempted: "XcmV3TraitsOutcome",
            Sent: "(XcmV3MultiLocation,XcmV3MultiLocation,XcmV3Xcm)",
            UnexpectedResponse: "(XcmV3MultiLocation,u64)",
            ResponseReady: "(u64,XcmV3Response)",
            Notified: "(u64,u8,u8)",
            NotifyOverweight: "(u64,u8,u8,SpWeightsWeightV2Weight,SpWeightsWeightV2Weight)",
            NotifyDispatchError: "(u64,u8,u8)",
            NotifyDecodeFailed: "(u64,u8,u8)",
            InvalidResponder: "(XcmV3MultiLocation,u64,Option<XcmV3MultiLocation>)",
            InvalidResponderVersion: "(XcmV3MultiLocation,u64)",
            ResponseTaken: "u64",
            AssetsTrapped: "(H256,XcmV3MultiLocation,XcmVersionedMultiAssets)",
            VersionChangeNotified: "(XcmV3MultiLocation,u32,XcmV3MultiassetMultiAssets)",
            SupportedVersionChanged: "(XcmV3MultiLocation,u32)",
            NotifyTargetSendFail: "(XcmV3MultiLocation,u64,XcmV3TraitsError)",
            NotifyTargetMigrationFail: "(XcmVersionedMultiLocation,u64)",
            InvalidQuerierVersion: "(XcmV3MultiLocation,u64)",
            InvalidQuerier: "(XcmV3MultiLocation,u64,XcmV3MultiLocation,Option<XcmV3MultiLocation>)",
            VersionNotifyStarted: "(XcmV3MultiLocation,XcmV3MultiassetMultiAssets)",
            VersionNotifyRequested: "(XcmV3MultiLocation,XcmV3MultiassetMultiAssets)",
            VersionNotifyUnrequested: "(XcmV3MultiLocation,XcmV3MultiassetMultiAssets)",
            FeesPaid: "(XcmV3MultiLocation,XcmV3MultiassetMultiAssets)",
            AssetsClaimed: "(H256,XcmV3MultiLocation,XcmVersionedMultiAssets)",
        },
    },
    /**
     * Lookup72: xcm::v3::traits::Outcome
     **/
    XcmV3TraitsOutcome: {
        _enum: {
            Complete: "SpWeightsWeightV2Weight",
            Incomplete: "(SpWeightsWeightV2Weight,XcmV3TraitsError)",
            Error: "XcmV3TraitsError",
        },
    },
    /**
     * Lookup73: xcm::v3::multilocation::MultiLocation
     **/
    XcmV3MultiLocation: {
        parents: "u8",
        interior: "XcmV3Junctions",
    },
    /**
     * Lookup74: xcm::v3::junctions::Junctions
     **/
    XcmV3Junctions: {
        _enum: {
            Here: "Null",
            X1: "XcmV3Junction",
            X2: "(XcmV3Junction,XcmV3Junction)",
            X3: "(XcmV3Junction,XcmV3Junction,XcmV3Junction)",
            X4: "(XcmV3Junction,XcmV3Junction,XcmV3Junction,XcmV3Junction)",
            X5: "(XcmV3Junction,XcmV3Junction,XcmV3Junction,XcmV3Junction,XcmV3Junction)",
            X6: "(XcmV3Junction,XcmV3Junction,XcmV3Junction,XcmV3Junction,XcmV3Junction,XcmV3Junction)",
            X7: "(XcmV3Junction,XcmV3Junction,XcmV3Junction,XcmV3Junction,XcmV3Junction,XcmV3Junction,XcmV3Junction)",
            X8: "(XcmV3Junction,XcmV3Junction,XcmV3Junction,XcmV3Junction,XcmV3Junction,XcmV3Junction,XcmV3Junction,XcmV3Junction)",
        },
    },
    /**
     * Lookup75: xcm::v3::junction::Junction
     **/
    XcmV3Junction: {
        _enum: {
            Parachain: "Compact<u32>",
            AccountId32: {
                network: "Option<XcmV3JunctionNetworkId>",
                id: "[u8;32]",
            },
            AccountIndex64: {
                network: "Option<XcmV3JunctionNetworkId>",
                index: "Compact<u64>",
            },
            AccountKey20: {
                network: "Option<XcmV3JunctionNetworkId>",
                key: "[u8;20]",
            },
            PalletInstance: "u8",
            GeneralIndex: "Compact<u128>",
            GeneralKey: {
                length: "u8",
                data: "[u8;32]",
            },
            OnlyChild: "Null",
            Plurality: {
                id: "XcmV3JunctionBodyId",
                part: "XcmV3JunctionBodyPart",
            },
            GlobalConsensus: "XcmV3JunctionNetworkId",
        },
    },
    /**
     * Lookup78: xcm::v3::junction::NetworkId
     **/
    XcmV3JunctionNetworkId: {
        _enum: {
            ByGenesis: "[u8;32]",
            ByFork: {
                blockNumber: "u64",
                blockHash: "[u8;32]",
            },
            Polkadot: "Null",
            Kusama: "Null",
            Westend: "Null",
            Rococo: "Null",
            Wococo: "Null",
            Ethereum: {
                chainId: "Compact<u64>",
            },
            BitcoinCore: "Null",
            BitcoinCash: "Null",
        },
    },
    /**
     * Lookup81: xcm::v3::junction::BodyId
     **/
    XcmV3JunctionBodyId: {
        _enum: {
            Unit: "Null",
            Moniker: "[u8;4]",
            Index: "Compact<u32>",
            Executive: "Null",
            Technical: "Null",
            Legislative: "Null",
            Judicial: "Null",
            Defense: "Null",
            Administration: "Null",
            Treasury: "Null",
        },
    },
    /**
     * Lookup82: xcm::v3::junction::BodyPart
     **/
    XcmV3JunctionBodyPart: {
        _enum: {
            Voice: "Null",
            Members: {
                count: "Compact<u32>",
            },
            Fraction: {
                nom: "Compact<u32>",
                denom: "Compact<u32>",
            },
            AtLeastProportion: {
                nom: "Compact<u32>",
                denom: "Compact<u32>",
            },
            MoreThanProportion: {
                nom: "Compact<u32>",
                denom: "Compact<u32>",
            },
        },
    },
    /**
     * Lookup83: xcm::v3::Xcm<Call>
     **/
    XcmV3Xcm: "Vec<XcmV3Instruction>",
    /**
     * Lookup85: xcm::v3::Instruction<Call>
     **/
    XcmV3Instruction: {
        _enum: {
            WithdrawAsset: "XcmV3MultiassetMultiAssets",
            ReserveAssetDeposited: "XcmV3MultiassetMultiAssets",
            ReceiveTeleportedAsset: "XcmV3MultiassetMultiAssets",
            QueryResponse: {
                queryId: "Compact<u64>",
                response: "XcmV3Response",
                maxWeight: "SpWeightsWeightV2Weight",
                querier: "Option<XcmV3MultiLocation>",
            },
            TransferAsset: {
                assets: "XcmV3MultiassetMultiAssets",
                beneficiary: "XcmV3MultiLocation",
            },
            TransferReserveAsset: {
                assets: "XcmV3MultiassetMultiAssets",
                dest: "XcmV3MultiLocation",
                xcm: "XcmV3Xcm",
            },
            Transact: {
                originKind: "XcmV2OriginKind",
                requireWeightAtMost: "SpWeightsWeightV2Weight",
                call: "XcmDoubleEncoded",
            },
            HrmpNewChannelOpenRequest: {
                sender: "Compact<u32>",
                maxMessageSize: "Compact<u32>",
                maxCapacity: "Compact<u32>",
            },
            HrmpChannelAccepted: {
                recipient: "Compact<u32>",
            },
            HrmpChannelClosing: {
                initiator: "Compact<u32>",
                sender: "Compact<u32>",
                recipient: "Compact<u32>",
            },
            ClearOrigin: "Null",
            DescendOrigin: "XcmV3Junctions",
            ReportError: "XcmV3QueryResponseInfo",
            DepositAsset: {
                assets: "XcmV3MultiassetMultiAssetFilter",
                beneficiary: "XcmV3MultiLocation",
            },
            DepositReserveAsset: {
                assets: "XcmV3MultiassetMultiAssetFilter",
                dest: "XcmV3MultiLocation",
                xcm: "XcmV3Xcm",
            },
            ExchangeAsset: {
                give: "XcmV3MultiassetMultiAssetFilter",
                want: "XcmV3MultiassetMultiAssets",
                maximal: "bool",
            },
            InitiateReserveWithdraw: {
                assets: "XcmV3MultiassetMultiAssetFilter",
                reserve: "XcmV3MultiLocation",
                xcm: "XcmV3Xcm",
            },
            InitiateTeleport: {
                assets: "XcmV3MultiassetMultiAssetFilter",
                dest: "XcmV3MultiLocation",
                xcm: "XcmV3Xcm",
            },
            ReportHolding: {
                responseInfo: "XcmV3QueryResponseInfo",
                assets: "XcmV3MultiassetMultiAssetFilter",
            },
            BuyExecution: {
                fees: "XcmV3MultiAsset",
                weightLimit: "XcmV3WeightLimit",
            },
            RefundSurplus: "Null",
            SetErrorHandler: "XcmV3Xcm",
            SetAppendix: "XcmV3Xcm",
            ClearError: "Null",
            ClaimAsset: {
                assets: "XcmV3MultiassetMultiAssets",
                ticket: "XcmV3MultiLocation",
            },
            Trap: "Compact<u64>",
            SubscribeVersion: {
                queryId: "Compact<u64>",
                maxResponseWeight: "SpWeightsWeightV2Weight",
            },
            UnsubscribeVersion: "Null",
            BurnAsset: "XcmV3MultiassetMultiAssets",
            ExpectAsset: "XcmV3MultiassetMultiAssets",
            ExpectOrigin: "Option<XcmV3MultiLocation>",
            ExpectError: "Option<(u32,XcmV3TraitsError)>",
            ExpectTransactStatus: "XcmV3MaybeErrorCode",
            QueryPallet: {
                moduleName: "Bytes",
                responseInfo: "XcmV3QueryResponseInfo",
            },
            ExpectPallet: {
                index: "Compact<u32>",
                name: "Bytes",
                moduleName: "Bytes",
                crateMajor: "Compact<u32>",
                minCrateMinor: "Compact<u32>",
            },
            ReportTransactStatus: "XcmV3QueryResponseInfo",
            ClearTransactStatus: "Null",
            UniversalOrigin: "XcmV3Junction",
            ExportMessage: {
                network: "XcmV3JunctionNetworkId",
                destination: "XcmV3Junctions",
                xcm: "XcmV3Xcm",
            },
            LockAsset: {
                asset: "XcmV3MultiAsset",
                unlocker: "XcmV3MultiLocation",
            },
            UnlockAsset: {
                asset: "XcmV3MultiAsset",
                target: "XcmV3MultiLocation",
            },
            NoteUnlockable: {
                asset: "XcmV3MultiAsset",
                owner: "XcmV3MultiLocation",
            },
            RequestUnlock: {
                asset: "XcmV3MultiAsset",
                locker: "XcmV3MultiLocation",
            },
            SetFeesMode: {
                jitWithdraw: "bool",
            },
            SetTopic: "[u8;32]",
            ClearTopic: "Null",
            AliasOrigin: "XcmV3MultiLocation",
            UnpaidExecution: {
                weightLimit: "XcmV3WeightLimit",
                checkOrigin: "Option<XcmV3MultiLocation>",
            },
        },
    },
    /**
     * Lookup86: xcm::v3::multiasset::MultiAssets
     **/
    XcmV3MultiassetMultiAssets: "Vec<XcmV3MultiAsset>",
    /**
     * Lookup88: xcm::v3::multiasset::MultiAsset
     **/
    XcmV3MultiAsset: {
        id: "XcmV3MultiassetAssetId",
        fun: "XcmV3MultiassetFungibility",
    },
    /**
     * Lookup89: xcm::v3::multiasset::AssetId
     **/
    XcmV3MultiassetAssetId: {
        _enum: {
            Concrete: "XcmV3MultiLocation",
            Abstract: "[u8;32]",
        },
    },
    /**
     * Lookup90: xcm::v3::multiasset::Fungibility
     **/
    XcmV3MultiassetFungibility: {
        _enum: {
            Fungible: "Compact<u128>",
            NonFungible: "XcmV3MultiassetAssetInstance",
        },
    },
    /**
     * Lookup91: xcm::v3::multiasset::AssetInstance
     **/
    XcmV3MultiassetAssetInstance: {
        _enum: {
            Undefined: "Null",
            Index: "Compact<u128>",
            Array4: "[u8;4]",
            Array8: "[u8;8]",
            Array16: "[u8;16]",
            Array32: "[u8;32]",
        },
    },
    /**
     * Lookup94: xcm::v3::Response
     **/
    XcmV3Response: {
        _enum: {
            Null: "Null",
            Assets: "XcmV3MultiassetMultiAssets",
            ExecutionResult: "Option<(u32,XcmV3TraitsError)>",
            Version: "u32",
            PalletsInfo: "Vec<XcmV3PalletInfo>",
            DispatchResult: "XcmV3MaybeErrorCode",
        },
    },
    /**
     * Lookup98: xcm::v3::PalletInfo
     **/
    XcmV3PalletInfo: {
        index: "Compact<u32>",
        name: "Bytes",
        moduleName: "Bytes",
        major: "Compact<u32>",
        minor: "Compact<u32>",
        patch: "Compact<u32>",
    },
    /**
     * Lookup101: xcm::v3::MaybeErrorCode
     **/
    XcmV3MaybeErrorCode: {
        _enum: {
            Success: "Null",
            Error: "Bytes",
            TruncatedError: "Bytes",
        },
    },
    /**
     * Lookup104: xcm::v2::OriginKind
     **/
    XcmV2OriginKind: {
        _enum: ["Native", "SovereignAccount", "Superuser", "Xcm"],
    },
    /**
     * Lookup105: xcm::double_encoded::DoubleEncoded<T>
     **/
    XcmDoubleEncoded: {
        encoded: "Bytes",
    },
    /**
     * Lookup106: xcm::v3::QueryResponseInfo
     **/
    XcmV3QueryResponseInfo: {
        destination: "XcmV3MultiLocation",
        queryId: "Compact<u64>",
        maxWeight: "SpWeightsWeightV2Weight",
    },
    /**
     * Lookup107: xcm::v3::multiasset::MultiAssetFilter
     **/
    XcmV3MultiassetMultiAssetFilter: {
        _enum: {
            Definite: "XcmV3MultiassetMultiAssets",
            Wild: "XcmV3MultiassetWildMultiAsset",
        },
    },
    /**
     * Lookup108: xcm::v3::multiasset::WildMultiAsset
     **/
    XcmV3MultiassetWildMultiAsset: {
        _enum: {
            All: "Null",
            AllOf: {
                id: "XcmV3MultiassetAssetId",
                fun: "XcmV3MultiassetWildFungibility",
            },
            AllCounted: "Compact<u32>",
            AllOfCounted: {
                id: "XcmV3MultiassetAssetId",
                fun: "XcmV3MultiassetWildFungibility",
                count: "Compact<u32>",
            },
        },
    },
    /**
     * Lookup109: xcm::v3::multiasset::WildFungibility
     **/
    XcmV3MultiassetWildFungibility: {
        _enum: ["Fungible", "NonFungible"],
    },
    /**
     * Lookup110: xcm::v3::WeightLimit
     **/
    XcmV3WeightLimit: {
        _enum: {
            Unlimited: "Null",
            Limited: "SpWeightsWeightV2Weight",
        },
    },
    /**
     * Lookup111: xcm::VersionedMultiAssets
     **/
    XcmVersionedMultiAssets: {
        _enum: {
            __Unused0: "Null",
            V2: "XcmV2MultiassetMultiAssets",
            __Unused2: "Null",
            V3: "XcmV3MultiassetMultiAssets",
        },
    },
    /**
     * Lookup112: xcm::v2::multiasset::MultiAssets
     **/
    XcmV2MultiassetMultiAssets: "Vec<XcmV2MultiAsset>",
    /**
     * Lookup114: xcm::v2::multiasset::MultiAsset
     **/
    XcmV2MultiAsset: {
        id: "XcmV2MultiassetAssetId",
        fun: "XcmV2MultiassetFungibility",
    },
    /**
     * Lookup115: xcm::v2::multiasset::AssetId
     **/
    XcmV2MultiassetAssetId: {
        _enum: {
            Concrete: "XcmV2MultiLocation",
            Abstract: "Bytes",
        },
    },
    /**
     * Lookup116: xcm::v2::multilocation::MultiLocation
     **/
    XcmV2MultiLocation: {
        parents: "u8",
        interior: "XcmV2MultilocationJunctions",
    },
    /**
     * Lookup117: xcm::v2::multilocation::Junctions
     **/
    XcmV2MultilocationJunctions: {
        _enum: {
            Here: "Null",
            X1: "XcmV2Junction",
            X2: "(XcmV2Junction,XcmV2Junction)",
            X3: "(XcmV2Junction,XcmV2Junction,XcmV2Junction)",
            X4: "(XcmV2Junction,XcmV2Junction,XcmV2Junction,XcmV2Junction)",
            X5: "(XcmV2Junction,XcmV2Junction,XcmV2Junction,XcmV2Junction,XcmV2Junction)",
            X6: "(XcmV2Junction,XcmV2Junction,XcmV2Junction,XcmV2Junction,XcmV2Junction,XcmV2Junction)",
            X7: "(XcmV2Junction,XcmV2Junction,XcmV2Junction,XcmV2Junction,XcmV2Junction,XcmV2Junction,XcmV2Junction)",
            X8: "(XcmV2Junction,XcmV2Junction,XcmV2Junction,XcmV2Junction,XcmV2Junction,XcmV2Junction,XcmV2Junction,XcmV2Junction)",
        },
    },
    /**
     * Lookup118: xcm::v2::junction::Junction
     **/
    XcmV2Junction: {
        _enum: {
            Parachain: "Compact<u32>",
            AccountId32: {
                network: "XcmV2NetworkId",
                id: "[u8;32]",
            },
            AccountIndex64: {
                network: "XcmV2NetworkId",
                index: "Compact<u64>",
            },
            AccountKey20: {
                network: "XcmV2NetworkId",
                key: "[u8;20]",
            },
            PalletInstance: "u8",
            GeneralIndex: "Compact<u128>",
            GeneralKey: "Bytes",
            OnlyChild: "Null",
            Plurality: {
                id: "XcmV2BodyId",
                part: "XcmV2BodyPart",
            },
        },
    },
    /**
     * Lookup119: xcm::v2::NetworkId
     **/
    XcmV2NetworkId: {
        _enum: {
            Any: "Null",
            Named: "Bytes",
            Polkadot: "Null",
            Kusama: "Null",
        },
    },
    /**
     * Lookup121: xcm::v2::BodyId
     **/
    XcmV2BodyId: {
        _enum: {
            Unit: "Null",
            Named: "Bytes",
            Index: "Compact<u32>",
            Executive: "Null",
            Technical: "Null",
            Legislative: "Null",
            Judicial: "Null",
            Defense: "Null",
            Administration: "Null",
            Treasury: "Null",
        },
    },
    /**
     * Lookup122: xcm::v2::BodyPart
     **/
    XcmV2BodyPart: {
        _enum: {
            Voice: "Null",
            Members: {
                count: "Compact<u32>",
            },
            Fraction: {
                nom: "Compact<u32>",
                denom: "Compact<u32>",
            },
            AtLeastProportion: {
                nom: "Compact<u32>",
                denom: "Compact<u32>",
            },
            MoreThanProportion: {
                nom: "Compact<u32>",
                denom: "Compact<u32>",
            },
        },
    },
    /**
     * Lookup123: xcm::v2::multiasset::Fungibility
     **/
    XcmV2MultiassetFungibility: {
        _enum: {
            Fungible: "Compact<u128>",
            NonFungible: "XcmV2MultiassetAssetInstance",
        },
    },
    /**
     * Lookup124: xcm::v2::multiasset::AssetInstance
     **/
    XcmV2MultiassetAssetInstance: {
        _enum: {
            Undefined: "Null",
            Index: "Compact<u128>",
            Array4: "[u8;4]",
            Array8: "[u8;8]",
            Array16: "[u8;16]",
            Array32: "[u8;32]",
            Blob: "Bytes",
        },
    },
    /**
     * Lookup125: xcm::VersionedMultiLocation
     **/
    XcmVersionedMultiLocation: {
        _enum: {
            __Unused0: "Null",
            V2: "XcmV2MultiLocation",
            __Unused2: "Null",
            V3: "XcmV3MultiLocation",
        },
    },
    /**
     * Lookup126: cumulus_pallet_xcm::pallet::Event<T>
     **/
    CumulusPalletXcmEvent: {
        _enum: {
            InvalidFormat: "[u8;32]",
            UnsupportedVersion: "[u8;32]",
            ExecutedDownward: "([u8;32],XcmV3TraitsOutcome)",
        },
    },
    /**
     * Lookup127: cumulus_pallet_dmp_queue::pallet::Event<T>
     **/
    CumulusPalletDmpQueueEvent: {
        _enum: {
            InvalidFormat: {
                messageId: "[u8;32]",
            },
            UnsupportedVersion: {
                messageId: "[u8;32]",
            },
            ExecutedDownward: {
                messageId: "[u8;32]",
                outcome: "XcmV3TraitsOutcome",
            },
            WeightExhausted: {
                messageId: "[u8;32]",
                remainingWeight: "SpWeightsWeightV2Weight",
                requiredWeight: "SpWeightsWeightV2Weight",
            },
            OverweightEnqueued: {
                messageId: "[u8;32]",
                overweightIndex: "u64",
                requiredWeight: "SpWeightsWeightV2Weight",
            },
            OverweightServiced: {
                overweightIndex: "u64",
                weightUsed: "SpWeightsWeightV2Weight",
            },
            MaxMessagesExhausted: {
                messageId: "[u8;32]",
            },
        },
    },
    /**
     * Lookup128: orml_xtokens::module::Event<T>
     **/
    OrmlXtokensModuleEvent: {
        _enum: {
            TransferredMultiAssets: {
                sender: "AccountId32",
                assets: "XcmV3MultiassetMultiAssets",
                fee: "XcmV3MultiAsset",
                dest: "XcmV3MultiLocation",
            },
        },
    },
    /**
     * Lookup129: orml_tokens::module::Event<T>
     **/
    OrmlTokensModuleEvent: {
        _enum: {
            Endowed: {
                currencyId: "u128",
                who: "AccountId32",
                amount: "u128",
            },
            DustLost: {
                currencyId: "u128",
                who: "AccountId32",
                amount: "u128",
            },
            Transfer: {
                currencyId: "u128",
                from: "AccountId32",
                to: "AccountId32",
                amount: "u128",
            },
            Reserved: {
                currencyId: "u128",
                who: "AccountId32",
                amount: "u128",
            },
            Unreserved: {
                currencyId: "u128",
                who: "AccountId32",
                amount: "u128",
            },
            ReserveRepatriated: {
                currencyId: "u128",
                from: "AccountId32",
                to: "AccountId32",
                amount: "u128",
                status: "FrameSupportTokensMiscBalanceStatus",
            },
            BalanceSet: {
                currencyId: "u128",
                who: "AccountId32",
                free: "u128",
                reserved: "u128",
            },
            TotalIssuanceSet: {
                currencyId: "u128",
                amount: "u128",
            },
            Withdrawn: {
                currencyId: "u128",
                who: "AccountId32",
                amount: "u128",
            },
            Slashed: {
                currencyId: "u128",
                who: "AccountId32",
                freeAmount: "u128",
                reservedAmount: "u128",
            },
            Deposited: {
                currencyId: "u128",
                who: "AccountId32",
                amount: "u128",
            },
            LockSet: {
                lockId: "[u8;8]",
                currencyId: "u128",
                who: "AccountId32",
                amount: "u128",
            },
            LockRemoved: {
                lockId: "[u8;8]",
                currencyId: "u128",
                who: "AccountId32",
            },
            Locked: {
                currencyId: "u128",
                who: "AccountId32",
                amount: "u128",
            },
            Unlocked: {
                currencyId: "u128",
                who: "AccountId32",
                amount: "u128",
            },
        },
    },
    /**
     * Lookup130: pallet_bridge::pallet::Event<T>
     **/
    PalletBridgeEvent: {
        _enum: {
            RelayerThresholdChanged: "u32",
            ChainWhitelisted: "u8",
            RelayerAdded: "AccountId32",
            RelayerRemoved: "AccountId32",
            FungibleTransfer: "(u8,u64,[u8;32],u128,Bytes)",
            NonFungibleTransfer: "(u8,u64,[u8;32],Bytes,Bytes,Bytes)",
            GenericTransfer: "(u8,u64,[u8;32],Bytes)",
            VoteFor: "(u8,u64,AccountId32)",
            VoteAgainst: "(u8,u64,AccountId32)",
            ProposalApproved: "(u8,u64)",
            ProposalRejected: "(u8,u64)",
            ProposalSucceeded: "(u8,u64)",
            ProposalFailed: "(u8,u64)",
            FeeUpdated: {
                destId: "u8",
                fee: "u128",
            },
        },
    },
    /**
     * Lookup131: pallet_bridge_transfer::pallet::Event<T>
     **/
    PalletBridgeTransferEvent: {
        _enum: {
            MaximumIssuanceChanged: {
                oldValue: "u128",
            },
            NativeTokenMinted: {
                to: "AccountId32",
                amount: "u128",
            },
        },
    },
    /**
     * Lookup132: pallet_extrinsic_filter::pallet::Event<T>
     **/
    PalletExtrinsicFilterEvent: {
        _enum: {
            ModeSet: {
                newMode: "PalletExtrinsicFilterOperationalMode",
            },
            ExtrinsicsBlocked: {
                palletNameBytes: "Bytes",
                functionNameBytes: "Option<Bytes>",
            },
            ExtrinsicsUnblocked: {
                palletNameBytes: "Bytes",
                functionNameBytes: "Option<Bytes>",
            },
        },
    },
    /**
     * Lookup133: pallet_extrinsic_filter::OperationalMode
     **/
    PalletExtrinsicFilterOperationalMode: {
        _enum: ["Normal", "Safe", "Test"],
    },
    /**
     * Lookup135: pallet_identity_management::pallet::Event<T>
     **/
    PalletIdentityManagementEvent: {
        _enum: {
            DelegateeAdded: {
                account: "AccountId32",
            },
            DelegateeRemoved: {
                account: "AccountId32",
            },
            LinkIdentityRequested: {
                shard: "H256",
                account: "AccountId32",
                encryptedIdentity: "Bytes",
                encryptedValidationData: "Bytes",
                encryptedWeb3networks: "Bytes",
            },
            DeactivateIdentityRequested: {
                shard: "H256",
                account: "AccountId32",
                encryptedIdentity: "Bytes",
            },
            ActivateIdentityRequested: {
                shard: "H256",
                account: "AccountId32",
                encryptedIdentity: "Bytes",
            },
            IdentityLinked: {
                primeIdentity: "CorePrimitivesIdentity",
                idGraphHash: "H256",
                reqExtHash: "H256",
            },
            IdentityDeactivated: {
                primeIdentity: "CorePrimitivesIdentity",
                idGraphHash: "H256",
                reqExtHash: "H256",
            },
            IdentityActivated: {
                primeIdentity: "CorePrimitivesIdentity",
                idGraphHash: "H256",
                reqExtHash: "H256",
            },
            IdentityNetworksSet: {
                primeIdentity: "CorePrimitivesIdentity",
                idGraphHash: "H256",
                reqExtHash: "H256",
            },
            LinkIdentityFailed: {
                primeIdentity: "Option<CorePrimitivesIdentity>",
                detail: "CorePrimitivesErrorErrorDetail",
                reqExtHash: "H256",
            },
            DeactivateIdentityFailed: {
                primeIdentity: "Option<CorePrimitivesIdentity>",
                detail: "CorePrimitivesErrorErrorDetail",
                reqExtHash: "H256",
            },
            ActivateIdentityFailed: {
                primeIdentity: "Option<CorePrimitivesIdentity>",
                detail: "CorePrimitivesErrorErrorDetail",
                reqExtHash: "H256",
            },
            ImportScheduledEnclaveFailed: "Null",
            UnclassifiedError: {
                primeIdentity: "Option<CorePrimitivesIdentity>",
                detail: "CorePrimitivesErrorErrorDetail",
                reqExtHash: "H256",
            },
        },
    },
    /**
     * Lookup136: core_primitives::identity::Identity
     **/
    CorePrimitivesIdentity: {
        _enum: {
            Twitter: "Bytes",
            Discord: "Bytes",
            Github: "Bytes",
            Substrate: "CorePrimitivesIdentityAddress32",
            Evm: "CorePrimitivesIdentityAddress20",
            Bitcoin: "CorePrimitivesIdentityAddress33",
            Solana: "CorePrimitivesIdentityAddress32",
        },
    },
    /**
     * Lookup138: core_primitives::identity::Address32
     **/
    CorePrimitivesIdentityAddress32: "[u8;32]",
    /**
     * Lookup139: core_primitives::identity::Address20
     **/
    CorePrimitivesIdentityAddress20: "[u8;20]",
    /**
     * Lookup140: core_primitives::identity::Address33
     **/
    CorePrimitivesIdentityAddress33: "[u8;33]",
    /**
     * Lookup143: core_primitives::error::ErrorDetail
     **/
    CorePrimitivesErrorErrorDetail: {
        _enum: {
            ImportError: "Null",
            UnauthorizedSigner: "Null",
            StfError: "Bytes",
            SendStfRequestFailed: "Null",
            ParseError: "Null",
            DataProviderError: "Bytes",
            InvalidIdentity: "Null",
            WrongWeb2Handle: "Null",
            UnexpectedMessage: "Null",
            __Unused9: "Null",
            VerifyWeb3SignatureFailed: "Null",
            NoEligibleIdentity: "Null",
        },
    },
    /**
     * Lookup145: pallet_asset_manager::pallet::Event<T>
     **/
    PalletAssetManagerEvent: {
        _enum: {
            ForeignAssetMetadataUpdated: {
                assetId: "u128",
                metadata: "PalletAssetManagerAssetMetadata",
            },
            ForeignAssetTrackerUpdated: {
                oldAssetTracker: "u128",
                newAssetTracker: "u128",
            },
            ForeignAssetTypeRegistered: {
                assetId: "u128",
                assetType: "RuntimeCommonXcmImplCurrencyId",
            },
            ForeignAssetTypeRemoved: {
                assetId: "u128",
                removedAssetType: "RuntimeCommonXcmImplCurrencyId",
                defaultAssetType: "RuntimeCommonXcmImplCurrencyId",
            },
            UnitsPerSecondChanged: {
                assetId: "u128",
                unitsPerSecond: "u128",
            },
        },
    },
    /**
     * Lookup146: pallet_asset_manager::pallet::AssetMetadata<Balance>
     **/
    PalletAssetManagerAssetMetadata: {
        name: "Bytes",
        symbol: "Bytes",
        decimals: "u8",
        minimalBalance: "u128",
        isFrozen: "bool",
    },
    /**
     * Lookup147: runtime_common::xcm_impl::CurrencyId<rococo_parachain_runtime::Runtime>
     **/
    RuntimeCommonXcmImplCurrencyId: {
        _enum: {
            SelfReserve: "Null",
            ParachainReserve: "XcmV3MultiLocation",
        },
    },
    /**
     * Lookup148: rococo_parachain_runtime::Runtime
     **/
    RococoParachainRuntimeRuntime: "Null",
    /**
     * Lookup149: pallet_vc_management::pallet::Event<T>
     **/
    PalletVcManagementEvent: {
        _enum: {
            DelegateeAdded: {
                account: "AccountId32",
            },
            DelegateeRemoved: {
                account: "AccountId32",
            },
            VCRequested: {
                account: "AccountId32",
                shard: "H256",
                assertion: "CorePrimitivesAssertion",
            },
            VCIssued: {
                identity: "CorePrimitivesIdentity",
                assertion: "CorePrimitivesAssertion",
                idGraphHash: "H256",
                reqExtHash: "H256",
            },
            AdminChanged: {
                oldAdmin: "Option<AccountId32>",
                newAdmin: "Option<AccountId32>",
            },
            SchemaIssued: {
                account: "AccountId32",
                shard: "H256",
                index: "u64",
            },
            SchemaDisabled: {
                account: "AccountId32",
                shard: "H256",
                index: "u64",
            },
            SchemaActivated: {
                account: "AccountId32",
                shard: "H256",
                index: "u64",
            },
            SchemaRevoked: {
                account: "AccountId32",
                shard: "H256",
                index: "u64",
            },
            RequestVCFailed: {
                identity: "Option<CorePrimitivesIdentity>",
                assertion: "CorePrimitivesAssertion",
                detail: "CorePrimitivesErrorErrorDetail",
                reqExtHash: "H256",
            },
            UnclassifiedError: {
                identity: "Option<CorePrimitivesIdentity>",
                detail: "CorePrimitivesErrorErrorDetail",
                reqExtHash: "H256",
            },
        },
    },
    /**
     * Lookup150: core_primitives::assertion::Assertion
     **/
    CorePrimitivesAssertion: {
        _enum: {
            A1: "Null",
            A2: "Bytes",
            A3: "(Bytes,Bytes,Bytes)",
            A4: "Bytes",
            A6: "Null",
            A7: "Bytes",
            A8: "Vec<CorePrimitivesAssertionNetworkWeb3Network>",
            A10: "Bytes",
            A11: "Bytes",
            A13: "AccountId32",
            A14: "Null",
            Achainable: "CorePrimitivesAssertionAchainableAchainableParams",
            A20: "Null",
            OneBlock: "CorePrimitivesAssertionOneblockOneBlockCourseType",
            GenericDiscordRole: "CorePrimitivesAssertionGenericDiscordRoleGenericDiscordRoleType",
            __Unused15: "Null",
            BnbDomainHolding: "Null",
            BnbDigitDomainClub: "CorePrimitivesAssertionBnbDomainBnbDigitDomainType",
            VIP3MembershipCard: "CorePrimitivesAssertionVip3Vip3MembershipCardLevel",
            WeirdoGhostGangHolder: "Null",
            LITStaking: "Null",
            EVMAmountHolding: "CorePrimitivesAssertionEvmAmountHoldingEvmTokenType",
            BRC20AmountHolder: "Null",
            CryptoSummary: "Null",
            TokenHoldingAmount: "CorePrimitivesAssertionWeb3TokenWeb3TokenType",
            PlatformUser: "CorePrimitivesAssertionPlatformUserPlatformUserType",
            NftHolder: "CorePrimitivesAssertionWeb3NftWeb3NftType",
            Dynamic: "(H160,Bytes)",
        },
    },
    /**
     * Lookup152: core_primitives::assertion::network::Web3Network
     **/
    CorePrimitivesAssertionNetworkWeb3Network: {
        _enum: [
            "Polkadot",
            "Kusama",
            "Litentry",
            "Litmus",
            "LitentryRococo",
            "Khala",
            "SubstrateTestnet",
            "Ethereum",
            "Bsc",
            "BitcoinP2tr",
            "BitcoinP2pkh",
            "BitcoinP2sh",
            "BitcoinP2wpkh",
            "BitcoinP2wsh",
            "Polygon",
            "Arbitrum",
            "Solana",
            "Combo",
        ],
    },
    /**
     * Lookup154: core_primitives::assertion::achainable::AchainableParams
     **/
    CorePrimitivesAssertionAchainableAchainableParams: {
        _enum: {
            AmountHolding: "CorePrimitivesAssertionAchainableAchainableAmountHolding",
            AmountToken: "CorePrimitivesAssertionAchainableAchainableAmountToken",
            Amount: "CorePrimitivesAssertionAchainableAchainableAmount",
            Amounts: "CorePrimitivesAssertionAchainableAchainableAmounts",
            Basic: "CorePrimitivesAssertionAchainableAchainableBasic",
            BetweenPercents: "CorePrimitivesAssertionAchainableAchainableBetweenPercents",
            ClassOfYear: "CorePrimitivesAssertionAchainableAchainableClassOfYear",
            DateInterval: "CorePrimitivesAssertionAchainableAchainableDateInterval",
            DatePercent: "CorePrimitivesAssertionAchainableAchainableDatePercent",
            Date: "CorePrimitivesAssertionAchainableAchainableDate",
            Token: "CorePrimitivesAssertionAchainableAchainableToken",
            Mirror: "CorePrimitivesAssertionAchainableAchainableMirror",
        },
    },
    /**
     * Lookup155: core_primitives::assertion::achainable::AchainableAmountHolding
     **/
    CorePrimitivesAssertionAchainableAchainableAmountHolding: {
        name: "Bytes",
        chain: "Vec<CorePrimitivesAssertionNetworkWeb3Network>",
        amount: "Bytes",
        date: "Bytes",
        token: "Option<Bytes>",
    },
    /**
     * Lookup157: core_primitives::assertion::achainable::AchainableAmountToken
     **/
    CorePrimitivesAssertionAchainableAchainableAmountToken: {
        name: "Bytes",
        chain: "Vec<CorePrimitivesAssertionNetworkWeb3Network>",
        amount: "Bytes",
        token: "Option<Bytes>",
    },
    /**
     * Lookup158: core_primitives::assertion::achainable::AchainableAmount
     **/
    CorePrimitivesAssertionAchainableAchainableAmount: {
        name: "Bytes",
        chain: "Vec<CorePrimitivesAssertionNetworkWeb3Network>",
        amount: "Bytes",
    },
    /**
     * Lookup159: core_primitives::assertion::achainable::AchainableAmounts
     **/
    CorePrimitivesAssertionAchainableAchainableAmounts: {
        name: "Bytes",
        chain: "Vec<CorePrimitivesAssertionNetworkWeb3Network>",
        amount1: "Bytes",
        amount2: "Bytes",
    },
    /**
     * Lookup160: core_primitives::assertion::achainable::AchainableBasic
     **/
    CorePrimitivesAssertionAchainableAchainableBasic: {
        name: "Bytes",
        chain: "Vec<CorePrimitivesAssertionNetworkWeb3Network>",
    },
    /**
     * Lookup161: core_primitives::assertion::achainable::AchainableBetweenPercents
     **/
    CorePrimitivesAssertionAchainableAchainableBetweenPercents: {
        name: "Bytes",
        chain: "Vec<CorePrimitivesAssertionNetworkWeb3Network>",
        greaterThanOrEqualTo: "Bytes",
        lessThanOrEqualTo: "Bytes",
    },
    /**
     * Lookup162: core_primitives::assertion::achainable::AchainableClassOfYear
     **/
    CorePrimitivesAssertionAchainableAchainableClassOfYear: {
        name: "Bytes",
        chain: "Vec<CorePrimitivesAssertionNetworkWeb3Network>",
    },
    /**
     * Lookup163: core_primitives::assertion::achainable::AchainableDateInterval
     **/
    CorePrimitivesAssertionAchainableAchainableDateInterval: {
        name: "Bytes",
        chain: "Vec<CorePrimitivesAssertionNetworkWeb3Network>",
        startDate: "Bytes",
        endDate: "Bytes",
    },
    /**
     * Lookup164: core_primitives::assertion::achainable::AchainableDatePercent
     **/
    CorePrimitivesAssertionAchainableAchainableDatePercent: {
        name: "Bytes",
        chain: "Vec<CorePrimitivesAssertionNetworkWeb3Network>",
        token: "Bytes",
        date: "Bytes",
        percent: "Bytes",
    },
    /**
     * Lookup165: core_primitives::assertion::achainable::AchainableDate
     **/
    CorePrimitivesAssertionAchainableAchainableDate: {
        name: "Bytes",
        chain: "Vec<CorePrimitivesAssertionNetworkWeb3Network>",
        date: "Bytes",
    },
    /**
     * Lookup166: core_primitives::assertion::achainable::AchainableToken
     **/
    CorePrimitivesAssertionAchainableAchainableToken: {
        name: "Bytes",
        chain: "Vec<CorePrimitivesAssertionNetworkWeb3Network>",
        token: "Bytes",
    },
    /**
     * Lookup167: core_primitives::assertion::achainable::AchainableMirror
     **/
    CorePrimitivesAssertionAchainableAchainableMirror: {
        name: "Bytes",
        chain: "Vec<CorePrimitivesAssertionNetworkWeb3Network>",
        postQuantity: "Option<Bytes>",
    },
    /**
     * Lookup168: core_primitives::assertion::oneblock::OneBlockCourseType
     **/
    CorePrimitivesAssertionOneblockOneBlockCourseType: {
        _enum: ["CourseCompletion", "CourseOutstanding", "CourseParticipation"],
    },
    /**
     * Lookup169: core_primitives::assertion::generic_discord_role::GenericDiscordRoleType
     **/
    CorePrimitivesAssertionGenericDiscordRoleGenericDiscordRoleType: {
        _enum: {
            Contest: "CorePrimitivesAssertionContestContestType",
            SoraQuiz: "CorePrimitivesAssertionSoraquizSoraQuizType",
        },
    },
    /**
     * Lookup170: core_primitives::assertion::contest::ContestType
     **/
    CorePrimitivesAssertionContestContestType: {
        _enum: ["Legend", "Popularity", "Participant"],
    },
    /**
     * Lookup171: core_primitives::assertion::soraquiz::SoraQuizType
     **/
    CorePrimitivesAssertionSoraquizSoraQuizType: {
        _enum: ["Attendee", "Master"],
    },
    /**
     * Lookup172: core_primitives::assertion::bnb_domain::BnbDigitDomainType
     **/
    CorePrimitivesAssertionBnbDomainBnbDigitDomainType: {
        _enum: ["Bnb999ClubMember", "Bnb10kClubMember"],
    },
    /**
     * Lookup173: core_primitives::assertion::vip3::VIP3MembershipCardLevel
     **/
    CorePrimitivesAssertionVip3Vip3MembershipCardLevel: {
        _enum: ["Gold", "Silver"],
    },
    /**
     * Lookup174: core_primitives::assertion::evm_amount_holding::EVMTokenType
     **/
    CorePrimitivesAssertionEvmAmountHoldingEvmTokenType: {
        _enum: ["Ton", "Trx"],
    },
    /**
     * Lookup175: core_primitives::assertion::web3_token::Web3TokenType
     **/
    CorePrimitivesAssertionWeb3TokenWeb3TokenType: {
        _enum: [
            "Bnb",
            "Eth",
            "SpaceId",
            "Lit",
            "Wbtc",
            "Usdc",
            "Usdt",
            "Crv",
            "Matic",
            "Dydx",
            "Amp",
            "Cvx",
            "Tusd",
            "Usdd",
            "Gusd",
            "Link",
            "Grt",
            "Comp",
            "People",
            "Gtc",
            "Ton",
            "Trx",
            "Nfp",
            "Sol",
            "Mcrt",
            "Btc",
            "Ada",
            "Doge",
            "Shib",
            "Uni",
            "Bch",
            "Etc",
            "Atom",
            "Dai",
            "Leo",
            "Fil",
            "Imx",
            "Cro",
            "Inj",
            "Bean",
        ],
    },
    /**
     * Lookup176: core_primitives::assertion::platform_user::PlatformUserType
     **/
    CorePrimitivesAssertionPlatformUserPlatformUserType: {
        _enum: ["KaratDaoUser", "MagicCraftStakingUser"],
    },
    /**
     * Lookup177: core_primitives::assertion::web3_nft::Web3NftType
     **/
    CorePrimitivesAssertionWeb3NftWeb3NftType: {
        _enum: ["WeirdoGhostGang", "Club3Sbt"],
    },
    /**
     * Lookup181: pallet_group::pallet::Event<T, I>
     **/
    PalletGroupEvent: {
        _enum: {
            GroupMemberAdded: "AccountId32",
            GroupMemberRemoved: "AccountId32",
        },
    },
    /**
     * Lookup183: pallet_bitacross::pallet::Event<T>
     **/
    PalletBitacrossEvent: {
        _enum: {
            AdminSet: {
                newAdmin: "Option<AccountId32>",
            },
            RelayerAdded: {
                who: "CorePrimitivesIdentity",
            },
            RelayerRemoved: {
                who: "CorePrimitivesIdentity",
            },
            BtcWalletGenerated: {
                pubKey: "[u8;33]",
                accountId: "AccountId32",
            },
            EthWalletGenerated: {
                pubKey: "[u8;33]",
            },
        },
    },
    /**
     * Lookup184: pallet_bitacross_mimic::pallet::Event<T>
     **/
    PalletBitacrossMimicEvent: {
        _enum: {
            BtcToEthSaved: {
                btc: "[u8;33]",
                txIndex: "u64",
            },
            EthToBtcSaved: {
                eth: "H160",
                txIndex: "u64",
            },
        },
    },
    /**
     * Lookup185: pallet_evm_assertions::pallet::Event<T>
     **/
    PalletEvmAssertionsEvent: {
        _enum: {
            AssertionCreated: {
                id: "H160",
                byteCode: "Bytes",
                secrets: "Vec<Bytes>",
            },
        },
    },
    /**
     * Lookup187: pallet_teebag::pallet::Event<T>
     **/
    PalletTeebagEvent: {
        _enum: {
            ModeSet: {
                newMode: "PalletTeebagOperationalMode",
            },
            AdminSet: {
                newAdmin: "Option<AccountId32>",
            },
            EnclaveAdded: {
                who: "AccountId32",
                workerType: "PalletTeebagWorkerType",
                url: "Bytes",
            },
            EnclaveRemoved: {
                who: "AccountId32",
            },
            OpaqueTaskPosted: {
                request: "PalletTeebagRsaRequest",
            },
            ParentchainBlockProcessed: {
                who: "AccountId32",
                blockNumber: "u32",
                blockHash: "H256",
                taskMerkleRoot: "H256",
            },
            SidechainBlockFinalized: {
                who: "AccountId32",
                sidechainBlockNumber: "u64",
            },
            ScheduledEnclaveSet: {
                workerType: "PalletTeebagWorkerType",
                sidechainBlockNumber: "u64",
                mrenclave: "[u8;32]",
            },
            ScheduledEnclaveRemoved: {
                workerType: "PalletTeebagWorkerType",
                sidechainBlockNumber: "u64",
            },
        },
    },
    /**
     * Lookup188: pallet_teebag::types::OperationalMode
     **/
    PalletTeebagOperationalMode: {
        _enum: ["Production", "Development", "Maintenance"],
    },
    /**
     * Lookup189: pallet_teebag::types::WorkerType
     **/
    PalletTeebagWorkerType: {
        _enum: ["Identity", "BitAcross"],
    },
    /**
     * Lookup190: pallet_teebag::types::RsaRequest
     **/
    PalletTeebagRsaRequest: {
        shard: "H256",
        payload: "Bytes",
    },
    /**
     * Lookup191: pallet_evm::pallet::Event<T>
     **/
    PalletEvmEvent: {
        _enum: {
            Log: {
                log: "EthereumLog",
            },
            Created: {
                address: "H160",
            },
            CreatedFailed: {
                address: "H160",
            },
            Executed: {
                address: "H160",
            },
            ExecutedFailed: {
                address: "H160",
            },
        },
    },
    /**
     * Lookup192: ethereum::log::Log
     **/
    EthereumLog: {
        address: "H160",
        topics: "Vec<H256>",
        data: "Bytes",
    },
    /**
     * Lookup194: pallet_ethereum::pallet::Event
     **/
    PalletEthereumEvent: {
        _enum: {
            Executed: {
                from: "H160",
                to: "H160",
                transactionHash: "H256",
                exitReason: "EvmCoreErrorExitReason",
                extraData: "Bytes",
            },
        },
    },
    /**
     * Lookup195: evm_core::error::ExitReason
     **/
    EvmCoreErrorExitReason: {
        _enum: {
            Succeed: "EvmCoreErrorExitSucceed",
            Error: "EvmCoreErrorExitError",
            Revert: "EvmCoreErrorExitRevert",
            Fatal: "EvmCoreErrorExitFatal",
        },
    },
    /**
     * Lookup196: evm_core::error::ExitSucceed
     **/
    EvmCoreErrorExitSucceed: {
        _enum: ["Stopped", "Returned", "Suicided"],
    },
    /**
     * Lookup197: evm_core::error::ExitError
     **/
    EvmCoreErrorExitError: {
        _enum: {
            StackUnderflow: "Null",
            StackOverflow: "Null",
            InvalidJump: "Null",
            InvalidRange: "Null",
            DesignatedInvalid: "Null",
            CallTooDeep: "Null",
            CreateCollision: "Null",
            CreateContractLimit: "Null",
            OutOfOffset: "Null",
            OutOfGas: "Null",
            OutOfFund: "Null",
            PCUnderflow: "Null",
            CreateEmpty: "Null",
            Other: "Text",
            MaxNonce: "Null",
            InvalidCode: "u8",
        },
    },
    /**
     * Lookup201: evm_core::error::ExitRevert
     **/
    EvmCoreErrorExitRevert: {
        _enum: ["Reverted"],
    },
    /**
     * Lookup202: evm_core::error::ExitFatal
     **/
    EvmCoreErrorExitFatal: {
        _enum: {
            NotSupported: "Null",
            UnhandledInterrupt: "Null",
            CallErrorAsFatal: "EvmCoreErrorExitError",
            Other: "Text",
        },
    },
    /**
     * Lookup203: pallet_sudo::pallet::Event<T>
     **/
    PalletSudoEvent: {
        _enum: {
            Sudid: {
                sudoResult: "Result<Null, SpRuntimeDispatchError>",
            },
            KeyChanged: {
                oldSudoer: "Option<AccountId32>",
            },
            SudoAsDone: {
                sudoResult: "Result<Null, SpRuntimeDispatchError>",
            },
        },
    },
    /**
     * Lookup204: frame_system::Phase
     **/
    FrameSystemPhase: {
        _enum: {
            ApplyExtrinsic: "u32",
            Finalization: "Null",
            Initialization: "Null",
        },
    },
    /**
     * Lookup206: frame_system::LastRuntimeUpgradeInfo
     **/
    FrameSystemLastRuntimeUpgradeInfo: {
        specVersion: "Compact<u32>",
        specName: "Text",
    },
    /**
     * Lookup207: frame_system::pallet::Call<T>
     **/
    FrameSystemCall: {
        _enum: {
            remark: {
                remark: "Bytes",
            },
            set_heap_pages: {
                pages: "u64",
            },
            set_code: {
                code: "Bytes",
            },
            set_code_without_checks: {
                code: "Bytes",
            },
            set_storage: {
                items: "Vec<(Bytes,Bytes)>",
            },
            kill_storage: {
                _alias: {
                    keys_: "keys",
                },
                keys_: "Vec<Bytes>",
            },
            kill_prefix: {
                prefix: "Bytes",
                subkeys: "u32",
            },
            remark_with_event: {
                remark: "Bytes",
            },
        },
    },
    /**
     * Lookup210: frame_system::limits::BlockWeights
     **/
    FrameSystemLimitsBlockWeights: {
        baseBlock: "SpWeightsWeightV2Weight",
        maxBlock: "SpWeightsWeightV2Weight",
        perClass: "FrameSupportDispatchPerDispatchClassWeightsPerClass",
    },
    /**
     * Lookup211: frame_support::dispatch::PerDispatchClass<frame_system::limits::WeightsPerClass>
     **/
    FrameSupportDispatchPerDispatchClassWeightsPerClass: {
        normal: "FrameSystemLimitsWeightsPerClass",
        operational: "FrameSystemLimitsWeightsPerClass",
        mandatory: "FrameSystemLimitsWeightsPerClass",
    },
    /**
     * Lookup212: frame_system::limits::WeightsPerClass
     **/
    FrameSystemLimitsWeightsPerClass: {
        baseExtrinsic: "SpWeightsWeightV2Weight",
        maxExtrinsic: "Option<SpWeightsWeightV2Weight>",
        maxTotal: "Option<SpWeightsWeightV2Weight>",
        reserved: "Option<SpWeightsWeightV2Weight>",
    },
    /**
     * Lookup214: frame_system::limits::BlockLength
     **/
    FrameSystemLimitsBlockLength: {
        max: "FrameSupportDispatchPerDispatchClassU32",
    },
    /**
     * Lookup215: frame_support::dispatch::PerDispatchClass<T>
     **/
    FrameSupportDispatchPerDispatchClassU32: {
        normal: "u32",
        operational: "u32",
        mandatory: "u32",
    },
    /**
     * Lookup216: sp_weights::RuntimeDbWeight
     **/
    SpWeightsRuntimeDbWeight: {
        read: "u64",
        write: "u64",
    },
    /**
     * Lookup217: sp_version::RuntimeVersion
     **/
    SpVersionRuntimeVersion: {
        specName: "Text",
        implName: "Text",
        authoringVersion: "u32",
        specVersion: "u32",
        implVersion: "u32",
        apis: "Vec<([u8;8],u32)>",
        transactionVersion: "u32",
        stateVersion: "u8",
    },
    /**
     * Lookup221: frame_system::pallet::Error<T>
     **/
    FrameSystemError: {
        _enum: [
            "InvalidSpecName",
            "SpecVersionNeedsToIncrease",
            "FailedToExtractRuntimeVersion",
            "NonDefaultComposite",
            "NonZeroRefCount",
            "CallFiltered",
        ],
    },
    /**
     * Lookup222: pallet_timestamp::pallet::Call<T>
     **/
    PalletTimestampCall: {
        _enum: {
            set: {
                now: "Compact<u64>",
            },
        },
    },
    /**
     * Lookup225: pallet_scheduler::Scheduled<Name, frame_support::traits::preimages::Bounded<rococo_parachain_runtime::RuntimeCall>, BlockNumber, rococo_parachain_runtime::OriginCaller, sp_core::crypto::AccountId32>
     **/
    PalletSchedulerScheduled: {
        maybeId: "Option<[u8;32]>",
        priority: "u8",
        call: "FrameSupportPreimagesBounded",
        maybePeriodic: "Option<(u32,u32)>",
        origin: "RococoParachainRuntimeOriginCaller",
    },
    /**
     * Lookup226: frame_support::traits::preimages::Bounded<rococo_parachain_runtime::RuntimeCall>
     **/
    FrameSupportPreimagesBounded: {
        _enum: {
            Legacy: {
                _alias: {
                    hash_: "hash",
                },
                hash_: "H256",
            },
            Inline: "Bytes",
            Lookup: {
                _alias: {
                    hash_: "hash",
                },
                hash_: "H256",
                len: "u32",
            },
        },
    },
    /**
     * Lookup228: pallet_scheduler::pallet::Call<T>
     **/
    PalletSchedulerCall: {
        _enum: {
            schedule: {
                when: "u32",
                maybePeriodic: "Option<(u32,u32)>",
                priority: "u8",
                call: "Call",
            },
            cancel: {
                when: "u32",
                index: "u32",
            },
            schedule_named: {
                id: "[u8;32]",
                when: "u32",
                maybePeriodic: "Option<(u32,u32)>",
                priority: "u8",
                call: "Call",
            },
            cancel_named: {
                id: "[u8;32]",
            },
            schedule_after: {
                after: "u32",
                maybePeriodic: "Option<(u32,u32)>",
                priority: "u8",
                call: "Call",
            },
            schedule_named_after: {
                id: "[u8;32]",
                after: "u32",
                maybePeriodic: "Option<(u32,u32)>",
                priority: "u8",
                call: "Call",
            },
        },
    },
    /**
     * Lookup230: pallet_utility::pallet::Call<T>
     **/
    PalletUtilityCall: {
        _enum: {
            batch: {
                calls: "Vec<Call>",
            },
            as_derivative: {
                index: "u16",
                call: "Call",
            },
            batch_all: {
                calls: "Vec<Call>",
            },
            dispatch_as: {
                asOrigin: "RococoParachainRuntimeOriginCaller",
                call: "Call",
            },
            force_batch: {
                calls: "Vec<Call>",
            },
            with_weight: {
                call: "Call",
                weight: "SpWeightsWeightV2Weight",
            },
        },
    },
    /**
     * Lookup232: rococo_parachain_runtime::OriginCaller
     **/
    RococoParachainRuntimeOriginCaller: {
        _enum: {
            system: "FrameSupportDispatchRawOrigin",
            __Unused1: "Null",
            __Unused2: "Null",
            __Unused3: "Null",
            __Unused4: "Null",
            __Unused5: "Null",
            Void: "SpCoreVoid",
            __Unused7: "Null",
            __Unused8: "Null",
            __Unused9: "Null",
            __Unused10: "Null",
            __Unused11: "Null",
            __Unused12: "Null",
            __Unused13: "Null",
            __Unused14: "Null",
            __Unused15: "Null",
            __Unused16: "Null",
            __Unused17: "Null",
            __Unused18: "Null",
            __Unused19: "Null",
            __Unused20: "Null",
            __Unused21: "Null",
            Council: "PalletCollectiveRawOrigin",
            __Unused23: "Null",
            TechnicalCommittee: "PalletCollectiveRawOrigin",
            __Unused25: "Null",
            __Unused26: "Null",
            __Unused27: "Null",
            __Unused28: "Null",
            __Unused29: "Null",
            __Unused30: "Null",
            __Unused31: "Null",
            __Unused32: "Null",
            __Unused33: "Null",
            __Unused34: "Null",
            __Unused35: "Null",
            __Unused36: "Null",
            __Unused37: "Null",
            __Unused38: "Null",
            __Unused39: "Null",
            __Unused40: "Null",
            __Unused41: "Null",
            __Unused42: "Null",
            __Unused43: "Null",
            __Unused44: "Null",
            __Unused45: "Null",
            __Unused46: "Null",
            __Unused47: "Null",
            __Unused48: "Null",
            __Unused49: "Null",
            __Unused50: "Null",
            PolkadotXcm: "PalletXcmOrigin",
            CumulusXcm: "CumulusPalletXcmOrigin",
            __Unused53: "Null",
            __Unused54: "Null",
            __Unused55: "Null",
            __Unused56: "Null",
            __Unused57: "Null",
            __Unused58: "Null",
            __Unused59: "Null",
            __Unused60: "Null",
            __Unused61: "Null",
            __Unused62: "Null",
            __Unused63: "Null",
            __Unused64: "Null",
            __Unused65: "Null",
            __Unused66: "Null",
            __Unused67: "Null",
            __Unused68: "Null",
            __Unused69: "Null",
            __Unused70: "Null",
            __Unused71: "Null",
            __Unused72: "Null",
            __Unused73: "Null",
            __Unused74: "Null",
            __Unused75: "Null",
            __Unused76: "Null",
            __Unused77: "Null",
            __Unused78: "Null",
            __Unused79: "Null",
            __Unused80: "Null",
            __Unused81: "Null",
            __Unused82: "Null",
            __Unused83: "Null",
            __Unused84: "Null",
            __Unused85: "Null",
            __Unused86: "Null",
            __Unused87: "Null",
            __Unused88: "Null",
            __Unused89: "Null",
            __Unused90: "Null",
            __Unused91: "Null",
            __Unused92: "Null",
            __Unused93: "Null",
            __Unused94: "Null",
            __Unused95: "Null",
            __Unused96: "Null",
            __Unused97: "Null",
            __Unused98: "Null",
            __Unused99: "Null",
            __Unused100: "Null",
            __Unused101: "Null",
            __Unused102: "Null",
            __Unused103: "Null",
            __Unused104: "Null",
            __Unused105: "Null",
            __Unused106: "Null",
            __Unused107: "Null",
            __Unused108: "Null",
            __Unused109: "Null",
            __Unused110: "Null",
            __Unused111: "Null",
            __Unused112: "Null",
            __Unused113: "Null",
            __Unused114: "Null",
            __Unused115: "Null",
            __Unused116: "Null",
            __Unused117: "Null",
            __Unused118: "Null",
            __Unused119: "Null",
            __Unused120: "Null",
            Ethereum: "PalletEthereumRawOrigin",
        },
    },
    /**
     * Lookup233: frame_support::dispatch::RawOrigin<sp_core::crypto::AccountId32>
     **/
    FrameSupportDispatchRawOrigin: {
        _enum: {
            Root: "Null",
            Signed: "AccountId32",
            None: "Null",
        },
    },
    /**
     * Lookup234: pallet_collective::RawOrigin<sp_core::crypto::AccountId32, I>
     **/
    PalletCollectiveRawOrigin: {
        _enum: {
            Members: "(u32,u32)",
            Member: "AccountId32",
            _Phantom: "Null",
        },
    },
    /**
     * Lookup236: pallet_xcm::pallet::Origin
     **/
    PalletXcmOrigin: {
        _enum: {
            Xcm: "XcmV3MultiLocation",
            Response: "XcmV3MultiLocation",
        },
    },
    /**
     * Lookup237: cumulus_pallet_xcm::pallet::Origin
     **/
    CumulusPalletXcmOrigin: {
        _enum: {
            Relay: "Null",
            SiblingParachain: "u32",
        },
    },
    /**
     * Lookup238: pallet_ethereum::RawOrigin
     **/
    PalletEthereumRawOrigin: {
        _enum: {
            EthereumTransaction: "H160",
        },
    },
    /**
     * Lookup239: sp_core::Void
     **/
    SpCoreVoid: "Null",
    /**
     * Lookup240: pallet_multisig::pallet::Call<T>
     **/
    PalletMultisigCall: {
        _enum: {
            as_multi_threshold_1: {
                otherSignatories: "Vec<AccountId32>",
                call: "Call",
            },
            as_multi: {
                threshold: "u16",
                otherSignatories: "Vec<AccountId32>",
                maybeTimepoint: "Option<PalletMultisigTimepoint>",
                call: "Call",
                maxWeight: "SpWeightsWeightV2Weight",
            },
            approve_as_multi: {
                threshold: "u16",
                otherSignatories: "Vec<AccountId32>",
                maybeTimepoint: "Option<PalletMultisigTimepoint>",
                callHash: "[u8;32]",
                maxWeight: "SpWeightsWeightV2Weight",
            },
            cancel_as_multi: {
                threshold: "u16",
                otherSignatories: "Vec<AccountId32>",
                timepoint: "PalletMultisigTimepoint",
                callHash: "[u8;32]",
            },
        },
    },
    /**
     * Lookup243: pallet_proxy::pallet::Call<T>
     **/
    PalletProxyCall: {
        _enum: {
            proxy: {
                real: "MultiAddress",
                forceProxyType: "Option<RococoParachainRuntimeProxyType>",
                call: "Call",
            },
            add_proxy: {
                delegate: "MultiAddress",
                proxyType: "RococoParachainRuntimeProxyType",
                delay: "u32",
            },
            remove_proxy: {
                delegate: "MultiAddress",
                proxyType: "RococoParachainRuntimeProxyType",
                delay: "u32",
            },
            remove_proxies: "Null",
            create_pure: {
                proxyType: "RococoParachainRuntimeProxyType",
                delay: "u32",
                index: "u16",
            },
            kill_pure: {
                spawner: "MultiAddress",
                proxyType: "RococoParachainRuntimeProxyType",
                index: "u16",
                height: "Compact<u32>",
                extIndex: "Compact<u32>",
            },
            announce: {
                real: "MultiAddress",
                callHash: "H256",
            },
            remove_announcement: {
                real: "MultiAddress",
                callHash: "H256",
            },
            reject_announcement: {
                delegate: "MultiAddress",
                callHash: "H256",
            },
            proxy_announced: {
                delegate: "MultiAddress",
                real: "MultiAddress",
                forceProxyType: "Option<RococoParachainRuntimeProxyType>",
                call: "Call",
            },
        },
    },
    /**
     * Lookup247: pallet_preimage::pallet::Call<T>
     **/
    PalletPreimageCall: {
        _enum: {
            note_preimage: {
                bytes: "Bytes",
            },
            unnote_preimage: {
                _alias: {
                    hash_: "hash",
                },
                hash_: "H256",
            },
            request_preimage: {
                _alias: {
                    hash_: "hash",
                },
                hash_: "H256",
            },
            unrequest_preimage: {
                _alias: {
                    hash_: "hash",
                },
                hash_: "H256",
            },
        },
    },
    /**
     * Lookup248: pallet_balances::pallet::Call<T, I>
     **/
    PalletBalancesCall: {
        _enum: {
            transfer_allow_death: {
                dest: "MultiAddress",
                value: "Compact<u128>",
            },
            set_balance_deprecated: {
                who: "MultiAddress",
                newFree: "Compact<u128>",
                oldReserved: "Compact<u128>",
            },
            force_transfer: {
                source: "MultiAddress",
                dest: "MultiAddress",
                value: "Compact<u128>",
            },
            transfer_keep_alive: {
                dest: "MultiAddress",
                value: "Compact<u128>",
            },
            transfer_all: {
                dest: "MultiAddress",
                keepAlive: "bool",
            },
            force_unreserve: {
                who: "MultiAddress",
                amount: "u128",
            },
            upgrade_accounts: {
                who: "Vec<AccountId32>",
            },
            transfer: {
                dest: "MultiAddress",
                value: "Compact<u128>",
            },
            force_set_balance: {
                who: "MultiAddress",
                newFree: "Compact<u128>",
            },
        },
    },
    /**
     * Lookup249: pallet_vesting::pallet::Call<T>
     **/
    PalletVestingCall: {
        _enum: {
            vest: "Null",
            vest_other: {
                target: "MultiAddress",
            },
            vested_transfer: {
                target: "MultiAddress",
                schedule: "PalletVestingVestingInfo",
            },
            force_vested_transfer: {
                source: "MultiAddress",
                target: "MultiAddress",
                schedule: "PalletVestingVestingInfo",
            },
            merge_schedules: {
                schedule1Index: "u32",
                schedule2Index: "u32",
            },
        },
    },
    /**
     * Lookup250: pallet_vesting::vesting_info::VestingInfo<Balance, BlockNumber>
     **/
    PalletVestingVestingInfo: {
        locked: "u128",
        perBlock: "u128",
        startingBlock: "u32",
    },
    /**
     * Lookup251: pallet_treasury::pallet::Call<T, I>
     **/
    PalletTreasuryCall: {
        _enum: {
            propose_spend: {
                value: "Compact<u128>",
                beneficiary: "MultiAddress",
            },
            reject_proposal: {
                proposalId: "Compact<u32>",
            },
            approve_proposal: {
                proposalId: "Compact<u32>",
            },
            spend: {
                amount: "Compact<u128>",
                beneficiary: "MultiAddress",
            },
            remove_approval: {
                proposalId: "Compact<u32>",
            },
        },
    },
    /**
     * Lookup252: pallet_democracy::pallet::Call<T>
     **/
    PalletDemocracyCall: {
        _enum: {
            propose: {
                proposal: "FrameSupportPreimagesBounded",
                value: "Compact<u128>",
            },
            second: {
                proposal: "Compact<u32>",
            },
            vote: {
                refIndex: "Compact<u32>",
                vote: "PalletDemocracyVoteAccountVote",
            },
            emergency_cancel: {
                refIndex: "u32",
            },
            external_propose: {
                proposal: "FrameSupportPreimagesBounded",
            },
            external_propose_majority: {
                proposal: "FrameSupportPreimagesBounded",
            },
            external_propose_default: {
                proposal: "FrameSupportPreimagesBounded",
            },
            fast_track: {
                proposalHash: "H256",
                votingPeriod: "u32",
                delay: "u32",
            },
            veto_external: {
                proposalHash: "H256",
            },
            cancel_referendum: {
                refIndex: "Compact<u32>",
            },
            delegate: {
                to: "MultiAddress",
                conviction: "PalletDemocracyConviction",
                balance: "u128",
            },
            undelegate: "Null",
            clear_public_proposals: "Null",
            unlock: {
                target: "MultiAddress",
            },
            remove_vote: {
                index: "u32",
            },
            remove_other_vote: {
                target: "MultiAddress",
                index: "u32",
            },
            blacklist: {
                proposalHash: "H256",
                maybeRefIndex: "Option<u32>",
            },
            cancel_proposal: {
                propIndex: "Compact<u32>",
            },
            set_metadata: {
                owner: "PalletDemocracyMetadataOwner",
                maybeHash: "Option<H256>",
            },
        },
    },
    /**
     * Lookup253: pallet_democracy::conviction::Conviction
     **/
    PalletDemocracyConviction: {
        _enum: ["None", "Locked1x", "Locked2x", "Locked3x", "Locked4x", "Locked5x", "Locked6x"],
    },
    /**
     * Lookup256: pallet_collective::pallet::Call<T, I>
     **/
    PalletCollectiveCall: {
        _enum: {
            set_members: {
                newMembers: "Vec<AccountId32>",
                prime: "Option<AccountId32>",
                oldCount: "u32",
            },
            execute: {
                proposal: "Call",
                lengthBound: "Compact<u32>",
            },
            propose: {
                threshold: "Compact<u32>",
                proposal: "Call",
                lengthBound: "Compact<u32>",
            },
            vote: {
                proposal: "H256",
                index: "Compact<u32>",
                approve: "bool",
            },
            __Unused4: "Null",
            disapprove_proposal: {
                proposalHash: "H256",
            },
            close: {
                proposalHash: "H256",
                index: "Compact<u32>",
                proposalWeightBound: "SpWeightsWeightV2Weight",
                lengthBound: "Compact<u32>",
            },
        },
    },
    /**
     * Lookup257: pallet_membership::pallet::Call<T, I>
     **/
    PalletMembershipCall: {
        _enum: {
            add_member: {
                who: "MultiAddress",
            },
            remove_member: {
                who: "MultiAddress",
            },
            swap_member: {
                remove: "MultiAddress",
                add: "MultiAddress",
            },
            reset_members: {
                members: "Vec<AccountId32>",
            },
            change_key: {
                _alias: {
                    new_: "new",
                },
                new_: "MultiAddress",
            },
            set_prime: {
                who: "MultiAddress",
            },
            clear_prime: "Null",
        },
    },
    /**
     * Lookup260: pallet_bounties::pallet::Call<T, I>
     **/
    PalletBountiesCall: {
        _enum: {
            propose_bounty: {
                value: "Compact<u128>",
                description: "Bytes",
            },
            approve_bounty: {
                bountyId: "Compact<u32>",
            },
            propose_curator: {
                bountyId: "Compact<u32>",
                curator: "MultiAddress",
                fee: "Compact<u128>",
            },
            unassign_curator: {
                bountyId: "Compact<u32>",
            },
            accept_curator: {
                bountyId: "Compact<u32>",
            },
            award_bounty: {
                bountyId: "Compact<u32>",
                beneficiary: "MultiAddress",
            },
            claim_bounty: {
                bountyId: "Compact<u32>",
            },
            close_bounty: {
                bountyId: "Compact<u32>",
            },
            extend_bounty_expiry: {
                bountyId: "Compact<u32>",
                remark: "Bytes",
            },
        },
    },
    /**
     * Lookup261: pallet_tips::pallet::Call<T, I>
     **/
    PalletTipsCall: {
        _enum: {
            report_awesome: {
                reason: "Bytes",
                who: "MultiAddress",
            },
            retract_tip: {
                _alias: {
                    hash_: "hash",
                },
                hash_: "H256",
            },
            tip_new: {
                reason: "Bytes",
                who: "MultiAddress",
                tipValue: "Compact<u128>",
            },
            tip: {
                _alias: {
                    hash_: "hash",
                },
                hash_: "H256",
                tipValue: "Compact<u128>",
            },
            close_tip: {
                _alias: {
                    hash_: "hash",
                },
                hash_: "H256",
            },
            slash_tip: {
                _alias: {
                    hash_: "hash",
                },
                hash_: "H256",
            },
        },
    },
    /**
     * Lookup262: pallet_identity::pallet::Call<T>
     **/
    PalletIdentityCall: {
        _enum: {
            add_registrar: {
                account: "MultiAddress",
            },
            set_identity: {
                info: "PalletIdentityIdentityInfo",
            },
            set_subs: {
                subs: "Vec<(AccountId32,Data)>",
            },
            clear_identity: "Null",
            request_judgement: {
                regIndex: "Compact<u32>",
                maxFee: "Compact<u128>",
            },
            cancel_request: {
                regIndex: "u32",
            },
            set_fee: {
                index: "Compact<u32>",
                fee: "Compact<u128>",
            },
            set_account_id: {
                _alias: {
                    new_: "new",
                },
                index: "Compact<u32>",
                new_: "MultiAddress",
            },
            set_fields: {
                index: "Compact<u32>",
                fields: "PalletIdentityBitFlags",
            },
            provide_judgement: {
                regIndex: "Compact<u32>",
                target: "MultiAddress",
                judgement: "PalletIdentityJudgement",
                identity: "H256",
            },
            kill_identity: {
                target: "MultiAddress",
            },
            add_sub: {
                sub: "MultiAddress",
                data: "Data",
            },
            rename_sub: {
                sub: "MultiAddress",
                data: "Data",
            },
            remove_sub: {
                sub: "MultiAddress",
            },
            quit_sub: "Null",
        },
    },
    /**
     * Lookup263: pallet_identity::types::IdentityInfo<FieldLimit>
     **/
    PalletIdentityIdentityInfo: {
        additional: "Vec<(Data,Data)>",
        display: "Data",
        legal: "Data",
        web: "Data",
        riot: "Data",
        email: "Data",
        pgpFingerprint: "Option<[u8;20]>",
        image: "Data",
        twitter: "Data",
    },
    /**
     * Lookup299: pallet_identity::types::BitFlags<pallet_identity::types::IdentityField>
     **/
    PalletIdentityBitFlags: {
        _bitLength: 64,
        Display: 1,
        Legal: 2,
        Web: 4,
        Riot: 8,
        Email: 16,
        PgpFingerprint: 32,
        Image: 64,
        Twitter: 128,
    },
    /**
     * Lookup300: pallet_identity::types::IdentityField
     **/
    PalletIdentityIdentityField: {
        _enum: [
            "__Unused0",
            "Display",
            "Legal",
            "__Unused3",
            "Web",
            "__Unused5",
            "__Unused6",
            "__Unused7",
            "Riot",
            "__Unused9",
            "__Unused10",
            "__Unused11",
            "__Unused12",
            "__Unused13",
            "__Unused14",
            "__Unused15",
            "Email",
            "__Unused17",
            "__Unused18",
            "__Unused19",
            "__Unused20",
            "__Unused21",
            "__Unused22",
            "__Unused23",
            "__Unused24",
            "__Unused25",
            "__Unused26",
            "__Unused27",
            "__Unused28",
            "__Unused29",
            "__Unused30",
            "__Unused31",
            "PgpFingerprint",
            "__Unused33",
            "__Unused34",
            "__Unused35",
            "__Unused36",
            "__Unused37",
            "__Unused38",
            "__Unused39",
            "__Unused40",
            "__Unused41",
            "__Unused42",
            "__Unused43",
            "__Unused44",
            "__Unused45",
            "__Unused46",
            "__Unused47",
            "__Unused48",
            "__Unused49",
            "__Unused50",
            "__Unused51",
            "__Unused52",
            "__Unused53",
            "__Unused54",
            "__Unused55",
            "__Unused56",
            "__Unused57",
            "__Unused58",
            "__Unused59",
            "__Unused60",
            "__Unused61",
            "__Unused62",
            "__Unused63",
            "Image",
            "__Unused65",
            "__Unused66",
            "__Unused67",
            "__Unused68",
            "__Unused69",
            "__Unused70",
            "__Unused71",
            "__Unused72",
            "__Unused73",
            "__Unused74",
            "__Unused75",
            "__Unused76",
            "__Unused77",
            "__Unused78",
            "__Unused79",
            "__Unused80",
            "__Unused81",
            "__Unused82",
            "__Unused83",
            "__Unused84",
            "__Unused85",
            "__Unused86",
            "__Unused87",
            "__Unused88",
            "__Unused89",
            "__Unused90",
            "__Unused91",
            "__Unused92",
            "__Unused93",
            "__Unused94",
            "__Unused95",
            "__Unused96",
            "__Unused97",
            "__Unused98",
            "__Unused99",
            "__Unused100",
            "__Unused101",
            "__Unused102",
            "__Unused103",
            "__Unused104",
            "__Unused105",
            "__Unused106",
            "__Unused107",
            "__Unused108",
            "__Unused109",
            "__Unused110",
            "__Unused111",
            "__Unused112",
            "__Unused113",
            "__Unused114",
            "__Unused115",
            "__Unused116",
            "__Unused117",
            "__Unused118",
            "__Unused119",
            "__Unused120",
            "__Unused121",
            "__Unused122",
            "__Unused123",
            "__Unused124",
            "__Unused125",
            "__Unused126",
            "__Unused127",
            "Twitter",
        ],
    },
    /**
     * Lookup301: pallet_identity::types::Judgement<Balance>
     **/
    PalletIdentityJudgement: {
        _enum: {
            Unknown: "Null",
            FeePaid: "u128",
            Reasonable: "Null",
            KnownGood: "Null",
            OutOfDate: "Null",
            LowQuality: "Null",
            Erroneous: "Null",
        },
    },
    /**
     * Lookup302: cumulus_pallet_parachain_system::pallet::Call<T>
     **/
    CumulusPalletParachainSystemCall: {
        _enum: {
            set_validation_data: {
                data: "CumulusPrimitivesParachainInherentParachainInherentData",
            },
            sudo_send_upward_message: {
                message: "Bytes",
            },
            authorize_upgrade: {
                codeHash: "H256",
                checkVersion: "bool",
            },
            enact_authorized_upgrade: {
                code: "Bytes",
            },
        },
    },
    /**
     * Lookup303: cumulus_primitives_parachain_inherent::ParachainInherentData
     **/
    CumulusPrimitivesParachainInherentParachainInherentData: {
        validationData: "PolkadotPrimitivesV4PersistedValidationData",
        relayChainState: "SpTrieStorageProof",
        downwardMessages: "Vec<PolkadotCorePrimitivesInboundDownwardMessage>",
        horizontalMessages: "BTreeMap<u32, Vec<PolkadotCorePrimitivesInboundHrmpMessage>>",
    },
    /**
     * Lookup304: polkadot_primitives::v4::PersistedValidationData<primitive_types::H256, N>
     **/
    PolkadotPrimitivesV4PersistedValidationData: {
        parentHead: "Bytes",
        relayParentNumber: "u32",
        relayParentStorageRoot: "H256",
        maxPovSize: "u32",
    },
    /**
     * Lookup306: sp_trie::storage_proof::StorageProof
     **/
    SpTrieStorageProof: {
        trieNodes: "BTreeSet<Bytes>",
    },
    /**
     * Lookup309: polkadot_core_primitives::InboundDownwardMessage<BlockNumber>
     **/
    PolkadotCorePrimitivesInboundDownwardMessage: {
        sentAt: "u32",
        msg: "Bytes",
    },
    /**
     * Lookup312: polkadot_core_primitives::InboundHrmpMessage<BlockNumber>
     **/
    PolkadotCorePrimitivesInboundHrmpMessage: {
        sentAt: "u32",
        data: "Bytes",
    },
    /**
     * Lookup315: parachain_info::pallet::Call<T>
     **/
    ParachainInfoCall: "Null",
    /**
     * Lookup316: pallet_session::pallet::Call<T>
     **/
    PalletSessionCall: {
        _enum: {
            set_keys: {
                _alias: {
                    keys_: "keys",
                },
                keys_: "RococoParachainRuntimeSessionKeys",
                proof: "Bytes",
            },
            purge_keys: "Null",
        },
    },
    /**
     * Lookup317: rococo_parachain_runtime::SessionKeys
     **/
    RococoParachainRuntimeSessionKeys: {
        aura: "SpConsensusAuraSr25519AppSr25519Public",
    },
    /**
     * Lookup318: sp_consensus_aura::sr25519::app_sr25519::Public
     **/
    SpConsensusAuraSr25519AppSr25519Public: "SpCoreSr25519Public",
    /**
     * Lookup319: sp_core::sr25519::Public
     **/
    SpCoreSr25519Public: "[u8;32]",
    /**
     * Lookup320: pallet_parachain_staking::pallet::Call<T>
     **/
    PalletParachainStakingCall: {
        _enum: {
            set_staking_expectations: {
                expectations: {
                    min: "u128",
                    ideal: "u128",
                    max: "u128",
                },
            },
            set_inflation: {
                schedule: {
                    min: "Perbill",
                    ideal: "Perbill",
                    max: "Perbill",
                },
            },
            set_parachain_bond_account: {
                _alias: {
                    new_: "new",
                },
                new_: "AccountId32",
            },
            set_parachain_bond_reserve_percent: {
                _alias: {
                    new_: "new",
                },
                new_: "Percent",
            },
            set_total_selected: {
                _alias: {
                    new_: "new",
                },
                new_: "u32",
            },
            set_collator_commission: {
                _alias: {
                    new_: "new",
                },
                new_: "Perbill",
            },
            set_blocks_per_round: {
                _alias: {
                    new_: "new",
                },
                new_: "u32",
            },
            add_candidates_whitelist: {
                candidate: "AccountId32",
            },
            remove_candidates_whitelist: {
                candidate: "AccountId32",
            },
            join_candidates: {
                bond: "u128",
            },
            schedule_leave_candidates: "Null",
            execute_leave_candidates: {
                candidate: "AccountId32",
            },
            cancel_leave_candidates: "Null",
            go_offline: "Null",
            go_online: "Null",
            candidate_bond_more: {
                more: "u128",
            },
            schedule_candidate_bond_less: {
                less: "u128",
            },
            execute_candidate_bond_less: {
                candidate: "AccountId32",
            },
            cancel_candidate_bond_less: "Null",
            delegate: {
                candidate: "AccountId32",
                amount: "u128",
            },
            delegate_with_auto_compound: {
                candidate: "AccountId32",
                amount: "u128",
                autoCompound: "Percent",
            },
            schedule_leave_delegators: "Null",
            execute_leave_delegators: {
                delegator: "AccountId32",
            },
            cancel_leave_delegators: "Null",
            schedule_revoke_delegation: {
                collator: "AccountId32",
            },
            delegator_bond_more: {
                candidate: "AccountId32",
                more: "u128",
            },
            schedule_delegator_bond_less: {
                candidate: "AccountId32",
                less: "u128",
            },
            execute_delegation_request: {
                delegator: "AccountId32",
                candidate: "AccountId32",
            },
            cancel_delegation_request: {
                candidate: "AccountId32",
            },
            set_auto_compound: {
                candidate: "AccountId32",
                value: "Percent",
            },
        },
    },
    /**
     * Lookup323: cumulus_pallet_xcmp_queue::pallet::Call<T>
     **/
    CumulusPalletXcmpQueueCall: {
        _enum: {
            service_overweight: {
                index: "u64",
                weightLimit: "SpWeightsWeightV2Weight",
            },
            suspend_xcm_execution: "Null",
            resume_xcm_execution: "Null",
            update_suspend_threshold: {
                _alias: {
                    new_: "new",
                },
                new_: "u32",
            },
            update_drop_threshold: {
                _alias: {
                    new_: "new",
                },
                new_: "u32",
            },
            update_resume_threshold: {
                _alias: {
                    new_: "new",
                },
                new_: "u32",
            },
            update_threshold_weight: {
                _alias: {
                    new_: "new",
                },
                new_: "SpWeightsWeightV2Weight",
            },
            update_weight_restrict_decay: {
                _alias: {
                    new_: "new",
                },
                new_: "SpWeightsWeightV2Weight",
            },
            update_xcmp_max_individual_weight: {
                _alias: {
                    new_: "new",
                },
                new_: "SpWeightsWeightV2Weight",
            },
        },
    },
    /**
     * Lookup324: pallet_xcm::pallet::Call<T>
     **/
    PalletXcmCall: {
        _enum: {
            send: {
                dest: "XcmVersionedMultiLocation",
                message: "XcmVersionedXcm",
            },
            teleport_assets: {
                dest: "XcmVersionedMultiLocation",
                beneficiary: "XcmVersionedMultiLocation",
                assets: "XcmVersionedMultiAssets",
                feeAssetItem: "u32",
            },
            reserve_transfer_assets: {
                dest: "XcmVersionedMultiLocation",
                beneficiary: "XcmVersionedMultiLocation",
                assets: "XcmVersionedMultiAssets",
                feeAssetItem: "u32",
            },
            execute: {
                message: "XcmVersionedXcm",
                maxWeight: "SpWeightsWeightV2Weight",
            },
            force_xcm_version: {
                location: "XcmV3MultiLocation",
                xcmVersion: "u32",
            },
            force_default_xcm_version: {
                maybeXcmVersion: "Option<u32>",
            },
            force_subscribe_version_notify: {
                location: "XcmVersionedMultiLocation",
            },
            force_unsubscribe_version_notify: {
                location: "XcmVersionedMultiLocation",
            },
            limited_reserve_transfer_assets: {
                dest: "XcmVersionedMultiLocation",
                beneficiary: "XcmVersionedMultiLocation",
                assets: "XcmVersionedMultiAssets",
                feeAssetItem: "u32",
                weightLimit: "XcmV3WeightLimit",
            },
            limited_teleport_assets: {
                dest: "XcmVersionedMultiLocation",
                beneficiary: "XcmVersionedMultiLocation",
                assets: "XcmVersionedMultiAssets",
                feeAssetItem: "u32",
                weightLimit: "XcmV3WeightLimit",
            },
            force_suspension: {
                suspended: "bool",
            },
        },
    },
    /**
     * Lookup325: xcm::VersionedXcm<RuntimeCall>
     **/
    XcmVersionedXcm: {
        _enum: {
            __Unused0: "Null",
            __Unused1: "Null",
            V2: "XcmV2Xcm",
            V3: "XcmV3Xcm",
        },
    },
    /**
     * Lookup326: xcm::v2::Xcm<RuntimeCall>
     **/
    XcmV2Xcm: "Vec<XcmV2Instruction>",
    /**
     * Lookup328: xcm::v2::Instruction<RuntimeCall>
     **/
    XcmV2Instruction: {
        _enum: {
            WithdrawAsset: "XcmV2MultiassetMultiAssets",
            ReserveAssetDeposited: "XcmV2MultiassetMultiAssets",
            ReceiveTeleportedAsset: "XcmV2MultiassetMultiAssets",
            QueryResponse: {
                queryId: "Compact<u64>",
                response: "XcmV2Response",
                maxWeight: "Compact<u64>",
            },
            TransferAsset: {
                assets: "XcmV2MultiassetMultiAssets",
                beneficiary: "XcmV2MultiLocation",
            },
            TransferReserveAsset: {
                assets: "XcmV2MultiassetMultiAssets",
                dest: "XcmV2MultiLocation",
                xcm: "XcmV2Xcm",
            },
            Transact: {
                originType: "XcmV2OriginKind",
                requireWeightAtMost: "Compact<u64>",
                call: "XcmDoubleEncoded",
            },
            HrmpNewChannelOpenRequest: {
                sender: "Compact<u32>",
                maxMessageSize: "Compact<u32>",
                maxCapacity: "Compact<u32>",
            },
            HrmpChannelAccepted: {
                recipient: "Compact<u32>",
            },
            HrmpChannelClosing: {
                initiator: "Compact<u32>",
                sender: "Compact<u32>",
                recipient: "Compact<u32>",
            },
            ClearOrigin: "Null",
            DescendOrigin: "XcmV2MultilocationJunctions",
            ReportError: {
                queryId: "Compact<u64>",
                dest: "XcmV2MultiLocation",
                maxResponseWeight: "Compact<u64>",
            },
            DepositAsset: {
                assets: "XcmV2MultiassetMultiAssetFilter",
                maxAssets: "Compact<u32>",
                beneficiary: "XcmV2MultiLocation",
            },
            DepositReserveAsset: {
                assets: "XcmV2MultiassetMultiAssetFilter",
                maxAssets: "Compact<u32>",
                dest: "XcmV2MultiLocation",
                xcm: "XcmV2Xcm",
            },
            ExchangeAsset: {
                give: "XcmV2MultiassetMultiAssetFilter",
                receive: "XcmV2MultiassetMultiAssets",
            },
            InitiateReserveWithdraw: {
                assets: "XcmV2MultiassetMultiAssetFilter",
                reserve: "XcmV2MultiLocation",
                xcm: "XcmV2Xcm",
            },
            InitiateTeleport: {
                assets: "XcmV2MultiassetMultiAssetFilter",
                dest: "XcmV2MultiLocation",
                xcm: "XcmV2Xcm",
            },
            QueryHolding: {
                queryId: "Compact<u64>",
                dest: "XcmV2MultiLocation",
                assets: "XcmV2MultiassetMultiAssetFilter",
                maxResponseWeight: "Compact<u64>",
            },
            BuyExecution: {
                fees: "XcmV2MultiAsset",
                weightLimit: "XcmV2WeightLimit",
            },
            RefundSurplus: "Null",
            SetErrorHandler: "XcmV2Xcm",
            SetAppendix: "XcmV2Xcm",
            ClearError: "Null",
            ClaimAsset: {
                assets: "XcmV2MultiassetMultiAssets",
                ticket: "XcmV2MultiLocation",
            },
            Trap: "Compact<u64>",
            SubscribeVersion: {
                queryId: "Compact<u64>",
                maxResponseWeight: "Compact<u64>",
            },
            UnsubscribeVersion: "Null",
        },
    },
    /**
     * Lookup329: xcm::v2::Response
     **/
    XcmV2Response: {
        _enum: {
            Null: "Null",
            Assets: "XcmV2MultiassetMultiAssets",
            ExecutionResult: "Option<(u32,XcmV2TraitsError)>",
            Version: "u32",
        },
    },
    /**
     * Lookup332: xcm::v2::traits::Error
     **/
    XcmV2TraitsError: {
        _enum: {
            Overflow: "Null",
            Unimplemented: "Null",
            UntrustedReserveLocation: "Null",
            UntrustedTeleportLocation: "Null",
            MultiLocationFull: "Null",
            MultiLocationNotInvertible: "Null",
            BadOrigin: "Null",
            InvalidLocation: "Null",
            AssetNotFound: "Null",
            FailedToTransactAsset: "Null",
            NotWithdrawable: "Null",
            LocationCannotHold: "Null",
            ExceedsMaxMessageSize: "Null",
            DestinationUnsupported: "Null",
            Transport: "Null",
            Unroutable: "Null",
            UnknownClaim: "Null",
            FailedToDecode: "Null",
            MaxWeightInvalid: "Null",
            NotHoldingFees: "Null",
            TooExpensive: "Null",
            Trap: "u64",
            UnhandledXcmVersion: "Null",
            WeightLimitReached: "u64",
            Barrier: "Null",
            WeightNotComputable: "Null",
        },
    },
    /**
     * Lookup333: xcm::v2::multiasset::MultiAssetFilter
     **/
    XcmV2MultiassetMultiAssetFilter: {
        _enum: {
            Definite: "XcmV2MultiassetMultiAssets",
            Wild: "XcmV2MultiassetWildMultiAsset",
        },
    },
    /**
     * Lookup334: xcm::v2::multiasset::WildMultiAsset
     **/
    XcmV2MultiassetWildMultiAsset: {
        _enum: {
            All: "Null",
            AllOf: {
                id: "XcmV2MultiassetAssetId",
                fun: "XcmV2MultiassetWildFungibility",
            },
        },
    },
    /**
     * Lookup335: xcm::v2::multiasset::WildFungibility
     **/
    XcmV2MultiassetWildFungibility: {
        _enum: ["Fungible", "NonFungible"],
    },
    /**
     * Lookup336: xcm::v2::WeightLimit
     **/
    XcmV2WeightLimit: {
        _enum: {
            Unlimited: "Null",
            Limited: "Compact<u64>",
        },
    },
    /**
     * Lookup345: cumulus_pallet_xcm::pallet::Call<T>
     **/
    CumulusPalletXcmCall: "Null",
    /**
     * Lookup346: cumulus_pallet_dmp_queue::pallet::Call<T>
     **/
    CumulusPalletDmpQueueCall: {
        _enum: {
            service_overweight: {
                index: "u64",
                weightLimit: "SpWeightsWeightV2Weight",
            },
        },
    },
    /**
     * Lookup347: orml_xtokens::module::Call<T>
     **/
    OrmlXtokensModuleCall: {
        _enum: {
            transfer: {
                currencyId: "RuntimeCommonXcmImplCurrencyId",
                amount: "u128",
                dest: "XcmVersionedMultiLocation",
                destWeightLimit: "XcmV3WeightLimit",
            },
            transfer_multiasset: {
                asset: "XcmVersionedMultiAsset",
                dest: "XcmVersionedMultiLocation",
                destWeightLimit: "XcmV3WeightLimit",
            },
            transfer_with_fee: {
                currencyId: "RuntimeCommonXcmImplCurrencyId",
                amount: "u128",
                fee: "u128",
                dest: "XcmVersionedMultiLocation",
                destWeightLimit: "XcmV3WeightLimit",
            },
            transfer_multiasset_with_fee: {
                asset: "XcmVersionedMultiAsset",
                fee: "XcmVersionedMultiAsset",
                dest: "XcmVersionedMultiLocation",
                destWeightLimit: "XcmV3WeightLimit",
            },
            transfer_multicurrencies: {
                currencies: "Vec<(RuntimeCommonXcmImplCurrencyId,u128)>",
                feeItem: "u32",
                dest: "XcmVersionedMultiLocation",
                destWeightLimit: "XcmV3WeightLimit",
            },
            transfer_multiassets: {
                assets: "XcmVersionedMultiAssets",
                feeItem: "u32",
                dest: "XcmVersionedMultiLocation",
                destWeightLimit: "XcmV3WeightLimit",
            },
        },
    },
    /**
     * Lookup348: xcm::VersionedMultiAsset
     **/
    XcmVersionedMultiAsset: {
        _enum: {
            __Unused0: "Null",
            V2: "XcmV2MultiAsset",
            __Unused2: "Null",
            V3: "XcmV3MultiAsset",
        },
    },
    /**
     * Lookup351: orml_tokens::module::Call<T>
     **/
    OrmlTokensModuleCall: {
        _enum: {
            transfer: {
                dest: "MultiAddress",
                currencyId: "u128",
                amount: "Compact<u128>",
            },
            transfer_all: {
                dest: "MultiAddress",
                currencyId: "u128",
                keepAlive: "bool",
            },
            transfer_keep_alive: {
                dest: "MultiAddress",
                currencyId: "u128",
                amount: "Compact<u128>",
            },
            force_transfer: {
                source: "MultiAddress",
                dest: "MultiAddress",
                currencyId: "u128",
                amount: "Compact<u128>",
            },
            set_balance: {
                who: "MultiAddress",
                currencyId: "u128",
                newFree: "Compact<u128>",
                newReserved: "Compact<u128>",
            },
        },
    },
    /**
     * Lookup352: pallet_bridge::pallet::Call<T>
     **/
    PalletBridgeCall: {
        _enum: {
            set_threshold: {
                threshold: "u32",
            },
            set_resource: {
                id: "[u8;32]",
                method: "Bytes",
            },
            remove_resource: {
                id: "[u8;32]",
            },
            whitelist_chain: {
                id: "u8",
            },
            add_relayer: {
                v: "AccountId32",
            },
            remove_relayer: {
                v: "AccountId32",
            },
            update_fee: {
                destId: "u8",
                fee: "u128",
            },
            acknowledge_proposal: {
                nonce: "u64",
                srcId: "u8",
                rId: "[u8;32]",
                call: "Call",
            },
            reject_proposal: {
                nonce: "u64",
                srcId: "u8",
                rId: "[u8;32]",
                call: "Call",
            },
            eval_vote_state: {
                nonce: "u64",
                srcId: "u8",
                prop: "Call",
            },
        },
    },
    /**
     * Lookup353: pallet_bridge_transfer::pallet::Call<T>
     **/
    PalletBridgeTransferCall: {
        _enum: {
            transfer_native: {
                amount: "u128",
                recipient: "Bytes",
                destId: "u8",
            },
            transfer: {
                to: "AccountId32",
                amount: "u128",
                rid: "[u8;32]",
            },
            set_maximum_issuance: {
                maximumIssuance: "u128",
            },
            set_external_balances: {
                externalBalances: "u128",
            },
        },
    },
    /**
     * Lookup354: pallet_extrinsic_filter::pallet::Call<T>
     **/
    PalletExtrinsicFilterCall: {
        _enum: {
            set_mode: {
                mode: "PalletExtrinsicFilterOperationalMode",
            },
            block_extrinsics: {
                palletNameBytes: "Bytes",
                functionNameBytes: "Option<Bytes>",
            },
            unblock_extrinsics: {
                palletNameBytes: "Bytes",
                functionNameBytes: "Option<Bytes>",
            },
        },
    },
    /**
     * Lookup355: pallet_identity_management::pallet::Call<T>
     **/
    PalletIdentityManagementCall: {
        _enum: {
            add_delegatee: {
                account: "AccountId32",
            },
            remove_delegatee: {
                account: "AccountId32",
            },
            __Unused2: "Null",
            link_identity: {
                shard: "H256",
                user: "AccountId32",
                encryptedIdentity: "Bytes",
                encryptedValidationData: "Bytes",
                encryptedWeb3networks: "Bytes",
            },
            deactivate_identity: {
                shard: "H256",
                encryptedIdentity: "Bytes",
            },
            activate_identity: {
                shard: "H256",
                encryptedIdentity: "Bytes",
            },
            __Unused6: "Null",
            __Unused7: "Null",
            __Unused8: "Null",
            __Unused9: "Null",
            __Unused10: "Null",
            __Unused11: "Null",
            __Unused12: "Null",
            __Unused13: "Null",
            __Unused14: "Null",
            __Unused15: "Null",
            __Unused16: "Null",
            __Unused17: "Null",
            __Unused18: "Null",
            __Unused19: "Null",
            __Unused20: "Null",
            __Unused21: "Null",
            __Unused22: "Null",
            __Unused23: "Null",
            __Unused24: "Null",
            __Unused25: "Null",
            __Unused26: "Null",
            __Unused27: "Null",
            __Unused28: "Null",
            __Unused29: "Null",
            __Unused30: "Null",
            identity_linked: {
                primeIdentity: "CorePrimitivesIdentity",
                idGraphHash: "H256",
                reqExtHash: "H256",
            },
            identity_deactivated: {
                primeIdentity: "CorePrimitivesIdentity",
                idGraphHash: "H256",
                reqExtHash: "H256",
            },
            identity_activated: {
                primeIdentity: "CorePrimitivesIdentity",
                idGraphHash: "H256",
                reqExtHash: "H256",
            },
            identity_networks_set: {
                primeIdentity: "CorePrimitivesIdentity",
                idGraphHash: "H256",
                reqExtHash: "H256",
            },
            some_error: {
                primeIdentity: "Option<CorePrimitivesIdentity>",
                error: "CorePrimitivesErrorImpError",
                reqExtHash: "H256",
            },
        },
    },
    /**
     * Lookup356: core_primitives::error::IMPError
     **/
    CorePrimitivesErrorImpError: {
        _enum: {
            LinkIdentityFailed: "CorePrimitivesErrorErrorDetail",
            DeactivateIdentityFailed: "CorePrimitivesErrorErrorDetail",
            ActivateIdentityFailed: "CorePrimitivesErrorErrorDetail",
            ImportScheduledEnclaveFailed: "Null",
            UnclassifiedError: "CorePrimitivesErrorErrorDetail",
        },
    },
    /**
     * Lookup357: pallet_asset_manager::pallet::Call<T>
     **/
    PalletAssetManagerCall: {
        _enum: {
            register_foreign_asset_type: {
                assetType: "RuntimeCommonXcmImplCurrencyId",
                metadata: "PalletAssetManagerAssetMetadata",
            },
            update_foreign_asset_metadata: {
                assetId: "u128",
                metadata: "PalletAssetManagerAssetMetadata",
            },
            set_asset_units_per_second: {
                assetId: "u128",
                unitsPerSecond: "u128",
            },
            add_asset_type: {
                assetId: "u128",
                newAssetType: "RuntimeCommonXcmImplCurrencyId",
            },
            remove_asset_type: {
                assetType: "RuntimeCommonXcmImplCurrencyId",
                newDefaultAssetType: "Option<RuntimeCommonXcmImplCurrencyId>",
            },
        },
    },
    /**
     * Lookup359: pallet_vc_management::pallet::Call<T>
     **/
    PalletVcManagementCall: {
        _enum: {
            add_delegatee: {
                account: "AccountId32",
            },
            remove_delegatee: {
                account: "AccountId32",
            },
            request_vc: {
                shard: "H256",
                assertion: "CorePrimitivesAssertion",
            },
            __Unused3: "Null",
            __Unused4: "Null",
            set_admin: {
                _alias: {
                    new_: "new",
                },
                new_: "AccountId32",
            },
            add_schema: {
                shard: "H256",
                id: "Bytes",
                content: "Bytes",
            },
            disable_schema: {
                shard: "H256",
                index: "u64",
            },
            activate_schema: {
                shard: "H256",
                index: "u64",
            },
            revoke_schema: {
                shard: "H256",
                index: "u64",
            },
            __Unused10: "Null",
            __Unused11: "Null",
            __Unused12: "Null",
            __Unused13: "Null",
            __Unused14: "Null",
            __Unused15: "Null",
            __Unused16: "Null",
            __Unused17: "Null",
            __Unused18: "Null",
            __Unused19: "Null",
            __Unused20: "Null",
            __Unused21: "Null",
            __Unused22: "Null",
            __Unused23: "Null",
            __Unused24: "Null",
            __Unused25: "Null",
            __Unused26: "Null",
            __Unused27: "Null",
            __Unused28: "Null",
            __Unused29: "Null",
            vc_issued: {
                identity: "CorePrimitivesIdentity",
                assertion: "CorePrimitivesAssertion",
                idGraphHash: "H256",
                reqExtHash: "H256",
            },
            some_error: {
                identity: "Option<CorePrimitivesIdentity>",
                error: "CorePrimitivesErrorVcmpError",
                reqExtHash: "H256",
            },
        },
    },
    /**
     * Lookup360: core_primitives::error::VCMPError
     **/
    CorePrimitivesErrorVcmpError: {
        _enum: {
            RequestVCFailed: "(CorePrimitivesAssertion,CorePrimitivesErrorErrorDetail)",
            UnclassifiedError: "CorePrimitivesErrorErrorDetail",
        },
    },
    /**
     * Lookup361: pallet_group::pallet::Call<T, I>
     **/
    PalletGroupCall: {
        _enum: {
            add_group_member: {
                v: "AccountId32",
            },
            batch_add_group_members: {
                vs: "Vec<AccountId32>",
            },
            remove_group_member: {
                v: "AccountId32",
            },
            batch_remove_group_members: {
                vs: "Vec<AccountId32>",
            },
            switch_group_control_on: "Null",
            switch_group_control_off: "Null",
        },
    },
    /**
     * Lookup363: pallet_bitacross::pallet::Call<T>
     **/
    PalletBitacrossCall: {
        _enum: {
            set_admin: {
                newAdmin: "AccountId32",
            },
            add_relayer: {
                account: "CorePrimitivesIdentity",
            },
            remove_relayer: {
                account: "CorePrimitivesIdentity",
            },
            __Unused3: "Null",
            __Unused4: "Null",
            __Unused5: "Null",
            __Unused6: "Null",
            __Unused7: "Null",
            __Unused8: "Null",
            __Unused9: "Null",
            __Unused10: "Null",
            __Unused11: "Null",
            __Unused12: "Null",
            __Unused13: "Null",
            __Unused14: "Null",
            __Unused15: "Null",
            __Unused16: "Null",
            __Unused17: "Null",
            __Unused18: "Null",
            __Unused19: "Null",
            __Unused20: "Null",
            __Unused21: "Null",
            __Unused22: "Null",
            __Unused23: "Null",
            __Unused24: "Null",
            __Unused25: "Null",
            __Unused26: "Null",
            __Unused27: "Null",
            __Unused28: "Null",
            __Unused29: "Null",
            btc_wallet_generated: {
                pubKey: "[u8;33]",
            },
            eth_wallet_generated: {
                pubKey: "[u8;33]",
            },
            task_complete: "Null",
        },
    },
    /**
     * Lookup364: pallet_bitacross_mimic::pallet::Call<T>
     **/
    PalletBitacrossMimicCall: {
        _enum: {
            write_btc_to_eth: {
                btcSender: "[u8;33]",
                data: "PalletBitacrossMimicCustodialTypeBtcToEth",
            },
            write_eth_to_btc: {
                ethSender: "H160",
                data: "PalletBitacrossMimicCustodialTypeEthToBtc",
            },
        },
    },
    /**
     * Lookup365: pallet_bitacross_mimic::custodial_type::BtcToEth
     **/
    PalletBitacrossMimicCustodialTypeBtcToEth: {
        txIndex: "u64",
        btcTxHash: "H256",
        ethereumReceiver: "H160",
        ethereumTxHash: "H256",
        ethTxStatus: "bool",
        symbol: "Text",
        amount: "U256",
        txTimestamp: "u64",
    },
    /**
     * Lookup368: pallet_bitacross_mimic::custodial_type::EthToBtc
     **/
    PalletBitacrossMimicCustodialTypeEthToBtc: {
        txIndex: "u64",
        btcTxHash: "H256",
        btcReceiver: "[u8;33]",
        btcReceiverLength: "u32",
        ethereumTxHash: "H256",
        ethTxStatus: "bool",
        symbol: "Text",
        amount: "U256",
        txTimestamp: "u64",
    },
    /**
     * Lookup369: pallet_evm_assertions::pallet::Call<T>
     **/
    PalletEvmAssertionsCall: {
        _enum: {
            create_assertion: {
                id: "H160",
                byteCode: "Bytes",
                secrets: "Vec<Bytes>",
            },
        },
    },
    /**
     * Lookup370: pallet_teebag::pallet::Call<T>
     **/
    PalletTeebagCall: {
        _enum: {
            set_admin: {
                newAdmin: "AccountId32",
            },
            set_mode: {
                newMode: "PalletTeebagOperationalMode",
            },
            force_add_enclave: {
                who: "AccountId32",
                enclave: "PalletTeebagEnclave",
            },
            force_remove_enclave: {
                who: "AccountId32",
            },
            force_remove_enclave_by_mrenclave: {
                mrenclave: "[u8;32]",
            },
            force_remove_enclave_by_worker_type: {
                workerType: "PalletTeebagWorkerType",
            },
            set_scheduled_enclave: {
                workerType: "PalletTeebagWorkerType",
                sidechainBlockNumber: "u64",
                mrenclave: "[u8;32]",
            },
            remove_scheduled_enclave: {
                workerType: "PalletTeebagWorkerType",
                sidechainBlockNumber: "u64",
            },
            register_enclave: {
                workerType: "PalletTeebagWorkerType",
                workerMode: "PalletTeebagWorkerMode",
                attestation: "Bytes",
                workerUrl: "Bytes",
                shieldingPubkey: "Option<Bytes>",
                vcPubkey: "Option<SpCoreEd25519Public>",
                attestationType: "PalletTeebagAttestationType",
            },
            unregister_enclave: "Null",
            register_quoting_enclave: {
                enclaveIdentity: "Bytes",
                signature: "Bytes",
                certificateChain: "Bytes",
            },
            register_tcb_info: {
                tcbInfo: "Bytes",
                signature: "Bytes",
                certificateChain: "Bytes",
            },
            __Unused12: "Null",
            __Unused13: "Null",
            __Unused14: "Null",
            __Unused15: "Null",
            __Unused16: "Null",
            __Unused17: "Null",
            __Unused18: "Null",
            __Unused19: "Null",
            post_opaque_task: {
                request: "PalletTeebagRsaRequest",
            },
            parentchain_block_processed: {
                blockHash: "H256",
                blockNumber: "u32",
                taskMerkleRoot: "H256",
            },
            sidechain_block_imported: {
                shard: "H256",
                blockNumber: "u64",
                nextFinalizationCandidateBlockNumber: "u64",
                blockHeaderHash: "H256",
            },
        },
    },
    /**
     * Lookup371: pallet_teebag::types::Enclave
     **/
    PalletTeebagEnclave: {
        workerType: "PalletTeebagWorkerType",
        workerMode: "PalletTeebagWorkerMode",
        mrenclave: "[u8;32]",
        lastSeenTimestamp: "u64",
        url: "Bytes",
        shieldingPubkey: "Option<Bytes>",
        vcPubkey: "Option<SpCoreEd25519Public>",
        sgxBuildMode: "PalletTeebagSgxBuildMode",
        attestationType: "PalletTeebagAttestationType",
    },
    /**
     * Lookup372: pallet_teebag::types::WorkerMode
     **/
    PalletTeebagWorkerMode: {
        _enum: ["OffChainWorker", "Sidechain"],
    },
    /**
     * Lookup374: sp_core::ed25519::Public
     **/
    SpCoreEd25519Public: "[u8;32]",
    /**
     * Lookup375: pallet_teebag::types::SgxBuildMode
     **/
    PalletTeebagSgxBuildMode: {
        _enum: ["Production", "Debug"],
    },
    /**
     * Lookup376: pallet_teebag::types::AttestationType
     **/
    PalletTeebagAttestationType: {
        _enum: {
            Ignore: "Null",
            Ias: "Null",
            Dcap: "PalletTeebagDcapProvider",
        },
    },
    /**
     * Lookup377: pallet_teebag::types::DcapProvider
     **/
    PalletTeebagDcapProvider: {
        _enum: ["MAA", "Intel", "Local", "Integritee"],
    },
    /**
     * Lookup378: pallet_evm::pallet::Call<T>
     **/
    PalletEvmCall: {
        _enum: {
            withdraw: {
                address: "H160",
                value: "u128",
            },
            call: {
                source: "H160",
                target: "H160",
                input: "Bytes",
                value: "U256",
                gasLimit: "u64",
                maxFeePerGas: "U256",
                maxPriorityFeePerGas: "Option<U256>",
                nonce: "Option<U256>",
                accessList: "Vec<(H160,Vec<H256>)>",
            },
            create: {
                source: "H160",
                init: "Bytes",
                value: "U256",
                gasLimit: "u64",
                maxFeePerGas: "U256",
                maxPriorityFeePerGas: "Option<U256>",
                nonce: "Option<U256>",
                accessList: "Vec<(H160,Vec<H256>)>",
            },
            create2: {
                source: "H160",
                init: "Bytes",
                salt: "H256",
                value: "U256",
                gasLimit: "u64",
                maxFeePerGas: "U256",
                maxPriorityFeePerGas: "Option<U256>",
                nonce: "Option<U256>",
                accessList: "Vec<(H160,Vec<H256>)>",
            },
        },
    },
    /**
     * Lookup382: pallet_ethereum::pallet::Call<T>
     **/
    PalletEthereumCall: {
        _enum: {
            transact: {
                transaction: "EthereumTransactionTransactionV2",
            },
        },
    },
    /**
     * Lookup383: ethereum::transaction::TransactionV2
     **/
    EthereumTransactionTransactionV2: {
        _enum: {
            Legacy: "EthereumTransactionLegacyTransaction",
            EIP2930: "EthereumTransactionEip2930Transaction",
            EIP1559: "EthereumTransactionEip1559Transaction",
        },
    },
    /**
     * Lookup384: ethereum::transaction::LegacyTransaction
     **/
    EthereumTransactionLegacyTransaction: {
        nonce: "U256",
        gasPrice: "U256",
        gasLimit: "U256",
        action: "EthereumTransactionTransactionAction",
        value: "U256",
        input: "Bytes",
        signature: "EthereumTransactionTransactionSignature",
    },
    /**
     * Lookup385: ethereum::transaction::TransactionAction
     **/
    EthereumTransactionTransactionAction: {
        _enum: {
            Call: "H160",
            Create: "Null",
        },
    },
    /**
     * Lookup386: ethereum::transaction::TransactionSignature
     **/
    EthereumTransactionTransactionSignature: {
        v: "u64",
        r: "H256",
        s: "H256",
    },
    /**
     * Lookup388: ethereum::transaction::EIP2930Transaction
     **/
    EthereumTransactionEip2930Transaction: {
        chainId: "u64",
        nonce: "U256",
        gasPrice: "U256",
        gasLimit: "U256",
        action: "EthereumTransactionTransactionAction",
        value: "U256",
        input: "Bytes",
        accessList: "Vec<EthereumTransactionAccessListItem>",
        oddYParity: "bool",
        r: "H256",
        s: "H256",
    },
    /**
     * Lookup390: ethereum::transaction::AccessListItem
     **/
    EthereumTransactionAccessListItem: {
        address: "H160",
        storageKeys: "Vec<H256>",
    },
    /**
     * Lookup391: ethereum::transaction::EIP1559Transaction
     **/
    EthereumTransactionEip1559Transaction: {
        chainId: "u64",
        nonce: "U256",
        maxPriorityFeePerGas: "U256",
        maxFeePerGas: "U256",
        gasLimit: "U256",
        action: "EthereumTransactionTransactionAction",
        value: "U256",
        input: "Bytes",
        accessList: "Vec<EthereumTransactionAccessListItem>",
        oddYParity: "bool",
        r: "H256",
        s: "H256",
    },
    /**
     * Lookup392: pallet_account_fix::pallet::Call<T>
     **/
    PalletAccountFixCall: {
        _enum: {
            upgrade_accounts: {
                who: "Vec<AccountId32>",
            },
            set_balance: {
                who: "MultiAddress",
                addFree: "Compact<u128>",
                addReserved: "Compact<u128>",
            },
        },
    },
    /**
     * Lookup393: pallet_sudo::pallet::Call<T>
     **/
    PalletSudoCall: {
        _enum: {
            sudo: {
                call: "Call",
            },
            sudo_unchecked_weight: {
                call: "Call",
                weight: "SpWeightsWeightV2Weight",
            },
            set_key: {
                _alias: {
                    new_: "new",
                },
                new_: "MultiAddress",
            },
            sudo_as: {
                who: "MultiAddress",
                call: "Call",
            },
        },
    },
    /**
     * Lookup396: pallet_scheduler::pallet::Error<T>
     **/
    PalletSchedulerError: {
        _enum: ["FailedToSchedule", "NotFound", "TargetBlockNumberInPast", "RescheduleNoChange", "Named"],
    },
    /**
     * Lookup397: pallet_utility::pallet::Error<T>
     **/
    PalletUtilityError: {
        _enum: ["TooManyCalls"],
    },
    /**
     * Lookup399: pallet_multisig::Multisig<BlockNumber, Balance, sp_core::crypto::AccountId32, MaxApprovals>
     **/
    PalletMultisigMultisig: {
        when: "PalletMultisigTimepoint",
        deposit: "u128",
        depositor: "AccountId32",
        approvals: "Vec<AccountId32>",
    },
    /**
     * Lookup401: pallet_multisig::pallet::Error<T>
     **/
    PalletMultisigError: {
        _enum: [
            "MinimumThreshold",
            "AlreadyApproved",
            "NoApprovalsNeeded",
            "TooFewSignatories",
            "TooManySignatories",
            "SignatoriesOutOfOrder",
            "SenderInSignatories",
            "NotFound",
            "NotOwner",
            "NoTimepoint",
            "WrongTimepoint",
            "UnexpectedTimepoint",
            "MaxWeightTooLow",
            "AlreadyStored",
        ],
    },
    /**
     * Lookup404: pallet_proxy::ProxyDefinition<sp_core::crypto::AccountId32, rococo_parachain_runtime::ProxyType, BlockNumber>
     **/
    PalletProxyProxyDefinition: {
        delegate: "AccountId32",
        proxyType: "RococoParachainRuntimeProxyType",
        delay: "u32",
    },
    /**
     * Lookup408: pallet_proxy::Announcement<sp_core::crypto::AccountId32, primitive_types::H256, BlockNumber>
     **/
    PalletProxyAnnouncement: {
        real: "AccountId32",
        callHash: "H256",
        height: "u32",
    },
    /**
     * Lookup410: pallet_proxy::pallet::Error<T>
     **/
    PalletProxyError: {
        _enum: ["TooMany", "NotFound", "NotProxy", "Unproxyable", "Duplicate", "NoPermission", "Unannounced", "NoSelfProxy"],
    },
    /**
     * Lookup411: pallet_preimage::RequestStatus<sp_core::crypto::AccountId32, Balance>
     **/
    PalletPreimageRequestStatus: {
        _enum: {
            Unrequested: {
                deposit: "(AccountId32,u128)",
                len: "u32",
            },
            Requested: {
                deposit: "Option<(AccountId32,u128)>",
                count: "u32",
                len: "Option<u32>",
            },
        },
    },
    /**
     * Lookup416: pallet_preimage::pallet::Error<T>
     **/
    PalletPreimageError: {
        _enum: ["TooBig", "AlreadyNoted", "NotAuthorized", "NotNoted", "Requested", "NotRequested"],
    },
    /**
     * Lookup418: pallet_balances::types::BalanceLock<Balance>
     **/
    PalletBalancesBalanceLock: {
        id: "[u8;8]",
        amount: "u128",
        reasons: "PalletBalancesReasons",
    },
    /**
     * Lookup419: pallet_balances::types::Reasons
     **/
    PalletBalancesReasons: {
        _enum: ["Fee", "Misc", "All"],
    },
    /**
     * Lookup422: pallet_balances::types::ReserveData<ReserveIdentifier, Balance>
     **/
    PalletBalancesReserveData: {
        id: "[u8;8]",
        amount: "u128",
    },
    /**
     * Lookup425: pallet_balances::types::IdAmount<Id, Balance>
     **/
    PalletBalancesIdAmount: {
        id: "Null",
        amount: "u128",
    },
    /**
     * Lookup427: pallet_balances::pallet::Error<T, I>
     **/
    PalletBalancesError: {
        _enum: [
            "VestingBalance",
            "LiquidityRestrictions",
            "InsufficientBalance",
            "ExistentialDeposit",
            "Expendability",
            "ExistingVestingSchedule",
            "DeadAccount",
            "TooManyReserves",
            "TooManyHolds",
            "TooManyFreezes",
        ],
    },
    /**
     * Lookup430: pallet_vesting::Releases
     **/
    PalletVestingReleases: {
        _enum: ["V0", "V1"],
    },
    /**
     * Lookup431: pallet_vesting::pallet::Error<T>
     **/
    PalletVestingError: {
        _enum: ["NotVesting", "AtMaxVestingSchedules", "AmountLow", "ScheduleIndexOutOfBounds", "InvalidScheduleParams"],
    },
    /**
     * Lookup433: pallet_transaction_payment::Releases
     **/
    PalletTransactionPaymentReleases: {
        _enum: ["V1Ancient", "V2"],
    },
    /**
     * Lookup434: pallet_treasury::Proposal<sp_core::crypto::AccountId32, Balance>
     **/
    PalletTreasuryProposal: {
        proposer: "AccountId32",
        value: "u128",
        beneficiary: "AccountId32",
        bond: "u128",
    },
    /**
     * Lookup439: frame_support::PalletId
     **/
    FrameSupportPalletId: "[u8;8]",
    /**
     * Lookup440: pallet_treasury::pallet::Error<T, I>
     **/
    PalletTreasuryError: {
        _enum: ["InsufficientProposersBalance", "InvalidIndex", "TooManyApprovals", "InsufficientPermission", "ProposalNotApproved"],
    },
    /**
     * Lookup445: pallet_democracy::types::ReferendumInfo<BlockNumber, frame_support::traits::preimages::Bounded<rococo_parachain_runtime::RuntimeCall>, Balance>
     **/
    PalletDemocracyReferendumInfo: {
        _enum: {
            Ongoing: "PalletDemocracyReferendumStatus",
            Finished: {
                approved: "bool",
                end: "u32",
            },
        },
    },
    /**
     * Lookup446: pallet_democracy::types::ReferendumStatus<BlockNumber, frame_support::traits::preimages::Bounded<rococo_parachain_runtime::RuntimeCall>, Balance>
     **/
    PalletDemocracyReferendumStatus: {
        end: "u32",
        proposal: "FrameSupportPreimagesBounded",
        threshold: "PalletDemocracyVoteThreshold",
        delay: "u32",
        tally: "PalletDemocracyTally",
    },
    /**
     * Lookup447: pallet_democracy::types::Tally<Balance>
     **/
    PalletDemocracyTally: {
        ayes: "u128",
        nays: "u128",
        turnout: "u128",
    },
    /**
     * Lookup448: pallet_democracy::vote::Voting<Balance, sp_core::crypto::AccountId32, BlockNumber, MaxVotes>
     **/
    PalletDemocracyVoteVoting: {
        _enum: {
            Direct: {
                votes: "Vec<(u32,PalletDemocracyVoteAccountVote)>",
                delegations: "PalletDemocracyDelegations",
                prior: "PalletDemocracyVotePriorLock",
            },
            Delegating: {
                balance: "u128",
                target: "AccountId32",
                conviction: "PalletDemocracyConviction",
                delegations: "PalletDemocracyDelegations",
                prior: "PalletDemocracyVotePriorLock",
            },
        },
    },
    /**
     * Lookup452: pallet_democracy::types::Delegations<Balance>
     **/
    PalletDemocracyDelegations: {
        votes: "u128",
        capital: "u128",
    },
    /**
     * Lookup453: pallet_democracy::vote::PriorLock<BlockNumber, Balance>
     **/
    PalletDemocracyVotePriorLock: "(u32,u128)",
    /**
     * Lookup456: pallet_democracy::pallet::Error<T>
     **/
    PalletDemocracyError: {
        _enum: [
            "ValueLow",
            "ProposalMissing",
            "AlreadyCanceled",
            "DuplicateProposal",
            "ProposalBlacklisted",
            "NotSimpleMajority",
            "InvalidHash",
            "NoProposal",
            "AlreadyVetoed",
            "ReferendumInvalid",
            "NoneWaiting",
            "NotVoter",
            "NoPermission",
            "AlreadyDelegating",
            "InsufficientFunds",
            "NotDelegating",
            "VotesExist",
            "InstantNotAllowed",
            "Nonsense",
            "WrongUpperBound",
            "MaxVotesReached",
            "TooMany",
            "VotingPeriodLow",
            "PreimageNotExist",
        ],
    },
    /**
     * Lookup458: pallet_collective::Votes<sp_core::crypto::AccountId32, BlockNumber>
     **/
    PalletCollectiveVotes: {
        index: "u32",
        threshold: "u32",
        ayes: "Vec<AccountId32>",
        nays: "Vec<AccountId32>",
        end: "u32",
    },
    /**
     * Lookup459: pallet_collective::pallet::Error<T, I>
     **/
    PalletCollectiveError: {
        _enum: [
            "NotMember",
            "DuplicateProposal",
            "ProposalMissing",
            "WrongIndex",
            "DuplicateVote",
            "AlreadyInitialized",
            "TooEarly",
            "TooManyProposals",
            "WrongProposalWeight",
            "WrongProposalLength",
        ],
    },
    /**
     * Lookup461: pallet_membership::pallet::Error<T, I>
     **/
    PalletMembershipError: {
        _enum: ["AlreadyMember", "NotMember", "TooManyMembers"],
    },
    /**
     * Lookup464: pallet_bounties::Bounty<sp_core::crypto::AccountId32, Balance, BlockNumber>
     **/
    PalletBountiesBounty: {
        proposer: "AccountId32",
        value: "u128",
        fee: "u128",
        curatorDeposit: "u128",
        bond: "u128",
        status: "PalletBountiesBountyStatus",
    },
    /**
     * Lookup465: pallet_bounties::BountyStatus<sp_core::crypto::AccountId32, BlockNumber>
     **/
    PalletBountiesBountyStatus: {
        _enum: {
            Proposed: "Null",
            Approved: "Null",
            Funded: "Null",
            CuratorProposed: {
                curator: "AccountId32",
            },
            Active: {
                curator: "AccountId32",
                updateDue: "u32",
            },
            PendingPayout: {
                curator: "AccountId32",
                beneficiary: "AccountId32",
                unlockAt: "u32",
            },
        },
    },
    /**
     * Lookup467: pallet_bounties::pallet::Error<T, I>
     **/
    PalletBountiesError: {
        _enum: [
            "InsufficientProposersBalance",
            "InvalidIndex",
            "ReasonTooBig",
            "UnexpectedStatus",
            "RequireCurator",
            "InvalidValue",
            "InvalidFee",
            "PendingPayout",
            "Premature",
            "HasActiveChildBounty",
            "TooManyQueued",
        ],
    },
    /**
     * Lookup468: pallet_tips::OpenTip<sp_core::crypto::AccountId32, Balance, BlockNumber, primitive_types::H256>
     **/
    PalletTipsOpenTip: {
        reason: "H256",
        who: "AccountId32",
        finder: "AccountId32",
        deposit: "u128",
        closes: "Option<u32>",
        tips: "Vec<(AccountId32,u128)>",
        findersFee: "bool",
    },
    /**
     * Lookup470: pallet_tips::pallet::Error<T, I>
     **/
    PalletTipsError: {
        _enum: ["ReasonTooBig", "AlreadyKnown", "UnknownTip", "NotFinder", "StillOpen", "Premature"],
    },
    /**
     * Lookup471: pallet_identity::types::Registration<Balance, MaxJudgements, MaxAdditionalFields>
     **/
    PalletIdentityRegistration: {
        judgements: "Vec<(u32,PalletIdentityJudgement)>",
        deposit: "u128",
        info: "PalletIdentityIdentityInfo",
    },
    /**
     * Lookup478: pallet_identity::types::RegistrarInfo<Balance, sp_core::crypto::AccountId32>
     **/
    PalletIdentityRegistrarInfo: {
        account: "AccountId32",
        fee: "u128",
        fields: "PalletIdentityBitFlags",
    },
    /**
     * Lookup480: pallet_identity::pallet::Error<T>
     **/
    PalletIdentityError: {
        _enum: [
            "TooManySubAccounts",
            "NotFound",
            "NotNamed",
            "EmptyIndex",
            "FeeChanged",
            "NoIdentity",
            "StickyJudgement",
            "JudgementGiven",
            "InvalidJudgement",
            "InvalidIndex",
            "InvalidTarget",
            "TooManyFields",
            "TooManyRegistrars",
            "AlreadyClaimed",
            "NotSub",
            "NotOwned",
            "JudgementForDifferentIdentity",
            "JudgementPaymentFailed",
        ],
    },
    /**
     * Lookup482: polkadot_primitives::v4::UpgradeRestriction
     **/
    PolkadotPrimitivesV4UpgradeRestriction: {
        _enum: ["Present"],
    },
    /**
     * Lookup483: cumulus_pallet_parachain_system::relay_state_snapshot::MessagingStateSnapshot
     **/
    CumulusPalletParachainSystemRelayStateSnapshotMessagingStateSnapshot: {
        dmqMqcHead: "H256",
        relayDispatchQueueSize: "(u32,u32)",
        ingressChannels: "Vec<(u32,PolkadotPrimitivesV4AbridgedHrmpChannel)>",
        egressChannels: "Vec<(u32,PolkadotPrimitivesV4AbridgedHrmpChannel)>",
    },
    /**
     * Lookup486: polkadot_primitives::v4::AbridgedHrmpChannel
     **/
    PolkadotPrimitivesV4AbridgedHrmpChannel: {
        maxCapacity: "u32",
        maxTotalSize: "u32",
        maxMessageSize: "u32",
        msgCount: "u32",
        totalSize: "u32",
        mqcHead: "Option<H256>",
    },
    /**
     * Lookup487: polkadot_primitives::v4::AbridgedHostConfiguration
     **/
    PolkadotPrimitivesV4AbridgedHostConfiguration: {
        maxCodeSize: "u32",
        maxHeadDataSize: "u32",
        maxUpwardQueueCount: "u32",
        maxUpwardQueueSize: "u32",
        maxUpwardMessageSize: "u32",
        maxUpwardMessageNumPerCandidate: "u32",
        hrmpMaxMessageNumPerCandidate: "u32",
        validationUpgradeCooldown: "u32",
        validationUpgradeDelay: "u32",
    },
    /**
     * Lookup493: polkadot_core_primitives::OutboundHrmpMessage<polkadot_parachain::primitives::Id>
     **/
    PolkadotCorePrimitivesOutboundHrmpMessage: {
        recipient: "u32",
        data: "Bytes",
    },
    /**
     * Lookup494: cumulus_pallet_parachain_system::CodeUpgradeAuthorization<T>
     **/
    CumulusPalletParachainSystemCodeUpgradeAuthorization: {
        codeHash: "H256",
        checkVersion: "bool",
    },
    /**
     * Lookup495: cumulus_pallet_parachain_system::pallet::Error<T>
     **/
    CumulusPalletParachainSystemError: {
        _enum: [
            "OverlappingUpgrades",
            "ProhibitedByPolkadot",
            "TooBig",
            "ValidationDataNotAvailable",
            "HostConfigurationNotAvailable",
            "NotScheduled",
            "NothingAuthorized",
            "Unauthorized",
        ],
    },
    /**
     * Lookup499: sp_core::crypto::KeyTypeId
     **/
    SpCoreCryptoKeyTypeId: "[u8;4]",
    /**
     * Lookup500: pallet_session::pallet::Error<T>
     **/
    PalletSessionError: {
        _enum: ["InvalidProof", "NoAssociatedValidatorId", "DuplicatedKey", "NoKeys", "NoAccount"],
    },
    /**
     * Lookup504: pallet_parachain_staking::types::ParachainBondConfig<sp_core::crypto::AccountId32>
     **/
    PalletParachainStakingParachainBondConfig: {
        account: "AccountId32",
        percent: "Percent",
    },
    /**
     * Lookup505: pallet_parachain_staking::types::RoundInfo<BlockNumber>
     **/
    PalletParachainStakingRoundInfo: {
        current: "u32",
        first: "u32",
        length: "u32",
    },
    /**
     * Lookup506: pallet_parachain_staking::types::Delegator<sp_core::crypto::AccountId32, Balance>
     **/
    PalletParachainStakingDelegator: {
        id: "AccountId32",
        delegations: "PalletParachainStakingSetOrderedSet",
        total: "u128",
        lessTotal: "u128",
        status: "PalletParachainStakingDelegatorStatus",
    },
    /**
     * Lookup507: pallet_parachain_staking::set::OrderedSet<pallet_parachain_staking::types::Bond<sp_core::crypto::AccountId32, Balance>>
     **/
    PalletParachainStakingSetOrderedSet: "Vec<PalletParachainStakingBond>",
    /**
     * Lookup508: pallet_parachain_staking::types::Bond<sp_core::crypto::AccountId32, Balance>
     **/
    PalletParachainStakingBond: {
        owner: "AccountId32",
        amount: "u128",
    },
    /**
     * Lookup510: pallet_parachain_staking::types::DelegatorStatus
     **/
    PalletParachainStakingDelegatorStatus: {
        _enum: ["Active"],
    },
    /**
     * Lookup511: pallet_parachain_staking::types::CandidateMetadata<Balance>
     **/
    PalletParachainStakingCandidateMetadata: {
        bond: "u128",
        delegationCount: "u32",
        totalCounted: "u128",
        lowestTopDelegationAmount: "u128",
        highestBottomDelegationAmount: "u128",
        lowestBottomDelegationAmount: "u128",
        topCapacity: "PalletParachainStakingCapacityStatus",
        bottomCapacity: "PalletParachainStakingCapacityStatus",
        request: "Option<PalletParachainStakingCandidateBondLessRequest>",
        status: "PalletParachainStakingCollatorStatus",
    },
    /**
     * Lookup512: pallet_parachain_staking::types::CapacityStatus
     **/
    PalletParachainStakingCapacityStatus: {
        _enum: ["Full", "Empty", "Partial"],
    },
    /**
     * Lookup514: pallet_parachain_staking::types::CandidateBondLessRequest<Balance>
     **/
    PalletParachainStakingCandidateBondLessRequest: {
        amount: "u128",
        whenExecutable: "u32",
    },
    /**
     * Lookup515: pallet_parachain_staking::types::CollatorStatus
     **/
    PalletParachainStakingCollatorStatus: {
        _enum: {
            Active: "Null",
            Idle: "Null",
            Leaving: "u32",
        },
    },
    /**
     * Lookup517: pallet_parachain_staking::delegation_requests::ScheduledRequest<sp_core::crypto::AccountId32, Balance>
     **/
    PalletParachainStakingDelegationRequestsScheduledRequest: {
        delegator: "AccountId32",
        whenExecutable: "u32",
        action: "PalletParachainStakingDelegationRequestsDelegationAction",
    },
    /**
     * Lookup519: pallet_parachain_staking::auto_compound::AutoCompoundConfig<sp_core::crypto::AccountId32>
     **/
    PalletParachainStakingAutoCompoundAutoCompoundConfig: {
        delegator: "AccountId32",
        value: "Percent",
    },
    /**
     * Lookup520: pallet_parachain_staking::types::Delegations<sp_core::crypto::AccountId32, Balance>
     **/
    PalletParachainStakingDelegations: {
        delegations: "Vec<PalletParachainStakingBond>",
        total: "u128",
    },
    /**
     * Lookup522: pallet_parachain_staking::types::CollatorSnapshot<sp_core::crypto::AccountId32, Balance>
     **/
    PalletParachainStakingCollatorSnapshot: {
        bond: "u128",
        delegations: "Vec<PalletParachainStakingBondWithAutoCompound>",
        total: "u128",
    },
    /**
     * Lookup524: pallet_parachain_staking::types::BondWithAutoCompound<sp_core::crypto::AccountId32, Balance>
     **/
    PalletParachainStakingBondWithAutoCompound: {
        owner: "AccountId32",
        amount: "u128",
        autoCompound: "Percent",
    },
    /**
     * Lookup525: pallet_parachain_staking::types::DelayedPayout<Balance>
     **/
    PalletParachainStakingDelayedPayout: {
        roundIssuance: "u128",
        totalStakingReward: "u128",
        collatorCommission: "Perbill",
    },
    /**
     * Lookup526: pallet_parachain_staking::inflation::InflationInfo<Balance>
     **/
    PalletParachainStakingInflationInflationInfo: {
        expect: {
            min: "u128",
            ideal: "u128",
            max: "u128",
        },
        annual: {
            min: "Perbill",
            ideal: "Perbill",
            max: "Perbill",
        },
        round: {
            min: "Perbill",
            ideal: "Perbill",
            max: "Perbill",
        },
    },
    /**
     * Lookup527: pallet_parachain_staking::pallet::Error<T>
     **/
    PalletParachainStakingError: {
        _enum: [
            "DelegatorDNE",
            "DelegatorDNEinTopNorBottom",
            "DelegatorDNEInDelegatorSet",
            "CandidateDNE",
            "DelegationDNE",
            "DelegatorExists",
            "CandidateExists",
            "CandidateBondBelowMin",
            "InsufficientBalance",
            "DelegatorBondBelowMin",
            "DelegationBelowMin",
            "AlreadyOffline",
            "AlreadyActive",
            "DelegatorAlreadyLeaving",
            "DelegatorNotLeaving",
            "DelegatorCannotLeaveYet",
            "CannotDelegateIfLeaving",
            "CandidateAlreadyLeaving",
            "CandidateNotLeaving",
            "CandidateCannotLeaveYet",
            "CannotGoOnlineIfLeaving",
            "ExceedMaxDelegationsPerDelegator",
            "AlreadyDelegatedCandidate",
            "InvalidSchedule",
            "CannotSetBelowMin",
            "RoundLengthMustBeGreaterThanTotalSelectedCollators",
            "NoWritingSameValue",
            "TooLowCandidateCountWeightHintCancelLeaveCandidates",
            "TooLowCandidateDelegationCountToLeaveCandidates",
            "PendingCandidateRequestsDNE",
            "PendingCandidateRequestAlreadyExists",
            "PendingCandidateRequestNotDueYet",
            "PendingDelegationRequestDNE",
            "PendingDelegationRequestAlreadyExists",
            "PendingDelegationRequestNotDueYet",
            "CannotDelegateLessThanOrEqualToLowestBottomWhenFull",
            "PendingDelegationRevoke",
            "CandidateUnauthorized",
        ],
    },
    /**
     * Lookup529: cumulus_pallet_xcmp_queue::InboundChannelDetails
     **/
    CumulusPalletXcmpQueueInboundChannelDetails: {
        sender: "u32",
        state: "CumulusPalletXcmpQueueInboundState",
        messageMetadata: "Vec<(u32,PolkadotParachainPrimitivesXcmpMessageFormat)>",
    },
    /**
     * Lookup530: cumulus_pallet_xcmp_queue::InboundState
     **/
    CumulusPalletXcmpQueueInboundState: {
        _enum: ["Ok", "Suspended"],
    },
    /**
     * Lookup533: polkadot_parachain::primitives::XcmpMessageFormat
     **/
    PolkadotParachainPrimitivesXcmpMessageFormat: {
        _enum: ["ConcatenatedVersionedXcm", "ConcatenatedEncodedBlob", "Signals"],
    },
    /**
     * Lookup536: cumulus_pallet_xcmp_queue::OutboundChannelDetails
     **/
    CumulusPalletXcmpQueueOutboundChannelDetails: {
        recipient: "u32",
        state: "CumulusPalletXcmpQueueOutboundState",
        signalsExist: "bool",
        firstIndex: "u16",
        lastIndex: "u16",
    },
    /**
     * Lookup537: cumulus_pallet_xcmp_queue::OutboundState
     **/
    CumulusPalletXcmpQueueOutboundState: {
        _enum: ["Ok", "Suspended"],
    },
    /**
     * Lookup539: cumulus_pallet_xcmp_queue::QueueConfigData
     **/
    CumulusPalletXcmpQueueQueueConfigData: {
        suspendThreshold: "u32",
        dropThreshold: "u32",
        resumeThreshold: "u32",
        thresholdWeight: "SpWeightsWeightV2Weight",
        weightRestrictDecay: "SpWeightsWeightV2Weight",
        xcmpMaxIndividualWeight: "SpWeightsWeightV2Weight",
    },
    /**
     * Lookup541: cumulus_pallet_xcmp_queue::pallet::Error<T>
     **/
    CumulusPalletXcmpQueueError: {
        _enum: ["FailedToSend", "BadXcmOrigin", "BadXcm", "BadOverweightIndex", "WeightOverLimit"],
    },
    /**
     * Lookup542: pallet_xcm::pallet::QueryStatus<BlockNumber>
     **/
    PalletXcmQueryStatus: {
        _enum: {
            Pending: {
                responder: "XcmVersionedMultiLocation",
                maybeMatchQuerier: "Option<XcmVersionedMultiLocation>",
                maybeNotify: "Option<(u8,u8)>",
                timeout: "u32",
            },
            VersionNotifier: {
                origin: "XcmVersionedMultiLocation",
                isActive: "bool",
            },
            Ready: {
                response: "XcmVersionedResponse",
                at: "u32",
            },
        },
    },
    /**
     * Lookup546: xcm::VersionedResponse
     **/
    XcmVersionedResponse: {
        _enum: {
            __Unused0: "Null",
            __Unused1: "Null",
            V2: "XcmV2Response",
            V3: "XcmV3Response",
        },
    },
    /**
     * Lookup552: pallet_xcm::pallet::VersionMigrationStage
     **/
    PalletXcmVersionMigrationStage: {
        _enum: {
            MigrateSupportedVersion: "Null",
            MigrateVersionNotifiers: "Null",
            NotifyCurrentTargets: "Option<Bytes>",
            MigrateAndNotifyOldTargets: "Null",
        },
    },
    /**
     * Lookup554: xcm::VersionedAssetId
     **/
    XcmVersionedAssetId: {
        _enum: {
            __Unused0: "Null",
            __Unused1: "Null",
            __Unused2: "Null",
            V3: "XcmV3MultiassetAssetId",
        },
    },
    /**
     * Lookup555: pallet_xcm::pallet::RemoteLockedFungibleRecord
     **/
    PalletXcmRemoteLockedFungibleRecord: {
        amount: "u128",
        owner: "XcmVersionedMultiLocation",
        locker: "XcmVersionedMultiLocation",
        users: "u32",
    },
    /**
     * Lookup559: pallet_xcm::pallet::Error<T>
     **/
    PalletXcmError: {
        _enum: [
            "Unreachable",
            "SendFailure",
            "Filtered",
            "UnweighableMessage",
            "DestinationNotInvertible",
            "Empty",
            "CannotReanchor",
            "TooManyAssets",
            "InvalidOrigin",
            "BadVersion",
            "BadLocation",
            "NoSubscription",
            "AlreadySubscribed",
            "InvalidAsset",
            "LowBalance",
            "TooManyLocks",
            "AccountNotSovereign",
            "FeesNotMet",
            "LockNotFound",
            "InUse",
        ],
    },
    /**
     * Lookup560: cumulus_pallet_xcm::pallet::Error<T>
     **/
    CumulusPalletXcmError: "Null",
    /**
     * Lookup561: cumulus_pallet_dmp_queue::ConfigData
     **/
    CumulusPalletDmpQueueConfigData: {
        maxIndividual: "SpWeightsWeightV2Weight",
    },
    /**
     * Lookup562: cumulus_pallet_dmp_queue::PageIndexData
     **/
    CumulusPalletDmpQueuePageIndexData: {
        beginUsed: "u32",
        endUsed: "u32",
        overweightCount: "u64",
    },
    /**
     * Lookup565: cumulus_pallet_dmp_queue::pallet::Error<T>
     **/
    CumulusPalletDmpQueueError: {
        _enum: ["Unknown", "OverLimit"],
    },
    /**
     * Lookup566: orml_xtokens::module::Error<T>
     **/
    OrmlXtokensModuleError: {
        _enum: [
            "AssetHasNoReserve",
            "NotCrossChainTransfer",
            "InvalidDest",
            "NotCrossChainTransferableCurrency",
            "UnweighableMessage",
            "XcmExecutionFailed",
            "CannotReanchor",
            "InvalidAncestry",
            "InvalidAsset",
            "DestinationNotInvertible",
            "BadVersion",
            "DistinctReserveForAssetAndFee",
            "ZeroFee",
            "ZeroAmount",
            "TooManyAssetsBeingSent",
            "AssetIndexNonExistent",
            "FeeNotEnough",
            "NotSupportedMultiLocation",
            "MinXcmFeeNotDefined",
        ],
    },
    /**
     * Lookup568: orml_tokens::BalanceLock<Balance>
     **/
    OrmlTokensBalanceLock: {
        id: "[u8;8]",
        amount: "u128",
    },
    /**
     * Lookup570: orml_tokens::AccountData<Balance>
     **/
    OrmlTokensAccountData: {
        free: "u128",
        reserved: "u128",
        frozen: "u128",
    },
    /**
     * Lookup572: orml_tokens::ReserveData<ReserveIdentifier, Balance>
     **/
    OrmlTokensReserveData: {
        id: "[u8;8]",
        amount: "u128",
    },
    /**
     * Lookup574: orml_tokens::module::Error<T>
     **/
    OrmlTokensModuleError: {
        _enum: [
            "BalanceTooLow",
            "AmountIntoBalanceFailed",
            "LiquidityRestrictions",
            "MaxLocksExceeded",
            "KeepAlive",
            "ExistentialDeposit",
            "DeadAccount",
            "TooManyReserves",
        ],
    },
    /**
     * Lookup577: pallet_bridge::pallet::ProposalVotes<sp_core::crypto::AccountId32, BlockNumber>
     **/
    PalletBridgeProposalVotes: {
        votesFor: "Vec<AccountId32>",
        votesAgainst: "Vec<AccountId32>",
        status: "PalletBridgeProposalStatus",
        expiry: "u32",
    },
    /**
     * Lookup578: pallet_bridge::pallet::ProposalStatus
     **/
    PalletBridgeProposalStatus: {
        _enum: ["Initiated", "Approved", "Rejected"],
    },
    /**
     * Lookup580: pallet_bridge::pallet::BridgeEvent
     **/
    PalletBridgeBridgeEvent: {
        _enum: {
            FungibleTransfer: "(u8,u64,[u8;32],u128,Bytes)",
            NonFungibleTransfer: "(u8,u64,[u8;32],Bytes,Bytes,Bytes)",
            GenericTransfer: "(u8,u64,[u8;32],Bytes)",
        },
    },
    /**
     * Lookup581: pallet_bridge::pallet::Error<T>
     **/
    PalletBridgeError: {
        _enum: [
            "ThresholdNotSet",
            "InvalidChainId",
            "InvalidThreshold",
            "ChainNotWhitelisted",
            "ChainAlreadyWhitelisted",
            "ResourceDoesNotExist",
            "RelayerAlreadyExists",
            "RelayerInvalid",
            "MustBeRelayer",
            "RelayerAlreadyVoted",
            "ProposalAlreadyExists",
            "ProposalDoesNotExist",
            "ProposalNotComplete",
            "ProposalAlreadyComplete",
            "ProposalExpired",
            "FeeTooExpensive",
            "FeeDoesNotExist",
            "InsufficientBalance",
            "CannotPayAsFee",
            "NonceOverFlow",
        ],
    },
    /**
     * Lookup583: pallet_bridge_transfer::pallet::Error<T>
     **/
    PalletBridgeTransferError: {
        _enum: ["InvalidCommand", "InvalidResourceId", "ReachMaximumSupply", "OverFlow"],
    },
    /**
     * Lookup584: pallet_extrinsic_filter::pallet::Error<T>
     **/
    PalletExtrinsicFilterError: {
        _enum: ["CannotBlock", "CannotConvertToString", "ExtrinsicAlreadyBlocked", "ExtrinsicNotBlocked"],
    },
    /**
     * Lookup585: pallet_identity_management::pallet::Error<T>
     **/
    PalletIdentityManagementError: {
        _enum: ["DelegateeNotExist", "UnauthorizedUser"],
    },
    /**
     * Lookup586: pallet_asset_manager::pallet::Error<T>
     **/
    PalletAssetManagerError: {
        _enum: ["AssetAlreadyExists", "AssetTypeDoesNotExist", "AssetIdDoesNotExist", "DefaultAssetTypeRemoved", "AssetIdLimitReached"],
    },
    /**
     * Lookup587: pallet_vc_management::schema::VCSchema<T>
     **/
    PalletVcManagementSchemaVcSchema: {
        id: "Bytes",
        author: "AccountId32",
        content: "Bytes",
        status: "PalletVcManagementSchemaStatus",
    },
    /**
     * Lookup590: pallet_vc_management::schema::Status
     **/
    PalletVcManagementSchemaStatus: {
        _enum: ["Active", "Disabled"],
    },
    /**
     * Lookup591: pallet_vc_management::pallet::Error<T>
     **/
    PalletVcManagementError: {
        _enum: [
            "DelegateeNotExist",
            "UnauthorizedUser",
            "VCAlreadyExists",
            "VCNotExist",
            "VCSubjectMismatch",
            "VCAlreadyDisabled",
            "RequireAdmin",
            "SchemaNotExists",
            "SchemaAlreadyDisabled",
            "SchemaAlreadyActivated",
            "SchemaIndexOverFlow",
            "LengthMismatch",
        ],
    },
    /**
     * Lookup592: pallet_group::pallet::Error<T, I>
     **/
    PalletGroupError: {
        _enum: ["GroupMemberAlreadyExists", "GroupMemberInvalid"],
    },
    /**
     * Lookup594: pallet_bitacross::custodial_wallet::CustodialWallet
     **/
    PalletBitacrossCustodialWallet: {
        btc: "Option<[u8;33]>",
        eth: "Option<[u8;33]>",
    },
    /**
     * Lookup596: pallet_bitacross::pallet::Error<T>
     **/
    PalletBitacrossError: {
        _enum: ["RequireAdminOrRoot", "RelayerNotExist", "UnsupportedRelayerType", "BtcWalletAlreadyExist", "EthWalletAlreadyExist"],
    },
    /**
     * Lookup599: pallet_evm_assertions::pallet::Assertion
     **/
    PalletEvmAssertionsAssertion: {
        byteCode: "Bytes",
        secrets: "Vec<Bytes>",
    },
    /**
     * Lookup600: pallet_evm_assertions::pallet::Error<T>
     **/
    PalletEvmAssertionsError: {
        _enum: ["AssertionExists"],
    },
    /**
     * Lookup602: pallet_teebag::quoting_enclave::QuotingEnclave
     **/
    PalletTeebagQuotingEnclave: {
        issueDate: "u64",
        nextUpdate: "u64",
        miscselect: "[u8;4]",
        miscselectMask: "[u8;4]",
        attributes: "[u8;16]",
        attributesMask: "[u8;16]",
        mrsigner: "[u8;32]",
        isvprodid: "u16",
        tcb: "Vec<PalletTeebagTcbQeTcb>",
    },
    /**
     * Lookup604: pallet_teebag::tcb::QeTcb
     **/
    PalletTeebagTcbQeTcb: {
        isvsvn: "u16",
    },
    /**
     * Lookup605: pallet_teebag::tcb::TcbInfoOnChain
     **/
    PalletTeebagTcbTcbInfoOnChain: {
        issueDate: "u64",
        nextUpdate: "u64",
        tcbLevels: "Vec<PalletTeebagTcbTcbVersionStatus>",
    },
    /**
     * Lookup607: pallet_teebag::tcb::TcbVersionStatus
     **/
    PalletTeebagTcbTcbVersionStatus: {
        cpusvn: "[u8;16]",
        pcesvn: "u16",
    },
    /**
     * Lookup609: pallet_teebag::types::SidechainBlockConfirmation
     **/
    PalletTeebagSidechainBlockConfirmation: {
        blockNumber: "u64",
        blockHeaderHash: "H256",
    },
    /**
     * Lookup610: pallet_teebag::pallet::Error<T>
     **/
    PalletTeebagError: {
        _enum: [
            "RequireAdminOrRoot",
            "EnclaveSignerDecodeError",
            "SenderIsNotAttestedEnclave",
            "RemoteAttestationVerificationFailed",
            "RemoteAttestationTooOld",
            "InvalidAttestationType",
            "InvalidSgxMode",
            "EnclaveNotExist",
            "EnclaveIdentifierNotExist",
            "EnclaveIdentifierAlreadyExist",
            "UnexpectedWorkerType",
            "WrongMrenclaveForShard",
            "EnclaveUrlTooLong",
            "AttestationTooLong",
            "UnexpectedWorkerMode",
            "ScheduledEnclaveNotExist",
            "EnclaveNotInSchedule",
            "CollateralInvalid",
            "MaxEnclaveIdentifierOverflow",
            "ReceivedUnexpectedSidechainBlock",
            "InvalidNextFinalizationCandidateBlockNumber",
        ],
    },
    /**
     * Lookup611: pallet_evm::CodeMetadata
     **/
    PalletEvmCodeMetadata: {
        _alias: {
            size_: "size",
            hash_: "hash",
        },
        size_: "u64",
        hash_: "H256",
    },
    /**
     * Lookup613: pallet_evm::pallet::Error<T>
     **/
    PalletEvmError: {
        _enum: [
            "BalanceLow",
            "FeeOverflow",
            "PaymentOverflow",
            "WithdrawFailed",
            "GasPriceTooLow",
            "InvalidNonce",
            "GasLimitTooLow",
            "GasLimitTooHigh",
            "Undefined",
            "Reentrancy",
            "TransactionMustComeFromEOA",
        ],
    },
    /**
     * Lookup616: fp_rpc::TransactionStatus
     **/
    FpRpcTransactionStatus: {
        transactionHash: "H256",
        transactionIndex: "u32",
        from: "H160",
        to: "Option<H160>",
        contractAddress: "Option<H160>",
        logs: "Vec<EthereumLog>",
        logsBloom: "EthbloomBloom",
    },
    /**
     * Lookup619: ethbloom::Bloom
     **/
    EthbloomBloom: "[u8;256]",
    /**
     * Lookup621: ethereum::receipt::ReceiptV3
     **/
    EthereumReceiptReceiptV3: {
        _enum: {
            Legacy: "EthereumReceiptEip658ReceiptData",
            EIP2930: "EthereumReceiptEip658ReceiptData",
            EIP1559: "EthereumReceiptEip658ReceiptData",
        },
    },
    /**
     * Lookup622: ethereum::receipt::EIP658ReceiptData
     **/
    EthereumReceiptEip658ReceiptData: {
        statusCode: "u8",
        usedGas: "U256",
        logsBloom: "EthbloomBloom",
        logs: "Vec<EthereumLog>",
    },
    /**
     * Lookup623: ethereum::block::Block<ethereum::transaction::TransactionV2>
     **/
    EthereumBlock: {
        header: "EthereumHeader",
        transactions: "Vec<EthereumTransactionTransactionV2>",
        ommers: "Vec<EthereumHeader>",
    },
    /**
     * Lookup624: ethereum::header::Header
     **/
    EthereumHeader: {
        parentHash: "H256",
        ommersHash: "H256",
        beneficiary: "H160",
        stateRoot: "H256",
        transactionsRoot: "H256",
        receiptsRoot: "H256",
        logsBloom: "EthbloomBloom",
        difficulty: "U256",
        number: "U256",
        gasLimit: "U256",
        gasUsed: "U256",
        timestamp: "u64",
        extraData: "Bytes",
        mixHash: "H256",
        nonce: "EthereumTypesHashH64",
    },
    /**
     * Lookup625: ethereum_types::hash::H64
     **/
    EthereumTypesHashH64: "[u8;8]",
    /**
     * Lookup630: pallet_ethereum::pallet::Error<T>
     **/
    PalletEthereumError: {
        _enum: ["InvalidSignature", "PreLogExists"],
    },
    /**
     * Lookup631: pallet_sudo::pallet::Error<T>
     **/
    PalletSudoError: {
        _enum: ["RequireSudo"],
    },
    /**
     * Lookup633: sp_runtime::MultiSignature
     **/
    SpRuntimeMultiSignature: {
        _enum: {
            Ed25519: "SpCoreEd25519Signature",
            Sr25519: "SpCoreSr25519Signature",
            Ecdsa: "SpCoreEcdsaSignature",
        },
    },
    /**
     * Lookup634: sp_core::ed25519::Signature
     **/
    SpCoreEd25519Signature: "[u8;64]",
    /**
     * Lookup636: sp_core::sr25519::Signature
     **/
    SpCoreSr25519Signature: "[u8;64]",
    /**
     * Lookup637: sp_core::ecdsa::Signature
     **/
    SpCoreEcdsaSignature: "[u8;65]",
    /**
     * Lookup640: frame_system::extensions::check_non_zero_sender::CheckNonZeroSender<T>
     **/
    FrameSystemExtensionsCheckNonZeroSender: "Null",
    /**
     * Lookup641: frame_system::extensions::check_spec_version::CheckSpecVersion<T>
     **/
    FrameSystemExtensionsCheckSpecVersion: "Null",
    /**
     * Lookup642: frame_system::extensions::check_tx_version::CheckTxVersion<T>
     **/
    FrameSystemExtensionsCheckTxVersion: "Null",
    /**
     * Lookup643: frame_system::extensions::check_genesis::CheckGenesis<T>
     **/
    FrameSystemExtensionsCheckGenesis: "Null",
    /**
     * Lookup646: frame_system::extensions::check_nonce::CheckNonce<T>
     **/
    FrameSystemExtensionsCheckNonce: "Compact<u32>",
    /**
     * Lookup647: frame_system::extensions::check_weight::CheckWeight<T>
     **/
    FrameSystemExtensionsCheckWeight: "Null",
    /**
     * Lookup648: pallet_transaction_payment::ChargeTransactionPayment<T>
     **/
    PalletTransactionPaymentChargeTransactionPayment: "Compact<u128>",
};
