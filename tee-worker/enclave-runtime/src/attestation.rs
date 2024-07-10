// Copyright 2022 Integritee AG and Supercomputing Systems AG
// Copyright (C) 2017-2019 Baidu, Inc. All Rights Reserved.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met:
//
//  * Redistributions of source code must retain the above copyright
//    notice, this list of conditions and the following disclaimer.
//  * Redistributions in binary form must reproduce the above copyright
//    notice, this list of conditions and the following disclaimer in
//    the documentation and/or other materials provided with the
//    distribution.
//  * Neither the name of Baidu, Inc., nor the names of its
//    contributors may be used to endorse or promote products derived
//    from this software without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
// "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
// LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
// A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
// OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
// SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
// LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
// DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
// THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
// (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
// OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

use crate::{
	initialization::global_components::{
		GLOBAL_ATTESTATION_HANDLER_COMPONENT, GLOBAL_SHIELDING_KEY_REPOSITORY_COMPONENT,
	},
	utils::{
		get_extrinsic_factory_from_integritee_solo_or_parachain,
		get_node_metadata_repository_from_integritee_solo_or_parachain,
	},
	Error as EnclaveError, Result as EnclaveResult,
};
use codec::{Decode, Encode};
use itp_attestation_handler::{AttestationHandler, RemoteAttestationType, SgxQlQveCollateral};
use itp_component_container::ComponentGetter;
use itp_extrinsics_factory::CreateExtrinsics;
use itp_node_api::metadata::{
	pallet_teebag::TeebagCallIndexes,
	provider::{AccessNodeMetadata, Error as MetadataProviderError},
	Error as MetadataError,
};
use itp_node_api_metadata::NodeMetadata;
use itp_settings::{
	worker::MR_ENCLAVE_SIZE,
	worker_mode::{ProvideWorkerMode, WorkerModeProvider},
};
use itp_sgx_crypto::{ed25519_derivation::DeriveEd25519, key_repository::AccessKey};
use itp_types::{AttestationType, OpaqueCall, WorkerType};
use itp_utils::write_slice_and_whitespace_pad;
use log::*;
use sgx_types::{error::*, types::*};
use sp_core::{ed25519::Public as Ed25519Public, Pair};
use sp_runtime::OpaqueExtrinsic;
use std::{prelude::v1::*, slice, vec::Vec};

#[no_mangle]
pub unsafe extern "C" fn get_mrenclave(mrenclave: *mut u8, mrenclave_size: usize) -> SgxStatus {
	if mrenclave.is_null() || mrenclave_size < MR_ENCLAVE_SIZE {
		return SgxStatus::InvalidParameter
	}
	let attestation_handler = match GLOBAL_ATTESTATION_HANDLER_COMPONENT.get() {
		Ok(r) => r,
		Err(e) => {
			error!("Component get failure: {:?}", e);
			return SgxStatus::Unexpected
		},
	};
	match attestation_handler.get_mrenclave() {
		Ok(mrenclave_value) => {
			let mrenclave_slice = slice::from_raw_parts_mut(mrenclave, mrenclave_size);
			if let Err(e) =
				write_slice_and_whitespace_pad(mrenclave_slice, mrenclave_value.to_vec())
			{
				error!("Failed to transfer mrenclave to o-call buffer: {:?}", e);
				return SgxStatus::Unexpected
			}
			SgxStatus::Success
		},
		Err(e) => e.into(),
	}
}

// FIXME: add dcap suppoort for call site
pub fn create_ra_report_and_signature(
	skip_ra: bool,
	remote_attestation_type: RemoteAttestationType,
	sign_type: QuoteSignType,
	quoting_enclave_target_info: Option<&TargetInfo>,
	quote_size: Option<&u32>,
) -> EnclaveResult<(Vec<u8>, Vec<u8>)> {
	let attestation_handler = match GLOBAL_ATTESTATION_HANDLER_COMPONENT.get() {
		Ok(r) => r,
		Err(e) => {
			error!("Component get failure: {:?}", e);
			return Err(e.into())
		},
	};

	match remote_attestation_type {
		RemoteAttestationType::Epid => {
			match attestation_handler.create_epid_ra_report_and_signature(sign_type, skip_ra) {
				Ok(epid) => Ok(epid),
				Err(e) => {
					error!("create_epid_ra_report_and_signature failure: {:?}", e);
					Err(e.into())
				},
			}
		},
		RemoteAttestationType::Dcap => {
			match attestation_handler.generate_dcap_ra_cert(
				quoting_enclave_target_info,
				quote_size,
				skip_ra,
			) {
				Ok((key_der, cert_der, _qe_quote)) => Ok((key_der, cert_der)),
				Err(e) => {
					error!("generate_dcap_ra_cert failure: {:?}", e);
					Err(e.into())
				},
			}
		},
	}
}

