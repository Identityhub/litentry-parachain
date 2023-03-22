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

use crate::{
	helpers::{enclave_signer_account, ensure_enclave_signer_account, generate_challenge_code},
	is_root, AccountId, Encode, IdentityManagement, MetadataOf, Runtime, StfError, StfResult,
	TrustedCall, TrustedCallSigned,
};
use frame_support::{dispatch::UnfilteredDispatchable, ensure};
use itp_stf_primitives::types::ShardIdentifier;
use itp_utils::stringify::account_id_to_string;
use lc_stf_task_sender::{
	stf_task_sender::{SendStfRequest, StfRequestSender},
	AssertionBuildRequest, MaxIdentityLength, RequestType, SetUserShieldingKeyRequest,
	Web2IdentityVerificationRequest, Web3IdentityVerificationRequest,
};
use litentry_primitives::{
	Assertion, ChallengeCode, Identity, ParentchainBlockNumber, UserShieldingKeyType,
	ValidationData,
};
use log::*;
use sp_runtime::BoundedVec;
use std::{format, string::ToString, vec, vec::Vec};

impl TrustedCallSigned {
	pub fn set_user_shielding_key_preflight(
		root: AccountId,
		shard: &ShardIdentifier,
		who: AccountId,
		key: UserShieldingKeyType,
	) -> StfResult<()> {
		debug!(
			"set user shielding key preflight, who = {:?}, key = {:?}",
			account_id_to_string(&who),
			key.clone()
		);
		ensure!(is_root::<Runtime, AccountId>(&root), StfError::MissingPrivileges(root));
		let encoded_callback =
			TrustedCall::set_user_shielding_key_runtime(enclave_signer_account(), who.clone(), key)
				.encode();
		let encoded_shard = shard.encode();
		let request = SetUserShieldingKeyRequest { encoded_shard, who, encoded_callback }.into();
		let sender = StfRequestSender::new();
		sender.send_stf_request(request).map_err(|_| StfError::VerifyIdentityFailed)
	}

	pub fn set_user_shielding_key_runtime(
		enclave_account: AccountId,
		who: AccountId,
		key: UserShieldingKeyType,
	) -> StfResult<()> {
		debug!("set user shielding key runtime, who = {:?}", account_id_to_string(&who));
		ensure_enclave_signer_account(&enclave_account)?;
		ita_sgx_runtime::IdentityManagementCall::<Runtime>::set_user_shielding_key { who, key }
			.dispatch_bypass_filter(ita_sgx_runtime::RuntimeOrigin::root())
			.map_err(|e| StfError::Dispatch(format!("{:?}", e.error)))?;
		Ok(())
	}

	pub fn create_identity_runtime(
		enclave_account: AccountId,
		who: AccountId,
		identity: Identity,
		metadata: Option<MetadataOf<Runtime>>,
		bn: ParentchainBlockNumber,
		parent_ss58_prefix: u16,
	) -> StfResult<ChallengeCode> {
		debug!(
			"create identity runtime, who = {:?}, identity = {:?}, metadata = {:?}, bn = {:?}, parent_ss58_prefix = {}",
			account_id_to_string(&who),
			identity,
			metadata,
			bn,
			parent_ss58_prefix,
		);
		ensure_enclave_signer_account(&enclave_account)?;
		ita_sgx_runtime::IdentityManagementCall::<Runtime>::create_identity {
			who: who.clone(),
			identity: identity.clone(),
			metadata,
			creation_request_block: bn,
			parent_ss58_prefix,
		}
		.dispatch_bypass_filter(ita_sgx_runtime::RuntimeOrigin::root())
		.map_err(|e| StfError::Dispatch(format!("{:?}", e.error)))?;

		// generate challenge code
		let code = generate_challenge_code();
		debug!("challenge code generated, who = {:?}", account_id_to_string(&who));

		ita_sgx_runtime::IdentityManagementCall::<Runtime>::set_challenge_code {
			who,
			identity,
			code,
		}
		.dispatch_bypass_filter(ita_sgx_runtime::RuntimeOrigin::root())
		.map_err(|e| StfError::Dispatch(format!("{:?}", e.error)))?;

		Ok(code)
	}

	pub fn remove_identity_runtime(
		enclave_account: AccountId,
		who: AccountId,
		identity: Identity,
	) -> StfResult<()> {
		debug!(
			"remove identity runtime, who = {:?}, identity = {:?}",
			account_id_to_string(&who),
			identity,
		);
		ensure_enclave_signer_account(&enclave_account)?;
		ita_sgx_runtime::IdentityManagementCall::<Runtime>::remove_identity { who, identity }
			.dispatch_bypass_filter(ita_sgx_runtime::RuntimeOrigin::root())
			.map_err(|e| StfError::Dispatch(format!("{:?}", e.error)))?;
		Ok(())
	}

