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
	command_utils::{get_chain_api, get_pair_from_str, get_shielding_key, get_worker_api_direct},
	error::Error,
	trusted_cli::TrustedCli,
	Cli,
};
use base58::{FromBase58, ToBase58};
use codec::{Decode, Encode, Input};
use ita_stf::{trusted_call_result::RequestVcResultOrError, Getter, StfError, TrustedCallSigned};
use itc_rpc_client::direct_client::{DirectApi, DirectClient};
use itp_node_api::api_client::{ParentchainApi, TEEBAG};
use itp_rpc::{Id, RpcRequest, RpcResponse, RpcReturnValue};
use itp_sgx_crypto::ShieldingCryptoEncrypt;
use itp_stf_primitives::types::{ShardIdentifier, TrustedOperation};
use itp_types::{BlockNumber, DirectRequestStatus, RsaRequest, TrustedOperationStatus};
use itp_utils::{FromHexPrefixed, ToHexPrefixed};
use litentry_primitives::{
	aes_encrypt_default, AesRequest, ParentchainHash as Hash, RequestAesKey,
};
use log::*;
use my_node_runtime::{pallet_teebag::Event as TeebagEvent, RuntimeEvent};
use sp_core::H256;
use std::{
	fmt::Debug,
	result::Result as StdResult,
	sync::mpsc::{channel, Receiver, Sender as MpscSender},
	time::Instant,
};
use substrate_api_client::{
	ac_compose_macros::compose_extrinsic, GetChainInfo, SubmitAndWatch, SubscribeEvents, XtStatus,
};
use thiserror::Error;

#[derive(Debug, Error)]
pub(crate) enum TrustedOperationError {
	#[error("extrinsic L1 error: {msg:?}")]
	Extrinsic { msg: String },
	#[error("default error: {msg:?}")]
	Default { msg: String },
}

pub(crate) type TrustedOpResult<T> = StdResult<T, TrustedOperationError>;

pub(crate) fn perform_trusted_operation<T: Decode + Debug>(
	cli: &Cli,
	trusted_args: &TrustedCli,
	top: &TrustedOperation<TrustedCallSigned, Getter>,
) -> TrustedOpResult<T> {
	match top {
		TrustedOperation::indirect_call(_) => send_indirect_request::<T>(cli, trusted_args, top),
		TrustedOperation::direct_call(_) => send_direct_request::<T>(cli, trusted_args, top),
		TrustedOperation::get(getter) =>
			execute_getter_from_cli_args::<T>(cli, trusted_args, getter),
	}
}

fn execute_getter_from_cli_args<T: Decode + Debug>(
	cli: &Cli,
	trusted_args: &TrustedCli,
	getter: &Getter,
) -> TrustedOpResult<T> {
	let shard = read_shard(trusted_args, cli).unwrap();
	let direct_api = get_worker_api_direct(cli);
	get_state(&direct_api, shard, getter)
}

pub(crate) fn get_state<T: Decode + Debug>(
	direct_api: &DirectClient,
	shard: ShardIdentifier,
	getter: &Getter,
) -> TrustedOpResult<T> {
	// Compose jsonrpc call.
	let data = RsaRequest::new(shard, getter.encode());
	let rpc_method = "state_executeGetter".to_owned();
	let jsonrpc_call: String = RpcRequest::compose_jsonrpc_call(
		Id::Text("1".to_string()),
		rpc_method,
		vec![data.to_hex()],
	)
	.unwrap();

	let rpc_response_str = direct_api.get(&jsonrpc_call).unwrap();

	// Decode RPC response.
	let rpc_response: RpcResponse = serde_json::from_str(&rpc_response_str)
		.map_err(|err| TrustedOperationError::Default { msg: err.to_string() })?;
	let rpc_return_value = RpcReturnValue::from_hex(&rpc_response.result)
		// Replace with `inspect_err` once it's stable.
		.map_err(|err| {
			error!("Failed to decode RpcReturnValue: {:?}", err);
			TrustedOperationError::Default { msg: "RpcReturnValue::from_hex".to_string() }
		})?;

	if rpc_return_value.status == DirectRequestStatus::Error {
		println!("[Error] {}", String::decode(&mut rpc_return_value.value.as_slice()).unwrap());
		return Err(TrustedOperationError::Default {
			msg: "[Error] DirectRequestStatus::Error".to_string(),
		})
	}

	let maybe_state: Option<Vec<u8>> = Option::decode(&mut rpc_return_value.value.as_slice())
		// Replace with `inspect_err` once it's stable.
		.map_err(|err| {
			error!("Failed to decode return value: {:?}", err);
			TrustedOperationError::Default { msg: "Option::decode".to_string() }
		})?;

	match maybe_state {
		Some(state) => {
			let decoded = decode_response_value(&mut state.as_slice())?;
			Ok(decoded)
		},
		None => Err(TrustedOperationError::Default { msg: "Value not present".to_string() }),
	}
}