#[no_mangle]
pub unsafe extern "C" fn generate_ias_ra_extrinsic(
	w_url: *const u8,
	w_url_size: u32,
	unchecked_extrinsic: *mut u8,
	unchecked_extrinsic_max_size: u32,
	unchecked_extrinsic_size: *mut u32,
	skip_ra: c_int,
) -> SgxStatus {
	if w_url.is_null() || unchecked_extrinsic.is_null() {
		return SgxStatus::InvalidParameter
	}
	let mut url_slice = slice::from_raw_parts(w_url, w_url_size as usize);
	let url = match String::decode(&mut url_slice) {
		// Litentry: the teebag extrinsic expects an URL with plain utf8 encoded Vec<u8>, not string scale-encoded
		Ok(url) => url.as_bytes().to_vec(),
		Err(_) =>
			return EnclaveError::Other("Could not decode url slice to a valid String".into()).into(),
	};
	let extrinsic_slice =
		slice::from_raw_parts_mut(unchecked_extrinsic, unchecked_extrinsic_max_size as usize);

	let extrinsic = match generate_ias_ra_extrinsic_internal(url, skip_ra == 1) {
		Ok(xt) => xt,
		Err(e) => return e.into(),
	};

	*unchecked_extrinsic_size =
		match write_slice_and_whitespace_pad(extrinsic_slice, extrinsic.encode()) {
			Ok(l) => l as u32,
			Err(e) => return EnclaveError::BufferError(e).into(),
		};
	SgxStatus::Success
}

#[no_mangle]
pub unsafe extern "C" fn generate_dcap_ra_extrinsic(
	w_url: *const u8,
	w_url_size: u32,
	unchecked_extrinsic: *mut u8,
	unchecked_extrinsic_max_size: u32,
	unchecked_extrinsic_size: *mut u32,
	skip_ra: c_int,
	quoting_enclave_target_info: Option<&TargetInfo>,
	quote_size: Option<&u32>,
) -> SgxStatus {
	if w_url.is_null() || unchecked_extrinsic.is_null() {
		return SgxStatus::InvalidParameter
	}
	let mut url_slice = slice::from_raw_parts(w_url, w_url_size as usize);
	let url = match String::decode(&mut url_slice) {
		// Litentry: the teebag extrinsic expects an URL with plain utf8 encoded Vec<u8>, not string scale-encoded
		Ok(url) => url.as_bytes().to_vec(),
		Err(_) =>
			return EnclaveError::Other("Could not decode url slice to a valid String".into()).into(),
	};
	let extrinsic_slice =
		slice::from_raw_parts_mut(unchecked_extrinsic, unchecked_extrinsic_max_size as usize);

	let extrinsic = match generate_dcap_ra_extrinsic_internal(
		url,
		skip_ra == 1,
		quoting_enclave_target_info,
		quote_size,
	) {
		Ok(xt) => xt,
		Err(e) => return e.into(),
	};

	*unchecked_extrinsic_size =
		match write_slice_and_whitespace_pad(extrinsic_slice, extrinsic.encode()) {
			Ok(l) => l as u32,
			Err(e) => return EnclaveError::BufferError(e).into(),
		};
	SgxStatus::Success
}

pub fn generate_dcap_ra_extrinsic_internal(
	url: Vec<u8>,
	skip_ra: bool,
	quoting_enclave_target_info: Option<&TargetInfo>,
	quote_size: Option<&u32>,
) -> EnclaveResult<OpaqueExtrinsic> {
	let attestation_handler = GLOBAL_ATTESTATION_HANDLER_COMPONENT.get()?;

	if !skip_ra {
		let (_priv_key_der, _cert_der, dcap_quote) = attestation_handler.generate_dcap_ra_cert(
			quoting_enclave_target_info,
			quote_size,
			skip_ra,
		)?;

		generate_dcap_ra_extrinsic_from_quote_internal(url, &dcap_quote)
	} else {
		generate_dcap_skip_ra_extrinsic_from_mr_enclave(
			url,
			&attestation_handler.get_mrenclave()?.encode(),
		)
	}
}

