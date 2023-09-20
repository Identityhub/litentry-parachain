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

//! Autogenerated weights for pallet_drop3
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2021-12-30, STEPS: `20`, REPEAT: 50, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 20

// Executed Command:
// ./target/release/litentry-collator
// benchmark
// --chain=dev
// --execution=wasm
// --db-cache=20
// --wasm-execution=compiled
// --pallet=pallet_drop3
// --extrinsic=*
// --heap-pages=4096
// --steps=20
// --repeat=50
// --header=./LICENSE_HEADER
// --template=./templates/benchmark/pallet-weight-template.hbs
// --output=./pallets/drop3/src/weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_drop3.
pub trait WeightInfo {
	fn set_admin() -> Weight;
	fn approve_reward_pool() -> Weight;
	fn reject_reward_pool() -> Weight;
	fn start_reward_pool() -> Weight;
	fn stop_reward_pool() -> Weight;
	fn close_reward_pool() -> Weight;
	fn propose_reward_pool(n: u32, ) -> Weight;
	fn send_reward() -> Weight;
}

/// Weights for pallet_drop3 using the Litentry node and recommended hardware.
pub struct LitentryWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for LitentryWeight<T> {
	// Storage: Drop3 Admin (r:1 w:1)
	fn set_admin() -> Weight {
		Weight::from_parts(25_000_000 as u64, 0)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Drop3 Admin (r:1 w:0)
	// Storage: Drop3 RewardPools (r:1 w:1)
	fn approve_reward_pool() -> Weight {
		Weight::from_parts(38_000_000 as u64, 0)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Drop3 Admin (r:1 w:0)
	// Storage: Drop3 RewardPools (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: Drop3 RewardPoolOwners (r:0 w:1)
	fn reject_reward_pool() -> Weight {
		Weight::from_parts(112_000_000 as u64, 0)
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: Drop3 RewardPools (r:1 w:1)
	// Storage: Drop3 Admin (r:1 w:0)
	fn start_reward_pool() -> Weight {
		Weight::from_parts(40_000_000 as u64, 0)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Drop3 RewardPools (r:1 w:1)
	// Storage: Drop3 Admin (r:1 w:0)
	fn stop_reward_pool() -> Weight {
		Weight::from_parts(41_000_000 as u64, 0)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Drop3 RewardPools (r:1 w:1)
	// Storage: Drop3 Admin (r:1 w:0)
	// Storage: System Account (r:1 w:1)
	// Storage: Drop3 RewardPoolOwners (r:0 w:1)
	fn close_reward_pool() -> Weight {
		Weight::from_parts(68_000_000 as u64, 0)
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: System Account (r:1 w:1)
	// Storage: Drop3 CurrentMaxPoolId (r:1 w:1)
	// Storage: Drop3 RewardPoolOwners (r:0 w:1)
	// Storage: Drop3 RewardPools (r:0 w:1)
	fn propose_reward_pool(n: u32, ) -> Weight {
		Weight::from_parts(61_962_000 as u64, 0)
			// Standard Error: 36_000
			.saturating_add(Weight::from_parts(1_135_000 as u64, 0).saturating_mul(n as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
	}
	// Storage: Drop3 RewardPools (r:1 w:1)
	// Storage: System Account (r:2 w:2)
	fn send_reward() -> Weight {
		Weight::from_parts(84_000_000 as u64, 0)
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: Drop3 Admin (r:1 w:1)
	fn set_admin() -> Weight {
		Weight::from_parts(25_000_000 as u64, 0)
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: Drop3 Admin (r:1 w:0)
	// Storage: Drop3 RewardPools (r:1 w:1)
	fn approve_reward_pool() -> Weight {
		Weight::from_parts(38_000_000 as u64, 0)
			.saturating_add(RocksDbWeight::get().reads(2 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: Drop3 Admin (r:1 w:0)
	// Storage: Drop3 RewardPools (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: Drop3 RewardPoolOwners (r:0 w:1)
	fn reject_reward_pool() -> Weight {
		Weight::from_parts(112_000_000 as u64, 0)
			.saturating_add(RocksDbWeight::get().reads(3 as u64))
			.saturating_add(RocksDbWeight::get().writes(3 as u64))
	}
	// Storage: Drop3 RewardPools (r:1 w:1)
	// Storage: Drop3 Admin (r:1 w:0)
	fn start_reward_pool() -> Weight {
		Weight::from_parts(40_000_000 as u64, 0)
			.saturating_add(RocksDbWeight::get().reads(2 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: Drop3 RewardPools (r:1 w:1)
	// Storage: Drop3 Admin (r:1 w:0)
	fn stop_reward_pool() -> Weight {
		Weight::from_parts(41_000_000 as u64, 0)
			.saturating_add(RocksDbWeight::get().reads(2 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: Drop3 RewardPools (r:1 w:1)
	// Storage: Drop3 Admin (r:1 w:0)
	// Storage: System Account (r:1 w:1)
	// Storage: Drop3 RewardPoolOwners (r:0 w:1)
	fn close_reward_pool() -> Weight {
		Weight::from_parts(68_000_000 as u64, 0)
			.saturating_add(RocksDbWeight::get().reads(3 as u64))
			.saturating_add(RocksDbWeight::get().writes(3 as u64))
	}
	// Storage: System Account (r:1 w:1)
	// Storage: Drop3 CurrentMaxPoolId (r:1 w:1)
	// Storage: Drop3 RewardPoolOwners (r:0 w:1)
	// Storage: Drop3 RewardPools (r:0 w:1)
	fn propose_reward_pool(n: u32, ) -> Weight {
		Weight::from_parts(61_962_000 as u64, 0)
			// Standard Error: 36_000
			.saturating_add(Weight::from_parts(1_135_000 as u64, 0).saturating_mul(n as u64))
			.saturating_add(RocksDbWeight::get().reads(2 as u64))
			.saturating_add(RocksDbWeight::get().writes(4 as u64))
	}
	// Storage: Drop3 RewardPools (r:1 w:1)
	// Storage: System Account (r:2 w:2)
	fn send_reward() -> Weight {
		Weight::from_parts(84_000_000 as u64, 0)
			.saturating_add(RocksDbWeight::get().reads(3 as u64))
			.saturating_add(RocksDbWeight::get().writes(3 as u64))
	}
}
