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
//! DATE: 2023-06-22, STEPS: `20`, REPEAT: `50`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `parachain-benchmark`, CPU: `Intel(R) Xeon(R) Platinum 8259CL CPU @ 2.50GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("litentry-dev"), DB CACHE: 20

// Executed Command:
// ./litentry-collator
// benchmark
// pallet
// --chain=litentry-dev
// --execution=wasm
// --db-cache=20
// --wasm-execution=compiled
// --pallet=pallet_democracy
// --extrinsic=*
// --heap-pages=4096
// --steps=20
// --repeat=50
// --header=./LICENSE_HEADER
// --output=./runtime/litentry/src/weights/pallet_democracy.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_democracy`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_democracy::WeightInfo for WeightInfo<T> {
	/// Storage: Democracy PublicPropCount (r:1 w:1)
	/// Proof: Democracy PublicPropCount (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Democracy PublicProps (r:1 w:1)
	/// Proof: Democracy PublicProps (max_values: Some(1), max_size: Some(16702), added: 17197, mode: MaxEncodedLen)
	/// Storage: Democracy Blacklist (r:1 w:0)
	/// Proof: Democracy Blacklist (max_values: None, max_size: Some(3238), added: 5713, mode: MaxEncodedLen)
	/// Storage: Democracy DepositOf (r:0 w:1)
	/// Proof: Democracy DepositOf (max_values: None, max_size: Some(3230), added: 5705, mode: MaxEncodedLen)
	fn propose() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4797`
		//  Estimated: `23409`
		// Minimum execution time: 56_993 nanoseconds.
		Weight::from_ref_time(61_353_000)
			.saturating_add(Weight::from_proof_size(23409))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: Democracy DepositOf (r:1 w:1)
	/// Proof: Democracy DepositOf (max_values: None, max_size: Some(3230), added: 5705, mode: MaxEncodedLen)
	fn second() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3553`
		//  Estimated: `5705`
		// Minimum execution time: 52_331 nanoseconds.
		Weight::from_ref_time(53_636_000)
			.saturating_add(Weight::from_proof_size(5705))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Democracy ReferendumInfoOf (r:1 w:1)
	/// Proof: Democracy ReferendumInfoOf (max_values: None, max_size: Some(201), added: 2676, mode: MaxEncodedLen)
	/// Storage: Democracy VotingOf (r:1 w:1)
	/// Proof: Democracy VotingOf (max_values: None, max_size: Some(3795), added: 6270, mode: MaxEncodedLen)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	fn vote_new() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3460`
		//  Estimated: `12720`
		// Minimum execution time: 74_561 nanoseconds.
		Weight::from_ref_time(79_977_000)
			.saturating_add(Weight::from_proof_size(12720))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: Democracy ReferendumInfoOf (r:1 w:1)
	/// Proof: Democracy ReferendumInfoOf (max_values: None, max_size: Some(201), added: 2676, mode: MaxEncodedLen)
	/// Storage: Democracy VotingOf (r:1 w:1)
	/// Proof: Democracy VotingOf (max_values: None, max_size: Some(3795), added: 6270, mode: MaxEncodedLen)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	fn vote_existing() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3482`
		//  Estimated: `12720`
		// Minimum execution time: 73_079 nanoseconds.
		Weight::from_ref_time(79_801_000)
			.saturating_add(Weight::from_proof_size(12720))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: Democracy ReferendumInfoOf (r:1 w:1)
	/// Proof: Democracy ReferendumInfoOf (max_values: None, max_size: Some(201), added: 2676, mode: MaxEncodedLen)
	/// Storage: Democracy Cancellations (r:1 w:1)
	/// Proof: Democracy Cancellations (max_values: None, max_size: Some(33), added: 2508, mode: MaxEncodedLen)
	/// Storage: Democracy MetadataOf (r:1 w:1)
	/// Proof: Democracy MetadataOf (max_values: None, max_size: Some(53), added: 2528, mode: MaxEncodedLen)
	fn emergency_cancel() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `331`
		//  Estimated: `7712`
		// Minimum execution time: 37_537 nanoseconds.
		Weight::from_ref_time(38_578_000)
			.saturating_add(Weight::from_proof_size(7712))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: Democracy PublicProps (r:1 w:1)
	/// Proof: Democracy PublicProps (max_values: Some(1), max_size: Some(16702), added: 17197, mode: MaxEncodedLen)
	/// Storage: Democracy DepositOf (r:1 w:1)
	/// Proof: Democracy DepositOf (max_values: None, max_size: Some(3230), added: 5705, mode: MaxEncodedLen)
	/// Storage: System Account (r:2 w:2)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Democracy MetadataOf (r:3 w:1)
	/// Proof: Democracy MetadataOf (max_values: None, max_size: Some(53), added: 2528, mode: MaxEncodedLen)
	/// Storage: Democracy NextExternal (r:1 w:1)
	/// Proof: Democracy NextExternal (max_values: Some(1), max_size: Some(132), added: 627, mode: MaxEncodedLen)
	/// Storage: Democracy ReferendumInfoOf (r:1 w:1)
	/// Proof: Democracy ReferendumInfoOf (max_values: None, max_size: Some(201), added: 2676, mode: MaxEncodedLen)
	/// Storage: Democracy Blacklist (r:0 w:1)
	/// Proof: Democracy Blacklist (max_values: None, max_size: Some(3238), added: 5713, mode: MaxEncodedLen)
	fn blacklist() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `6340`
		//  Estimated: `38995`
		// Minimum execution time: 149_915 nanoseconds.
		Weight::from_ref_time(157_687_000)
			.saturating_add(Weight::from_proof_size(38995))
			.saturating_add(T::DbWeight::get().reads(9))
			.saturating_add(T::DbWeight::get().writes(8))
	}
	/// Storage: Democracy NextExternal (r:1 w:1)
	/// Proof: Democracy NextExternal (max_values: Some(1), max_size: Some(132), added: 627, mode: MaxEncodedLen)
	/// Storage: Democracy Blacklist (r:1 w:0)
	/// Proof: Democracy Blacklist (max_values: None, max_size: Some(3238), added: 5713, mode: MaxEncodedLen)
	fn external_propose() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3381`
		//  Estimated: `6340`
		// Minimum execution time: 18_746 nanoseconds.
		Weight::from_ref_time(19_441_000)
			.saturating_add(Weight::from_proof_size(6340))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Democracy NextExternal (r:0 w:1)
	/// Proof: Democracy NextExternal (max_values: Some(1), max_size: Some(132), added: 627, mode: MaxEncodedLen)
	fn external_propose_majority() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 4_308 nanoseconds.
		Weight::from_ref_time(4_557_000)
			.saturating_add(Weight::from_proof_size(0))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Democracy NextExternal (r:0 w:1)
	/// Proof: Democracy NextExternal (max_values: Some(1), max_size: Some(132), added: 627, mode: MaxEncodedLen)
	fn external_propose_default() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 4_501 nanoseconds.
		Weight::from_ref_time(4_945_000)
			.saturating_add(Weight::from_proof_size(0))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Democracy NextExternal (r:1 w:1)
	/// Proof: Democracy NextExternal (max_values: Some(1), max_size: Some(132), added: 627, mode: MaxEncodedLen)
	/// Storage: Democracy ReferendumCount (r:1 w:1)
	/// Proof: Democracy ReferendumCount (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Democracy MetadataOf (r:1 w:2)
	/// Proof: Democracy MetadataOf (max_values: None, max_size: Some(53), added: 2528, mode: MaxEncodedLen)
	/// Storage: Democracy ReferendumInfoOf (r:0 w:1)
	/// Proof: Democracy ReferendumInfoOf (max_values: None, max_size: Some(201), added: 2676, mode: MaxEncodedLen)
	fn fast_track() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `219`
		//  Estimated: `3654`
		// Minimum execution time: 40_047 nanoseconds.
		Weight::from_ref_time(40_644_000)
			.saturating_add(Weight::from_proof_size(3654))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	/// Storage: Democracy NextExternal (r:1 w:1)
	/// Proof: Democracy NextExternal (max_values: Some(1), max_size: Some(132), added: 627, mode: MaxEncodedLen)
	/// Storage: Democracy Blacklist (r:1 w:1)
	/// Proof: Democracy Blacklist (max_values: None, max_size: Some(3238), added: 5713, mode: MaxEncodedLen)
	/// Storage: Democracy MetadataOf (r:1 w:1)
	/// Proof: Democracy MetadataOf (max_values: None, max_size: Some(53), added: 2528, mode: MaxEncodedLen)
	fn veto_external() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3484`
		//  Estimated: `8868`
		// Minimum execution time: 45_777 nanoseconds.
		Weight::from_ref_time(46_635_000)
			.saturating_add(Weight::from_proof_size(8868))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: Democracy PublicProps (r:1 w:1)
	/// Proof: Democracy PublicProps (max_values: Some(1), max_size: Some(16702), added: 17197, mode: MaxEncodedLen)
	/// Storage: Democracy DepositOf (r:1 w:1)
	/// Proof: Democracy DepositOf (max_values: None, max_size: Some(3230), added: 5705, mode: MaxEncodedLen)
	/// Storage: System Account (r:2 w:2)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Democracy MetadataOf (r:1 w:1)
	/// Proof: Democracy MetadataOf (max_values: None, max_size: Some(53), added: 2528, mode: MaxEncodedLen)
	fn cancel_proposal() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `6219`
		//  Estimated: `30636`
		// Minimum execution time: 123_247 nanoseconds.
		Weight::from_ref_time(131_371_000)
			.saturating_add(Weight::from_proof_size(30636))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	/// Storage: Democracy MetadataOf (r:1 w:1)
	/// Proof: Democracy MetadataOf (max_values: None, max_size: Some(53), added: 2528, mode: MaxEncodedLen)
	/// Storage: Democracy ReferendumInfoOf (r:0 w:1)
	/// Proof: Democracy ReferendumInfoOf (max_values: None, max_size: Some(201), added: 2676, mode: MaxEncodedLen)
	fn cancel_referendum() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `204`
		//  Estimated: `2528`
		// Minimum execution time: 27_639 nanoseconds.
		Weight::from_ref_time(29_381_000)
			.saturating_add(Weight::from_proof_size(2528))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Democracy LowestUnbaked (r:1 w:1)
	/// Proof: Democracy LowestUnbaked (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Democracy ReferendumCount (r:1 w:0)
	/// Proof: Democracy ReferendumCount (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Democracy ReferendumInfoOf (r:99 w:0)
	/// Proof: Democracy ReferendumInfoOf (max_values: None, max_size: Some(201), added: 2676, mode: MaxEncodedLen)
	/// The range of component `r` is `[0, 99]`.
	fn on_initialize_base(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `176 + r * (117 ±0)`
		//  Estimated: `998 + r * (2676 ±0)`
		// Minimum execution time: 9_911 nanoseconds.
		Weight::from_ref_time(7_799_222)
			.saturating_add(Weight::from_proof_size(998))
			// Standard Error: 30_722
			.saturating_add(Weight::from_ref_time(4_515_323).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(r.into())))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(Weight::from_proof_size(2676).saturating_mul(r.into()))
	}
	/// Storage: Democracy LowestUnbaked (r:1 w:1)
	/// Proof: Democracy LowestUnbaked (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Democracy ReferendumCount (r:1 w:0)
	/// Proof: Democracy ReferendumCount (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Democracy LastTabledWasExternal (r:1 w:0)
	/// Proof: Democracy LastTabledWasExternal (max_values: Some(1), max_size: Some(1), added: 496, mode: MaxEncodedLen)
	/// Storage: Democracy NextExternal (r:1 w:0)
	/// Proof: Democracy NextExternal (max_values: Some(1), max_size: Some(132), added: 627, mode: MaxEncodedLen)
	/// Storage: Democracy PublicProps (r:1 w:0)
	/// Proof: Democracy PublicProps (max_values: Some(1), max_size: Some(16702), added: 17197, mode: MaxEncodedLen)
	/// Storage: Democracy ReferendumInfoOf (r:99 w:0)
	/// Proof: Democracy ReferendumInfoOf (max_values: None, max_size: Some(201), added: 2676, mode: MaxEncodedLen)
	/// The range of component `r` is `[0, 99]`.
	fn on_initialize_base_with_launch_period(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `176 + r * (117 ±0)`
		//  Estimated: `19318 + r * (2676 ±0)`
		// Minimum execution time: 13_863 nanoseconds.
		Weight::from_ref_time(5_758_464)
			.saturating_add(Weight::from_proof_size(19318))
			// Standard Error: 60_051
			.saturating_add(Weight::from_ref_time(4_547_636).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(r.into())))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(Weight::from_proof_size(2676).saturating_mul(r.into()))
	}
	/// Storage: Democracy VotingOf (r:3 w:3)
	/// Proof: Democracy VotingOf (max_values: None, max_size: Some(3795), added: 6270, mode: MaxEncodedLen)
	/// Storage: Democracy ReferendumInfoOf (r:99 w:99)
	/// Proof: Democracy ReferendumInfoOf (max_values: None, max_size: Some(201), added: 2676, mode: MaxEncodedLen)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	/// The range of component `r` is `[0, 99]`.
	fn delegate(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `854 + r * (139 ±0)`
		//  Estimated: `22584 + r * (2676 ±0)`
		// Minimum execution time: 52_997 nanoseconds.
		Weight::from_ref_time(42_242_415)
			.saturating_add(Weight::from_proof_size(22584))
			// Standard Error: 57_414
			.saturating_add(Weight::from_ref_time(6_748_096).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(r.into())))
			.saturating_add(T::DbWeight::get().writes(4))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(r.into())))
			.saturating_add(Weight::from_proof_size(2676).saturating_mul(r.into()))
	}
	/// Storage: Democracy VotingOf (r:2 w:2)
	/// Proof: Democracy VotingOf (max_values: None, max_size: Some(3795), added: 6270, mode: MaxEncodedLen)
	/// Storage: Democracy ReferendumInfoOf (r:99 w:99)
	/// Proof: Democracy ReferendumInfoOf (max_values: None, max_size: Some(201), added: 2676, mode: MaxEncodedLen)
	/// The range of component `r` is `[0, 99]`.
	fn undelegate(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `488 + r * (139 ±0)`
		//  Estimated: `12540 + r * (2676 ±0)`
		// Minimum execution time: 30_688 nanoseconds.
		Weight::from_ref_time(27_071_837)
			.saturating_add(Weight::from_proof_size(12540))
			// Standard Error: 31_872
			.saturating_add(Weight::from_ref_time(6_280_035).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(r.into())))
			.saturating_add(T::DbWeight::get().writes(2))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(r.into())))
			.saturating_add(Weight::from_proof_size(2676).saturating_mul(r.into()))
	}
	/// Storage: Democracy PublicProps (r:0 w:1)
	/// Proof: Democracy PublicProps (max_values: Some(1), max_size: Some(16702), added: 17197, mode: MaxEncodedLen)
	fn clear_public_proposals() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 3_928 nanoseconds.
		Weight::from_ref_time(4_199_000)
			.saturating_add(Weight::from_proof_size(0))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Democracy VotingOf (r:1 w:1)
	/// Proof: Democracy VotingOf (max_values: None, max_size: Some(3795), added: 6270, mode: MaxEncodedLen)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// The range of component `r` is `[0, 99]`.
	fn unlock_remove(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `485`
		//  Estimated: `12647`
		// Minimum execution time: 26_317 nanoseconds.
		Weight::from_ref_time(37_694_752)
			.saturating_add(Weight::from_proof_size(12647))
			// Standard Error: 4_266
			.saturating_add(Weight::from_ref_time(120_917).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: Democracy VotingOf (r:1 w:1)
	/// Proof: Democracy VotingOf (max_values: None, max_size: Some(3795), added: 6270, mode: MaxEncodedLen)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// The range of component `r` is `[0, 99]`.
	fn unlock_set(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `485 + r * (22 ±0)`
		//  Estimated: `12647`
		// Minimum execution time: 34_364 nanoseconds.
		Weight::from_ref_time(36_375_232)
			.saturating_add(Weight::from_proof_size(12647))
			// Standard Error: 3_180
			.saturating_add(Weight::from_ref_time(214_113).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: Democracy ReferendumInfoOf (r:1 w:1)
	/// Proof: Democracy ReferendumInfoOf (max_values: None, max_size: Some(201), added: 2676, mode: MaxEncodedLen)
	/// Storage: Democracy VotingOf (r:1 w:1)
	/// Proof: Democracy VotingOf (max_values: None, max_size: Some(3795), added: 6270, mode: MaxEncodedLen)
	/// The range of component `r` is `[1, 100]`.
	fn remove_vote(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `701 + r * (27 ±0)`
		//  Estimated: `8946`
		// Minimum execution time: 22_027 nanoseconds.
		Weight::from_ref_time(27_742_910)
			.saturating_add(Weight::from_proof_size(8946))
			// Standard Error: 2_583
			.saturating_add(Weight::from_ref_time(176_545).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Democracy ReferendumInfoOf (r:1 w:1)
	/// Proof: Democracy ReferendumInfoOf (max_values: None, max_size: Some(201), added: 2676, mode: MaxEncodedLen)
	/// Storage: Democracy VotingOf (r:1 w:1)
	/// Proof: Democracy VotingOf (max_values: None, max_size: Some(3795), added: 6270, mode: MaxEncodedLen)
	/// The range of component `r` is `[1, 100]`.
	fn remove_other_vote(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `701 + r * (27 ±0)`
		//  Estimated: `8946`
		// Minimum execution time: 23_002 nanoseconds.
		Weight::from_ref_time(26_448_785)
			.saturating_add(Weight::from_proof_size(8946))
			// Standard Error: 1_986
			.saturating_add(Weight::from_ref_time(212_347).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Democracy NextExternal (r:1 w:0)
	/// Proof: Democracy NextExternal (max_values: Some(1), max_size: Some(132), added: 627, mode: MaxEncodedLen)
	/// Storage: Preimage StatusFor (r:1 w:0)
	/// Proof: Preimage StatusFor (max_values: None, max_size: Some(91), added: 2566, mode: MaxEncodedLen)
	/// Storage: Democracy MetadataOf (r:0 w:1)
	/// Proof: Democracy MetadataOf (max_values: None, max_size: Some(53), added: 2528, mode: MaxEncodedLen)
	fn set_external_metadata() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `289`
		//  Estimated: `3193`
		// Minimum execution time: 26_834 nanoseconds.
		Weight::from_ref_time(27_804_000)
			.saturating_add(Weight::from_proof_size(3193))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Democracy NextExternal (r:1 w:0)
	/// Proof: Democracy NextExternal (max_values: Some(1), max_size: Some(132), added: 627, mode: MaxEncodedLen)
	/// Storage: Democracy MetadataOf (r:1 w:1)
	/// Proof: Democracy MetadataOf (max_values: None, max_size: Some(53), added: 2528, mode: MaxEncodedLen)
	fn clear_external_metadata() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `219`
		//  Estimated: `3155`
		// Minimum execution time: 23_140 nanoseconds.
		Weight::from_ref_time(24_330_000)
			.saturating_add(Weight::from_proof_size(3155))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Democracy PublicProps (r:1 w:0)
	/// Proof: Democracy PublicProps (max_values: Some(1), max_size: Some(16702), added: 17197, mode: MaxEncodedLen)
	/// Storage: Preimage StatusFor (r:1 w:0)
	/// Proof: Preimage StatusFor (max_values: None, max_size: Some(91), added: 2566, mode: MaxEncodedLen)
	/// Storage: Democracy MetadataOf (r:0 w:1)
	/// Proof: Democracy MetadataOf (max_values: None, max_size: Some(53), added: 2528, mode: MaxEncodedLen)
	fn set_proposal_metadata() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4852`
		//  Estimated: `19763`
		// Minimum execution time: 60_781 nanoseconds.
		Weight::from_ref_time(65_515_000)
			.saturating_add(Weight::from_proof_size(19763))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Democracy PublicProps (r:1 w:0)
	/// Proof: Democracy PublicProps (max_values: Some(1), max_size: Some(16702), added: 17197, mode: MaxEncodedLen)
	/// Storage: Democracy MetadataOf (r:1 w:1)
	/// Proof: Democracy MetadataOf (max_values: None, max_size: Some(53), added: 2528, mode: MaxEncodedLen)
	fn clear_proposal_metadata() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4786`
		//  Estimated: `19725`
		// Minimum execution time: 52_801 nanoseconds.
		Weight::from_ref_time(57_034_000)
			.saturating_add(Weight::from_proof_size(19725))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Preimage StatusFor (r:1 w:0)
	/// Proof: Preimage StatusFor (max_values: None, max_size: Some(91), added: 2566, mode: MaxEncodedLen)
	/// Storage: Democracy MetadataOf (r:0 w:1)
	/// Proof: Democracy MetadataOf (max_values: None, max_size: Some(53), added: 2528, mode: MaxEncodedLen)
	fn set_referendum_metadata() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `144`
		//  Estimated: `2566`
		// Minimum execution time: 21_000 nanoseconds.
		Weight::from_ref_time(21_493_000)
			.saturating_add(Weight::from_proof_size(2566))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Democracy ReferendumInfoOf (r:1 w:0)
	/// Proof: Democracy ReferendumInfoOf (max_values: None, max_size: Some(201), added: 2676, mode: MaxEncodedLen)
	/// Storage: Democracy MetadataOf (r:1 w:1)
	/// Proof: Democracy MetadataOf (max_values: None, max_size: Some(53), added: 2528, mode: MaxEncodedLen)
	fn clear_referendum_metadata() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `235`
		//  Estimated: `5204`
		// Minimum execution time: 26_272 nanoseconds.
		Weight::from_ref_time(29_909_000)
			.saturating_add(Weight::from_proof_size(5204))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
