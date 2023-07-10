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

#[cfg(all(not(feature = "std"), feature = "sgx"))]
extern crate sgx_tstd as std;

#[cfg(all(not(feature = "sgx"), feature = "std"))]
use serde::{Deserialize, Serialize};

use codec::{Decode, Encode, MaxEncodedLen};
use parentchain_primitives::Web3Network;
use scale_info::TypeInfo;
use sp_runtime::{traits::ConstU32, BoundedVec};
use sp_std::vec::Vec;
use strum_macros::EnumIter;

pub type MaxStringLength = ConstU32<64>;
pub type IdentityString = BoundedVec<u8, MaxStringLength>;

#[derive(Encode, Decode, Copy, Clone, Debug, Default, PartialEq, Eq, TypeInfo, MaxEncodedLen)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct Address20([u8; 20]);

impl AsRef<[u8; 20]> for Address20 {
	fn as_ref(&self) -> &[u8; 20] {
		&self.0
	}
}

impl From<[u8; 20]> for Address20 {
	fn from(value: [u8; 20]) -> Self {
		Self(value)
	}
}

#[derive(Encode, Decode, Copy, Clone, Debug, Default, PartialEq, Eq, TypeInfo, MaxEncodedLen)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct Address32([u8; 32]);
impl AsRef<[u8; 32]> for Address32 {
	fn as_ref(&self) -> &[u8; 32] {
		&self.0
	}
}

impl From<[u8; 32]> for Address32 {
	fn from(value: [u8; 32]) -> Self {
		Self(value)
	}
}

/// Web2 and Web3 Identity based on handle/public key
/// We only include the network categories (substrate/evm) without concrete types
/// see https://github.com/litentry/litentry-parachain/issues/1841
#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq, TypeInfo, MaxEncodedLen, EnumIter)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum Identity {
	// web2
	Twitter(IdentityString),
	Discord(IdentityString),
	Github(IdentityString),

	// web3
	Substrate(Address32),
	Evm(Address20),
}

impl Identity {
	pub fn is_web2(&self) -> bool {
		matches!(self, Self::Twitter(..) | Self::Discord(..) | Self::Github(..))
	}

	pub fn is_web3(&self) -> bool {
		matches!(self, Self::Substrate(..) | Self::Evm(..))
	}

	pub fn is_substrate(&self) -> bool {
		matches!(self, Self::Substrate(..))
	}

	pub fn is_evm(&self) -> bool {
		matches!(self, Self::Evm(..))
	}

	// check if the given web3networks match the identity
	pub fn matches_web3networks(&self, networks: &Vec<Web3Network>) -> bool {
		(self.is_substrate() && !networks.is_empty() && networks.iter().all(|n| n.is_substrate()))
			|| (self.is_evm() && !networks.is_empty() && networks.iter().all(|n| n.is_evm()))
			|| (self.is_web2() && networks.is_empty())
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use sp_std::vec;
	use strum::IntoEnumIterator;

	#[test]
	fn is_web2_works() {
		Identity::iter().for_each(|identity| {
			assert_eq!(
				identity.is_web2(),
				match identity {
					Identity::Twitter(..) => true,
					Identity::Discord(..) => true,
					Identity::Github(..) => true,
					Identity::Substrate(..) => false,
					Identity::Evm(..) => false,
				}
			)
		})
	}

	#[test]
	fn is_web3_works() {
		Identity::iter().for_each(|identity| {
			assert_eq!(
				identity.is_web3(),
				match identity {
					Identity::Twitter(..) => false,
					Identity::Discord(..) => false,
					Identity::Github(..) => false,
					Identity::Substrate(..) => true,
					Identity::Evm(..) => true,
				}
			)
		})
	}

	#[test]
	fn is_substrate_works() {
		Identity::iter().for_each(|identity| {
			assert_eq!(
				identity.is_substrate(),
				match identity {
					Identity::Twitter(..) => false,
					Identity::Discord(..) => false,
					Identity::Github(..) => false,
					Identity::Substrate(..) => true,
					Identity::Evm(..) => false,
				}
			)
		})
	}

	#[test]
	fn is_evm_works() {
		Identity::iter().for_each(|identity| {
			assert_eq!(
				identity.is_evm(),
				match identity {
					Identity::Twitter(..) => false,
					Identity::Discord(..) => false,
					Identity::Github(..) => false,
					Identity::Substrate(..) => false,
					Identity::Evm(..) => true,
				}
			)
		})
	}

	#[test]
	fn matches_web3networks_works() {
		// web2 identity
		let mut id = Identity::Twitter("alice".as_bytes().to_vec().try_into().unwrap());
		let mut networks: Vec<Web3Network> = vec![];
		assert!(id.matches_web3networks(&networks));
		networks = vec![Web3Network::Litentry];
		assert!(!id.matches_web3networks(&networks));

		// substrate identity
		id = Identity::Substrate(Default::default());
		networks = vec![];
		assert!(!id.matches_web3networks(&networks));
		networks = vec![Web3Network::BSC, Web3Network::Litentry];
		assert!(!id.matches_web3networks(&networks));
		networks = vec![Web3Network::Litentry, Web3Network::Kusama];
		assert!(id.matches_web3networks(&networks));

		// evm identity
		id = Identity::Evm(Default::default());
		networks = vec![];
		assert!(!id.matches_web3networks(&networks));
		networks = vec![Web3Network::BSC, Web3Network::Litentry];
		assert!(!id.matches_web3networks(&networks));
		networks = vec![Web3Network::BSC, Web3Network::Ethereum];
		assert!(id.matches_web3networks(&networks));
	}
}
