// Copyright 2020-2024 Trust Computing GmbH.
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
// along with Litentry. If not, see <https://www.gnu.org/licenses/>.

#[cfg(all(feature = "std", feature = "sgx"))]
compile_error!("feature \"std\" and feature \"sgx\" cannot be enabled at the same time");

#[cfg(all(not(feature = "std"), feature = "sgx"))]
extern crate sgx_tstd as std;

use core::result::Result;

use lc_common::web3_nft::NftAddress;
use lc_data_providers::{
	moralis::{
		GetNftsByWalletParam, MoralisChainParam, MoralisClient, NftApiList as MoralisNftApiList,
	},
	nodereal_jsonrpc::{
		GetTokenBalance721Param, NftApiList as NoderealNftApiList, Web3NetworkNoderealJsonrpcClient,
	},
};
use litentry_primitives::ErrorDetail;

use crate::*;

// support ERC721/BEP721 nft token
#[cfg(not(feature = "async"))]
pub fn has_nft_721(
	addresses: Vec<(Web3Network, String)>,
	nft_type: Web3NftType,
	data_provider_config: &DataProviderConfig,
) -> Result<bool, Error> {
	for address in addresses.iter() {
		let network = address.0;
		let token_address = nft_type.get_nft_address(network).unwrap_or_default();

		match network {
			Web3Network::Bsc | Web3Network::Ethereum => {
				if let Some(mut client) =
					network.create_nodereal_jsonrpc_client(data_provider_config)
				{
					let param = GetTokenBalance721Param {
						token_address: token_address.into(),
						account_address: address.1.clone(),
						block_number: "latest".into(),
					};
					match client.get_token_balance_721(&param, false) {
						Ok(balance) =>
							if balance > 0 {
								return Ok(true)
							},
						Err(err) => return Err(err.into_error_detail()),
					}
				}
			},
			_ => {},
		}
	}

	Ok(false)
}

#[cfg(feature = "async")]
pub async fn has_nft_721(
	addresses: Vec<(Web3Network, String)>,
	nft_type: Web3NftType,
	data_provider_config: &DataProviderConfig,
) -> Result<bool, Error> {
	for address in addresses.iter() {
		let network = address.0;
		let token_address = nft_type.get_nft_address(network).unwrap_or_default();

		match network {
			Web3Network::Bsc | Web3Network::Ethereum => {
				if let Some(mut client) =
					network.create_nodereal_jsonrpc_client(data_provider_config)
				{
					let param = GetTokenBalance721Param {
						token_address: token_address.into(),
						account_address: address.1.clone(),
						block_number: "latest".into(),
					};
					match client.get_token_balance_721(&param, false).await {
						Ok(balance) =>
							if balance > 0 {
								return Ok(true)
							},
						Err(err) => return Err(err.into_error_detail()),
					}
				}
			},
			_ => {},
		}
	}

	Ok(false)
}

// support ERC1155/BEP1155 nft token
#[cfg(not(feature = "async"))]
pub fn has_nft_1155(
	addresses: Vec<(Web3Network, String)>,
	nft_type: Web3NftType,
	data_provider_config: &DataProviderConfig,
) -> Result<bool, Error> {
	let mut client = MoralisClient::new(data_provider_config);
	for address in addresses.iter() {
		let network = address.0;
		let token_address = nft_type.get_nft_address(network).unwrap_or_default();

		match network {
			Web3Network::Bsc
			| Web3Network::Ethereum
			| Web3Network::Polygon
			| Web3Network::Arbitrum => {
				let mut cursor: Option<String> = None;
				'inner: loop {
					let param = GetNftsByWalletParam {
						address: address.1.clone(),
						chain: MoralisChainParam::new(&network),
						token_addresses: Some(vec![token_address.into()]),
						limit: None,
						cursor,
					};
					match client.get_nfts_by_wallet(&param, false) {
						Ok(resp) => {
							cursor = resp.cursor;
							for item in &resp.result {
								match item.amount.parse::<u32>() {
									Ok(balance) =>
										if balance > 0 {
											return Ok(true)
										},
									Err(_) => return Err(ErrorDetail::ParseError),
								}
							}
						},
						Err(err) => return Err(err.into_error_detail()),
					}
					if cursor.is_none() {
						break 'inner
					}
				}
			},
			_ => {},
		}
	}

	Ok(false)
}

#[cfg(feature = "async")]
pub async fn has_nft_1155(
	addresses: Vec<(Web3Network, String)>,
	nft_type: Web3NftType,
	data_provider_config: &DataProviderConfig,
) -> Result<bool, Error> {
	let mut client = MoralisClient::new(data_provider_config);
	for address in addresses.iter() {
		let network = address.0;
		let token_address = nft_type.get_nft_address(network).unwrap_or_default();

		match network {
			Web3Network::Bsc
			| Web3Network::Ethereum
			| Web3Network::Polygon
			| Web3Network::Arbitrum => {
				let mut cursor: Option<String> = None;
				'inner: loop {
					let param = GetNftsByWalletParam {
						address: address.1.clone(),
						chain: MoralisChainParam::new(&network),
						token_addresses: Some(vec![token_address.into()]),
						limit: None,
						cursor,
					};
					match client.get_nfts_by_wallet(&param, false).await {
						Ok(resp) => {
							cursor = resp.cursor;
							for item in &resp.result {
								match item.amount.parse::<u32>() {
									Ok(balance) =>
										if balance > 0 {
											return Ok(true)
										},
									Err(_) => return Err(ErrorDetail::ParseError),
								}
							}
						},
						Err(err) => return Err(err.into_error_detail()),
					}
					if cursor.is_none() {
						break 'inner
					}
				}
			},
			_ => {},
		}
	}

	Ok(false)
}
