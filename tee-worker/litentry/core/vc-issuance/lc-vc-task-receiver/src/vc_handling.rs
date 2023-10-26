#![allow(clippy::result_large_err)]

use ita_sgx_runtime::Hash;
pub use ita_stf::{aes_encrypt_default, IdentityManagement};
use ita_stf::{hash::Hash as TopHash, TrustedCall, TrustedOperation};
use itp_ocall_api::{EnclaveMetricsOCallApi, EnclaveOnChainOCallApi};
use itp_sgx_crypto::{ShieldingCryptoDecrypt, ShieldingCryptoEncrypt};
use itp_sgx_externalities::SgxExternalitiesTrait;
use itp_stf_executor::traits::StfEnclaveSigning;
use itp_stf_state_handler::handle_state::HandleState;
use itp_top_pool_author::traits::AuthorApi;
use itp_types::{ShardIdentifier, H256};
use lc_data_providers::{DataProviderConfigReader, ReadDataProviderConfig};
use lc_stf_task_receiver::{handler::TaskHandler, StfTaskContext};
use lc_stf_task_sender::AssertionBuildRequest;
use lc_vc_task_sender::{VCRequest, VCResponse};
use litentry_primitives::{
	AmountHoldingTimeType, Assertion, ErrorDetail, ErrorString, Identity, ParameterString,
	VCMPError,
};
use log::*;
use sp_core::hashing::blake2_256;
use std::{
	format,
	sync::{mpsc::Sender, Arc},
	vec::Vec,
};

pub(crate) struct VCRequestHandler<
	K: ShieldingCryptoDecrypt + ShieldingCryptoEncrypt + Clone,
	A: AuthorApi<Hash, Hash>,
	S: StfEnclaveSigning,
	H: HandleState,
	O: EnclaveOnChainOCallApi,
> {
	pub(crate) req: VCRequest,
	pub(crate) context: Arc<StfTaskContext<K, A, S, H, O>>,
}

impl<K, A, S, H, O> VCRequestHandler<K, A, S, H, O>
where
	K: ShieldingCryptoDecrypt + ShieldingCryptoEncrypt + Clone,
	A: AuthorApi<Hash, Hash>,
	S: StfEnclaveSigning,
	H: HandleState,
	H::StateT: SgxExternalitiesTrait,
	O: EnclaveOnChainOCallApi,
{
	pub fn process(self, sender: Sender<(VCResponse, Sender<Vec<u8>>)>) -> Result<(), VCMPError> {
		// create the initial credential
		// TODO: maybe we can further simplify this
		let mut credential = match self.req.assertion.assertion.clone() {
			Assertion::A1 => lc_assertion_build::a1::build(&self.req.assertion),

			Assertion::A2(guild_id) => lc_assertion_build::a2::build(&self.req.assertion, guild_id),

			Assertion::A3(guild_id, channel_id, role_id) =>
				lc_assertion_build::a3::build(&self.req.assertion, guild_id, channel_id, role_id),

			Assertion::A4(min_balance) =>
				build_holding_time(&self.req.assertion, AmountHoldingTimeType::LIT, min_balance),

			Assertion::A6 => lc_assertion_build::a6::build(&self.req.assertion),

			Assertion::A7(min_balance) =>
				build_holding_time(&self.req.assertion, AmountHoldingTimeType::DOT, min_balance),

			// no need to pass `networks` again because it's the same as the `get_supported_web3networks`
			Assertion::A8(_networks) => lc_assertion_build::a8::build(&self.req.assertion),

			Assertion::A10(min_balance) =>
				build_holding_time(&self.req.assertion, AmountHoldingTimeType::WBTC, min_balance),

			Assertion::A11(min_balance) =>
				build_holding_time(&self.req.assertion, AmountHoldingTimeType::ETH, min_balance),

			Assertion::A13(owner) => lc_assertion_build::a13::build(
				&self.req.assertion,
				self.context.ocall_api.clone(),
				&owner,
			),

			Assertion::A14 => lc_assertion_build::a14::build(&self.req.assertion),

			Assertion::Achainable(param) =>
				lc_assertion_build::achainable::build(&self.req.assertion, param),

			Assertion::A20 => lc_assertion_build::a20::build(&self.req.assertion),

			Assertion::Oneblock(course_type) =>
				lc_assertion_build::oneblock::course::build(&self.req.assertion, course_type),
		}?;

		// post-process the credential
		let signer = self.context.enclave_signer.as_ref();
		let enclave_account = signer.get_enclave_account().map_err(|e| {
			VCMPError::RequestVCFailed(
				self.req.assertion.assertion.clone(),
				ErrorDetail::StfError(ErrorString::truncate_from(format!("{e:?}").into())),
			)
		})?;

		let data_provider_config = DataProviderConfigReader::read()
			.map_err(|e| VCMPError::RequestVCFailed(self.req.assertion.assertion.clone(), e))?;
		credential
			.credential_subject
			.set_endpoint(data_provider_config.credential_endpoint);

		credential.issuer.id =
			Identity::Substrate(enclave_account.into()).to_did().map_err(|e| {
				VCMPError::RequestVCFailed(
					self.req.assertion.assertion.clone(),
					ErrorDetail::StfError(ErrorString::truncate_from(format!("{e:?}").into())),
				)
			})?;
		let payload = credential.issuer.mrenclave.clone();
		let (enclave_account, sig) = signer.sign_vc_with_self(payload.as_bytes()).map_err(|e| {
			VCMPError::RequestVCFailed(
				self.req.assertion.assertion.clone(),
				ErrorDetail::StfError(ErrorString::truncate_from(format!("{e:?}").into())),
			)
		})?;
		debug!("Credential Payload signature: {:?}", sig);

		credential.add_proof(&sig, &enclave_account);
		credential.validate().map_err(|e| {
			VCMPError::RequestVCFailed(
				self.req.assertion.assertion.clone(),
				ErrorDetail::StfError(ErrorString::truncate_from(format!("{e:?}").into())),
			)
		})?;

		let vc_index: H256 = credential
			.get_index()
			.map_err(|e| {
				VCMPError::RequestVCFailed(
					self.req.assertion.assertion.clone(),
					ErrorDetail::StfError(ErrorString::truncate_from(format!("{e:?}").into())),
				)
			})?
			.into();
		let credential_str = credential.to_json().map_err(|_| {
			VCMPError::RequestVCFailed(
				self.req.assertion.assertion.clone(),
				ErrorDetail::ParseError,
			)
		})?;
		log::error!("Credential: {}, length: {}", credential_str, credential_str.len());
		let vc_hash: H256 = blake2_256(credential_str.as_bytes()).into();
		log::error!("VC hash: {:?}", vc_hash);

		let vc_response = VCResponse {
			assertion_request: self.req.assertion.clone(),
			vc_hash,
			vc_payload: credential_str.as_bytes().to_vec(),
			vc_index,
		};
		log::error!("Finished processing request_vc in isolated thread");
		sender.send((vc_response, self.req.sender.clone())).unwrap();

		Ok(())
	}
}

fn build_holding_time(
	req: &AssertionBuildRequest,
	htype: AmountHoldingTimeType,
	min_balance: ParameterString,
) -> Result<lc_credentials::Credential, VCMPError> {
	lc_assertion_build::holding_time::build(req, htype, min_balance)
}
