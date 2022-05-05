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

use super::*;
use crate as pallet_asset_manager;
use codec::{Decode, Encode};

use frame_support::{construct_runtime, parameter_types, traits::Everything, RuntimeDebug};
use frame_system::EnsureRoot;
use scale_info::TypeInfo;
use sp_core::H256;
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, Hash as THash, IdentityLookup},
};
use xcm::latest::prelude::*;

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

pub type AccountId = u64;
pub type Balance = u64;

construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
		Balances: pallet_balances::{Pallet, Call, Storage, Event<T>},
		AssetManager: pallet_asset_manager::{Pallet, Call, Storage, Event<T>},
	}
);

parameter_types! {
	pub const BlockHashCount: u64 = 250;
}
impl frame_system::Config for Test {
	type BaseCallFilter = Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type Origin = Origin;
	type Call = Call;
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = AccountId;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = Event;
	type BlockHashCount = BlockHashCount;
	type DbWeight = ();
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = pallet_balances::AccountData<u64>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ();
	type OnSetCode = ();
	type MaxConsumers = frame_support::traits::ConstU32<16>;
}

parameter_types! {
	pub const ExistentialDeposit: u64 = 1;
}

impl pallet_balances::Config for Test {
	type Balance = Balance;
	type DustRemoval = ();
	type Event = Event;
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = System;
	type WeightInfo = ();
	type MaxLocks = ();
	type MaxReserves = ();
	type ReserveIdentifier = [u8; 8];
}

pub type AssetId = u32;
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Encode, Decode, RuntimeDebug, TypeInfo)]
pub enum MockAssetType {
	MockAsset(AssetId),
	Xcm(Box<MultiLocation>),
}

impl Default for MockAssetType {
	fn default() -> Self {
		Self::MockAsset(0)
	}
}

impl From<MockAssetType> for AssetId {
	fn from(asset: MockAssetType) -> AssetId {
		match asset {
			MockAssetType::MockAsset(id) => id,
			MockAssetType::Xcm(id) => {
				let mut result: [u8; 4] = [0u8; 4];
				let hash: H256 = (*id).using_encoded(<Test as frame_system::Config>::Hashing::hash);
				result.copy_from_slice(&hash.as_fixed_bytes()[0..4]);
				u32::from_le_bytes(result)
			},
		}
	}
}

impl From<MultiLocation> for MockAssetType {
	fn from(location: MultiLocation) -> Self {
		Self::Xcm(Box::new(location))
	}
}

impl From<MockAssetType> for Option<MultiLocation> {
	fn from(asset: MockAssetType) -> Option<MultiLocation> {
		match asset {
			MockAssetType::Xcm(location) => Some(*location),
			_ => None,
		}
	}
}

impl Config for Test {
	type Event = Event;
	type Balance = u64;
	type AssetId = u32;
	type ForeignAssetType = MockAssetType;
	type ForeignAssetModifierOrigin = EnsureRoot<u64>;
	type Currency = Balances;
	type WeightInfo = ();
}

#[derive(Default)]
pub(crate) struct ExtBuilder {}

impl ExtBuilder {
	pub(crate) fn build(self) -> sp_io::TestExternalities {
		let t = frame_system::GenesisConfig::default()
			.build_storage::<Test>()
			.expect("Frame system builds valid default genesis config");

		let mut ext = sp_io::TestExternalities::new(t);
		ext.execute_with(|| System::set_block_number(1));
		ext
	}
}

pub(crate) fn events() -> Vec<super::Event<Test>> {
	System::events()
		.into_iter()
		.map(|r| r.event)
		.filter_map(|e| if let Event::AssetManager(inner) = e { Some(inner) } else { None })
		.collect::<Vec<_>>()
}

pub fn expect_events(e: Vec<super::Event<Test>>) {
	assert_eq!(events(), e);
}