fn send_indirect_request<T: Decode + Debug>(
	cli: &Cli,
	trusted_args: &TrustedCli,
	trusted_operation: &TrustedOperation<TrustedCallSigned, Getter>,
) -> TrustedOpResult<T> {
	let mut chain_api = get_chain_api(cli);
	let encryption_key = get_shielding_key(cli).unwrap();
	let call_encrypted = encryption_key.encrypt(&trusted_operation.encode()).unwrap();

	let shard = read_shard(trusted_args, cli).unwrap();
	debug!(
		"invoke indirect send_request: trusted operation: {:?},  shard: {}",
		trusted_operation,
		shard.encode().to_base58()
	);
	let arg_signer = &trusted_args.xt_signer;
	let signer = get_pair_from_str(arg_signer);
	chain_api.set_signer(signer.into());

	let request = RsaRequest::new(shard, call_encrypted);
	let xt = compose_extrinsic!(&chain_api, TEEBAG, "post_opaque_task", request);

	let block_hash = match chain_api.submit_and_watch_extrinsic_until(xt, XtStatus::InBlock) {
		Ok(xt_report) => {
			println!(
				"[+] invoke TrustedOperation extrinsic success. extrinsic hash: {:?} / status: {:?} / block hash: {:?}",
				xt_report.extrinsic_hash, xt_report.status, xt_report.block_hash.unwrap()
			);
			xt_report.block_hash.unwrap()
		},
		Err(e) => {
			error!("invoke TrustedOperation extrinsic failed {:?}", e);
			return Err(TrustedOperationError::Extrinsic { msg: format!("{:?}", e) })
		},
	};

	info!(
		"Trusted call extrinsic sent and successfully included in parentchain block with hash {:?}.",
		block_hash
	);
	info!("Waiting for execution confirmation from enclave...");
	let mut subscription = chain_api.subscribe_events().unwrap();
	loop {
		let event_result = subscription.next_events::<RuntimeEvent, Hash>();
		if let Some(Ok(event_records)) = event_result {
			for event_record in event_records {
				if let RuntimeEvent::Teebag(TeebagEvent::ParentchainBlockProcessed {
					who: _signer,
					block_number: confirmed_block_number,
					block_hash: confirmed_block_hash,
					task_merkle_root: trusted_calls_merkle_root,
				}) = event_record.event
				{
					info!("Confirmation of ParentchainBlockProcessed received");
					debug!("shard: {:?}", shard);
					debug!("confirmed parentchain block Hash: {:?}", block_hash);
					debug!("trusted calls merkle root: {:?}", trusted_calls_merkle_root);
					debug!("Confirmed stf block Hash: {:?}", confirmed_block_hash);
					if let Err(e) = check_if_received_event_exceeds_expected(
						&chain_api,
						block_hash,
						confirmed_block_hash,
						confirmed_block_number,
					) {
						error!("ParentchainBlockProcessed event: {:?}", e);
						return Err(TrustedOperationError::Default {
							msg: format!("ParentchainBlockProcessed event: {:?}", e),
						})
					};

					if confirmed_block_hash == block_hash {
						let value = decode_response_value(&mut block_hash.encode().as_slice())?;
						return Ok(value)
					}
				}
			}
		} else {
			warn!("Error in event subscription: {:?}", event_result)
		}
	}
}

fn check_if_received_event_exceeds_expected(
	chain_api: &ParentchainApi,
	block_hash: Hash,
	confirmed_block_hash: Hash,
	confirmed_block_number: BlockNumber,
) -> Result<(), Error> {
	let block_number = chain_api.get_header(Some(block_hash))?.ok_or(Error::MissingBlock)?.number;

	info!("Expected block Number: {:?}", block_number);
	info!("Confirmed block Number: {:?}", confirmed_block_number);
	// The returned block number belongs to a subsequent event. We missed our event and can break the loop.
	if confirmed_block_number > block_number {
		return Err(Error::ConfirmedBlockNumberTooHigh(confirmed_block_number, block_number))
	}
	// The block number is correct, but the block hash does not fit.
	if block_number == confirmed_block_number && block_hash != confirmed_block_hash {
		return Err(Error::ConfirmedBlockHashDoesNotMatchExpected(confirmed_block_hash, block_hash))
	}
	Ok(())
}

