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

//! Autogenerated weights for `pallet_asset_manager`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-12-01, STEPS: `20`, REPEAT: 50, LOW RANGE: `[]`, HIGH RANGE: `[]`
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
// --pallet=pallet_asset_manager
// --extrinsic=*
// --heap-pages=4096
// --steps=20
// --repeat=50
// --header=./LICENSE_HEADER
// --output=./runtime/litmus/src/weights/pallet_asset_manager.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_asset_manager`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_asset_manager::WeightInfo for WeightInfo<T> {
	// Storage: AssetManager AssetTypeId (r:1 w:1)
	// Storage: AssetManager ForeignAssetTracker (r:1 w:1)
	// Storage: AssetManager AssetIdType (r:0 w:1)
	// Storage: AssetManager AssetIdMetadata (r:0 w:1)
	fn register_foreign_asset_type() -> Weight {
		// Minimum execution time: 41_220 nanoseconds.
		Weight::from_ref_time(42_340_000 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
	}
	// Storage: AssetManager AssetIdType (r:1 w:0)
	// Storage: AssetManager AssetIdMetadata (r:0 w:1)
	fn update_foreign_asset_metadata() -> Weight {
		// Minimum execution time: 28_343 nanoseconds.
		Weight::from_ref_time(28_952_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: AssetManager AssetIdType (r:1 w:0)
	// Storage: AssetManager AssetIdUnitsPerSecond (r:0 w:1)
	fn set_asset_units_per_second() -> Weight {
		// Minimum execution time: 27_179 nanoseconds.
		Weight::from_ref_time(27_956_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: AssetManager AssetIdType (r:1 w:1)
	// Storage: AssetManager AssetTypeId (r:1 w:1)
	fn add_asset_type() -> Weight {
		// Minimum execution time: 32_727 nanoseconds.
		Weight::from_ref_time(33_383_000 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: AssetManager AssetTypeId (r:2 w:1)
	// Storage: AssetManager AssetIdType (r:0 w:1)
	fn remove_asset_type() -> Weight {
		// Minimum execution time: 40_791 nanoseconds.
		Weight::from_ref_time(42_554_000 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
}
