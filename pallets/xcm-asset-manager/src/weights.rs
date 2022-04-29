// Copyright 2019-2022 PureStake Inc.
// This file is part of Moonbeam.

// Moonbeam is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Moonbeam is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Moonbeam.  If not, see <http://www.gnu.org/licenses/>.

//! Autogenerated weights for pallet_asset_manager
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-03-16, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/moonbeam
// benchmark
// --chain
// dev
// --execution=wasm
// --wasm-execution=compiled
// --pallet
// *
// --extrinsic
// *
// --steps
// 50
// --repeat
// 20
// --template=./benchmarking/frame-weight-template.hbs
// --record-proof
// --json-file
// raw.json
// --output
// ./benchmarks/

#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use frame_support::{
	traits::Get,
	weights::{constants::RocksDbWeight, Weight},
};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_asset_manager.
pub trait WeightInfo {
	#[rustfmt::skip]
	fn register_foreign_asset() -> Weight;
	#[rustfmt::skip]
	fn relocate_foreign_asset_id() -> Weight;
	#[rustfmt::skip]
	fn update_foreign_asset_metadata() -> Weight;
	#[rustfmt::skip]
	fn set_asset_units_per_second(x: u32, ) -> Weight;
	#[rustfmt::skip]
	fn add_asset_type(x: u32, ) -> Weight;
	#[rustfmt::skip]
	fn remove_asset_type(x: u32, ) -> Weight;
}

/// Weights for pallet_asset_manager using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: AssetManager AssetIdType (r:1 w:1)
	// Storage: Assets Asset (r:1 w:1)
	// Storage: Assets Metadata (r:1 w:1)
	// Storage: AssetManager AssetTypeId (r:0 w:1)
	#[rustfmt::skip]
	fn register_foreign_asset() -> Weight {
		(47_597_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: AssetManager AssetIdType (r:1 w:1)
	// Storage: Assets Asset (r:1 w:1)
	// Storage: Assets Metadata (r:1 w:1)
	// Storage: AssetManager AssetTypeId (r:0 w:1)
	#[rustfmt::skip]
	fn relocate_foreign_asset_id() -> Weight {
		(47_597_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: AssetManager AssetIdType (r:1 w:1)
	// Storage: Assets Asset (r:1 w:1)
	// Storage: Assets Metadata (r:1 w:1)
	// Storage: AssetManager AssetTypeId (r:0 w:1)
	#[rustfmt::skip]
	fn update_foreign_asset_metadata() -> Weight {
		(47_597_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: AssetManager AssetTypeId (r:1 w:0)
	// Storage: AssetManager SupportedFeePaymentAssets (r:1 w:1)
	// Storage: AssetManager AssetTypeUnitsPerSecond (r:0 w:1)
	#[rustfmt::skip]
	fn set_asset_units_per_second(x: u32, ) -> Weight {
		(32_865_000 as Weight)
			// Standard Error: 3_000
			.saturating_add((1_210_000 as Weight).saturating_mul(x as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: AssetManager SupportedFeePaymentAssets (r:1 w:1)
	// Storage: AssetManager AssetIdType (r:1 w:1)
	// Storage: AssetManager AssetTypeUnitsPerSecond (r:1 w:2)
	// Storage: AssetManager AssetTypeId (r:0 w:2)
	#[rustfmt::skip]
	fn add_asset_type(x: u32, ) -> Weight {
		(40_757_000 as Weight)
			// Standard Error: 4_000
			.saturating_add((1_266_000 as Weight).saturating_mul(x as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(6 as Weight))
	}
	// Storage: AssetManager SupportedFeePaymentAssets (r:1 w:1)
	// Storage: AssetManager AssetTypeUnitsPerSecond (r:0 w:1)
	#[rustfmt::skip]
	fn remove_asset_type(x: u32, ) -> Weight {
		(25_700_000 as Weight)
			// Standard Error: 3_000
			.saturating_add((1_043_000 as Weight).saturating_mul(x as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: AssetManager AssetIdType (r:1 w:1)
	// Storage: Assets Asset (r:1 w:1)
	// Storage: Assets Metadata (r:1 w:1)
	// Storage: AssetManager AssetTypeId (r:0 w:1)
	#[rustfmt::skip]
	fn register_foreign_asset() -> Weight {
		(47_597_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(3 as Weight))
			.saturating_add(RocksDbWeight::get().writes(4 as Weight))
	}
	// Storage: AssetManager AssetIdType (r:1 w:1)
	// Storage: Assets Asset (r:1 w:1)
	// Storage: Assets Metadata (r:1 w:1)
	// Storage: AssetManager AssetTypeId (r:0 w:1)
	#[rustfmt::skip]
	fn relocate_foreign_asset_id() -> Weight {
		(47_597_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(3 as Weight))
			.saturating_add(RocksDbWeight::get().writes(4 as Weight))
	}
	// Storage: AssetManager AssetIdType (r:1 w:1)
	// Storage: Assets Asset (r:1 w:1)
	// Storage: Assets Metadata (r:1 w:1)
	// Storage: AssetManager AssetTypeId (r:0 w:1)
	#[rustfmt::skip]
	fn update_foreign_asset_metadata() -> Weight {
		(47_597_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(3 as Weight))
			.saturating_add(RocksDbWeight::get().writes(4 as Weight))
	}
	// Storage: AssetManager AssetTypeId (r:1 w:0)
	// Storage: AssetManager SupportedFeePaymentAssets (r:1 w:1)
	// Storage: AssetManager AssetTypeUnitsPerSecond (r:0 w:1)
	#[rustfmt::skip]
	fn set_asset_units_per_second(x: u32, ) -> Weight {
		(32_865_000 as Weight)
			// Standard Error: 3_000
			.saturating_add((1_210_000 as Weight).saturating_mul(x as Weight))
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	// Storage: AssetManager SupportedFeePaymentAssets (r:1 w:1)
	// Storage: AssetManager AssetIdType (r:1 w:1)
	// Storage: AssetManager AssetTypeUnitsPerSecond (r:1 w:2)
	// Storage: AssetManager AssetTypeId (r:0 w:2)
	#[rustfmt::skip]
	fn add_asset_type(x: u32, ) -> Weight {
		(40_757_000 as Weight)
			// Standard Error: 4_000
			.saturating_add((1_266_000 as Weight).saturating_mul(x as Weight))
			.saturating_add(RocksDbWeight::get().reads(3 as Weight))
			.saturating_add(RocksDbWeight::get().writes(6 as Weight))
	}
	// Storage: AssetManager SupportedFeePaymentAssets (r:1 w:1)
	// Storage: AssetManager AssetTypeUnitsPerSecond (r:0 w:1)
	#[rustfmt::skip]
	fn remove_asset_type(x: u32, ) -> Weight {
		(25_700_000 as Weight)
			// Standard Error: 3_000
			.saturating_add((1_043_000 as Weight).saturating_mul(x as Weight))
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
}