#[no_mangle]
pub unsafe extern "C" fn generate_dcap_ra_quote(
	skip_ra: c_int,
	quoting_enclave_target_info: &TargetInfo,
	quote_size: u32,
	dcap_quote_p: *mut u8,
	dcap_quote_size: u32,
) -> SgxStatus {
	if dcap_quote_p.is_null() {
		return SgxStatus::InvalidParameter
	}
	let dcap_quote = match generate_dcap_ra_quote_internal(
		skip_ra == 1,
		quoting_enclave_target_info,
		quote_size,
	) {
		Ok(dcap_quote) => dcap_quote,
		Err(e) => return e.into(),
	};

	let dcap_quote_slice = slice::from_raw_parts_mut(dcap_quote_p, dcap_quote_size as usize);

	if let Err(e) = write_slice_and_whitespace_pad(dcap_quote_slice, dcap_quote) {
		return EnclaveError::BufferError(e).into()
	};

	SgxStatus::Success
}

pub fn generate_dcap_ra_quote_internal(
	skip_ra: bool,
	quoting_enclave_target_info: &TargetInfo,
	quote_size: u32,
) -> EnclaveResult<Vec<u8>> {
	let attestation_handler = GLOBAL_ATTESTATION_HANDLER_COMPONENT.get()?;

	let (_priv_key_der, _cert_der, dcap_quote) = attestation_handler.generate_dcap_ra_cert(
		Some(quoting_enclave_target_info),
		Some(&quote_size),
		skip_ra,
	)?;

	Ok(dcap_quote)
}

#[no_mangle]
pub unsafe extern "C" fn generate_dcap_ra_extrinsic_from_quote(
	w_url: *const u8,
	w_url_size: u32,
	quote: *const u8,
	quote_size: u32,
	unchecked_extrinsic: *mut u8,
	unchecked_extrinsic_max_size: u32,
	unchecked_extrinsic_size: *mut u32,
) -> SgxStatus {
	if w_url.is_null() || unchecked_extrinsic.is_null() {
		return SgxStatus::InvalidParameter
	}
	let mut url_slice = slice::from_raw_parts(w_url, w_url_size as usize);
	let url = match String::decode(&mut url_slice) {
		// Litentry: the teebag extrinsic expects an URL with plain utf8 encoded Vec<u8>, not string scale-encoded
		Ok(url) => url.as_bytes().to_vec(),
		Err(_) =>
			return EnclaveError::Other("Could not decode url slice to a valid String".into()).into(),
	};

	let extrinsic_slice =
		slice::from_raw_parts_mut(unchecked_extrinsic, unchecked_extrinsic_max_size as usize);

	let quote_slice = slice::from_raw_parts(quote, quote_size as usize);

	let extrinsic = match generate_dcap_ra_extrinsic_from_quote_internal(url, quote_slice) {
		Ok(xt) => xt,
		Err(e) => return e.into(),
	};

	*unchecked_extrinsic_size =
		match write_slice_and_whitespace_pad(extrinsic_slice, extrinsic.encode()) {
			Ok(l) => l as u32,
			Err(e) => return EnclaveError::BufferError(e).into(),
		};
	SgxStatus::Success
}

pub fn generate_dcap_ra_extrinsic_from_quote_internal(
	url: Vec<u8>,
	quote: &[u8],
) -> EnclaveResult<OpaqueExtrinsic> {
	let node_metadata_repo = get_node_metadata_repository_from_integritee_solo_or_parachain()?;
	trace!("    [Enclave] Compose register enclave getting callIDs:");

	let call_ids = node_metadata_repo
		.get_from_metadata(|m| m.register_enclave_call_indexes())?
		.map_err(MetadataProviderError::MetadataError)?;
	trace!("    [Enclave] Compose register enclave call DCAP IDs: {:?}", call_ids);

	let shielding_pubkey = get_shielding_pubkey()?;
	let vc_pubkey = get_vc_pubkey()?;
	let attestation_type = AttestationType::Dcap(Default::default()); // skip_ra should be false here already

	let call = OpaqueCall::from_tuple(&(
		call_ids,
		WorkerType::Identity,
		WorkerModeProvider::worker_mode(),
		quote,
		url,
		shielding_pubkey,
		vc_pubkey,
		attestation_type,
	));

	trace!("    [Enclave] Compose register enclave got extrinsic, returning");
	create_extrinsics(call)
}

