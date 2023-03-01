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

#[cfg(all(feature = "std", feature = "sgx"))]
compile_error!("feature \"std\" and feature \"sgx\" cannot be enabled at the same time");

#[cfg(all(not(feature = "std"), feature = "sgx"))]
extern crate sgx_tstd as std;

use crate::{from_data_provider_error, Result};
use itp_stf_primitives::types::ShardIdentifier;
use itp_types::AccountId;
use lc_credentials::Credential;
use lc_data_providers::graphql::{
	GraphQLClient, VerifiedCredentialsIsHodlerIn, VerifiedCredentialsNetwork,
};
use litentry_primitives::{EvmNetwork, Identity, ParentchainBlockNumber};
use log::*;
use parachain_core_primitives::{Assertion, Balance, VCMPError, ASSERTION_FROM_DATE};
use std::{str::from_utf8, string::ToString, vec, vec::Vec};

const WBTC_TOKEN_ADDRESS: &str = "0x2260fac5e5542a773aa44fbcfedf7c193bc2c599";

// WBTC holder
pub fn build(
	identities: Vec<Identity>,
	min_balance: Balance,
	shard: &ShardIdentifier,
	who: &AccountId,
	bn: ParentchainBlockNumber,
) -> Result<Credential> {
	// WBTC decimals is 8.
	let q_min_balance: f64 = (min_balance / (10 ^ 8)) as f64;

	let mut client = GraphQLClient::new();
	let mut found = false;
	let mut from_date_index = 0_usize;

	for id in identities {
		if found {
			break
		}

		if let Identity::Evm { network, address } = id {
			if matches!(network, EvmNetwork::Ethereum) {
				if let Ok(addr) = from_utf8(address.as_ref()) {
					let addresses = vec![addr.to_string()];

					for (index, from_date) in ASSERTION_FROM_DATE.iter().enumerate() {
						// if found is true, no need to check it continually
						if found {
							from_date_index = index + 1;
							break
						}
						let credentials = VerifiedCredentialsIsHodlerIn::new(
							addresses.clone(),
							from_date.to_string(),
							VerifiedCredentialsNetwork::Ethereum,
							WBTC_TOKEN_ADDRESS.to_string(),
							q_min_balance,
						);

						let is_hodler_out = client
							.check_verified_credentials_is_hodler(credentials)
							.map_err(from_data_provider_error)?;
						for hodler in is_hodler_out.verified_credentials_is_hodler.iter() {
							found = found || hodler.is_hodler;
						}
					}
				};
			}
		}
	}

	let a10 = Assertion::A10(min_balance);
	match Credential::generate_unsigned_credential(&a10, who, &shard.clone(), bn) {
		Ok(mut credential_unsigned) => {
			credential_unsigned.update_holder(from_date_index, min_balance);
			return Ok(credential_unsigned)
		},
		Err(e) => {
			error!("Generate unsigned credential failed {:?}", e);
		},
	}

	Err(VCMPError::Assertion10Failed)
}
