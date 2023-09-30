// Copyright 2020-2023 Trust Computing GmbH.
// This file is part of Litentry.
//
// Litentry is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Litentry is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Litentry.  If not, see <https://www.gnu.org/licenses/>.

//! Autogenerated weights for pallet_parachain_staking
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-11-25, STEPS: `25`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("litentry-dev"), DB CACHE: 20

// Executed Command:
// ./target/release/litentry-collator
// benchmark
// pallet
// --chain=litentry-dev
// --execution=wasm
// --db-cache=20
// --wasm-execution=compiled
// --pallet=pallet_parachain_staking
// --extrinsic=*
// --heap-pages=4096
// --steps=25
// --repeat=20
// --header=./LICENSE_HEADER
// --template=./templates/benchmark/pallet-weight-template.hbs
// --output=./pallets/parachain-staking/src/weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_parachain_staking.
pub trait WeightInfo {
	fn add_candidates_whitelist(x: u32, ) -> Weight;
	fn remove_candidates_whitelist(x: u32, ) -> Weight;
	fn set_staking_expectations() -> Weight;
	fn set_inflation() -> Weight;
	fn set_parachain_bond_account() -> Weight;
	fn set_parachain_bond_reserve_percent() -> Weight;
	fn set_total_selected() -> Weight;
	fn set_collator_commission() -> Weight;
	fn set_blocks_per_round() -> Weight;
	fn join_candidates(x: u32, ) -> Weight;
	fn schedule_leave_candidates(x: u32, ) -> Weight;
	fn execute_leave_candidates(x: u32, ) -> Weight;
	fn cancel_leave_candidates(x: u32, ) -> Weight;
	fn go_offline() -> Weight;
	fn go_online() -> Weight;
	fn candidate_bond_more() -> Weight;
	fn schedule_candidate_bond_less() -> Weight;
	fn execute_candidate_bond_less() -> Weight;
	fn cancel_candidate_bond_less() -> Weight;
	fn schedule_leave_delegators() -> Weight;
	fn execute_leave_delegators(x: u32, ) -> Weight;
	fn cancel_leave_delegators() -> Weight;
	fn schedule_revoke_delegation() -> Weight;
	fn delegator_bond_more() -> Weight;
	fn schedule_delegator_bond_less() -> Weight;
	fn execute_revoke_delegation() -> Weight;
	fn execute_delegator_bond_less() -> Weight;
	fn cancel_revoke_delegation() -> Weight;
	fn cancel_delegator_bond_less() -> Weight;
	fn round_transition_on_initialize(x: u32, y: u32, ) -> Weight;
	fn pay_one_collator_reward(y: u32, ) -> Weight;
	fn base_on_initialize() -> Weight;
	fn set_auto_compound(x: u32, y: u32, ) -> Weight;
	fn delegate_with_auto_compound(x: u32, y: u32, z: u32, ) -> Weight;
}

