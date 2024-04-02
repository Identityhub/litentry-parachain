#![cfg_attr(not(feature = "std"), no_std)]

extern crate core;
#[cfg(all(not(feature = "std"), feature = "sgx"))]
extern crate sgx_tstd as std;

#[cfg(all(feature = "std", feature = "sgx"))]
compile_error!("feature \"std\" and feature \"sgx\" cannot be enabled at the same time");

use bc_enclave_registry::EnclaveRegistryLookup;
use codec::Encode;
use core::time::Duration;
use itc_direct_rpc_client::{RpcClient, RpcClientFactory};
use itc_direct_rpc_server::SendRpcResponse;
use itp_ocall_api::EnclaveAttestationOCallApi;
use itp_sgx_crypto::{
	key_repository::{AccessKey, AccessPubkey},
	ShieldingCryptoEncrypt,
};
use itp_utils::hex::ToHexPrefixed;
use log::{debug, error, info};
use sgx_crypto_helper::rsa3072::Rsa3072PubKey;
use sp_core::{blake2_256, ed25519, Pair as SpCorePair, H256};
use std::collections::HashMap;

#[cfg(feature = "std")]
use std::sync::Mutex;

use bc_musig2_ceremony::{CeremonyEvent, CeremonyRegistry};

use itp_rpc::{Id, RpcRequest};
use itp_sgx_crypto::schnorr::Pair as SchnorrPair;
use itp_types::{DirectRequestStatus, Hash};
use lc_direct_call::DirectCall;
use litentry_primitives::{aes_encrypt_default, Address32, AesRequest, Identity, ShardIdentifier};
#[cfg(feature = "sgx")]
use std::sync::SgxMutex as Mutex;
use std::{
	string::ToString,
	sync::{mpsc::sync_channel, Arc},
	vec,
};

pub fn run_ceremony_orchestration<ClientFactory, AK, ER, OCallApi, SIGNINGAK, SHIELDAK, Responder>(
	signing_key_access: Arc<SIGNINGAK>,
	shielding_key_access: Arc<SHIELDAK>,
	client_factory: Arc<ClientFactory>,
	enclave_registry: Arc<ER>,
	ceremony_registry: Arc<Mutex<CeremonyRegistry<AK>>>,
	ocall_api: Arc<OCallApi>,
	responder: Arc<Responder>,
) where
	ClientFactory: RpcClientFactory + Send + Sync + 'static,
	AK: AccessKey<KeyType = SchnorrPair> + Send + Sync + 'static,
	ER: EnclaveRegistryLookup + Send + Sync + 'static,
	OCallApi: EnclaveAttestationOCallApi + 'static,
	SIGNINGAK: AccessKey<KeyType = ed25519::Pair> + Send + Sync + 'static,
	SHIELDAK: AccessPubkey<KeyType = Rsa3072PubKey> + Send + Sync + 'static,
	Responder: SendRpcResponse<Hash = H256> + 'static,
{
	let (responses_sender, responses_receiver) = sync_channel(1000);
	std::thread::spawn(move || {
		let mut peers_map = HashMap::new();

		//todo: simplify below
		let my_identity: Address32 = signing_key_access.retrieve_key().unwrap().public().0.into();
		let identity =
			Identity::Substrate(signing_key_access.retrieve_key().unwrap().public().0.into());
		loop {
			enclave_registry.get_all().iter().for_each(|(identity, address)| {
				if my_identity != *identity && !peers_map.contains_key(identity.as_ref()) {
					info!("creating new connection to peer: {:?}", address);
					match client_factory.create(address, responses_sender.clone()) {
						Ok(client) => {
							peers_map.insert(*identity.as_ref(), client);
						},
						Err(e) => error!("Could not connect to peer {}, reason: {:?}", address, e),
					}
				}
			});

			let mut ceremony_registry = ceremony_registry.lock().unwrap();
			let mut ceremonies_to_remove = vec![];
			ceremony_registry.values_mut().for_each(|v| {
				let events = v.tick();
				// should be retrieved once, but cannot be at startup becuase it's not yet initialized so it panics ...
				let mr_enclave = ocall_api.get_mrenclave_of_self().unwrap().m;
				for event in events {
					debug!(
						"Processing ceremony event: {:?} for ceremony: {:?}",
						event,
						v.get_id_ref()
					);
					match event {
						CeremonyEvent::FirstRoundStarted(signers, message, nonce) => {
							signers.iter().for_each(|signer_id| {
								let aes_key = random_aes_key();
								let direct_call = DirectCall::NonceShare(
									identity.clone(),
									aes_key,
									message.clone(),
									nonce.serialize(),
								);

								debug!(
									"Sharing nonce with signer: {:?} for ceremony: {:?}",
									signer_id,
									v.get_id_ref()
								);

								let request = prepare_request(
									aes_key,
									shielding_key_access.as_ref(),
									signing_key_access.as_ref(),
									mr_enclave,
									direct_call,
								);
								if let Some(peer) = peers_map.get_mut(signer_id) {
									if let Err(e) = peer.send(&request) {
										error!(
											"Could not send request to signer: {:?}, reason: {:?}",
											signer_id, e
										)
									}
								} else {
									error!("Connection not found for signer: {:?}", signer_id)
								}
							});
						},
						CeremonyEvent::SecondRoundStarted(signers, message, signature) => {
							signers.iter().for_each(|signer_id| {
								let aes_key = random_aes_key();
								let direct_call = DirectCall::PartialSignatureShare(
									identity.clone(),
									aes_key,
									message.clone(),
									signature.serialize(),
								);

								debug!(
										"Sharing partial signature with signer: {:?} for ceremony: {:?}",
										signer_id,
										v.get_id_ref()
									);

								let request = prepare_request(
									aes_key,
									shielding_key_access.as_ref(),
									signing_key_access.as_ref(),
									mr_enclave,
									direct_call,
								);
								peers_map.get_mut(signer_id).unwrap().send(&request).unwrap();
							});
						},
						CeremonyEvent::CeremonyEnded(signature) => {
							debug!(
								"Ceremony {:?} ended, signature {:?}",
								v.get_id_ref(),
								signature
							);
							let hash = blake2_256(&v.get_id_ref().encode());
							let result: Result<[u8; 64], ()> = Ok(signature);
							let encrypted_result =
								aes_encrypt_default(v.get_aes_key(), &result.encode()).encode();
							if let Err(e) = responder.send_state_with_status(
								Hash::from_slice(&hash),
								encrypted_result,
								DirectRequestStatus::Processed(hash.into()),
							) {
								error!("Could not send response to {:?}, reason: {:?}", &hash, e);
							}
							ceremonies_to_remove.push(v.get_id_ref().clone());
						},
						CeremonyEvent::CeremonyError(error) => {
							debug!("Ceremony {:?} error {:?}", v.get_id_ref(), error);
							//todo: stop the ceremony and ping other peers
							let hash = blake2_256(&v.get_id_ref().encode());
							let result: Result<(), ()> = Err(());
							let encrypted_result =
								aes_encrypt_default(v.get_aes_key(), &result.encode()).encode();
							if let Err(e) = responder.send_state_with_status(
								Hash::from_slice(&hash),
								encrypted_result,
								DirectRequestStatus::Processed(hash.into()),
							) {
								error!("Could not send response to {:?}, reason: {:?}", &hash, e);
							}
							ceremonies_to_remove.push(v.get_id_ref().clone());
						},
						CeremonyEvent::CeremonyTimedOut => {
							debug!("Ceremony {:?} timed out", v.get_id_ref());
							let hash = blake2_256(&v.get_id_ref().encode());
							let result: Result<(), ()> = Err(());
							let encrypted_result =
								aes_encrypt_default(v.get_aes_key(), &result.encode()).encode();
							if let Err(e) = responder.send_state_with_status(
								Hash::from_slice(&hash),
								encrypted_result,
								DirectRequestStatus::Processed(hash.into()),
							) {
								error!("Could not send response to {:?}, reason: {:?}", &hash, e);
							}
							ceremonies_to_remove.push(v.get_id_ref().clone());
							ceremonies_to_remove.push(v.get_id_ref().clone());
						},
					}
				}
			});

			ceremonies_to_remove.iter().for_each(|ceremony_id| {
				debug!("Removing ceremony {:?}", ceremony_id);
				let _ = ceremony_registry.remove_entry(ceremony_id);
			});

			std::thread::sleep(Duration::from_millis(1000))
		}
	});

	// here we will process all responses
	std::thread::spawn(move || {
		while let Ok((_id, rpc_return_value)) = responses_receiver.recv() {
			if rpc_return_value.status == DirectRequestStatus::Error {
				error!("Got unexpected direct request status: {:?}", rpc_return_value.status);
			}
		}
	});
}