	pub fn verify_identity_preflight(
		enclave_account: AccountId,
		shard: &ShardIdentifier,
		who: AccountId,
		identity: Identity,
		validation_data: ValidationData,
		bn: ParentchainBlockNumber,
	) -> StfResult<()> {
		debug!("verify identity preflight, who:{:?}, identity:{:?}", who, identity);

		ensure_enclave_signer_account(&enclave_account)?;
		let code = IdentityManagement::challenge_codes(&who, &identity)
			.ok_or_else(|| StfError::Dispatch("code not found".to_string()))?;

		let encoded_callback = TrustedCall::verify_identity_runtime(
			enclave_signer_account(),
			who.clone(),
			identity.clone(),
			bn,
		)
		.encode();
		let encoded_shard = shard.encode();
		let request: RequestType = match validation_data {
			ValidationData::Web2(web2) => Web2IdentityVerificationRequest {
				encoded_shard,
				who,
				identity,
				challenge_code: code,
				validation_data: web2,
				bn,
				encoded_callback,
			}
			.into(),
			ValidationData::Web3(web3) => Web3IdentityVerificationRequest {
				encoded_shard,
				who,
				identity,
				challenge_code: code,
				validation_data: web3,
				bn,
				encoded_callback,
			}
			.into(),
		};

		let sender = StfRequestSender::new();
		sender.send_stf_request(request).map_err(|_| StfError::VerifyIdentityFailed)
	}

	pub fn verify_identity_runtime(
		enclave_account: AccountId,
		who: AccountId,
		identity: Identity,
		bn: ParentchainBlockNumber,
	) -> StfResult<()> {
		debug!(
			"verify identity runtime, who = {:?}, identity = {:?}, bn = {:?}",
			account_id_to_string(&who),
			identity,
			bn
		);
		ensure_enclave_signer_account(&enclave_account)?;
		ita_sgx_runtime::IdentityManagementCall::<Runtime>::verify_identity {
			who: who.clone(),
			identity: identity.clone(),
			verification_request_block: bn,
		}
		.dispatch_bypass_filter(ita_sgx_runtime::RuntimeOrigin::root())
		.map_err(|e| StfError::Dispatch(format!("{:?}", e.error)))?;

		// remove challenge code
		ita_sgx_runtime::IdentityManagementCall::<Runtime>::remove_challenge_code { who, identity }
			.dispatch_bypass_filter(ita_sgx_runtime::RuntimeOrigin::root())
			.map_err(|e| StfError::Dispatch(format!("{:?}", e.error)))?;

		Ok(())
	}

	pub fn send_blockchain_vc_runtime(
		enclave_account: AccountId,
		vc_info: Vec<u8>,
		chain_id: u64,
	) -> StfResult<()> {
		debug!("send blockchain vc runtime, vc_info = {:?}, chain_id = {:?}", vc_info, chain_id);
		ensure_enclave_signer_account(&enclave_account)?;
		ita_sgx_runtime::EthereumSenderDummyCall::<Runtime>::send_blockchain_vc {
			vc_info,
			chain_id,
		}
		.dispatch_bypass_filter(ita_sgx_runtime::RuntimeOrigin::root())
		.map_err(|e| StfError::Dispatch(format!("{:?}", e.error)))?;

		Ok(())
	}

	pub fn build_assertion(
		enclave_account: AccountId,
		shard: &ShardIdentifier,
		who: AccountId,
		assertion: Assertion,
		bn: ParentchainBlockNumber,
	) -> StfResult<()> {
		debug!("build assertion, who {:?}, assertion {:?}", account_id_to_string(&who), assertion);
		ensure_enclave_signer_account(&enclave_account)?;
		let id_graph = ita_sgx_runtime::pallet_imt::Pallet::<Runtime>::get_id_graph(&who);
		let mut vec_identity: BoundedVec<Identity, MaxIdentityLength> = vec![].try_into().unwrap();
		for id in &id_graph {
			if id.1.is_verified {
				vec_identity.try_push(id.0.clone()).map_err(|_| StfError::AssertionBuildFail)?;
			}
		}

		if let Some(key) = IdentityManagement::user_shielding_keys(&who) {
			let request: RequestType =
				AssertionBuildRequest { shard: *shard, who, assertion, vec_identity, bn, key }
					.into();
			let sender = StfRequestSender::new();
			sender.send_stf_request(request).map_err(|_| StfError::AssertionBuildFail)
		} else {
			error!(
				"user shielding key is missing, {:?}, {:?}",
				account_id_to_string(&who),
				assertion
			);
			Err(StfError::AssertionBuildFail)
		}
	}

	pub fn set_challenge_code_runtime(
		enclave_account: AccountId,
		who: AccountId,
		identity: Identity,
		code: ChallengeCode,
	) -> StfResult<()> {
		debug!("set challenge code runtime, who: {:?}", account_id_to_string(&who));
		ensure_enclave_signer_account(&enclave_account)?;
		ita_sgx_runtime::IdentityManagementCall::<Runtime>::set_challenge_code {
			who,
			identity,
			code,
		}
		.dispatch_bypass_filter(ita_sgx_runtime::RuntimeOrigin::root())
		.map_err(|e| StfError::Dispatch(format!("{:?}", e.error)))?;
		Ok(())
	}
}
