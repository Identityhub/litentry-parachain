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

use serde::{Deserialize, Serialize};
use sp_core::hexdisplay::HexDisplay;

use codec::{Decode, Encode, MaxEncodedLen};
use scale_info::TypeInfo;
use sp_runtime::{traits::ConstU32, BoundedVec};

pub type MaxStringLength = ConstU32<64>;
pub type IdentityString = BoundedVec<u8, MaxStringLength>;

#[derive(Encode, Decode, Copy, Clone, Debug, PartialEq, Eq, TypeInfo, MaxEncodedLen)]
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

#[derive(Encode, Decode, Copy, Clone, Debug, PartialEq, Eq, TypeInfo, MaxEncodedLen)]
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

#[derive(Encode, Decode, Copy, Clone, Debug, PartialEq, Eq, TypeInfo, MaxEncodedLen)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum SubstrateNetwork {
	Polkadot,
	Kusama,
	Litentry,
	Litmus,
}

#[derive(Encode, Decode, Copy, Clone, Debug, PartialEq, Eq, TypeInfo, MaxEncodedLen)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum EvmNetwork {
	Ethereum,
	BSC,
}

#[derive(Encode, Decode, Copy, Clone, Debug, PartialEq, Eq, TypeInfo, MaxEncodedLen)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum Web2Network {
	Twitter,
	Discord,
	Github,
}

#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq, TypeInfo, MaxEncodedLen)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct SubstrateIdentity {
	pub network: SubstrateNetwork,
	pub address: Address32,
}

#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq, TypeInfo, MaxEncodedLen)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct EvmIdentity {
	pub network: EvmNetwork,
	pub address: Address20,
}

#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq, TypeInfo, MaxEncodedLen)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct Web2Identity {
	pub network: Web2Network,
	pub address: IdentityString,
}

#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq, TypeInfo, MaxEncodedLen)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum Identity {
	Substrate(SubstrateIdentity),
	Evm(EvmIdentity),
	Web2(Web2Identity),
}

impl From<SubstrateIdentity> for Identity {
	fn from(id: SubstrateIdentity) -> Self {
		Self::Substrate(id)
	}
}

impl From<EvmIdentity> for Identity {
	fn from(id: EvmIdentity) -> Self {
		Self::Evm(id)
	}
}

impl From<Web2Identity> for Identity {
	fn from(id: Web2Identity) -> Self {
		Self::Web2(id)
	}
}

impl Identity {
	#[cfg(any(feature = "std", feature = "sgx"))]
	pub fn flat(&self) -> Vec<u8> {
		match &self {
			Identity::Substrate(identity) => {
				let mut data = format!("did:{:?}:web3:substrate:", identity.network)
					.to_ascii_lowercase()
					.as_bytes()
					.to_vec();
				let mut suffix = format!("0x{}", HexDisplay::from(identity.address.as_ref()))
					.as_bytes()
					.to_vec();
				data.append(&mut suffix);
				data
			},
			Identity::Evm(identity) => {
				let mut data = format!("did:{:?}:web3:evm:", identity.network)
					.to_ascii_lowercase()
					.as_bytes()
					.to_vec();
				let mut suffix = format!("0x{}", HexDisplay::from(identity.address.as_ref()))
					.as_bytes()
					.to_vec();
				data.append(&mut suffix);
				data
			},
			Identity::Web2(identity) => {
				let mut data = format!("did:{:?}:web2:_:", identity.network)
					.to_ascii_lowercase()
					.as_bytes()
					.to_vec();
				let mut suffix = identity.address.to_vec();
				data.append(&mut suffix);
				data
			},
		}
	}

	pub fn is_web2(&self) -> bool {
		matches!(self, Identity::Web2(_))
	}

	pub fn is_web3(&self) -> bool {
		matches!(self, Identity::Evm(_) | Identity::Substrate(_))
	}
}

#[cfg(test)]
mod tests {
	use crate::{
		Identity, IdentityString, SubstrateIdentity, SubstrateNetwork, Web2Identity, Web2Network,
	};
	use sp_core::Pair;

	#[test]
	fn identity() {
		let sub_pair = sp_core::sr25519::Pair::from_string("//Alice", None).unwrap();
		// let eth_pair = sp_core::ed25519::Pair::from_string("//Alice", None).unwrap();
		let polkadot_identity: Identity = SubstrateIdentity {
			network: SubstrateNetwork::Polkadot,
			address: sub_pair.public().0.into(),
		}
		.into();

		let twitter_identity: Identity = Web2Identity {
			network: Web2Network::Twitter,
			address: IdentityString::try_from("litentry".as_bytes().to_vec()).unwrap(),
		}
		.into();

		assert_eq!(
			"did:polkadot:web3:substrate:0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d",
			String::from_utf8(polkadot_identity.flat()).unwrap()
		);
		assert_eq!(
			"did:twitter:web2:_:litentry",
			String::from_utf8(twitter_identity.flat()).unwrap()
		);
	}
}
