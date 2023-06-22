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

//! Autogenerated weights for `pallet_sidechain`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-06-22, STEPS: `20`, REPEAT: `50`, LOW RANGE: `[]`, HIGH RANGE: `[]`
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
// --pallet=pallet_sidechain
// --extrinsic=*
// --heap-pages=4096
// --steps=20
// --repeat=50
// --header=./LICENSE_HEADER
// --output=./runtime/litmus/src/weights/pallet_sidechain.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_sidechain`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_sidechain::WeightInfo for WeightInfo<T> {
	/// Storage: Teerex EnclaveIndex (r:1 w:0)
	/// Proof Skipped: Teerex EnclaveIndex (max_values: None, max_size: None, mode: Measured)
	/// Storage: Teerex EnclaveRegistry (r:1 w:0)
	/// Proof Skipped: Teerex EnclaveRegistry (max_values: None, max_size: None, mode: Measured)
	/// Storage: Sidechain SidechainBlockFinalizationCandidate (r:1 w:1)
	/// Proof Skipped: Sidechain SidechainBlockFinalizationCandidate (max_values: None, max_size: None, mode: Measured)
	/// Storage: Sidechain WorkerForShard (r:0 w:1)
	/// Proof Skipped: Sidechain WorkerForShard (max_values: None, max_size: None, mode: Measured)
	/// Storage: Sidechain LatestSidechainBlockConfirmation (r:0 w:1)
	/// Proof Skipped: Sidechain LatestSidechainBlockConfirmation (max_values: None, max_size: None, mode: Measured)
	fn confirm_imported_sidechain_block() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `371`
		//  Estimated: `9280`
		// Minimum execution time: 36_446 nanoseconds.
		Weight::from_ref_time(37_209_000)
			.saturating_add(Weight::from_proof_size(9280))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
}