fn prepare_request<SHIELDAK, SIGNINGAK>(
	aes_key: [u8; 32],
	shielding_key_access: &SHIELDAK,
	signing_key_access: &SIGNINGAK,
	mr_enclave: [u8; 32],
	direct_call: DirectCall,
) -> RpcRequest
where
	SIGNINGAK: AccessKey<KeyType = ed25519::Pair> + Send + Sync + 'static,
	SHIELDAK: AccessPubkey<KeyType = Rsa3072PubKey> + Send + Sync + 'static,
{
	let aes_key_encrypted =
		shielding_key_access.retrieve_pubkey().unwrap().encrypt(&aes_key).unwrap();

	let shard = ShardIdentifier::from_slice(&mr_enclave);
	let dc_signed =
		direct_call.sign(&signing_key_access.retrieve_key().unwrap().into(), &mr_enclave, &shard);
	let encrypted_dc = aes_encrypt_default(&aes_key, &dc_signed.encode());
	let request = AesRequest { shard, key: aes_key_encrypted, payload: encrypted_dc };
	RpcRequest {
		jsonrpc: "2.0".to_string(),
		method: "bitacross_submitRequest".to_string(),
		params: vec![request.to_hex()],
		id: Id::Number(1),
	}
}

#[cfg(feature = "std")]
fn random_aes_key() -> [u8; 32] {
	use rand::{thread_rng, RngCore};

	let mut seed = [0u8; 32];
	let mut rand = thread_rng();
	rand.fill_bytes(&mut seed);
	seed
}

#[cfg(feature = "sgx")]
fn random_aes_key() -> [u8; 32] {
	use sgx_rand::{Rng, StdRng};
	let mut seed = [0u8; 32];
	let mut rand = StdRng::new().unwrap();
	rand.fill_bytes(&mut seed);
	seed
}
