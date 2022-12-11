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

//! Autogenerated weights for `pallet_teerex`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-12-09, STEPS: `20`, REPEAT: 50, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `parachain-benchmark`, CPU: `Intel(R) Xeon(R) Platinum 8175M CPU @ 2.50GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("rococo-dev"), DB CACHE: 20

// Executed Command:
// ./litentry-collator
// benchmark
// pallet
// --chain=rococo-dev
// --execution=wasm
// --db-cache=20
// --wasm-execution=compiled
// --pallet=pallet_teerex
// --extrinsic=*
// --heap-pages=4096
// --steps=20
// --repeat=50
// --header=./LICENSE_HEADER
// --output=./runtime/rococo/src/weights/pallet_teerex.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_teerex`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_teerex::WeightInfo for WeightInfo<T> {
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: Teerex AllowSGXDebugMode (r:1 w:0)
	// Storage: Teerex EnclaveIndex (r:1 w:0)
	// Storage: Teerex EnclaveRegistry (r:0 w:1)
	fn register_enclave() -> Weight {
		// Minimum execution time: 1_980_934 nanoseconds.
		Weight::from_ref_time(1_992_096_000 as u64)
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Teerex EnclaveIndex (r:1 w:2)
	// Storage: Teerex EnclaveCount (r:1 w:1)
	// Storage: Teerex EnclaveRegistry (r:1 w:2)
	fn unregister_enclave() -> Weight {
		// Minimum execution time: 54_164 nanoseconds.
		Weight::from_ref_time(55_182_000 as u64)
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(5 as u64))
	}
	fn call_worker() -> Weight {
		// Minimum execution time: 25_931 nanoseconds.
		Weight::from_ref_time(26_423_000 as u64)
	}
	// Storage: Teerex EnclaveIndex (r:1 w:0)
	fn confirm_processed_parentchain_block() -> Weight {
		// Minimum execution time: 32_325 nanoseconds.
		Weight::from_ref_time(33_073_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
	}
}