pub fn read_shard(trusted_args: &TrustedCli, cli: &Cli) -> Result<ShardIdentifier, codec::Error> {
	match &trusted_args.shard {
		Some(s) => match s.from_base58() {
			Ok(s) => ShardIdentifier::decode(&mut &s[..]),
			_ => panic!("shard argument must be base58 encoded"),
		},
		None => match trusted_args.mrenclave.clone() {
			Some(mrenclave) =>
				if let Ok(s) = mrenclave.from_base58() {
					ShardIdentifier::decode(&mut &s[..])
				} else {
					panic!("Mrenclave argument must be base58 encoded")
				},
			None => {
				// Fetch mrenclave from worker
				let direct_api = get_worker_api_direct(cli);
				if let Ok(s) = direct_api.get_state_mrenclave() {
					ShardIdentifier::decode(&mut &s[..])
				} else {
					panic!("Unable to fetch MRENCLAVE from worker endpoint");
				}
			},
		},
	}
}

/// sends a rpc watch request to the worker api server
fn send_direct_request<T: Decode + Debug>(
	cli: &Cli,
	trusted_args: &TrustedCli,
	top: &TrustedOperation<TrustedCallSigned, Getter>,
) -> TrustedOpResult<T> {
	let encryption_key = get_shielding_key(cli).unwrap();
	let shard = read_shard(trusted_args, cli).unwrap();
	let jsonrpc_call: String = get_json_request(shard, top, encryption_key);

	debug!("get direct api");
	let direct_api = get_worker_api_direct(cli);

	debug!("setup sender and receiver");
	let (sender, receiver) = channel();
	direct_api.watch(jsonrpc_call, sender);

	debug!("waiting for rpc response");
	loop {
		match receiver.recv() {
			Ok(response) => {
				debug!("received response");
				let response: RpcResponse = serde_json::from_str(&response).unwrap();
				if let Ok(return_value) = RpcReturnValue::from_hex(&response.result) {
					match return_value.status {
						DirectRequestStatus::Error => {
							debug!("request status is error");
							if let Ok(value) = String::decode(&mut return_value.value.as_slice()) {
								println!("[Error] {}", value);
							}
							direct_api.close().unwrap();
							return Err(TrustedOperationError::Default {
								msg: "[Error] DirectRequestStatus::Error".to_string(),
							})
						},
						DirectRequestStatus::TrustedOperationStatus(status, top_hash) => {
							debug!("request status is: {:?}, top_hash: {:?}", status, top_hash);

							if matches!(status, TrustedOperationStatus::Invalid) {
								let error = StfError::decode(&mut return_value.value.as_slice())
									.map_err(|e| TrustedOperationError::Default {
										msg: format!("Could not decode error value: {:?}", e),
									})?;
								return Err(TrustedOperationError::Default {
									msg: format!(
										"[Error] Error occurred while executing trusted call: {:?}",
										error
									),
								})
							}
							if let Ok(value) = Hash::decode(&mut return_value.value.as_slice()) {
								println!("Trusted call {:?} is {:?}", value, status);
							}
							if !return_value.do_watch {
								direct_api.close().unwrap();
								let value =
									decode_response_value(&mut return_value.value.as_slice())?;
								return Ok(value)
							}
						},
						DirectRequestStatus::Ok => {
							debug!("request status is ignored");
							direct_api.close().unwrap();
							return Err(TrustedOperationError::Default {
								msg: "Unexpected status: DirectRequestStatus::Ok".to_string(),
							})
						},
					}
				};
			},
			Err(e) => {
				error!("failed to receive rpc response: {:?}", e);
				direct_api.close().unwrap();
				return Err(TrustedOperationError::Default {
					msg: "failed to receive rpc response".to_string(),
				})
			},
		};
	}
}

