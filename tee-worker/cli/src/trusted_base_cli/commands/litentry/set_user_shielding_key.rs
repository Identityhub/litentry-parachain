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

use crate::{
	command_utils::{get_accountid_from_str, get_worker_api_direct},
	trusted_cli::TrustedCli,
	trusted_command_utils::{get_identifiers, get_pair_from_str},
	trusted_operation::perform_trusted_operation,
	Cli,
};
use ita_stf::{TrustedCall, TrustedOperation};
use itc_rpc_client::direct_client::DirectApi;
use itp_stf_primitives::types::KeyPair;
use litentry_primitives::UserShieldingKeyType;
use log::*;
use sp_core::Pair;

#[derive(Parser)]
pub struct SetUserShieldingKeyCommand {
	/// AccountId in ss58check format
	account: String,
	/// Shielding key in hex string
	key_hex: String,
}

impl SetUserShieldingKeyCommand {
	pub(crate) fn run(&self, cli: &Cli, trusted_cli: &TrustedCli) {
		info!("SetUserShieldingKeyCommand account: {}", self.account);
		let who = get_pair_from_str(trusted_cli, &self.account);

		let (mrenclave, shard) = get_identifiers(trusted_cli);
		let worker_api_direct = get_worker_api_direct(cli);
		let nonce = worker_api_direct
			.get_next_nonce(shard, get_accountid_from_str(&self.account))
			.unwrap()
			.parse::<u32>()
			.unwrap();

		let mut key = UserShieldingKeyType::default();

		hex::decode_to_slice(&self.key_hex, &mut key).expect("decoding shielding_key failed");

		let top: TrustedOperation = TrustedCall::set_user_shielding_key(
			who.public().into(),
			who.public().into(),
			key,
			Default::default(),
		)
		.sign(&KeyPair::Sr25519(Box::new(who)), nonce, &mrenclave, &shard)
		.into_trusted_operation(trusted_cli.direct);
		perform_trusted_operation(cli, trusted_cli, &top);
	}
}