pub fn generate_dcap_skip_ra_extrinsic_from_mr_enclave(
	url: Vec<u8>,
	quote: &[u8],
) -> EnclaveResult<OpaqueExtrinsic> {
	let node_metadata_repo = get_node_metadata_repository_from_integritee_solo_or_parachain()?;
	trace!("    [Enclave] Compose register enclave (skip-ra) getting callIDs:");

	let call_ids = node_metadata_repo
		.get_from_metadata(|m| m.register_enclave_call_indexes())?
		.map_err(MetadataProviderError::MetadataError)?;
	trace!("    [Enclave] Compose register enclave (skip-ra) call DCAP IDs: {:?}", call_ids);

	let shielding_pubkey = get_shielding_pubkey()?;
	let vc_pubkey = get_vc_pubkey()?;

	let call = OpaqueCall::from_tuple(&(
		call_ids,
		WorkerType::Identity,
		WorkerModeProvider::worker_mode(),
		quote,
		url,
		shielding_pubkey,
		vc_pubkey,
		AttestationType::Ignore,
	));

	info!("    [Enclave] Compose register enclave (skip-ra) got extrinsic, returning");
	create_extrinsics(call)
}

fn generate_ias_ra_extrinsic_internal(
	url: Vec<u8>,
	skip_ra: bool,
) -> EnclaveResult<OpaqueExtrinsic> {
	let attestation_handler = GLOBAL_ATTESTATION_HANDLER_COMPONENT.get()?;
	let cert_der = attestation_handler.generate_ias_ra_cert(skip_ra)?;

	generate_ias_ra_extrinsic_from_der_cert_internal(url, &cert_der, skip_ra)
}

pub fn generate_ias_ra_extrinsic_from_der_cert_internal(
	url: Vec<u8>,
	cert_der: &[u8],
	skip_ra: bool,
) -> EnclaveResult<OpaqueExtrinsic> {
	let node_metadata_repo = get_node_metadata_repository_from_integritee_solo_or_parachain()?;

	info!("    [Enclave] Compose register ias enclave (skip-ra) call");
	let call_ids = node_metadata_repo
		.get_from_metadata(|m| m.register_enclave_call_indexes())?
		.map_err(MetadataProviderError::MetadataError)?;

	let shielding_pubkey = get_shielding_pubkey()?;
	let vc_pubkey = get_vc_pubkey()?;
	let attestation_type = if skip_ra { AttestationType::Ignore } else { AttestationType::Ias };

	let call = OpaqueCall::from_tuple(&(
		call_ids,
		WorkerType::Identity,
		WorkerModeProvider::worker_mode(),
		cert_der,
		url,
		shielding_pubkey,
		vc_pubkey,
		attestation_type,
	));

	create_extrinsics(call)
}

fn create_extrinsics(call: OpaqueCall) -> EnclaveResult<OpaqueExtrinsic> {
	let extrinsics_factory = get_extrinsic_factory_from_integritee_solo_or_parachain()?;
	let extrinsics = extrinsics_factory.create_extrinsics(&[call], None)?;

	match extrinsics.first() {
		Some(xt) => Ok(xt.clone()),
		None => Err(EnclaveError::Other("Could not create extrinsic".into())),
	}
}

#[no_mangle]
pub unsafe extern "C" fn generate_register_quoting_enclave_extrinsic(
	collateral: *const CQlQveCollateral,
	unchecked_extrinsic: *mut u8,
	unchecked_extrinsic_max_size: u32,
	unchecked_extrinsic_size: *mut u32,
) -> SgxStatus {
	if unchecked_extrinsic.is_null() || collateral.is_null() {
		return SgxStatus::InvalidParameter
	}
	let extrinsic_slice =
		slice::from_raw_parts_mut(unchecked_extrinsic, unchecked_extrinsic_max_size as usize);
	let collateral = SgxQlQveCollateral::from_c_type(&*collateral);
	let collateral_data = match collateral.get_quoting_enclave_split() {
		Some(d) => d,
		None => return SgxStatus::InvalidParameter,
	};

	let call_index_getter = |m: &NodeMetadata| m.register_quoting_enclave_call_indexes();
	*unchecked_extrinsic_size = match generate_generic_register_collateral_extrinsic(
		call_index_getter,
		extrinsic_slice,
		&collateral_data.0,
		&collateral_data.1,
		&collateral.qe_identity_issuer_chain,
	) {
		Ok(l) => l as u32,
		Err(e) => return e.into(),
	};
	SgxStatus::Success
}

