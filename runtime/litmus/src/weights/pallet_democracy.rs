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

//! Autogenerated weights for `pallet_democracy`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-03-15, STEPS: `20`, REPEAT: 50, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `parachain-benchmark`, CPU: `Intel(R) Xeon(R) Platinum 8259CL CPU @ 2.50GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("litmus-dev"), DB CACHE: 20

// Executed Command:
// ./litentry-collator
// benchmark
// pallet
// --chain=litmus-dev
// --execution=wasm
// --db-cache=20
// --wasm-execution=compiled
// --pallet=pallet_democracy
// --extrinsic=*
// --heap-pages=4096
// --steps=20
// --repeat=50
// --header=./LICENSE_HEADER
// --output=./runtime/litmus/src/weights/pallet_democracy.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_democracy`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_democracy::WeightInfo for WeightInfo<T> {
	// Storage: Democracy PublicPropCount (r:1 w:1)
	// Storage: Democracy PublicProps (r:1 w:1)
	// Storage: Democracy Blacklist (r:1 w:0)
	// Storage: Democracy DepositOf (r:0 w:1)
	fn propose() -> Weight {
		// Minimum execution time: 71_215 nanoseconds.
		Weight::from_ref_time(73_318_000)
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	// Storage: Democracy DepositOf (r:1 w:1)
	fn second() -> Weight {
		// Minimum execution time: 61_909 nanoseconds.
		Weight::from_ref_time(63_772_000)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Democracy ReferendumInfoOf (r:1 w:1)
	// Storage: Democracy VotingOf (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	fn vote_new() -> Weight {
		// Minimum execution time: 80_965 nanoseconds.
		Weight::from_ref_time(82_680_000)
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	// Storage: Democracy ReferendumInfoOf (r:1 w:1)
	// Storage: Democracy VotingOf (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	fn vote_existing() -> Weight {
		// Minimum execution time: 78_414 nanoseconds.
		Weight::from_ref_time(82_876_000)
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	// Storage: Democracy ReferendumInfoOf (r:1 w:1)
	// Storage: Democracy Cancellations (r:1 w:1)
	fn emergency_cancel() -> Weight {
		// Minimum execution time: 31_461 nanoseconds.
		Weight::from_ref_time(32_009_000)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: Democracy PublicProps (r:1 w:1)
	// Storage: Democracy DepositOf (r:1 w:1)
	// Storage: System Account (r:2 w:2)
	// Storage: Democracy NextExternal (r:1 w:1)
	// Storage: Democracy ReferendumInfoOf (r:1 w:1)
	// Storage: Democracy Blacklist (r:0 w:1)
	fn blacklist() -> Weight {
		// Minimum execution time: 125_786 nanoseconds.
		Weight::from_ref_time(129_886_000)
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(7))
	}
	// Storage: Democracy NextExternal (r:1 w:1)
	// Storage: Democracy Blacklist (r:1 w:0)
	fn external_propose() -> Weight {
		// Minimum execution time: 23_807 nanoseconds.
		Weight::from_ref_time(24_965_000)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Democracy NextExternal (r:0 w:1)
	fn external_propose_majority() -> Weight {
		// Minimum execution time: 7_411 nanoseconds.
		Weight::from_ref_time(7_687_000)
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Democracy NextExternal (r:0 w:1)
	fn external_propose_default() -> Weight {
		// Minimum execution time: 7_633 nanoseconds.
		Weight::from_ref_time(7_909_000)
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Democracy NextExternal (r:1 w:1)
	// Storage: Democracy ReferendumCount (r:1 w:1)
	// Storage: Democracy ReferendumInfoOf (r:0 w:1)
	fn fast_track() -> Weight {
		// Minimum execution time: 31_155 nanoseconds.
		Weight::from_ref_time(31_998_000)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	// Storage: Democracy NextExternal (r:1 w:1)
	// Storage: Democracy Blacklist (r:1 w:1)
	fn veto_external() -> Weight {
		// Minimum execution time: 39_741 nanoseconds.
		Weight::from_ref_time(44_611_000)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: Democracy PublicProps (r:1 w:1)
	// Storage: Democracy DepositOf (r:1 w:1)
	// Storage: System Account (r:2 w:2)
	fn cancel_proposal() -> Weight {
		// Minimum execution time: 109_159 nanoseconds.
		Weight::from_ref_time(114_870_000)
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	// Storage: Democracy ReferendumInfoOf (r:0 w:1)
	fn cancel_referendum() -> Weight {
		// Minimum execution time: 20_283 nanoseconds.
		Weight::from_ref_time(21_815_000)
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Democracy LowestUnbaked (r:1 w:1)
	// Storage: Democracy ReferendumCount (r:1 w:0)
	// Storage: Democracy ReferendumInfoOf (r:5 w:0)
	/// The range of component `r` is `[0, 99]`.
	fn on_initialize_base(r: u32, ) -> Weight {
		// Minimum execution time: 8_916 nanoseconds.
		Weight::from_ref_time(5_606_729)
			// Standard Error: 26_863
			.saturating_add(Weight::from_ref_time(3_312_121).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(r.into())))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Democracy LowestUnbaked (r:1 w:1)
	// Storage: Democracy ReferendumCount (r:1 w:0)
	// Storage: Democracy LastTabledWasExternal (r:1 w:0)
	// Storage: Democracy NextExternal (r:1 w:0)
	// Storage: Democracy PublicProps (r:1 w:0)
	// Storage: Democracy ReferendumInfoOf (r:5 w:0)
	/// The range of component `r` is `[0, 99]`.
	fn on_initialize_base_with_launch_period(r: u32, ) -> Weight {
		// Minimum execution time: 12_193 nanoseconds.
		Weight::from_ref_time(6_189_453)
			// Standard Error: 42_402
			.saturating_add(Weight::from_ref_time(3_596_367).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(r.into())))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Democracy VotingOf (r:3 w:3)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: Democracy ReferendumInfoOf (r:5 w:5)
	/// The range of component `r` is `[0, 99]`.
	fn delegate(r: u32, ) -> Weight {
		// Minimum execution time: 63_636 nanoseconds.
		Weight::from_ref_time(80_591_265)
			// Standard Error: 81_896
			.saturating_add(Weight::from_ref_time(5_121_492).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(r.into())))
			.saturating_add(T::DbWeight::get().writes(4))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(r.into())))
	}
	// Storage: Democracy VotingOf (r:2 w:2)
	// Storage: Democracy ReferendumInfoOf (r:5 w:5)
	/// The range of component `r` is `[0, 99]`.
	fn undelegate(r: u32, ) -> Weight {
		// Minimum execution time: 38_641 nanoseconds.
		Weight::from_ref_time(28_711_611)
			// Standard Error: 40_643
			.saturating_add(Weight::from_ref_time(5_096_395).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(r.into())))
			.saturating_add(T::DbWeight::get().writes(2))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(r.into())))
	}
	// Storage: Democracy PublicProps (r:0 w:1)
	fn clear_public_proposals() -> Weight {
		// Minimum execution time: 7_688 nanoseconds.
		Weight::from_ref_time(7_965_000)
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Democracy VotingOf (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	/// The range of component `r` is `[0, 99]`.
	fn unlock_remove(r: u32, ) -> Weight {
		// Minimum execution time: 34_425 nanoseconds.
		Weight::from_ref_time(45_202_953)
			// Standard Error: 7_574
			.saturating_add(Weight::from_ref_time(171_400).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	// Storage: Democracy VotingOf (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	/// The range of component `r` is `[0, 99]`.
	fn unlock_set(r: u32, ) -> Weight {
		// Minimum execution time: 43_139 nanoseconds.
		Weight::from_ref_time(46_124_956)
			// Standard Error: 3_951
			.saturating_add(Weight::from_ref_time(167_647).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	// Storage: Democracy ReferendumInfoOf (r:1 w:1)
	// Storage: Democracy VotingOf (r:1 w:1)
	/// The range of component `r` is `[1, 100]`.
	fn remove_vote(r: u32, ) -> Weight {
		// Minimum execution time: 22_702 nanoseconds.
		Weight::from_ref_time(26_048_029)
			// Standard Error: 3_362
			.saturating_add(Weight::from_ref_time(198_327).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: Democracy ReferendumInfoOf (r:1 w:1)
	// Storage: Democracy VotingOf (r:1 w:1)
	/// The range of component `r` is `[1, 100]`.
	fn remove_other_vote(r: u32, ) -> Weight {
		// Minimum execution time: 22_738 nanoseconds.
		Weight::from_ref_time(24_794_744)
			// Standard Error: 3_315
			.saturating_add(Weight::from_ref_time(216_739).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
}
