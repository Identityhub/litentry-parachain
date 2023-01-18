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
//
// TEE Implementation of Verifiable Credentials Data Model v2.0
// W3C Editor's Draft 07 January 2023
// https://w3c.github.io/vc-data-model

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(all(not(feature = "std"), feature = "sgx"))]
#[macro_use]
extern crate sgx_tstd as std;

// re-export module to properly feature gate sgx and regular std environment
#[cfg(all(not(feature = "std"), feature = "sgx"))]
pub mod sgx_reexport_prelude {
	pub use chrono_sgx as chrono;
	pub use serde_json_sgx as serde_json;
	pub use thiserror_sgx as thiserror;
}

#[cfg(all(feature = "std", feature = "sgx"))]
compile_error!("feature \"std\" and feature \"sgx\" cannot be enabled at the same time");

use codec::{Decode, Encode};
use itp_types::AccountId;
use itp_utils::stringify::account_id_to_string;
use litentry_primitives::{Assertion, ParentchainBlockNumber};
use log::*;
use scale_info::TypeInfo;
use serde::{Deserialize, Serialize};
use std::{
	fmt::Debug,
	string::{String, ToString},
	vec::Vec,
};
pub mod error;
pub use error::Error;
pub mod schema;

pub const PROOF_PURPOSE: &str = "assertionMethod";
pub const MAX_CREDENTIAL_SIZE: usize = 2048;

/// Ed25519 Signature 2018, W3C, 23 July 2021, https://w3c-ccg.github.io/lds-ed25519-2018
/// May be registered in Linked Data Cryptographic Suite Registry, W3C, 29 December 2020
/// https://w3c-ccg.github.io/ld-cryptosuite-registry
#[derive(Serialize, Deserialize, Encode, Decode, Clone, Debug, PartialEq, Eq, TypeInfo)]
pub enum ProofType {
	Ed25519Signature2020,
}

#[derive(Serialize, Deserialize, Encode, Decode, Clone, Debug, PartialEq, Eq, TypeInfo)]
pub enum CredentialType {
	VerifiableCredential,
}

#[derive(Serialize, Deserialize, Encode, Decode, Clone, Debug, PartialEq, Eq, TypeInfo)]
#[serde(rename_all = "camelCase")]
pub struct DataSource {
	/// ID of the data provider
	pub data_provider_id: u32,
	/// Endpoint of the data provider
	pub data_provider: String,
}

#[derive(Serialize, Deserialize, Encode, Decode, Clone, Debug, PartialEq, Eq, TypeInfo)]
#[serde(rename_all = "camelCase")]
pub struct Issuer {
	/// ID of the TEE Worker
	pub id: String,
	pub name: String,
	/// Value of MRENCLAVE
	pub mrenclave: String,
}

impl Issuer {
	pub fn is_empty(&self) -> bool {
		self.mrenclave.is_empty()
	}

	pub fn new(id: String, name: String, mrenclave: String) -> Self {
		Self { id, name, mrenclave }
	}

	pub fn set_mrenclave(&mut self, value: String) {
		self.mrenclave = value;
	}
}

#[derive(Serialize, Deserialize, Encode, Decode, Clone, Debug, PartialEq, Eq, TypeInfo)]
#[serde(rename_all = "camelCase")]
pub struct CredentialSubject {
	/// Identifier for the only entity that the credential was issued
	pub id: String,
	pub description: String,
	#[serde(rename = "type")]
	pub types: String,
	/// (Optional) Some externally provided identifiers
	pub tag: Vec<String>,
	/// (Optional) Data source definitions for trusted data providers
	#[serde(skip_serializing_if = "Option::is_none")]
	pub data_source: Option<Vec<DataSource>>,
	/// Several sets of assertions.
	/// Each assertion contains multiple steps to describe how to fetch data and calculate the value
	pub assertions: String,
	/// Results of each set of assertions
	pub values: Vec<bool>,
	/// The extrinsic on Parentchain for credential verification purpose
	pub endpoint: String,
}

impl CredentialSubject {
	pub fn is_empty(&self) -> bool {
		self.id.is_empty()
	}

	pub fn set_value(&mut self, value: bool) {
		self.values[0] = value;
	}
}

/// Verifiable Credentials JSON Schema 2022, W3C, 8 November 2022
/// https://w3c-ccg.github.io/vc-json-schemas/
#[derive(Serialize, Deserialize, Encode, Decode, Clone, Debug, PartialEq, Eq, TypeInfo)]
#[serde(rename_all = "camelCase")]
pub struct CredentialSchema {
	/// Schema ID that is maintained by Parentchain VCMP
	pub id: String,
	/// The schema type, generally it is
	#[serde(rename = "type")]
	pub types: String,
}

#[derive(Serialize, Deserialize, Encode, Decode, Clone, Debug, PartialEq, Eq, TypeInfo)]
#[serde(rename_all = "camelCase")]
pub struct Proof {
	/// The block number when the signature was created
	pub created_block_number: ParentchainBlockNumber,
	/// The cryptographic signature suite that used to generate signature
	#[serde(rename = "type")]
	pub proof_type: ProofType,
	/// Purpose of this proof, generally it is expected as a fixed value, such as 'assertionMethod'
	pub proof_purpose: String,
	/// The digital signature value
	pub proof_value: String,
	/// The public key from Issuer
	pub verification_method: String,
}

