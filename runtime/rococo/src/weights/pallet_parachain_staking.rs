// Copyright 2020-2023 Litentry Technologies GmbH.
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

//! Autogenerated weights for `pallet_parachain_staking`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-06-22, STEPS: `25`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `parachain-benchmark`, CPU: `Intel(R) Xeon(R) Platinum 8259CL CPU @ 2.50GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("rococo-dev"), DB CACHE: 20

// Executed Command:
// ./litentry-collator
// benchmark
// pallet
// --chain=rococo-dev
// --execution=wasm
// --db-cache=20
// --wasm-execution=compiled
// --pallet=pallet_parachain_staking
// --extrinsic=*
// --heap-pages=4096
// --steps=25
// --repeat=20
// --header=./LICENSE_HEADER
// --output=./runtime/rococo/src/weights/pallet_parachain_staking.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_parachain_staking`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_parachain_staking::WeightInfo for WeightInfo<T> {
	/// Storage: ParachainStaking Candidates (r:1 w:1)
	/// Proof Skipped: ParachainStaking Candidates (max_values: Some(1), max_size: None, mode: Measured)
	/// The range of component `x` is `[1, 100]`.
	fn add_candidates_whitelist(x: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `369 + x * (32 ±0)`
		//  Estimated: `868 + x * (32 ±0)`
		// Minimum execution time: 19_137 nanoseconds.
		Weight::from_ref_time(23_246_007)
			.saturating_add(Weight::from_proof_size(868))
			// Standard Error: 4_362
			.saturating_add(Weight::from_ref_time(157_352).saturating_mul(x.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(Weight::from_proof_size(32).saturating_mul(x.into()))
	}
	/// Storage: ParachainStaking Candidates (r:1 w:1)
	/// Proof Skipped: ParachainStaking Candidates (max_values: Some(1), max_size: None, mode: Measured)
	/// The range of component `x` is `[1, 100]`.
	fn remove_candidates_whitelist(x: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `401 + x * (32 ±0)`
		//  Estimated: `901 + x * (32 ±0)`
		// Minimum execution time: 18_606 nanoseconds.
		Weight::from_ref_time(22_840_572)
			.saturating_add(Weight::from_proof_size(901))
			// Standard Error: 3_782
			.saturating_add(Weight::from_ref_time(134_413).saturating_mul(x.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(Weight::from_proof_size(32).saturating_mul(x.into()))
	}
	/// Storage: ParachainStaking InflationConfig (r:1 w:1)
	/// Proof Skipped: ParachainStaking InflationConfig (max_values: Some(1), max_size: None, mode: Measured)
	fn set_staking_expectations() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `497`
		//  Estimated: `992`
		// Minimum execution time: 20_575 nanoseconds.
		Weight::from_ref_time(21_153_000)
			.saturating_add(Weight::from_proof_size(992))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: ParachainStaking InflationConfig (r:1 w:1)
	/// Proof Skipped: ParachainStaking InflationConfig (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ParachainStaking Round (r:1 w:0)
	/// Proof Skipped: ParachainStaking Round (max_values: Some(1), max_size: None, mode: Measured)
	fn set_inflation() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `533`
		//  Estimated: `2056`
		// Minimum execution time: 74_009 nanoseconds.
		Weight::from_ref_time(74_519_000)
			.saturating_add(Weight::from_proof_size(2056))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: ParachainStaking ParachainBondInfo (r:1 w:1)
	/// Proof Skipped: ParachainStaking ParachainBondInfo (max_values: Some(1), max_size: None, mode: Measured)
	fn set_parachain_bond_account() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `465`
		//  Estimated: `960`
		// Minimum execution time: 20_174 nanoseconds.
		Weight::from_ref_time(20_764_000)
			.saturating_add(Weight::from_proof_size(960))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: ParachainStaking ParachainBondInfo (r:1 w:1)
	/// Proof Skipped: ParachainStaking ParachainBondInfo (max_values: Some(1), max_size: None, mode: Measured)
	fn set_parachain_bond_reserve_percent() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `465`
		//  Estimated: `960`
		// Minimum execution time: 19_297 nanoseconds.
		Weight::from_ref_time(20_235_000)
			.saturating_add(Weight::from_proof_size(960))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: ParachainStaking TotalSelected (r:1 w:1)
	/// Proof Skipped: ParachainStaking TotalSelected (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ParachainStaking Round (r:1 w:0)
	/// Proof Skipped: ParachainStaking Round (max_values: Some(1), max_size: None, mode: Measured)
	fn set_total_selected() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `442`
		//  Estimated: `1874`
		// Minimum execution time: 20_651 nanoseconds.
		Weight::from_ref_time(21_240_000)
			.saturating_add(Weight::from_proof_size(1874))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: ParachainStaking CollatorCommission (r:1 w:1)
	/// Proof Skipped: ParachainStaking CollatorCommission (max_values: Some(1), max_size: None, mode: Measured)
	fn set_collator_commission() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `405`
		//  Estimated: `900`
		// Minimum execution time: 17_496 nanoseconds.
		Weight::from_ref_time(18_414_000)
			.saturating_add(Weight::from_proof_size(900))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: ParachainStaking Round (r:1 w:1)
	/// Proof Skipped: ParachainStaking Round (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ParachainStaking TotalSelected (r:1 w:0)
	/// Proof Skipped: ParachainStaking TotalSelected (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ParachainStaking InflationConfig (r:1 w:1)
	/// Proof Skipped: ParachainStaking InflationConfig (max_values: Some(1), max_size: None, mode: Measured)
	fn set_blocks_per_round() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `561`
		//  Estimated: `3168`
		// Minimum execution time: 29_869 nanoseconds.
		Weight::from_ref_time(31_113_000)
			.saturating_add(Weight::from_proof_size(3168))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: ParachainStaking Candidates (r:1 w:0)
	/// Proof Skipped: ParachainStaking Candidates (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ParachainStaking CandidateInfo (r:1 w:1)
	/// Proof Skipped: ParachainStaking CandidateInfo (max_values: None, max_size: None, mode: Measured)
	/// Storage: ParachainStaking DelegatorState (r:1 w:0)
	/// Proof Skipped: ParachainStaking DelegatorState (max_values: None, max_size: None, mode: Measured)
	/// Storage: ParachainStaking CandidatePool (r:1 w:1)
	/// Proof Skipped: ParachainStaking CandidatePool (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: ParachainStaking Total (r:1 w:1)
	/// Proof Skipped: ParachainStaking Total (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ParachainStaking TopDelegations (r:0 w:1)
	/// Proof Skipped: ParachainStaking TopDelegations (max_values: None, max_size: None, mode: Measured)
	/// Storage: ParachainStaking BottomDelegations (r:0 w:1)
	/// Proof Skipped: ParachainStaking BottomDelegations (max_values: None, max_size: None, mode: Measured)
	/// The range of component `x` is `[3, 1000]`.
	fn join_candidates(x: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1607 + x * (80 ±0)`
		//  Estimated: `19153 + x * (567 ±0)`
		// Minimum execution time: 63_087 nanoseconds.
		Weight::from_ref_time(68_644_644)
			.saturating_add(Weight::from_proof_size(19153))
			// Standard Error: 3_187
			.saturating_add(Weight::from_ref_time(239_232).saturating_mul(x.into()))
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(6))
			.saturating_add(Weight::from_proof_size(567).saturating_mul(x.into()))
	}
	/// Storage: ParachainStaking CandidateInfo (r:1 w:1)
	/// Proof Skipped: ParachainStaking CandidateInfo (max_values: None, max_size: None, mode: Measured)
	/// Storage: ParachainStaking Round (r:1 w:0)
	/// Proof Skipped: ParachainStaking Round (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ParachainStaking CandidatePool (r:1 w:1)
	/// Proof Skipped: ParachainStaking CandidatePool (max_values: Some(1), max_size: None, mode: Measured)
	/// The range of component `x` is `[3, 1000]`.
	fn schedule_leave_candidates(x: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1390 + x * (48 ±0)`
		//  Estimated: `7332 + x * (147 ±0)`
		// Minimum execution time: 34_678 nanoseconds.
		Weight::from_ref_time(35_412_359)
			.saturating_add(Weight::from_proof_size(7332))
			// Standard Error: 2_718
			.saturating_add(Weight::from_ref_time(165_286).saturating_mul(x.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
			.saturating_add(Weight::from_proof_size(147).saturating_mul(x.into()))
	}
	/// Storage: ParachainStaking CandidateInfo (r:1 w:1)
	/// Proof Skipped: ParachainStaking CandidateInfo (max_values: None, max_size: None, mode: Measured)
	/// Storage: ParachainStaking Round (r:1 w:0)
	/// Proof Skipped: ParachainStaking Round (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ParachainStaking TopDelegations (r:1 w:1)
	/// Proof Skipped: ParachainStaking TopDelegations (max_values: None, max_size: None, mode: Measured)
	/// Storage: System Account (r:1200 w:1200)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: ParachainStaking DelegatorState (r:1199 w:1199)
	/// Proof Skipped: ParachainStaking DelegatorState (max_values: None, max_size: None, mode: Measured)
	/// Storage: ParachainStaking DelegationScheduledRequests (r:1 w:1)
	/// Proof Skipped: ParachainStaking DelegationScheduledRequests (max_values: None, max_size: None, mode: Measured)
	/// Storage: ParachainStaking AutoCompoundingDelegations (r:1 w:1)
	/// Proof Skipped: ParachainStaking AutoCompoundingDelegations (max_values: None, max_size: None, mode: Measured)
	/// Storage: ParachainStaking BottomDelegations (r:1 w:1)
	/// Proof Skipped: ParachainStaking BottomDelegations (max_values: None, max_size: None, mode: Measured)
	/// Storage: ParachainStaking Total (r:1 w:1)
	/// Proof Skipped: ParachainStaking Total (max_values: Some(1), max_size: None, mode: Measured)
	/// The range of component `x` is `[2, 1200]`.
	fn execute_leave_candidates(x: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `527 + x * (511 ±0)`
		//  Estimated: `21307 + x * (9159 ±0)`
		// Minimum execution time: 124_915 nanoseconds.
		Weight::from_ref_time(133_060_000)
			.saturating_add(Weight::from_proof_size(21307))
			// Standard Error: 794_850
			.saturating_add(Weight::from_ref_time(69_469_736).saturating_mul(x.into()))
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().reads((2_u64).saturating_mul(x.into())))
			.saturating_add(T::DbWeight::get().writes(5))
			.saturating_add(T::DbWeight::get().writes((2_u64).saturating_mul(x.into())))
			.saturating_add(Weight::from_proof_size(9159).saturating_mul(x.into()))
	}
	/// Storage: ParachainStaking CandidateInfo (r:1 w:1)
	/// Proof Skipped: ParachainStaking CandidateInfo (max_values: None, max_size: None, mode: Measured)
	/// Storage: ParachainStaking CandidatePool (r:1 w:1)
	/// Proof Skipped: ParachainStaking CandidatePool (max_values: Some(1), max_size: None, mode: Measured)
	/// The range of component `x` is `[3, 1000]`.
	fn cancel_leave_candidates(x: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1310 + x * (48 ±0)`
		//  Estimated: `5388 + x * (98 ±0)`
		// Minimum execution time: 31_549 nanoseconds.
		Weight::from_ref_time(29_583_930)
			.saturating_add(Weight::from_proof_size(5388))
			// Standard Error: 2_648
			.saturating_add(Weight::from_ref_time(178_018).saturating_mul(x.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
			.saturating_add(Weight::from_proof_size(98).saturating_mul(x.into()))
	}
	/// Storage: ParachainStaking CandidateInfo (r:1 w:1)
	/// Proof Skipped: ParachainStaking CandidateInfo (max_values: None, max_size: None, mode: Measured)
	/// Storage: ParachainStaking CandidatePool (r:1 w:1)
	/// Proof Skipped: ParachainStaking CandidatePool (max_values: Some(1), max_size: None, mode: Measured)
	fn go_offline() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `749`
		//  Estimated: `4468`
		// Minimum execution time: 30_752 nanoseconds.
		Weight::from_ref_time(31_229_000)
			.saturating_add(Weight::from_proof_size(4468))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: ParachainStaking CandidateInfo (r:1 w:1)
	/// Proof Skipped: ParachainStaking CandidateInfo (max_values: None, max_size: None, mode: Measured)
	/// Storage: ParachainStaking CandidatePool (r:1 w:1)
	/// Proof Skipped: ParachainStaking CandidatePool (max_values: Some(1), max_size: None, mode: Measured)
	fn go_online() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `700`
		//  Estimated: `4370`
		// Minimum execution time: 30_289 nanoseconds.
		Weight::from_ref_time(30_992_000)
			.saturating_add(Weight::from_proof_size(4370))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: ParachainStaking CandidateInfo (r:1 w:1)
	/// Proof Skipped: ParachainStaking CandidateInfo (max_values: None, max_size: None, mode: Measured)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: ParachainStaking Total (r:1 w:1)
	/// Proof Skipped: ParachainStaking Total (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ParachainStaking CandidatePool (r:1 w:1)
	/// Proof Skipped: ParachainStaking CandidatePool (max_values: Some(1), max_size: None, mode: Measured)
	fn candidate_bond_more() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `924`
		//  Estimated: `8840`
		// Minimum execution time: 53_549 nanoseconds.
		Weight::from_ref_time(54_288_000)
			.saturating_add(Weight::from_proof_size(8840))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: ParachainStaking CandidateInfo (r:1 w:1)
	/// Proof Skipped: ParachainStaking CandidateInfo (max_values: None, max_size: None, mode: Measured)
	/// Storage: ParachainStaking Round (r:1 w:0)
	/// Proof Skipped: ParachainStaking Round (max_values: Some(1), max_size: None, mode: Measured)
	fn schedule_candidate_bond_less() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `630`
		//  Estimated: `4230`
		// Minimum execution time: 28_217 nanoseconds.
		Weight::from_ref_time(28_568_000)
			.saturating_add(Weight::from_proof_size(4230))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: ParachainStaking CandidateInfo (r:1 w:1)
	/// Proof Skipped: ParachainStaking CandidateInfo (max_values: None, max_size: None, mode: Measured)
	/// Storage: ParachainStaking Round (r:1 w:0)
	/// Proof Skipped: ParachainStaking Round (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: ParachainStaking Total (r:1 w:1)
	/// Proof Skipped: ParachainStaking Total (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ParachainStaking CandidatePool (r:1 w:1)
	/// Proof Skipped: ParachainStaking CandidatePool (max_values: Some(1), max_size: None, mode: Measured)
	fn execute_candidate_bond_less() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1013`
		//  Estimated: `10615`
		// Minimum execution time: 67_804 nanoseconds.
		Weight::from_ref_time(73_286_000)
			.saturating_add(Weight::from_proof_size(10615))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: ParachainStaking CandidateInfo (r:1 w:1)
	/// Proof Skipped: ParachainStaking CandidateInfo (max_values: None, max_size: None, mode: Measured)
	fn cancel_candidate_bond_less() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `614`
		//  Estimated: `3089`
		// Minimum execution time: 24_883 nanoseconds.
		Weight::from_ref_time(26_447_000)
			.saturating_add(Weight::from_proof_size(3089))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: ParachainStaking DelegatorState (r:1 w:1)
	/// Proof Skipped: ParachainStaking DelegatorState (max_values: None, max_size: None, mode: Measured)
	/// Storage: ParachainStaking Round (r:1 w:0)
	/// Proof Skipped: ParachainStaking Round (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ParachainStaking DelegationScheduledRequests (r:1 w:1)
	/// Proof Skipped: ParachainStaking DelegationScheduledRequests (max_values: None, max_size: None, mode: Measured)
	fn schedule_leave_delegators() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `627`
		//  Estimated: `7326`
		// Minimum execution time: 33_533 nanoseconds.
		Weight::from_ref_time(38_613_000)
			.saturating_add(Weight::from_proof_size(7326))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: ParachainStaking DelegatorState (r:1 w:1)
	/// Proof Skipped: ParachainStaking DelegatorState (max_values: None, max_size: None, mode: Measured)
	/// Storage: ParachainStaking Round (r:1 w:0)
	/// Proof Skipped: ParachainStaking Round (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ParachainStaking DelegationScheduledRequests (r:99 w:99)
	/// Proof Skipped: ParachainStaking DelegationScheduledRequests (max_values: None, max_size: None, mode: Measured)
	/// Storage: ParachainStaking CandidateInfo (r:99 w:99)
	/// Proof Skipped: ParachainStaking CandidateInfo (max_values: None, max_size: None, mode: Measured)
	/// Storage: ParachainStaking TopDelegations (r:99 w:99)
	/// Proof Skipped: ParachainStaking TopDelegations (max_values: None, max_size: None, mode: Measured)
	/// Storage: ParachainStaking CandidatePool (r:1 w:1)
	/// Proof Skipped: ParachainStaking CandidatePool (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: ParachainStaking Total (r:1 w:1)
	/// Proof Skipped: ParachainStaking Total (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ParachainStaking AutoCompoundingDelegations (r:99 w:0)
	/// Proof Skipped: ParachainStaking AutoCompoundingDelegations (max_values: None, max_size: None, mode: Measured)
	/// The range of component `x` is `[2, 100]`.
	fn execute_leave_delegators(x: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `841 + x * (554 ±0)`
		//  Estimated: `25979 + x * (14004 ±3)`
		// Minimum execution time: 100_558 nanoseconds.
		Weight::from_ref_time(103_307_000)
			.saturating_add(Weight::from_proof_size(25979))
			// Standard Error: 388_716
			.saturating_add(Weight::from_ref_time(48_334_073).saturating_mul(x.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().reads((4_u64).saturating_mul(x.into())))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(T::DbWeight::get().writes((3_u64).saturating_mul(x.into())))
			.saturating_add(Weight::from_proof_size(14004).saturating_mul(x.into()))
	}
	/// Storage: ParachainStaking DelegatorState (r:1 w:1)
	/// Proof Skipped: ParachainStaking DelegatorState (max_values: None, max_size: None, mode: Measured)
	/// Storage: ParachainStaking DelegationScheduledRequests (r:1 w:1)
	/// Proof Skipped: ParachainStaking DelegationScheduledRequests (max_values: None, max_size: None, mode: Measured)
	fn cancel_leave_delegators() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `783`
		//  Estimated: `6516`
		// Minimum execution time: 36_743 nanoseconds.
		Weight::from_ref_time(41_221_000)
			.saturating_add(Weight::from_proof_size(6516))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: ParachainStaking DelegatorState (r:1 w:1)
	/// Proof Skipped: ParachainStaking DelegatorState (max_values: None, max_size: None, mode: Measured)
	/// Storage: ParachainStaking DelegationScheduledRequests (r:1 w:1)
	/// Proof Skipped: ParachainStaking DelegationScheduledRequests (max_values: None, max_size: None, mode: Measured)
	/// Storage: ParachainStaking Round (r:1 w:0)
	/// Proof Skipped: ParachainStaking Round (max_values: Some(1), max_size: None, mode: Measured)
	fn schedule_revoke_delegation() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `627`
		//  Estimated: `7326`
		// Minimum execution time: 33_719 nanoseconds.
		Weight::from_ref_time(34_340_000)
			.saturating_add(Weight::from_proof_size(7326))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: ParachainStaking DelegationScheduledRequests (r:1 w:0)
	/// Proof Skipped: ParachainStaking DelegationScheduledRequests (max_values: None, max_size: None, mode: Measured)
	/// Storage: ParachainStaking DelegatorState (r:1 w:1)
	/// Proof Skipped: ParachainStaking DelegatorState (max_values: None, max_size: None, mode: Measured)
	/// Storage: ParachainStaking CandidateInfo (r:1 w:1)
	/// Proof Skipped: ParachainStaking CandidateInfo (max_values: None, max_size: None, mode: Measured)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: ParachainStaking TopDelegations (r:1 w:1)
	/// Proof Skipped: ParachainStaking TopDelegations (max_values: None, max_size: None, mode: Measured)
	/// Storage: ParachainStaking CandidatePool (r:1 w:1)
	/// Proof Skipped: ParachainStaking CandidatePool (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ParachainStaking Total (r:1 w:1)
	/// Proof Skipped: ParachainStaking Total (max_values: Some(1), max_size: None, mode: Measured)
	fn delegator_bond_more() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1376`
		//  Estimated: `21749`
		// Minimum execution time: 81_632 nanoseconds.
		Weight::from_ref_time(82_952_000)
			.saturating_add(Weight::from_proof_size(21749))
			.saturating_add(T::DbWeight::get().reads(7))
			.saturating_add(T::DbWeight::get().writes(6))
	}
	/// Storage: ParachainStaking DelegatorState (r:1 w:1)
	/// Proof Skipped: ParachainStaking DelegatorState (max_values: None, max_size: None, mode: Measured)
	/// Storage: ParachainStaking DelegationScheduledRequests (r:1 w:1)
	/// Proof Skipped: ParachainStaking DelegationScheduledRequests (max_values: None, max_size: None, mode: Measured)
	/// Storage: ParachainStaking Round (r:1 w:0)
	/// Proof Skipped: ParachainStaking Round (max_values: Some(1), max_size: None, mode: Measured)
	fn schedule_delegator_bond_less() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `627`
		//  Estimated: `7326`
		// Minimum execution time: 33_837 nanoseconds.
		Weight::from_ref_time(34_595_000)
			.saturating_add(Weight::from_proof_size(7326))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: ParachainStaking DelegatorState (r:1 w:1)
	/// Proof Skipped: ParachainStaking DelegatorState (max_values: None, max_size: None, mode: Measured)
	/// Storage: ParachainStaking DelegationScheduledRequests (r:1 w:1)
	/// Proof Skipped: ParachainStaking DelegationScheduledRequests (max_values: None, max_size: None, mode: Measured)
	/// Storage: ParachainStaking Round (r:1 w:0)
	/// Proof Skipped: ParachainStaking Round (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ParachainStaking AutoCompoundingDelegations (r:1 w:0)
	/// Proof Skipped: ParachainStaking AutoCompoundingDelegations (max_values: None, max_size: None, mode: Measured)
	/// Storage: ParachainStaking CandidateInfo (r:1 w:1)
	/// Proof Skipped: ParachainStaking CandidateInfo (max_values: None, max_size: None, mode: Measured)
	/// Storage: ParachainStaking TopDelegations (r:1 w:1)
	/// Proof Skipped: ParachainStaking TopDelegations (max_values: None, max_size: None, mode: Measured)
	/// Storage: ParachainStaking CandidatePool (r:1 w:1)
	/// Proof Skipped: ParachainStaking CandidatePool (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: ParachainStaking Total (r:1 w:1)
	/// Proof Skipped: ParachainStaking Total (max_values: Some(1), max_size: None, mode: Measured)
	fn execute_revoke_delegation() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1620`
		//  Estimated: `29423`
		// Minimum execution time: 114_913 nanoseconds.
		Weight::from_ref_time(134_422_000)
			.saturating_add(Weight::from_proof_size(29423))
			.saturating_add(T::DbWeight::get().reads(9))
			.saturating_add(T::DbWeight::get().writes(7))
	}
	/// Storage: ParachainStaking DelegatorState (r:1 w:1)
	/// Proof Skipped: ParachainStaking DelegatorState (max_values: None, max_size: None, mode: Measured)
	/// Storage: ParachainStaking DelegationScheduledRequests (r:1 w:1)
	/// Proof Skipped: ParachainStaking DelegationScheduledRequests (max_values: None, max_size: None, mode: Measured)
	/// Storage: ParachainStaking Round (r:1 w:0)
	/// Proof Skipped: ParachainStaking Round (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ParachainStaking CandidateInfo (r:1 w:1)
	/// Proof Skipped: ParachainStaking CandidateInfo (max_values: None, max_size: None, mode: Measured)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: ParachainStaking TopDelegations (r:1 w:1)
	/// Proof Skipped: ParachainStaking TopDelegations (max_values: None, max_size: None, mode: Measured)
	/// Storage: ParachainStaking CandidatePool (r:1 w:1)
	/// Proof Skipped: ParachainStaking CandidatePool (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ParachainStaking Total (r:1 w:1)
	/// Proof Skipped: ParachainStaking Total (max_values: Some(1), max_size: None, mode: Measured)
	fn execute_delegator_bond_less() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1565`
		//  Estimated: `24943`
		// Minimum execution time: 92_734 nanoseconds.
		Weight::from_ref_time(96_056_000)
			.saturating_add(Weight::from_proof_size(24943))
			.saturating_add(T::DbWeight::get().reads(8))
			.saturating_add(T::DbWeight::get().writes(7))
	}
	/// Storage: ParachainStaking DelegatorState (r:1 w:1)
	/// Proof Skipped: ParachainStaking DelegatorState (max_values: None, max_size: None, mode: Measured)
	/// Storage: ParachainStaking DelegationScheduledRequests (r:1 w:1)
	/// Proof Skipped: ParachainStaking DelegationScheduledRequests (max_values: None, max_size: None, mode: Measured)
	fn cancel_revoke_delegation() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `783`
		//  Estimated: `6516`
		// Minimum execution time: 34_185 nanoseconds.
		Weight::from_ref_time(38_416_000)
			.saturating_add(Weight::from_proof_size(6516))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: ParachainStaking DelegatorState (r:1 w:1)
	/// Proof Skipped: ParachainStaking DelegatorState (max_values: None, max_size: None, mode: Measured)
	/// Storage: ParachainStaking DelegationScheduledRequests (r:1 w:1)
	/// Proof Skipped: ParachainStaking DelegationScheduledRequests (max_values: None, max_size: None, mode: Measured)
	fn cancel_delegator_bond_less() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `816`
		//  Estimated: `6582`
		// Minimum execution time: 39_527 nanoseconds.
		Weight::from_ref_time(43_720_000)
			.saturating_add(Weight::from_proof_size(6582))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: ParachainStaking Round (r:1 w:1)
	/// Proof Skipped: ParachainStaking Round (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ParachainStaking Points (r:1 w:0)
	/// Proof Skipped: ParachainStaking Points (max_values: None, max_size: None, mode: Measured)
	/// Storage: ParachainStaking Staked (r:1 w:2)
	/// Proof Skipped: ParachainStaking Staked (max_values: None, max_size: None, mode: Measured)
	/// Storage: ParachainStaking InflationConfig (r:1 w:0)
	/// Proof Skipped: ParachainStaking InflationConfig (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: BridgeTransfer ExternalBalances (r:1 w:0)
	/// Proof Skipped: BridgeTransfer ExternalBalances (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ParachainStaking ParachainBondInfo (r:1 w:0)
	/// Proof Skipped: ParachainStaking ParachainBondInfo (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ParachainStaking CollatorCommission (r:1 w:0)
	/// Proof Skipped: ParachainStaking CollatorCommission (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ParachainStaking CandidatePool (r:1 w:0)
	/// Proof Skipped: ParachainStaking CandidatePool (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ParachainStaking TotalSelected (r:1 w:0)
	/// Proof Skipped: ParachainStaking TotalSelected (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ParachainStaking CandidateInfo (r:99 w:0)
	/// Proof Skipped: ParachainStaking CandidateInfo (max_values: None, max_size: None, mode: Measured)
	/// Storage: ParachainStaking DelegationScheduledRequests (r:99 w:0)
	/// Proof Skipped: ParachainStaking DelegationScheduledRequests (max_values: None, max_size: None, mode: Measured)
	/// Storage: ParachainStaking TopDelegations (r:99 w:0)
	/// Proof Skipped: ParachainStaking TopDelegations (max_values: None, max_size: None, mode: Measured)
	/// Storage: ParachainStaking AutoCompoundingDelegations (r:99 w:0)
	/// Proof Skipped: ParachainStaking AutoCompoundingDelegations (max_values: None, max_size: None, mode: Measured)
	/// Storage: ParachainStaking Total (r:1 w:0)
	/// Proof Skipped: ParachainStaking Total (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ParachainStaking AwardedPts (r:2 w:1)
	/// Proof Skipped: ParachainStaking AwardedPts (max_values: None, max_size: None, mode: Measured)
	/// Storage: ParachainStaking AtStake (r:1 w:100)
	/// Proof Skipped: ParachainStaking AtStake (max_values: None, max_size: None, mode: Measured)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: ParachainStaking SelectedCandidates (r:0 w:1)
	/// Proof Skipped: ParachainStaking SelectedCandidates (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ParachainStaking DelayedPayouts (r:0 w:1)
	/// Proof Skipped: ParachainStaking DelayedPayouts (max_values: None, max_size: None, mode: Measured)
	/// The range of component `x` is `[8, 100]`.
	/// The range of component `y` is `[0, 5000]`.
	fn round_transition_on_initialize(x: u32, y: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4500 + x * (329 ±0) + y * (47 ±0)`
		//  Estimated: `1388241 + y * (828 ±0) + x * (9452 ±51)`
		// Minimum execution time: 1_196_330 nanoseconds.
		Weight::from_ref_time(4_904_513_148)
			.saturating_add(Weight::from_proof_size(1388241))
			.saturating_add(T::DbWeight::get().reads(214))
			.saturating_add(T::DbWeight::get().reads((2_u64).saturating_mul(x.into())))
			.saturating_add(T::DbWeight::get().writes(207))
			.saturating_add(Weight::from_proof_size(828).saturating_mul(y.into()))
			.saturating_add(Weight::from_proof_size(9452).saturating_mul(x.into()))
	}
	/// Storage: ParachainStaking DelayedPayouts (r:1 w:0)
	/// Proof Skipped: ParachainStaking DelayedPayouts (max_values: None, max_size: None, mode: Measured)
	/// Storage: ParachainStaking Points (r:1 w:0)
	/// Proof Skipped: ParachainStaking Points (max_values: None, max_size: None, mode: Measured)
	/// Storage: ParachainStaking AwardedPts (r:2 w:1)
	/// Proof Skipped: ParachainStaking AwardedPts (max_values: None, max_size: None, mode: Measured)
	/// Storage: ParachainStaking AtStake (r:1 w:1)
	/// Proof Skipped: ParachainStaking AtStake (max_values: None, max_size: None, mode: Measured)
	/// Storage: System Account (r:1001 w:1001)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// The range of component `y` is `[0, 1000]`.
	fn pay_one_collator_reward(y: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `495 + y * (217 ±0)`
		//  Estimated: `17298 + y * (3471 ±0)`
		// Minimum execution time: 55_189 nanoseconds.
		Weight::from_ref_time(55_967_000)
			.saturating_add(Weight::from_proof_size(17298))
			// Standard Error: 71_997
			.saturating_add(Weight::from_ref_time(22_770_553).saturating_mul(y.into()))
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(y.into())))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(y.into())))
			.saturating_add(Weight::from_proof_size(3471).saturating_mul(y.into()))
	}
	/// Storage: ParachainStaking Round (r:1 w:0)
	/// Proof Skipped: ParachainStaking Round (max_values: Some(1), max_size: None, mode: Measured)
	fn base_on_initialize() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `447`
		//  Estimated: `942`
		// Minimum execution time: 7_448 nanoseconds.
		Weight::from_ref_time(7_622_000)
			.saturating_add(Weight::from_proof_size(942))
			.saturating_add(T::DbWeight::get().reads(1))
	}
	/// Storage: ParachainStaking DelegatorState (r:1 w:0)
	/// Proof Skipped: ParachainStaking DelegatorState (max_values: None, max_size: None, mode: Measured)
	/// Storage: ParachainStaking AutoCompoundingDelegations (r:1 w:1)
	/// Proof Skipped: ParachainStaking AutoCompoundingDelegations (max_values: None, max_size: None, mode: Measured)
	/// The range of component `x` is `[0, 1000]`.
	/// The range of component `y` is `[0, 100]`.
	fn set_auto_compound(x: u32, y: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1336 + x * (33 ±0) + y * (48 ±0)`
		//  Estimated: `7282 + x * (68 ±0) + y * (98 ±0)`
		// Minimum execution time: 49_123 nanoseconds.
		Weight::from_ref_time(69_376_742)
			.saturating_add(Weight::from_proof_size(7282))
			// Standard Error: 2_166
			.saturating_add(Weight::from_ref_time(121_145).saturating_mul(x.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(Weight::from_proof_size(68).saturating_mul(x.into()))
			.saturating_add(Weight::from_proof_size(98).saturating_mul(y.into()))
	}
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: ParachainStaking DelegatorState (r:1 w:1)
	/// Proof Skipped: ParachainStaking DelegatorState (max_values: None, max_size: None, mode: Measured)
	/// Storage: ParachainStaking CandidateInfo (r:1 w:1)
	/// Proof Skipped: ParachainStaking CandidateInfo (max_values: None, max_size: None, mode: Measured)
	/// Storage: ParachainStaking AutoCompoundingDelegations (r:1 w:1)
	/// Proof Skipped: ParachainStaking AutoCompoundingDelegations (max_values: None, max_size: None, mode: Measured)
	/// Storage: ParachainStaking BottomDelegations (r:1 w:1)
	/// Proof Skipped: ParachainStaking BottomDelegations (max_values: None, max_size: None, mode: Measured)
	/// Storage: ParachainStaking Total (r:1 w:1)
	/// Proof Skipped: ParachainStaking Total (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ParachainStaking TopDelegations (r:1 w:1)
	/// Proof Skipped: ParachainStaking TopDelegations (max_values: None, max_size: None, mode: Measured)
	/// Storage: ParachainStaking CandidatePool (r:1 w:1)
	/// Proof Skipped: ParachainStaking CandidatePool (max_values: Some(1), max_size: None, mode: Measured)
	/// The range of component `x` is `[0, 1200]`.
	/// The range of component `y` is `[0, 1200]`.
	/// The range of component `z` is `[0, 100]`.
	fn delegate_with_auto_compound(x: u32, y: u32, z: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0 + x * (81 ±0) + y * (33 ±0) + z * (55 ±0)`
		//  Estimated: `82406 + x * (212 ±0) + y * (163 ±0) + z * (31 ±0)`
		// Minimum execution time: 125_221 nanoseconds.
		Weight::from_ref_time(130_418_000)
			.saturating_add(Weight::from_proof_size(82406))
			// Standard Error: 2_697
			.saturating_add(Weight::from_ref_time(67_853).saturating_mul(x.into()))
			// Standard Error: 2_697
			.saturating_add(Weight::from_ref_time(90_889).saturating_mul(y.into()))
			.saturating_add(T::DbWeight::get().reads(7))
			.saturating_add(T::DbWeight::get().writes(7))
			.saturating_add(Weight::from_proof_size(212).saturating_mul(x.into()))
			.saturating_add(Weight::from_proof_size(163).saturating_mul(y.into()))
			.saturating_add(Weight::from_proof_size(31).saturating_mul(z.into()))
	}
}