#[no_mangle]
pub unsafe extern "C" fn generate_register_tcb_info_extrinsic(
	collateral: *const CQlQveCollateral,
	unchecked_extrinsic: *mut u8,
	unchecked_extrinsic_max_size: u32,
	unchecked_extrinsic_size: *mut u32,
) -> SgxStatus {
	if unchecked_extrinsic.is_null() || collateral.is_null() {
		return SgxStatus::InvalidParameter
	}
	let extrinsic_slice =
		slice::from_raw_parts_mut(unchecked_extrinsic, unchecked_extrinsic_max_size as usize);
	let collateral = SgxQlQveCollateral::from_c_type(&*collateral);
	let collateral_data = match collateral.get_tcb_info_split() {
		Some(d) => d,
		None => return SgxStatus::InvalidParameter,
	};

	let call_index_getter = |m: &NodeMetadata| m.register_tcb_info_call_indexes();
	*unchecked_extrinsic_size = match generate_generic_register_collateral_extrinsic(
		call_index_getter,
		extrinsic_slice,
		&collateral_data.0,
		&collateral_data.1,
		&collateral.tcb_info_issuer_chain,
	) {
		Ok(l) => l as u32,
		Err(e) => return e.into(),
	};
	SgxStatus::Success
}

pub fn generate_generic_register_collateral_extrinsic<F>(
	getter: F,
	extrinsic_slice: &mut [u8],
	collateral_data: &str,
	data_signature: &[u8],
	issuer_chain: &[u8],
) -> EnclaveResult<usize>
where
	F: Fn(&NodeMetadata) -> Result<[u8; 2], MetadataError>,
{
	let node_metadata_repo = get_node_metadata_repository_from_integritee_solo_or_parachain()?;
	let call_ids = node_metadata_repo
		.get_from_metadata(getter)?
		.map_err(MetadataProviderError::MetadataError)?;
	info!("    [Enclave] Compose register collateral call: {:?}", call_ids);
	let call = OpaqueCall::from_tuple(&(call_ids, collateral_data, data_signature, issuer_chain));

	let xt = create_extrinsics(call)?;
	write_slice_and_whitespace_pad(extrinsic_slice, xt.encode())
		.map_err(|e| format!("{:?}", e).into())
}

#[no_mangle]
pub extern "C" fn dump_ias_ra_cert_to_disk() -> SgxStatus {
	let attestation_handler = match GLOBAL_ATTESTATION_HANDLER_COMPONENT.get() {
		Ok(r) => r,
		Err(e) => {
			error!("Component get failure: {:?}", e);
			return SgxStatus::Unexpected
		},
	};
	match attestation_handler.dump_ias_ra_cert_to_disk() {
		Ok(_) => SgxStatus::Success,
		Err(e) => e.into(),
	}
}

#[no_mangle]
pub unsafe extern "C" fn dump_dcap_ra_cert_to_disk(
	quoting_enclave_target_info: &TargetInfo,
	quote_size: u32,
) -> SgxStatus {
	let attestation_handler = match GLOBAL_ATTESTATION_HANDLER_COMPONENT.get() {
		Ok(r) => r,
		Err(e) => {
			error!("Component get failure: {:?}", e);
			return SgxStatus::Unexpected
		},
	};
	match attestation_handler.dump_dcap_ra_cert_to_disk(quoting_enclave_target_info, quote_size) {
		Ok(_) => SgxStatus::Success,
		Err(e) => e.into(),
	}
}

#[no_mangle]
pub unsafe extern "C" fn dump_dcap_collateral_to_disk(
	collateral: *const CQlQveCollateral,
) -> SgxStatus {
	let collateral = SgxQlQveCollateral::from_c_type(&*collateral);
	collateral.dump_to_disk();
	SgxStatus::Success
}

fn get_shielding_pubkey() -> EnclaveResult<Option<Vec<u8>>> {
	let shielding_pubkey = GLOBAL_SHIELDING_KEY_REPOSITORY_COMPONENT
		.get()?
		.retrieve_key()
		.and_then(|keypair| {
			let pubkey = keypair.public_key();
			sgx_serialize::json::encode(&pubkey)
				.map(|s| s.as_bytes().to_vec())
				.map_err(|_| itp_sgx_crypto::Error::Serde)
		})
		.map_err(|_| EnclaveError::Other("json encode error".into()))
		.ok();

	Ok(shielding_pubkey)
}

fn get_vc_pubkey() -> EnclaveResult<Option<Ed25519Public>> {
	let vc_pubkey = GLOBAL_SHIELDING_KEY_REPOSITORY_COMPONENT
		.get()?
		.retrieve_key()
		.and_then(|keypair| {
			// vc signing pubkey
			keypair.derive_ed25519().map(|keypair| keypair.public())
		})
		.ok();

	debug!("[Enclave] VC pubkey: {:?}", vc_pubkey);

	Ok(vc_pubkey)
}
