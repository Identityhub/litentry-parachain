export default {
    types: {
        VCRequested: {
            account: "AccountId",
            mrEnclave: "ShardIdentifier",
            assertion: "Assertion",
        },
        Assertion: {
            _enum: {
                A1: "Null",
                A2: "Bytes",
                A3: "(Bytes,Bytes,Bytes)",
                A4: "Bytes",
                A6: "Null",
                A7: "Bytes",
                A8: "Vec<AssertionSupportedNetwork>",
                A10: "Bytes",
                A11: "Bytes",
                A13: "AccountId32",
                A14: "Null",
                Achainable: "AchainableParams",
                A20: "Null",
                OneBlock: "OneBlockCourseType",
                GenericDiscordRole: "GenericDiscordRoleType",
                __Unused15: "Null",
                BnbDomainHolding: "Null",
                BnbDigitDomainClub: "BnbDigitDomainType",
                VIP3MembershipCard: "VIP3MembershipCardLevel",
                WeirdoGhostGangHolder: "Null",
                LITStaking: "Null",
                EVMAmountHolding: "EVMTokenType",
                BRC20AmountHolder: "Null",
                CyptoSummary: "Null",
                TokenHoldingAmount: "Web3TokenType",
                PlatformUser: "PlatformUserType",
            },
        },
        AssertionSupportedNetwork: {
            _enum: ["Litentry", "Litmus", "LitentryRococo", "Polkadot", "Kusama", "Khala", "Ethereum", "TestNet"],
        },
        RequestVCResult: {
            vc_payload: "AesOutput",
        },
        // Achainable
        AchainableParams: {
            _enum: {
                AmountHolding: "AchainableAmountHolding",
                AmountToken: "AchainableAmountToken",
                Amount: "AchainableAmount",
                Amounts: "AchainableAmounts",
                Basic: "AchainableBasic",
                BetweenPercents: "AchainableBetweenPercents",
                ClassOfYear: "AchainableClassOfYear",
                DateInterval: "AchainableDateInterval",
                DatePercent: "AchainableDatePercent",
                Date: "AchainableDate",
                Token: "AchainableToken",
            },
        },
        AchainableAmountHolding: {
            name: "Bytes",
            chain: "Web3Network",
            amount: "Bytes",
            date: "Bytes",
            token: "Option<Bytes>",
        },
        AchainableAmountToken: {
            name: "Bytes",
            chain: "Web3Network",
            amount: "Bytes",
            token: "Option<Bytes>",
        },
        AchainableAmount: {
            name: "Bytes",
            chain: "Web3Network",
            amount: "Bytes",
        },
        AchainableAmounts: {
            name: "Bytes",
            chain: "Web3Network",
            amount1: "Bytes",
            amount2: "Bytes",
        },
        AchainableBasic: {
            name: "Bytes",
            chain: "Web3Network",
        },
        AchainableBetweenPercents: {
            name: "Bytes",
            chain: "Web3Network",
            greaterThanOrEqualTo: "Bytes",
            lessThanOrEqualTo: "Bytes",
        },
        AchainableClassOfYear: {
            name: "Bytes",
            chain: "Web3Network",
        },
        AchainableDateInterval: {
            name: "Bytes",
            chain: "Web3Network",
            startDate: "Bytes",
            endDate: "Bytes",
        },
        AchainableDatePercent: {
            name: "Bytes",
            chain: "Web3Network",
            token: "Bytes",
            date: "Bytes",
            percent: "Bytes",
        },
        AchainableDate: {
            name: "Bytes",
            chain: "Web3Network",
            date: "Bytes",
        },
        AchainableToken: {
            name: "Bytes",
            chain: "Web3Network",
            token: "Bytes",
        },
        // OneBlock
        OneBlockCourseType: {
            _enum: ["CourseCompletion", "CourseOutstanding", "CourseParticipation"],
        },
        // Bnb
        BnbDigitDomainType: {
            _enum: ["Bnb999ClubMember", "Bnb10kClubMember"],
        },
        // GenericDiscordRole
        GenericDiscordRoleType: {
            _enum: {
                Contest: "GenericDiscordRoleContestType",
                SoraQuiz: "GenericDiscordRoleSoraQuizType",
            },
        },
        GenericDiscordRoleContestType: {
            _enum: ["Legend", "Popularity", "Participant"],
        },
        GenericDiscordRoleSoraQuizType: {
            _enum: ["Attendee", "Master"],
        },
        // VIP3MembershipCard
        VIP3MembershipCardLevel: {
            _enum: ["Gold", "Silver"],
        },
        // EVMAmountHolding
        EVMTokenType: {
            _enum: ["Ton", "Trx"],
        },
        // Web3TokenType
        Web3TokenType: {
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
            ],
        },
        // PlatformUserType
        PlatformUserType: {
            _enum: ["KaratDaoUser"],
        },
    },
};
