// Copyright 2020-2022 Litentry Technologies GmbH.
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

//! Autogenerated weights for `pallet_drop3`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-10-08, STEPS: `20`, REPEAT: 50, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `parachain-benchmark`, CPU: `Intel(R) Xeon(R) Platinum 8275CL CPU @ 3.00GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("litentry-dev"), DB CACHE: 20

// Executed Command:
// ./litentry-collator
// benchmark
// pallet
// --chain=litentry-dev
// --execution=wasm
// --db-cache=20
// --wasm-execution=compiled
// --pallet=pallet_drop3
// --extrinsic=*
// --heap-pages=4096
// --steps=20
// --repeat=50
// --header=./LICENSE_HEADER
// --output=./runtime/litentry/src/weights/pallet_drop3.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_drop3`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_drop3::WeightInfo for WeightInfo<T> {
	// Storage: Drop3 Admin (r:1 w:1)
	fn set_admin() -> Weight {
		(14_753_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Drop3 Admin (r:1 w:0)
	// Storage: Drop3 RewardPools (r:1 w:1)
	fn approve_reward_pool() -> Weight {
		(20_937_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Drop3 Admin (r:1 w:0)
	// Storage: Drop3 RewardPools (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: Drop3 RewardPoolOwners (r:0 w:1)
	fn reject_reward_pool() -> Weight {
		(56_084_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: Drop3 RewardPools (r:1 w:1)
	// Storage: Drop3 Admin (r:1 w:0)
	fn start_reward_pool() -> Weight {
		(21_008_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Drop3 RewardPools (r:1 w:1)
	// Storage: Drop3 Admin (r:1 w:0)
	fn stop_reward_pool() -> Weight {
		(21_850_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Drop3 RewardPools (r:1 w:1)
	// Storage: Drop3 Admin (r:1 w:0)
	// Storage: System Account (r:1 w:1)
	// Storage: Drop3 RewardPoolOwners (r:0 w:1)
	fn close_reward_pool() -> Weight {
		(37_480_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: System Account (r:1 w:1)
	// Storage: Drop3 CurrentMaxPoolId (r:1 w:1)
	// Storage: Drop3 RewardPoolOwners (r:0 w:1)
	// Storage: Drop3 RewardPools (r:0 w:1)
	/// The range of component `n` is `[0, 16]`.
	fn propose_reward_pool(n: u32, ) -> Weight {
		(33_521_000 as Weight)
			// Standard Error: 2_000
			.saturating_add((9_000 as Weight).saturating_mul(n as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: Drop3 RewardPools (r:1 w:1)
	// Storage: System Account (r:2 w:2)
	fn send_reward() -> Weight {
		(39_169_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
}
