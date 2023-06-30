/*
	Copyright 2021 Integritee AG and Supercomputing Systems AG

	Licensed under the Apache License, Version 2.0 (the "License");
	you may not use this file except in compliance with the License.
	You may obtain a copy of the License at

		http://www.apache.org/licenses/LICENSE-2.0

	Unless required by applicable law or agreed to in writing, software
	distributed under the License is distributed on an "AS IS" BASIS,
	WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
	See the License for the specific language governing permissions and
	limitations under the License.

*/

use crate::{
	command_utils::get_worker_api_direct,
	trusted_cli::TrustedCli,
	trusted_command_utils::{get_accountid_from_str, get_identifiers, get_pair_from_str},
	trusted_operation::perform_trusted_operation,
	Cli,
};
use codec::Decode;
use ita_stf::{TrustedCall, TrustedOperation};
use itc_rpc_client::direct_client::DirectApi;
use itp_rpc::{RpcResponse, RpcReturnValue};
use itp_stf_primitives::types::KeyPair;
use itp_types::DirectRequestStatus;
use itp_utils::FromHexPrefixed;
use litentry_primitives::ParentchainBalance as Balance;
use log::*;
use sp_core::{crypto::Ss58Codec, Pair};
use std::boxed::Box;

#[derive(Parser)]
pub struct UnshieldFundsCommand {
	/// Sender's incognito AccountId in ss58check format
	from: String,

	/// Recipient's parentchain AccountId in ss58check format
	to: String,

	/// amount to be transferred
	amount: Balance,
}

impl UnshieldFundsCommand {
	pub(crate) fn run(&self, cli: &Cli, trusted_args: &TrustedCli) {
		let from = get_pair_from_str(trusted_args, &self.from);
		let to = get_accountid_from_str(&self.to);
		let toclone = to.clone();
		println!("from ss58 is {}", from.public().to_ss58check());
		println!("to   ss58 is {}", to.to_ss58check());

		println!(
			"send trusted call unshield_funds from {} to {}: {}",
			from.public(),
			to,
			self.amount
		);

		let (mrenclave, shard) = get_identifiers(trusted_args);
		let worker_api_direct = get_worker_api_direct(cli);
		let nonce_ret = worker_api_direct.get_next_nonce(shard, toclone);
		info!("nonce_ret {:?} ", nonce_ret);
		let nonce_val = nonce_ret.unwrap();
		info!("nonce_val {:?} ", nonce_val);
		let rpc_response: RpcResponse = serde_json::from_str(&nonce_val).unwrap();
		let rpc_return_value = RpcReturnValue::from_hex(&rpc_response.result).unwrap();
		if rpc_return_value.status == DirectRequestStatus::Error {
			println!("[Error] {}", String::decode(&mut rpc_return_value.value.as_slice()).unwrap());
			worker_api_direct.close().unwrap();
			return
		}

		worker_api_direct.close().unwrap();
		let nonce: u32 = Decode::decode(&mut rpc_return_value.value.as_slice()).unwrap_or_default();

		let top: TrustedOperation =
			TrustedCall::balance_unshield(from.public().into(), to, self.amount, shard)
				.sign(&KeyPair::Sr25519(Box::new(from)), nonce, &mrenclave, &shard)
				.into_trusted_operation(trusted_args.direct);
		let _ = perform_trusted_operation(cli, trusted_args, &top);
	}
}
