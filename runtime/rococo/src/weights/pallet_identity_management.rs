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

//! Autogenerated weights for `pallet_identity_management`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-10-09, STEPS: `20`, REPEAT: 50, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `parachain-benchmark`, CPU: `Intel(R) Xeon(R) Platinum 8124M CPU @ 3.00GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("rococo-dev"), DB CACHE: 20

// Executed Command:
// ./litentry-collator
// benchmark
// pallet
// --chain=rococo-dev
// --execution=wasm
// --db-cache=20
// --wasm-execution=compiled
// --pallet=pallet_identity_management
// --extrinsic=*
// --heap-pages=4096
// --steps=20
// --repeat=50
// --header=./LICENSE_HEADER
// --output=./runtime/rococo/src/weights/pallet_identity_management.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_identity_management`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_identity_management::WeightInfo for WeightInfo<T> {
	fn create_identity() -> Weight {
		Weight::from_ref_time(16_109_000 as u64)
	}
	fn remove_identity() -> Weight {
		Weight::from_ref_time(14_568_000 as u64)
	}
	fn verify_identity() -> Weight {
		Weight::from_ref_time(15_977_000 as u64)
	}
	fn set_user_shielding_key() -> Weight {
		Weight::from_ref_time(14_703_000 as u64)
	}
}
