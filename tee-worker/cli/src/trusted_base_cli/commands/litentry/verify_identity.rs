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
	get_layer_two_nonce,
	trusted_cli::TrustedCli,
	trusted_command_utils::{get_accountid_from_str, get_identifiers, get_pair_from_str},
	trusted_operation::perform_trusted_operation,
	Cli, CliError, CliResult, CliResultOk,
};
use codec::Decode;
use ita_stf::{Index, TrustedCall, TrustedGetter, TrustedOperation};
use itp_stf_primitives::types::KeyPair;
use log::*;
use sp_core::Pair;

#[derive(Parser)]
pub struct VerifyIdentityCommand {
	/// AccountId in ss58check format
	account: String,
	did: String,
	validation_data: String,
	parent_block_number: u32,
}

// TODO: we'd need an "integration-test" with parentchain "verify_identity"
//       the origin of it needs to be re-considered if we want individual steps
impl VerifyIdentityCommand {
	pub(crate) fn run(&self, cli: &Cli, trusted_cli: &TrustedCli) -> CliResult {
		let who = get_accountid_from_str(&self.account);
		let root = get_pair_from_str(trusted_cli, "//Alice");

		let (mrenclave, shard) = get_identifiers(trusted_cli);
		let nonce = get_layer_two_nonce!(root, cli, trusted_cli);
		// compose the extrinsic
		let validation_data = serde_json::from_str(self.validation_data.as_str());
		if let Err(e) = validation_data {
			warn!("Deserialize ValidationData error: {:?}", e.to_string());
			return Err(CliError::TrustedOp {
				msg: format!("Deserialize ValidationData error: {:?}", e.to_string()),
			})
		}
		let identity = serde_json::from_str(self.did.as_str());
		if let Err(e) = identity {
			warn!("Deserialize Identity error: {:?}", e.to_string());
			return Err(CliError::TrustedOp {
				msg: format!("Deserialize Identity error: {:?}", e.to_string()),
			})
		}
		let top: TrustedOperation = TrustedCall::verify_identity(
			root.public().into(),
			who,
			identity.unwrap(),
			validation_data.unwrap(),
			self.parent_block_number,
			Default::default(),
		)
		.sign(&KeyPair::Sr25519(Box::new(root)), nonce, &mrenclave, &shard)
		.into_trusted_operation(trusted_cli.direct);
		Ok(perform_trusted_operation(cli, trusted_cli, &top).map(|_| CliResultOk::None)?)
	}
}
