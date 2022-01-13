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

#![allow(clippy::unnecessary_cast)]

pub mod frame_system;
pub mod pallet_balances;
// DEPRECATED: REMOVE AFTER RUNTIME UPGRADE
pub mod pallet_collator_selection;
pub mod pallet_collective;
pub mod pallet_democracy;
pub mod pallet_membership;
pub mod pallet_multisig;
pub mod pallet_proxy;
pub mod pallet_scheduler;
pub mod pallet_timestamp;
pub mod pallet_treasury;
pub mod pallet_utility;
pub mod parachain_staking;