impl Proof {
	pub fn new(type_: ProofType, bn: ParentchainBlockNumber) -> Self {
		Self {
			created_block_number: bn,
			proof_type: type_,
			proof_purpose: PROOF_PURPOSE.to_string(),
			proof_value: "".to_string(),
			verification_method: "".to_string(),
		}
	}

	pub fn is_empty(&self) -> bool {
		self.proof_value.is_empty()
	}
}

#[derive(Serialize, Deserialize, Encode, Decode, Clone, Debug, PartialEq, Eq, TypeInfo)]
#[serde(rename_all = "camelCase")]
pub struct Credential {
	/// Contexts defines the structure and data types of the credential
	#[serde(rename = "@context")]
	pub context: Vec<String>,
	/// The specific UUID of the credential, it is used for onchain verification
	pub id: String,
	/// Uniquely identifier of the type of the credential
	#[serde(rename = "type")]
	pub types: Vec<CredentialType>,
	/// Assertions claimed about the subjects of the credential
	pub credential_subject: CredentialSubject,
	/// The TEE enclave who issued the credential
	pub issuer: Issuer,
	pub issuance_block_number: ParentchainBlockNumber,
	/// (Optional)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub expiration_block_number: Option<ParentchainBlockNumber>,
	/// Digital proof with the signature of Issuer
	pub proof: Proof,
	#[serde(skip_deserializing)]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub credential_schema: Option<CredentialSchema>,
}

impl Credential {
	pub fn from_template(
		s: &str,
		who: &AccountId,
		bn: ParentchainBlockNumber,
	) -> Result<Self, Error> {
		let mut vc: Self =
			serde_json::from_str(s).map_err(|err| Error::ParseError(format!("{}", err)))?;
		vc.credential_subject.id = account_id_to_string(who);
		vc.issuance_block_number = bn;
		vc.expiration_block_number = None;
		vc.credential_schema = None;
		vc.validate_unsigned()?;
		Ok(vc)
	}

	pub fn to_json(&self) -> Result<String, Error> {
		let json_str =
			serde_json::to_string(&self).map_err(|err| Error::ParseError(format!("{}", err)))?;
		Ok(json_str)
	}

	pub fn validate_unsigned(&self) -> Result<(), Error> {
		if !self.types.contains(&CredentialType::VerifiableCredential) {
			return Err(Error::EmptyCredentialType)
		}

		if self.credential_subject.id.is_empty() {
			return Err(Error::EmptyCredentialSubject)
		}

		if self.issuance_block_number == 0 {
			return Err(Error::EmptyIssuanceBlockNumber)
		}

		Ok(())
	}

	pub fn validate(&self) -> Result<(), Error> {
		self.validate_unsigned()?;

		if self.credential_subject.is_empty() {
			return Err(Error::EmptyCredentialSubject)
		}

		// ToDo: validate issuer
		if self.issuer.is_empty() {
			return Err(Error::EmptyCredentialIssuer)
		}

		if self.proof.is_empty() {
			return Err(Error::EmptyCredentialProof)
		}

		if self.proof.created_block_number == 0 {
			return Err(Error::EmptyProofBlockNumber)
		}

		// the proof bn that is must be equal or after issuance bn
		if self.proof.created_block_number < self.issuance_block_number {
			return Err(Error::InvalidProof)
		}

		//ToDo: validate proof signature

		let exported = self.to_json()?;
		if exported.len() > MAX_CREDENTIAL_SIZE {
			return Err(Error::CredentialIsTooLong)
		}

		Ok(())
	}

	pub fn validate_schema(&self) -> Result<(), Error> {
		//ToDo: fetch schema from Parentchain and check its status
		Ok(())
	}

	pub fn generate_unsigned_credential(
		assertion: &Assertion,
		who: &AccountId,
		bn: ParentchainBlockNumber,
	) -> Result<Credential, Error> {
		debug!("generate unsigned credential {:?}", assertion);
		match assertion {
			Assertion::A1 => {
				let raw = include_str!("templates/vc.json");
				let credential: Credential = Credential::from_template(raw, who, bn)?;
				Ok(credential)
			},
			_ => Err(Error::UnsupportedAssertion),
		}
	}

	pub fn add_assertion_a1(&mut self, web2_cnt: i32, web3_cnt: i32) {
		self.credential_subject.assertions = format!(
			r#"{{"or": [{{"src": "$web2_account_cnt", "op": ">=", "dsc": "{}",}},{{"src": "$web3_account_cnt", "op": ">", "dsc": "{}"}}]}}"#,
			web2_cnt, web3_cnt
		);
	}

	pub fn generate_issuer() -> Result<Issuer, Error> {
		let issuer = Issuer::new("".to_string(), "".to_string(), "".to_string());
		Ok(issuer)
	}

	pub fn validate_proof(&self) -> Result<(), Error> {
		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn eval_simple_success() {
		let who = AccountId::from([0; 32]);
		let data = include_str!("templates/vc.json");

		let vc = Credential::from_template(data, &who, 1u32).unwrap();
		assert!(vc.validate_unsigned().is_ok());
		let id: String = vc.credential_subject.id.clone();
		assert_eq!(id, account_id_to_string(&who));
		assert_eq!(vc.proof.proof_purpose, "assertionMethod");
	}
}
