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

//! Autogenerated weights for `pallet_extrinsic_filter`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-03-15, STEPS: `20`, REPEAT: 50, LOW RANGE: `[]`, HIGH RANGE: `[]`
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
// --pallet=pallet_extrinsic_filter
// --extrinsic=*
// --heap-pages=4096
// --steps=20
// --repeat=50
// --header=./LICENSE_HEADER
// --output=./runtime/litentry/src/weights/pallet_extrinsic_filter.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_extrinsic_filter`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_extrinsic_filter::WeightInfo for WeightInfo<T> {
	// Storage: ExtrinsicFilter BlockedExtrinsics (r:1 w:1)
	/// The range of component `p` is `[1, 1024]`.
	/// The range of component `f` is `[1, 1024]`.
	fn block_extrinsics(p: u32, f: u32, ) -> Weight {
		// Minimum execution time: 29_413 nanoseconds.
		Weight::from_ref_time(27_079_250)
			// Standard Error: 60
			.saturating_add(Weight::from_ref_time(3_762).saturating_mul(p.into()))
			// Standard Error: 60
			.saturating_add(Weight::from_ref_time(3_814).saturating_mul(f.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: ExtrinsicFilter BlockedExtrinsics (r:1 w:1)
	/// The range of component `p` is `[1, 1024]`.
	/// The range of component `f` is `[1, 1024]`.
	fn unblock_extrinsics(p: u32, f: u32, ) -> Weight {
		// Minimum execution time: 44_942 nanoseconds.
		Weight::from_ref_time(28_101_445)
			// Standard Error: 73
			.saturating_add(Weight::from_ref_time(17_724).saturating_mul(p.into()))
			// Standard Error: 73
			.saturating_add(Weight::from_ref_time(17_918).saturating_mul(f.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
