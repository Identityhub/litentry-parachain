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
// along with Litentry.  If not, see <https://www.gnu.org/licenses/>.

use super::IMP;
use crate::{
	command_utils::{get_chain_api, *},
	Cli, CliResult, CliResultOk,
};
use base58::FromBase58;
use codec::{Decode, Encode};
use ita_stf::{helpers::get_expected_raw_message, Web3Network};
use itp_sgx_crypto::ShieldingCryptoEncrypt;
use itp_stf_primitives::types::ShardIdentifier;
use litentry_primitives::{Identity, LitentryMultiSignature, Web3CommonValidationData};
use log::*;
use sp_application_crypto::Pair;
use sp_core::sr25519 as sr25519_core;
use substrate_api_client::{ac_compose_macros::compose_extrinsic, SubmitAndWatch, XtStatus};

#[derive(Parser)]
pub struct LinkIdentityCommand {
	/// AccountId in ss58check format
	account: String,
	/// Identity to be created, in did form
	did: String,
	/// Shard identifier
	shard: String,
	/// Delegate signer for the account
	delegate: Option<String>,
}

impl LinkIdentityCommand {
	pub(crate) fn run(&self, cli: &Cli) -> CliResult {
		let mut chain_api = get_chain_api(cli);

		let shard_opt = match self.shard.from_base58() {
			Ok(s) => ShardIdentifier::decode(&mut &s[..]),
			_ => panic!("shard argument must be base58 encoded"),
		};

		let shard = match shard_opt {
			Ok(shard) => shard,
			Err(e) => panic!("{}", e),
		};

		let signer: sr25519_core::Pair = if let Some(account) = &self.delegate {
			get_pair_from_str(&account).into()
		} else {
			// Normal who which we use
			get_pair_from_str(&self.account).into()
		};
		chain_api.set_signer(signer.clone().into());

		let who: sr25519_core::Pair = get_pair_from_str(&self.account).into();

		let identity = Identity::from_did(self.did.as_str()).unwrap();
		let tee_shielding_key = get_shielding_key(cli).unwrap();
		let encrypted_identity = tee_shielding_key.encrypt(&identity.encode()).unwrap();
		let who_identity = Identity::from(who.public());
		let vdata = get_expected_raw_message(&who_identity, &identity, 1);
		let validation_payload = vdata.clone();
		let web3network = vec![Web3Network::Litentry];
		let encrypted_web3network = tee_shielding_key.encrypt(&web3network.encode()).unwrap();

		let signature: LitentryMultiSignature = who.sign(&validation_payload).into();
		let web3common = Web3CommonValidationData {
			message: validation_payload.clone().try_into().unwrap(),
			signature: signature.into(),
		};
		let validation_data = litentry_primitives::ValidationData::Web3(
			litentry_primitives::Web3ValidationData::Substrate(web3common),
		);
		let encrypted_validation_data =
			tee_shielding_key.encrypt(&validation_data.encode()).unwrap();

		let xt = compose_extrinsic!(
			chain_api,
			IMP,
			"link_identity",
			shard,
			who.public().0,
			encrypted_identity.to_vec(),
			encrypted_validation_data,
			encrypted_web3network
		);
		println!("Sending request");
		let tx_hash = chain_api.submit_and_watch_extrinsic_until(xt, XtStatus::Finalized).unwrap();
		println!("[+] LinkIdentityCommand TrustedOperation got finalized. Hash: {:?}\n", tx_hash);

		Ok(CliResultOk::None)
	}
}
