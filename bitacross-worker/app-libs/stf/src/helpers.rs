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
use crate::ENCLAVE_ACCOUNT_KEY;
use codec::{Decode, Encode};
use frame_support::ensure;
use hex_literal::hex;
use itp_stf_primitives::error::{StfError, StfResult};
use itp_storage::{storage_double_map_key, storage_map_key, storage_value_key, StorageHasher};
use itp_types::Index;
use ba_itp_utils::stringify::account_id_to_string;
use litentry_primitives::{ErrorDetail, Identity, Web3ValidationData};
use log::*;
use sp_core::blake2_256;
use sp_runtime::AccountId32;
use std::prelude::v1::*;

pub const ALICE_ACCOUNTID32: AccountId32 =
	AccountId32::new(hex!["d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d"]);

pub fn get_storage_value<V: Decode>(
	storage_prefix: &'static str,
	storage_key_name: &'static str,
) -> Option<V> {
	let key = storage_value_key(storage_prefix, storage_key_name);
	get_storage_by_key_hash(key)
}

pub fn get_storage_map<K: Encode, V: Decode + Clone>(
	storage_prefix: &'static str,
	storage_key_name: &'static str,
	map_key: &K,
	hasher: &StorageHasher,
) -> Option<V> {
	let key = storage_map_key::<K>(storage_prefix, storage_key_name, map_key, hasher);
	get_storage_by_key_hash(key)
}

pub fn get_storage_double_map<K: Encode, Q: Encode, V: Decode + Clone>(
	storage_prefix: &'static str,
	storage_key_name: &'static str,
	first: &K,
	first_hasher: &StorageHasher,
	second: &Q,
	second_hasher: &StorageHasher,
) -> Option<V> {
	let key = storage_double_map_key::<K, Q>(
		storage_prefix,
		storage_key_name,
		first,
		first_hasher,
		second,
		second_hasher,
	);
	get_storage_by_key_hash(key)
}

/// Get value in storage.
pub fn get_storage_by_key_hash<V: Decode>(key: Vec<u8>) -> Option<V> {
	if let Some(value_encoded) = sp_io::storage::get(&key) {
		if let Ok(value) = Decode::decode(&mut value_encoded.as_slice()) {
			Some(value)
		} else {
			error!("could not decode state for key {:x?}", key);
			None
		}
	} else {
		info!("key not found in state {:x?}", key);
		None
	}
}

/// Get the AccountInfo key where the account is stored.
pub fn account_key_hash<AccountId: Encode>(account: &AccountId) -> Vec<u8> {
	storage_map_key("System", "Account", account, &StorageHasher::Blake2_128Concat)
}

pub fn enclave_signer_account<AccountId: Decode>() -> AccountId {
	get_storage_value("Sudo", ENCLAVE_ACCOUNT_KEY).expect("No enclave account")
}

/// Ensures an account is a registered enclave account.
pub fn ensure_enclave_signer_account<AccountId: Encode + Decode + PartialEq>(
	account: &AccountId,
) -> StfResult<()> {
	let expected_enclave_account: AccountId = enclave_signer_account();
	if &expected_enclave_account == account {
		Ok(())
	} else {
		error!(
			"Expected enclave account {}, but found {}",
			account_id_to_string(&expected_enclave_account),
			account_id_to_string(account)
		);
		Err(StfError::RequireEnclaveSignerAccount)
	}
}

pub fn set_block_number(block_number: u32) {
	sp_io::storage::set(&storage_value_key("System", "Number"), &block_number.encode());
}

pub fn ensure_self<AccountId: PartialEq>(signer: &AccountId, who: &AccountId) -> bool {
	signer == who
}

pub fn ensure_enclave_signer_or_self<AccountId: Encode + Decode + PartialEq>(
	signer: &AccountId,
	who: Option<AccountId>,
) -> bool {
	match who {
		Some(ref who) =>
			signer == &enclave_signer_account::<AccountId>() || ensure_self(signer, who),
		None => false,
	}
}

#[cfg(not(feature = "production"))]
pub fn ensure_alice(signer: &AccountId32) -> bool {
	signer == &ALICE_ACCOUNTID32
}

#[cfg(not(feature = "production"))]
pub fn ensure_enclave_signer_or_alice(signer: &AccountId32) -> bool {
	signer == &enclave_signer_account::<AccountId32>() || ensure_alice(signer)
}

// verification message format:
// ```
// blake2_256(<sidechain nonce> + <primary account> + <identity-to-be-linked>)
// ```
// where <> means SCALE-encoded
// see https://github.com/litentry/litentry-parachain/issues/1739 and P-174
pub fn get_expected_raw_message(
	who: &Identity,
	identity: &Identity,
	sidechain_nonce: Index,
) -> Vec<u8> {
	let mut payload = Vec::new();
	payload.append(&mut sidechain_nonce.encode());
	payload.append(&mut who.encode());
	payload.append(&mut identity.encode());
	blake2_256(payload.as_slice()).to_vec()
}

pub fn verify_web3_identity(
	identity: &Identity,
	raw_msg: &[u8],
	data: &Web3ValidationData,
) -> StfResult<()> {
	ensure!(
		raw_msg == data.message().as_slice(),
		StfError::LinkIdentityFailed(ErrorDetail::UnexpectedMessage)
	);

	ensure!(
		data.signature().verify(raw_msg, identity),
		StfError::LinkIdentityFailed(ErrorDetail::VerifyWeb3SignatureFailed)
	);

	Ok(())
}