pub(crate) fn send_direct_batch_vc_request(
	cli: &Cli,
	trusted_args: &TrustedCli,
	top: &TrustedOperation<TrustedCallSigned, Getter>,
	key: RequestAesKey,
	resp_sender: MpscSender<Result<RequestVcResultOrError, String>>,
) {
	let encryption_key = get_shielding_key(cli).unwrap();
	let shard = read_shard(trusted_args, cli).unwrap();
	let jsonrpc_call: String = get_vc_json_request(shard, top, encryption_key, key);

	debug!("get direct api");
	let direct_api = get_worker_api_direct(cli);

	debug!("setup sender and receiver");
	let (sender, receiver) = channel();
	direct_api.watch(jsonrpc_call, sender);

	debug!("waiting for rpc response");
	let mut req_cnt = 0u8;
	let mut ignore_first = true;
	std::thread::spawn(move || loop {
		match receiver.recv() {
			Ok(response) => {
				req_cnt += 1;
				debug!("received response");
				let response: RpcResponse = serde_json::from_str(&response).unwrap();
				if let Ok(return_value) = RpcReturnValue::from_hex(&response.result) {
					debug!("successfully decoded rpc response: {:?}", return_value);
					if ignore_first {
						ignore_first = false;
						continue
					}
					match return_value.status {
						DirectRequestStatus::TrustedOperationStatus(status, top_hash) => {
							debug!("request status is: {:?}, top_hash: {:?}", status, top_hash);
							if let Ok(value) =
								RequestVcResultOrError::decode(&mut return_value.value.as_slice())
							{
								let len = value.len;
								resp_sender.send(Ok(value)).expect("failed to send back result");
								if req_cnt >= len {
									break
								}
							} else {
								// Should never happen.
								error!("failed to decode RequestVcResultOrError.");
								direct_api.close().unwrap();
								resp_sender
									.send(Err(
										"failed to decode RequestVcResultOrError. close channel"
											.to_string(),
									))
									.expect("failed to send back result");
								break
							}
						},
						_ => {
							// Should never happen. RpcReturnValue should always have DirectRequestStatus::TrustedOperationStatus.
							error!("Wrong RpcReturnValue. Should never happen.");
							direct_api.close().unwrap();
							resp_sender
								.send(Err(
									"Wrong RpcReturnValue. Should never happen. close channel"
										.to_string(),
								))
								.expect("failed to send back result");
							break
						},
					}
				} else {
					// Normally should not happen.
					error!("failed to decode RpcReturnValue.");
					direct_api.close().unwrap();
					resp_sender
						.send(Err("failed to decode RpcReturnValue. close channel".to_string()))
						.expect("failed to send back result");
					break
				}
			},
			Err(e) => {
				// Normally should not happen.
				error!("failed to receive rpc response: {:?}", e);
				direct_api.close().unwrap();
				resp_sender
					.send(Err("failed to receive rpc response. close channel".to_string()))
					.expect("failed to send back result");
				break
			},
		};
	});
}

pub(crate) fn get_vc_json_request(
	shard: ShardIdentifier,
	top: &TrustedOperation<TrustedCallSigned, Getter>,
	shielding_pubkey: sgx_crypto_helper::rsa3072::Rsa3072PubKey,
	key: RequestAesKey,
) -> String {
	let encrypted_key = shielding_pubkey.encrypt(&key).unwrap();
	let encrypted_top = aes_encrypt_default(&key, &top.encode());

	// compose jsonrpc call
	let request = AesRequest { shard, key: encrypted_key, payload: encrypted_top };
	RpcRequest::compose_jsonrpc_call(
		Id::Number(1),
		"author_requestVc".to_string(),
		vec![request.to_hex()],
	)
	.unwrap()
}

fn decode_response_value<T: Decode, I: Input>(
	value: &mut I,
) -> StdResult<T, TrustedOperationError> {
	T::decode(value).map_err(|e| TrustedOperationError::Default {
		msg: format!("Could not decode result value: {:?}", e),
	})
}

pub(crate) fn get_json_request(
	shard: ShardIdentifier,
	top: &TrustedOperation<TrustedCallSigned, Getter>,
	shielding_pubkey: sgx_crypto_helper::rsa3072::Rsa3072PubKey,
) -> String {
	let encrypted_top = shielding_pubkey.encrypt(&top.encode()).unwrap();

	// compose jsonrpc call
	let request = RsaRequest::new(shard, encrypted_top);
	RpcRequest::compose_jsonrpc_call(
		Id::Text("1".to_string()),
		"author_submitAndWatchRsaRequest".to_string(),
		vec![request.to_hex()],
	)
	.unwrap()
}

pub(crate) fn wait_until(
	receiver: &Receiver<String>,
	until: impl Fn(TrustedOperationStatus) -> bool,
) -> Option<(H256, Instant)> {
	debug!("waiting for rpc response");
	loop {
		match receiver.recv() {
			Ok(response) => {
				debug!("received response: {}", response);
				let parse_result: Result<RpcResponse, _> = serde_json::from_str(&response);
				if let Ok(response) = parse_result {
					if let Ok(return_value) = RpcReturnValue::from_hex(&response.result) {
						debug!("successfully decoded rpc response: {:?}", return_value);
						match return_value.status {
							DirectRequestStatus::Error => {
								debug!("request status is error");
								if let Ok(value) =
									String::decode(&mut return_value.value.as_slice())
								{
									println!("[Error] {}", value);
								}
								return None
							},
							DirectRequestStatus::TrustedOperationStatus(status, top_hash) => {
								debug!("request status is: {:?}, top_hash: {:?}", status, top_hash);
								if let Ok(value) = Hash::decode(&mut return_value.value.as_slice())
								{
									println!("Trusted call {:?} is {:?}", value, status);
									if until(status.clone()) {
										return Some((top_hash, Instant::now()))
									} else if status == TrustedOperationStatus::Invalid {
										error!("Invalid request");
										return None
									}
								}
							},
							DirectRequestStatus::Ok => {
								debug!("request status is ignored");
								return None
							},
						}
					};
				} else {
					error!("Could not parse response");
				};
			},
			Err(e) => {
				error!("failed to receive rpc response: {:?}", e);
				return None
			},
		};
	}
}