/// Weights for pallet_parachain_staking using the Litentry node and recommended hardware.
pub struct LitentryWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for LitentryWeight<T> {
	// Storage: ParachainStaking Candidates (r:1 w:1)
	fn add_candidates_whitelist(x: u32, ) -> Weight {
		Weight::from_ref_time(21_481_000 as u64)
			// Standard Error: 2_500
			.saturating_add(Weight::from_ref_time(257_590 as u64).saturating_mul(x as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: ParachainStaking Candidates (r:1 w:1)
	fn remove_candidates_whitelist(x: u32, ) -> Weight {
		Weight::from_ref_time(20_640_000 as u64)
			// Standard Error: 1_892
			.saturating_add(Weight::from_ref_time(261_532 as u64).saturating_mul(x as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: ParachainStaking InflationConfig (r:1 w:1)
	fn set_staking_expectations() -> Weight {
		Weight::from_ref_time(21_812_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: ParachainStaking InflationConfig (r:1 w:1)
	// Storage: ParachainStaking Round (r:1 w:0)
	fn set_inflation() -> Weight {
		Weight::from_ref_time(42_300_000 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: ParachainStaking ParachainBondInfo (r:1 w:1)
	fn set_parachain_bond_account() -> Weight {
		Weight::from_ref_time(20_930_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: ParachainStaking ParachainBondInfo (r:1 w:1)
	fn set_parachain_bond_reserve_percent() -> Weight {
		Weight::from_ref_time(20_799_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: ParachainStaking TotalSelected (r:1 w:1)
	// Storage: ParachainStaking Round (r:1 w:0)
	fn set_total_selected() -> Weight {
		Weight::from_ref_time(21_501_000 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: ParachainStaking CollatorCommission (r:1 w:1)
	fn set_collator_commission() -> Weight {
		Weight::from_ref_time(19_827_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: ParachainStaking Round (r:1 w:1)
	// Storage: ParachainStaking TotalSelected (r:1 w:0)
	// Storage: ParachainStaking InflationConfig (r:1 w:1)
	fn set_blocks_per_round() -> Weight {
		Weight::from_ref_time(26_921_000 as u64)
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: ParachainStaking Candidates (r:1 w:0)
	// Storage: ParachainStaking CandidateInfo (r:1 w:1)
	// Storage: ParachainStaking DelegatorState (r:1 w:0)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: ParachainStaking Total (r:1 w:1)
	// Storage: ParachainStaking TopDelegations (r:0 w:1)
	// Storage: ParachainStaking BottomDelegations (r:0 w:1)
	fn join_candidates(x: u32, ) -> Weight {
		Weight::from_ref_time(52_881_000 as u64)
			// Standard Error: 1_443
			.saturating_add(Weight::from_ref_time(229_993 as u64).saturating_mul(x as u64))
			.saturating_add(T::DbWeight::get().reads(6 as u64))
			.saturating_add(T::DbWeight::get().writes(6 as u64))
	}
	// Storage: ParachainStaking CandidateInfo (r:1 w:1)
	// Storage: ParachainStaking Round (r:1 w:0)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	fn schedule_leave_candidates(x: u32, ) -> Weight {
		Weight::from_ref_time(34_065_000 as u64)
			// Standard Error: 1_341
			.saturating_add(Weight::from_ref_time(167_933 as u64).saturating_mul(x as u64))
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: ParachainStaking CandidateInfo (r:1 w:1)
	// Storage: ParachainStaking Round (r:1 w:0)
	// Storage: ParachainStaking TopDelegations (r:1 w:1)
	// Storage: System Account (r:2 w:2)
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	// Storage: ParachainStaking DelegationScheduledRequests (r:1 w:1)
	// Storage: ParachainStaking AutoCompoundingDelegations (r:1 w:1)
	// Storage: ParachainStaking BottomDelegations (r:1 w:1)
	// Storage: ParachainStaking Total (r:1 w:1)
	fn execute_leave_candidates(x: u32, ) -> Weight {
		Weight::from_ref_time(123_285_000 as u64)
			// Standard Error: 292_430
			.saturating_add(Weight::from_ref_time(39_461_694 as u64).saturating_mul(x as u64))
			.saturating_add(T::DbWeight::get().reads(10 as u64))
			.saturating_add(T::DbWeight::get().reads((2 as u64).saturating_mul(x as u64)))
			.saturating_add(T::DbWeight::get().writes(9 as u64))
			.saturating_add(T::DbWeight::get().writes((2 as u64).saturating_mul(x as u64)))
	}
	// Storage: ParachainStaking CandidateInfo (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	fn cancel_leave_candidates(x: u32, ) -> Weight {
		Weight::from_ref_time(31_079_000 as u64)
			// Standard Error: 1_407
			.saturating_add(Weight::from_ref_time(178_098 as u64).saturating_mul(x as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: ParachainStaking CandidateInfo (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	fn go_offline() -> Weight {
		Weight::from_ref_time(29_215_000 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: ParachainStaking CandidateInfo (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	fn go_online() -> Weight {
		Weight::from_ref_time(29_166_000 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: ParachainStaking CandidateInfo (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: ParachainStaking Total (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	fn candidate_bond_more() -> Weight {
		Weight::from_ref_time(46_148_000 as u64)
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
	}
	// Storage: ParachainStaking CandidateInfo (r:1 w:1)
	// Storage: ParachainStaking Round (r:1 w:0)
	fn schedule_candidate_bond_less() -> Weight {
		Weight::from_ref_time(27_483_000 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: ParachainStaking CandidateInfo (r:1 w:1)
	// Storage: ParachainStaking Round (r:1 w:0)
	// Storage: System Account (r:1 w:1)
	// Storage: ParachainStaking Total (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	fn execute_candidate_bond_less() -> Weight {
		Weight::from_ref_time(95_361_000 as u64)
			.saturating_add(T::DbWeight::get().reads(5 as u64))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
	}
	// Storage: ParachainStaking CandidateInfo (r:1 w:1)
	fn cancel_candidate_bond_less() -> Weight {
		Weight::from_ref_time(25_408_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	// Storage: ParachainStaking Round (r:1 w:0)
	// Storage: ParachainStaking DelegationScheduledRequests (r:1 w:1)
	fn schedule_leave_delegators() -> Weight {
		Weight::from_ref_time(32_552_000 as u64)
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	// Storage: ParachainStaking Round (r:1 w:0)
	// Storage: ParachainStaking DelegationScheduledRequests (r:1 w:1)
	// Storage: ParachainStaking CandidateInfo (r:1 w:1)
	// Storage: ParachainStaking TopDelegations (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: ParachainStaking Total (r:1 w:1)
	// Storage: ParachainStaking AutoCompoundingDelegations (r:1 w:0)
	fn execute_leave_delegators(x: u32, ) -> Weight {
		Weight::from_ref_time(108_937_000 as u64)
			// Standard Error: 53_326
			.saturating_add(Weight::from_ref_time(32_900_424 as u64).saturating_mul(x as u64))
			.saturating_add(T::DbWeight::get().reads(9 as u64))
			.saturating_add(T::DbWeight::get().reads((4 as u64).saturating_mul(x as u64)))
			.saturating_add(T::DbWeight::get().writes(7 as u64))
			.saturating_add(T::DbWeight::get().writes((3 as u64).saturating_mul(x as u64)))
	}
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	// Storage: ParachainStaking DelegationScheduledRequests (r:1 w:1)
	fn cancel_leave_delegators() -> Weight {
		Weight::from_ref_time(34_025_000 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	// Storage: ParachainStaking DelegationScheduledRequests (r:1 w:1)
	// Storage: ParachainStaking Round (r:1 w:0)
	fn schedule_revoke_delegation() -> Weight {
		Weight::from_ref_time(32_111_000 as u64)
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: ParachainStaking DelegationScheduledRequests (r:1 w:0)
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	// Storage: ParachainStaking CandidateInfo (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: ParachainStaking TopDelegations (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	// Storage: ParachainStaking Total (r:1 w:1)
	fn delegator_bond_more() -> Weight {
		Weight::from_ref_time(60_915_000 as u64)
			.saturating_add(T::DbWeight::get().reads(7 as u64))
			.saturating_add(T::DbWeight::get().writes(6 as u64))
	}
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	// Storage: ParachainStaking DelegationScheduledRequests (r:1 w:1)
	// Storage: ParachainStaking Round (r:1 w:0)
	fn schedule_delegator_bond_less() -> Weight {
		Weight::from_ref_time(32_091_000 as u64)
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	// Storage: ParachainStaking DelegationScheduledRequests (r:1 w:1)
	// Storage: ParachainStaking Round (r:1 w:0)
	// Storage: ParachainStaking AutoCompoundingDelegations (r:1 w:0)
	// Storage: ParachainStaking CandidateInfo (r:1 w:1)
	// Storage: ParachainStaking TopDelegations (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: ParachainStaking Total (r:1 w:1)
	fn execute_revoke_delegation() -> Weight {
		Weight::from_ref_time(134_736_000 as u64)
			.saturating_add(T::DbWeight::get().reads(9 as u64))
			.saturating_add(T::DbWeight::get().writes(7 as u64))
	}
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	// Storage: ParachainStaking DelegationScheduledRequests (r:1 w:1)
	// Storage: ParachainStaking Round (r:1 w:0)
	// Storage: ParachainStaking CandidateInfo (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: ParachainStaking TopDelegations (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	// Storage: ParachainStaking Total (r:1 w:1)
	fn execute_delegator_bond_less() -> Weight {
		Weight::from_ref_time(110_771_000 as u64)
			.saturating_add(T::DbWeight::get().reads(8 as u64))
			.saturating_add(T::DbWeight::get().writes(7 as u64))
	}
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	// Storage: ParachainStaking DelegationScheduledRequests (r:1 w:1)
	fn cancel_revoke_delegation() -> Weight {
		Weight::from_ref_time(34_265_000 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	// Storage: ParachainStaking DelegationScheduledRequests (r:1 w:1)
	fn cancel_delegator_bond_less() -> Weight {
		Weight::from_ref_time(76_656_000 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: ParachainStaking Round (r:1 w:1)
	// Storage: ParachainStaking Points (r:1 w:0)
	// Storage: ParachainStaking Staked (r:1 w:2)
	// Storage: ParachainStaking InflationConfig (r:1 w:0)
	// Storage: BridgeTransfer ExternalBalances (r:1 w:0)
	// Storage: ParachainStaking ParachainBondInfo (r:1 w:0)
	// Storage: ParachainStaking CollatorCommission (r:1 w:0)
	// Storage: ParachainStaking CandidatePool (r:1 w:0)
	// Storage: ParachainStaking TotalSelected (r:1 w:0)
	// Storage: ParachainStaking CandidateInfo (r:8 w:0)
	// Storage: ParachainStaking DelegationScheduledRequests (r:8 w:0)
	// Storage: ParachainStaking TopDelegations (r:8 w:0)
	// Storage: ParachainStaking AutoCompoundingDelegations (r:8 w:0)
	// Storage: ParachainStaking Total (r:1 w:0)
	// Storage: ParachainStaking AwardedPts (r:2 w:1)
	// Storage: ParachainStaking AtStake (r:1 w:9)
	// Storage: System Account (r:1001 w:1001)
	// Storage: ParachainStaking SelectedCandidates (r:0 w:1)
	// Storage: ParachainStaking DelayedPayouts (r:0 w:1)
	fn round_transition_on_initialize(x: u32, y: u32, ) -> Weight {
		Weight::from_ref_time(765_886_000 as u64)
			// Standard Error: 1_703_506
			.saturating_add(Weight::from_ref_time(4_629_041 as u64).saturating_mul(x as u64))
			// Standard Error: 34_370
			.saturating_add(Weight::from_ref_time(211_677 as u64).saturating_mul(y as u64))
			.saturating_add(T::DbWeight::get().reads(58 as u64))
			.saturating_add(T::DbWeight::get().reads((3 as u64).saturating_mul(x as u64)))
			.saturating_add(T::DbWeight::get().writes(19 as u64))
	}
	// Storage: ParachainStaking DelayedPayouts (r:1 w:0)
	// Storage: ParachainStaking Points (r:1 w:0)
	// Storage: ParachainStaking AwardedPts (r:2 w:1)
	// Storage: ParachainStaking AtStake (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn pay_one_collator_reward(y: u32, ) -> Weight {
		Weight::from_ref_time(49_744_000 as u64)
			// Standard Error: 14_775
			.saturating_add(Weight::from_ref_time(15_057_596 as u64).saturating_mul(y as u64))
			.saturating_add(T::DbWeight::get().reads(6 as u64))
			.saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(y as u64)))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
			.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(y as u64)))
	}
	// Storage: ParachainStaking Round (r:1 w:0)
	fn base_on_initialize() -> Weight {
		Weight::from_ref_time(7_173_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
	}
	// Storage: ParachainStaking DelegatorState (r:1 w:0)
	// Storage: ParachainStaking AutoCompoundingDelegations (r:1 w:1)
	fn set_auto_compound(x: u32, y: u32, ) -> Weight {
		Weight::from_ref_time(70_163_000 as u64)
			// Standard Error: 1_217
			.saturating_add(Weight::from_ref_time(103_562 as u64).saturating_mul(x as u64))
			// Standard Error: 12_193
			.saturating_add(Weight::from_ref_time(190_128 as u64).saturating_mul(y as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: System Account (r:1 w:1)
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	// Storage: ParachainStaking CandidateInfo (r:1 w:1)
	// Storage: ParachainStaking AutoCompoundingDelegations (r:1 w:1)
	// Storage: ParachainStaking TopDelegations (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	// Storage: ParachainStaking Total (r:1 w:1)
	// Storage: ParachainStaking BottomDelegations (r:1 w:1)
	fn delegate_with_auto_compound(x: u32, y: u32, z: u32, ) -> Weight {
		Weight::from_ref_time(104_719_000 as u64)
			// Standard Error: 1_076
			.saturating_add(Weight::from_ref_time(107_330 as u64).saturating_mul(x as u64))
			// Standard Error: 1_076
			.saturating_add(Weight::from_ref_time(24_899 as u64).saturating_mul(y as u64))
			// Standard Error: 12_890
			.saturating_add(Weight::from_ref_time(134_826 as u64).saturating_mul(z as u64))
			.saturating_add(T::DbWeight::get().reads(6 as u64))
			.saturating_add(T::DbWeight::get().writes(6 as u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: ParachainStaking Candidates (r:1 w:1)
	fn add_candidates_whitelist(x: u32, ) -> Weight {
		Weight::from_ref_time(21_481_000 as u64)
			// Standard Error: 2_500
			.saturating_add(Weight::from_ref_time(257_590 as u64).saturating_mul(x as u64))
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: ParachainStaking Candidates (r:1 w:1)
	fn remove_candidates_whitelist(x: u32, ) -> Weight {
		Weight::from_ref_time(20_640_000 as u64)
			// Standard Error: 1_892
			.saturating_add(Weight::from_ref_time(261_532 as u64).saturating_mul(x as u64))
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: ParachainStaking InflationConfig (r:1 w:1)
	fn set_staking_expectations() -> Weight {
		Weight::from_ref_time(21_812_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: ParachainStaking InflationConfig (r:1 w:1)
	// Storage: ParachainStaking Round (r:1 w:0)
	fn set_inflation() -> Weight {
		Weight::from_ref_time(42_300_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(2 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: ParachainStaking ParachainBondInfo (r:1 w:1)
	fn set_parachain_bond_account() -> Weight {
		Weight::from_ref_time(20_930_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: ParachainStaking ParachainBondInfo (r:1 w:1)
	fn set_parachain_bond_reserve_percent() -> Weight {
		Weight::from_ref_time(20_799_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: ParachainStaking TotalSelected (r:1 w:1)
	// Storage: ParachainStaking Round (r:1 w:0)
	fn set_total_selected() -> Weight {
		Weight::from_ref_time(21_501_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(2 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: ParachainStaking CollatorCommission (r:1 w:1)
	fn set_collator_commission() -> Weight {
		Weight::from_ref_time(19_827_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: ParachainStaking Round (r:1 w:1)
	// Storage: ParachainStaking TotalSelected (r:1 w:0)
	// Storage: ParachainStaking InflationConfig (r:1 w:1)
	fn set_blocks_per_round() -> Weight {
		Weight::from_ref_time(26_921_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(3 as u64))
			.saturating_add(RocksDbWeight::get().writes(2 as u64))
	}
	// Storage: ParachainStaking Candidates (r:1 w:0)
	// Storage: ParachainStaking CandidateInfo (r:1 w:1)
	// Storage: ParachainStaking DelegatorState (r:1 w:0)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: ParachainStaking Total (r:1 w:1)
	// Storage: ParachainStaking TopDelegations (r:0 w:1)
	// Storage: ParachainStaking BottomDelegations (r:0 w:1)
	fn join_candidates(x: u32, ) -> Weight {
		Weight::from_ref_time(52_881_000 as u64)
			// Standard Error: 1_443
			.saturating_add(Weight::from_ref_time(229_993 as u64).saturating_mul(x as u64))
			.saturating_add(RocksDbWeight::get().reads(6 as u64))
			.saturating_add(RocksDbWeight::get().writes(6 as u64))
	}
	// Storage: ParachainStaking CandidateInfo (r:1 w:1)
	// Storage: ParachainStaking Round (r:1 w:0)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	fn schedule_leave_candidates(x: u32, ) -> Weight {
		Weight::from_ref_time(34_065_000 as u64)
			// Standard Error: 1_341
			.saturating_add(Weight::from_ref_time(167_933 as u64).saturating_mul(x as u64))
			.saturating_add(RocksDbWeight::get().reads(3 as u64))
			.saturating_add(RocksDbWeight::get().writes(2 as u64))
	}
	// Storage: ParachainStaking CandidateInfo (r:1 w:1)
	// Storage: ParachainStaking Round (r:1 w:0)
	// Storage: ParachainStaking TopDelegations (r:1 w:1)
	// Storage: System Account (r:2 w:2)
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	// Storage: ParachainStaking DelegationScheduledRequests (r:1 w:1)
	// Storage: ParachainStaking AutoCompoundingDelegations (r:1 w:1)
	// Storage: ParachainStaking BottomDelegations (r:1 w:1)
	// Storage: ParachainStaking Total (r:1 w:1)
	fn execute_leave_candidates(x: u32, ) -> Weight {
		Weight::from_ref_time(123_285_000 as u64)
			// Standard Error: 292_430
			.saturating_add(Weight::from_ref_time(39_461_694 as u64).saturating_mul(x as u64))
			.saturating_add(RocksDbWeight::get().reads(10 as u64))
			.saturating_add(RocksDbWeight::get().reads((2 as u64).saturating_mul(x as u64)))
			.saturating_add(RocksDbWeight::get().writes(9 as u64))
			.saturating_add(RocksDbWeight::get().writes((2 as u64).saturating_mul(x as u64)))
	}
	// Storage: ParachainStaking CandidateInfo (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	fn cancel_leave_candidates(x: u32, ) -> Weight {
		Weight::from_ref_time(31_079_000 as u64)
			// Standard Error: 1_407
			.saturating_add(Weight::from_ref_time(178_098 as u64).saturating_mul(x as u64))
			.saturating_add(RocksDbWeight::get().reads(2 as u64))
			.saturating_add(RocksDbWeight::get().writes(2 as u64))
	}
	// Storage: ParachainStaking CandidateInfo (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	fn go_offline() -> Weight {
		Weight::from_ref_time(29_215_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(2 as u64))
			.saturating_add(RocksDbWeight::get().writes(2 as u64))
	}
	// Storage: ParachainStaking CandidateInfo (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	fn go_online() -> Weight {
		Weight::from_ref_time(29_166_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(2 as u64))
			.saturating_add(RocksDbWeight::get().writes(2 as u64))
	}
	// Storage: ParachainStaking CandidateInfo (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: ParachainStaking Total (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	fn candidate_bond_more() -> Weight {
		Weight::from_ref_time(46_148_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(4 as u64))
			.saturating_add(RocksDbWeight::get().writes(4 as u64))
	}
	// Storage: ParachainStaking CandidateInfo (r:1 w:1)
	// Storage: ParachainStaking Round (r:1 w:0)
	fn schedule_candidate_bond_less() -> Weight {
		Weight::from_ref_time(27_483_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(2 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: ParachainStaking CandidateInfo (r:1 w:1)
	// Storage: ParachainStaking Round (r:1 w:0)
	// Storage: System Account (r:1 w:1)
	// Storage: ParachainStaking Total (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	fn execute_candidate_bond_less() -> Weight {
		Weight::from_ref_time(95_361_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(5 as u64))
			.saturating_add(RocksDbWeight::get().writes(4 as u64))
	}
	// Storage: ParachainStaking CandidateInfo (r:1 w:1)
	fn cancel_candidate_bond_less() -> Weight {
		Weight::from_ref_time(25_408_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	// Storage: ParachainStaking Round (r:1 w:0)
	// Storage: ParachainStaking DelegationScheduledRequests (r:1 w:1)
	fn schedule_leave_delegators() -> Weight {
		Weight::from_ref_time(32_552_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(3 as u64))
			.saturating_add(RocksDbWeight::get().writes(2 as u64))
	}
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	// Storage: ParachainStaking Round (r:1 w:0)
	// Storage: ParachainStaking DelegationScheduledRequests (r:1 w:1)
	// Storage: ParachainStaking CandidateInfo (r:1 w:1)
	// Storage: ParachainStaking TopDelegations (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: ParachainStaking Total (r:1 w:1)
	// Storage: ParachainStaking AutoCompoundingDelegations (r:1 w:0)
	fn execute_leave_delegators(x: u32, ) -> Weight {
		Weight::from_ref_time(108_937_000 as u64)
			// Standard Error: 53_326
			.saturating_add(Weight::from_ref_time(32_900_424 as u64).saturating_mul(x as u64))
			.saturating_add(RocksDbWeight::get().reads(9 as u64))
			.saturating_add(RocksDbWeight::get().reads((4 as u64).saturating_mul(x as u64)))
			.saturating_add(RocksDbWeight::get().writes(7 as u64))
			.saturating_add(RocksDbWeight::get().writes((3 as u64).saturating_mul(x as u64)))
	}
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	// Storage: ParachainStaking DelegationScheduledRequests (r:1 w:1)
	fn cancel_leave_delegators() -> Weight {
		Weight::from_ref_time(34_025_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(2 as u64))
			.saturating_add(RocksDbWeight::get().writes(2 as u64))
	}
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	// Storage: ParachainStaking DelegationScheduledRequests (r:1 w:1)
	// Storage: ParachainStaking Round (r:1 w:0)
	fn schedule_revoke_delegation() -> Weight {
		Weight::from_ref_time(32_111_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(3 as u64))
			.saturating_add(RocksDbWeight::get().writes(2 as u64))
	}
	// Storage: ParachainStaking DelegationScheduledRequests (r:1 w:0)
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	// Storage: ParachainStaking CandidateInfo (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: ParachainStaking TopDelegations (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	// Storage: ParachainStaking Total (r:1 w:1)
	fn delegator_bond_more() -> Weight {
		Weight::from_ref_time(60_915_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(7 as u64))
			.saturating_add(RocksDbWeight::get().writes(6 as u64))
	}
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	// Storage: ParachainStaking DelegationScheduledRequests (r:1 w:1)
	// Storage: ParachainStaking Round (r:1 w:0)
	fn schedule_delegator_bond_less() -> Weight {
		Weight::from_ref_time(32_091_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(3 as u64))
			.saturating_add(RocksDbWeight::get().writes(2 as u64))
	}
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	// Storage: ParachainStaking DelegationScheduledRequests (r:1 w:1)
	// Storage: ParachainStaking Round (r:1 w:0)
	// Storage: ParachainStaking AutoCompoundingDelegations (r:1 w:0)
	// Storage: ParachainStaking CandidateInfo (r:1 w:1)
	// Storage: ParachainStaking TopDelegations (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: ParachainStaking Total (r:1 w:1)
	fn execute_revoke_delegation() -> Weight {
		Weight::from_ref_time(134_736_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(9 as u64))
			.saturating_add(RocksDbWeight::get().writes(7 as u64))
	}
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	// Storage: ParachainStaking DelegationScheduledRequests (r:1 w:1)
	// Storage: ParachainStaking Round (r:1 w:0)
	// Storage: ParachainStaking CandidateInfo (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: ParachainStaking TopDelegations (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	// Storage: ParachainStaking Total (r:1 w:1)
	fn execute_delegator_bond_less() -> Weight {
		Weight::from_ref_time(110_771_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(8 as u64))
			.saturating_add(RocksDbWeight::get().writes(7 as u64))
	}
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	// Storage: ParachainStaking DelegationScheduledRequests (r:1 w:1)
	fn cancel_revoke_delegation() -> Weight {
		Weight::from_ref_time(34_265_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(2 as u64))
			.saturating_add(RocksDbWeight::get().writes(2 as u64))
	}
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	// Storage: ParachainStaking DelegationScheduledRequests (r:1 w:1)
	fn cancel_delegator_bond_less() -> Weight {
		Weight::from_ref_time(76_656_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(2 as u64))
			.saturating_add(RocksDbWeight::get().writes(2 as u64))
	}
	// Storage: ParachainStaking Round (r:1 w:1)
	// Storage: ParachainStaking Points (r:1 w:0)
	// Storage: ParachainStaking Staked (r:1 w:2)
	// Storage: ParachainStaking InflationConfig (r:1 w:0)
	// Storage: BridgeTransfer ExternalBalances (r:1 w:0)
	// Storage: ParachainStaking ParachainBondInfo (r:1 w:0)
	// Storage: ParachainStaking CollatorCommission (r:1 w:0)
	// Storage: ParachainStaking CandidatePool (r:1 w:0)
	// Storage: ParachainStaking TotalSelected (r:1 w:0)
	// Storage: ParachainStaking CandidateInfo (r:8 w:0)
	// Storage: ParachainStaking DelegationScheduledRequests (r:8 w:0)
	// Storage: ParachainStaking TopDelegations (r:8 w:0)
	// Storage: ParachainStaking AutoCompoundingDelegations (r:8 w:0)
	// Storage: ParachainStaking Total (r:1 w:0)
	// Storage: ParachainStaking AwardedPts (r:2 w:1)
	// Storage: ParachainStaking AtStake (r:1 w:9)
	// Storage: System Account (r:1001 w:1001)
	// Storage: ParachainStaking SelectedCandidates (r:0 w:1)
	// Storage: ParachainStaking DelayedPayouts (r:0 w:1)
	fn round_transition_on_initialize(x: u32, y: u32, ) -> Weight {
		Weight::from_ref_time(765_886_000 as u64)
			// Standard Error: 1_703_506
			.saturating_add(Weight::from_ref_time(4_629_041 as u64).saturating_mul(x as u64))
			// Standard Error: 34_370
			.saturating_add(Weight::from_ref_time(211_677 as u64).saturating_mul(y as u64))
			.saturating_add(RocksDbWeight::get().reads(58 as u64))
			.saturating_add(RocksDbWeight::get().reads((3 as u64).saturating_mul(x as u64)))
			.saturating_add(RocksDbWeight::get().writes(19 as u64))
	}
	// Storage: ParachainStaking DelayedPayouts (r:1 w:0)
	// Storage: ParachainStaking Points (r:1 w:0)
	// Storage: ParachainStaking AwardedPts (r:2 w:1)
	// Storage: ParachainStaking AtStake (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn pay_one_collator_reward(y: u32, ) -> Weight {
		Weight::from_ref_time(49_744_000 as u64)
			// Standard Error: 14_775
			.saturating_add(Weight::from_ref_time(15_057_596 as u64).saturating_mul(y as u64))
			.saturating_add(RocksDbWeight::get().reads(6 as u64))
			.saturating_add(RocksDbWeight::get().reads((1 as u64).saturating_mul(y as u64)))
			.saturating_add(RocksDbWeight::get().writes(3 as u64))
			.saturating_add(RocksDbWeight::get().writes((1 as u64).saturating_mul(y as u64)))
	}
	// Storage: ParachainStaking Round (r:1 w:0)
	fn base_on_initialize() -> Weight {
		Weight::from_ref_time(7_173_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
	}
	// Storage: ParachainStaking DelegatorState (r:1 w:0)
	// Storage: ParachainStaking AutoCompoundingDelegations (r:1 w:1)
	fn set_auto_compound(x: u32, y: u32, ) -> Weight {
		Weight::from_ref_time(70_163_000 as u64)
			// Standard Error: 1_217
			.saturating_add(Weight::from_ref_time(103_562 as u64).saturating_mul(x as u64))
			// Standard Error: 12_193
			.saturating_add(Weight::from_ref_time(190_128 as u64).saturating_mul(y as u64))
			.saturating_add(RocksDbWeight::get().reads(2 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: System Account (r:1 w:1)
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	// Storage: ParachainStaking CandidateInfo (r:1 w:1)
	// Storage: ParachainStaking AutoCompoundingDelegations (r:1 w:1)
	// Storage: ParachainStaking TopDelegations (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	// Storage: ParachainStaking Total (r:1 w:1)
	// Storage: ParachainStaking BottomDelegations (r:1 w:1)
	fn delegate_with_auto_compound(x: u32, y: u32, z: u32, ) -> Weight {
		Weight::from_ref_time(104_719_000 as u64)
			// Standard Error: 1_076
			.saturating_add(Weight::from_ref_time(107_330 as u64).saturating_mul(x as u64))
			// Standard Error: 1_076
			.saturating_add(Weight::from_ref_time(24_899 as u64).saturating_mul(y as u64))
			// Standard Error: 12_890
			.saturating_add(Weight::from_ref_time(134_826 as u64).saturating_mul(z as u64))
			.saturating_add(RocksDbWeight::get().reads(6 as u64))
			.saturating_add(RocksDbWeight::get().writes(6 as u64))
	}
}

