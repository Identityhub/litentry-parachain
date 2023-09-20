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

//! Autogenerated weights for `pallet_vc_management`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-06-27, STEPS: `20`, REPEAT: `50`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
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
// --pallet=pallet_vc_management
// --extrinsic=*
// --heap-pages=4096
// --steps=20
// --repeat=50
// --header=./LICENSE_HEADER
// --output=./runtime/litmus/src/weights/pallet_vc_management.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_vc_management`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_vc_management::WeightInfo for WeightInfo<T> {
	/// Storage: VCManagement Delegatee (r:0 w:1)
	/// Proof: VCManagement Delegatee (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	fn add_delegatee() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 12_934 nanoseconds.
		Weight::from_parts(13_290_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: VCManagement Delegatee (r:1 w:1)
	/// Proof: VCManagement Delegatee (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	fn remove_delegatee() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `79`
		//  Estimated: `2523`
		// Minimum execution time: 19_200 nanoseconds.
		Weight::from_parts(19_830_000, 0)
			.saturating_add(Weight::from_parts(0, 2523))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: VCMPExtrinsicWhitelist GroupControlOn (r:1 w:0)
	/// Proof Skipped: VCMPExtrinsicWhitelist GroupControlOn (max_values: Some(1), max_size: None, mode: Measured)
	fn request_vc() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `42`
		//  Estimated: `537`
		// Minimum execution time: 14_714 nanoseconds.
		Weight::from_parts(15_328_000, 0)
			.saturating_add(Weight::from_parts(0, 537))
			.saturating_add(T::DbWeight::get().reads(1))
	}
	/// Storage: VCMPExtrinsicWhitelist GroupControlOn (r:1 w:0)
	/// Proof Skipped: VCMPExtrinsicWhitelist GroupControlOn (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: VCManagement VCRegistry (r:1 w:1)
	/// Proof: VCManagement VCRegistry (max_values: None, max_size: Some(312), added: 2787, mode: MaxEncodedLen)
	fn disable_vc() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `221`
		//  Estimated: `3503`
		// Minimum execution time: 23_184 nanoseconds.
		Weight::from_parts(23_607_000, 0)
			.saturating_add(Weight::from_parts(0, 3503))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: VCMPExtrinsicWhitelist GroupControlOn (r:1 w:0)
	/// Proof Skipped: VCMPExtrinsicWhitelist GroupControlOn (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: VCManagement VCRegistry (r:1 w:1)
	/// Proof: VCManagement VCRegistry (max_values: None, max_size: Some(312), added: 2787, mode: MaxEncodedLen)
	fn revoke_vc() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `221`
		//  Estimated: `3503`
		// Minimum execution time: 23_474 nanoseconds.
		Weight::from_parts(24_544_000, 0)
			.saturating_add(Weight::from_parts(0, 3503))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Teerex EnclaveIndex (r:1 w:0)
	/// Proof Skipped: Teerex EnclaveIndex (max_values: None, max_size: None, mode: Measured)
	/// Storage: VCManagement VCRegistry (r:1 w:1)
	/// Proof: VCManagement VCRegistry (max_values: None, max_size: Some(312), added: 2787, mode: MaxEncodedLen)
	fn vc_issued() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `248`
		//  Estimated: `5510`
		// Minimum execution time: 27_255 nanoseconds.
		Weight::from_parts(28_362_000, 0)
			.saturating_add(Weight::from_parts(0, 5510))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Teerex EnclaveIndex (r:1 w:0)
	/// Proof Skipped: Teerex EnclaveIndex (max_values: None, max_size: None, mode: Measured)
	fn some_error() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `242`
		//  Estimated: `2717`
		// Minimum execution time: 20_296 nanoseconds.
		Weight::from_parts(21_697_000, 0)
			.saturating_add(Weight::from_parts(0, 2717))
			.saturating_add(T::DbWeight::get().reads(1))
	}
	/// Storage: VCManagement Admin (r:1 w:1)
	/// Proof: VCManagement Admin (max_values: Some(1), max_size: Some(32), added: 527, mode: MaxEncodedLen)
	fn set_admin() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `61`
		//  Estimated: `527`
		// Minimum execution time: 16_116 nanoseconds.
		Weight::from_parts(16_611_000, 0)
			.saturating_add(Weight::from_parts(0, 527))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: VCManagement Admin (r:1 w:0)
	/// Proof: VCManagement Admin (max_values: Some(1), max_size: Some(32), added: 527, mode: MaxEncodedLen)
	/// Storage: VCManagement SchemaRegistryIndex (r:1 w:1)
	/// Proof: VCManagement SchemaRegistryIndex (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: VCManagement SchemaRegistry (r:0 w:1)
	/// Proof: VCManagement SchemaRegistry (max_values: None, max_size: Some(2621), added: 5096, mode: MaxEncodedLen)
	fn add_schema() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `61`
		//  Estimated: `1030`
		// Minimum execution time: 20_844 nanoseconds.
		Weight::from_parts(21_315_000, 0)
			.saturating_add(Weight::from_parts(0, 1030))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: VCManagement Admin (r:1 w:0)
	/// Proof: VCManagement Admin (max_values: Some(1), max_size: Some(32), added: 527, mode: MaxEncodedLen)
	/// Storage: VCManagement SchemaRegistry (r:1 w:1)
	/// Proof: VCManagement SchemaRegistry (max_values: None, max_size: Some(2621), added: 5096, mode: MaxEncodedLen)
	fn disable_schema() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `211`
		//  Estimated: `5623`
		// Minimum execution time: 21_419 nanoseconds.
		Weight::from_parts(21_880_000, 0)
			.saturating_add(Weight::from_parts(0, 5623))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: VCManagement Admin (r:1 w:0)
	/// Proof: VCManagement Admin (max_values: Some(1), max_size: Some(32), added: 527, mode: MaxEncodedLen)
	/// Storage: VCManagement SchemaRegistry (r:1 w:1)
	/// Proof: VCManagement SchemaRegistry (max_values: None, max_size: Some(2621), added: 5096, mode: MaxEncodedLen)
	fn activate_schema() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `211`
		//  Estimated: `5623`
		// Minimum execution time: 21_486 nanoseconds.
		Weight::from_parts(21_865_000, 0)
			.saturating_add(Weight::from_parts(0, 5623))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: VCManagement Admin (r:1 w:0)
	/// Proof: VCManagement Admin (max_values: Some(1), max_size: Some(32), added: 527, mode: MaxEncodedLen)
	/// Storage: VCManagement SchemaRegistry (r:1 w:1)
	/// Proof: VCManagement SchemaRegistry (max_values: None, max_size: Some(2621), added: 5096, mode: MaxEncodedLen)
	fn revoke_schema() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `211`
		//  Estimated: `5623`
		// Minimum execution time: 21_962 nanoseconds.
		Weight::from_parts(22_383_000, 0)
			.saturating_add(Weight::from_parts(0, 5623))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: VCManagement Admin (r:1 w:0)
	/// Proof: VCManagement Admin (max_values: Some(1), max_size: Some(32), added: 527, mode: MaxEncodedLen)
	/// Storage: VCManagement VCRegistry (r:1 w:1)
	/// Proof: VCManagement VCRegistry (max_values: None, max_size: Some(312), added: 2787, mode: MaxEncodedLen)
	fn add_vc_registry_item() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `61`
		//  Estimated: `3314`
		// Minimum execution time: 20_999 nanoseconds.
		Weight::from_parts(21_896_000, 0)
			.saturating_add(Weight::from_parts(0, 3314))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: VCManagement Admin (r:1 w:0)
	/// Proof: VCManagement Admin (max_values: Some(1), max_size: Some(32), added: 527, mode: MaxEncodedLen)
	/// Storage: VCManagement VCRegistry (r:1 w:1)
	/// Proof: VCManagement VCRegistry (max_values: None, max_size: Some(312), added: 2787, mode: MaxEncodedLen)
	fn remove_vc_registry_item() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `231`
		//  Estimated: `3314`
		// Minimum execution time: 21_792 nanoseconds.
		Weight::from_parts(22_250_000, 0)
			.saturating_add(Weight::from_parts(0, 3314))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: VCManagement Admin (r:1 w:0)
	/// Proof: VCManagement Admin (max_values: Some(1), max_size: Some(32), added: 527, mode: MaxEncodedLen)
	/// Storage: VCManagement VCRegistry (r:100 w:100)
	/// Proof: VCManagement VCRegistry (max_values: None, max_size: Some(312), added: 2787, mode: MaxEncodedLen)
	/// The range of component `x` is `[0, 100]`.
	fn clear_vc_registry(x: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `91 + x * (85 ±0)`
		//  Estimated: `527 + x * (2787 ±0)`
		// Minimum execution time: 17_334 nanoseconds.
		Weight::from_parts(19_346_737, 0)
			.saturating_add(Weight::from_parts(0, 527))
			// Standard Error: 4_480
			.saturating_add(Weight::from_parts(1_428_328, 0).saturating_mul(x.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(x.into())))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(x.into())))
			.saturating_add(Weight::from_parts(0, 2787).saturating_mul(x.into()))
	}
}
