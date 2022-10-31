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

//! This is a mock to pallet-identity-management on parachain (IMP).
//! It hides/mocks all things happened on TEE side and returns the
//! result immediately.
//!
//! The idea is to give F/E an idea how the interface(extrinsic) would
//! look like and what kind of events can be expected.
//!
//! TODO: event/error handling
//! Currently the errors are synchronously emitted from this pallet itself,
//! meanwhile we have the `SomeError` **Event** which is callable from TEE
//! to represent any generic "error".
//! However, there are so many error cases in TEE that I'm not even sure
//! if it's a good idea to have a matching extrinsic for error propagation.

#![allow(dead_code)]
#![allow(clippy::needless_borrow)]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

use codec::alloc::string::ToString;
use frame_support::{pallet_prelude::*, traits::ConstU32};
use mock_tee_primitives::{
	Identity, IdentityHandle, IdentityMultiSignature, IdentityWebType, ValidationData,
	Web3CommonValidationData, Web3Network, Web3ValidationData,
};
pub use pallet::*;
use primitives::{ShardIdentifier, UserShieldingKeyType};
use sha2::Sha256;
use sp_core::{ed25519, sr25519};
use sp_io::{
	crypto::{
		ed25519_verify, secp256k1_ecdsa_recover, secp256k1_ecdsa_recover_compressed, sr25519_verify,
	},
	hashing::{blake2_128, blake2_256, keccak_256},
};
use sp_runtime::DispatchError;
use sp_std::prelude::*;

mod identity_context;
use identity_context::IdentityContext;

mod key;
use key::{aes_encrypt_default, get_mock_tee_shielding_key, AesOutput, PaddingScheme};

pub type ChallengeCode = [u8; 16]; // TODO: is 16 bytes enough?
pub(crate) type Metadata = BoundedVec<u8, ConstU32<2048>>;
pub(crate) type BlockNumberOf<T> = <T as frame_system::Config>::BlockNumber;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_system::pallet_prelude::*;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Event
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		/// origin to manage caller whitelist
		type ManageWhitelistOrigin: EnsureOrigin<Self::RuntimeOrigin>;
		// maximum delay in block numbers between linking an identity and verifying an identity
		#[pallet::constant]
		type MaxVerificationDelay: Get<BlockNumberOf<Self>>;
		// some extrinsics should only be called by origins from TEE
		type TEECallOrigin: EnsureOrigin<Self::RuntimeOrigin>;
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		// Events from this pallet
		LinkIdentityRequested {
			shard: ShardIdentifier,
		},
		UnlinkIdentityRequested {
			shard: ShardIdentifier,
		},
		VerifyIdentityRequested {
			shard: ShardIdentifier,
		},
		SetUserShieldingKeyRequested {
			shard: ShardIdentifier,
		},
		// =============================================
		// Mocked events that should have come from TEE
		// we have both the "plain" version and "encrypted" version for debugging
		// =============================================
		// set user's shielding key
		UserShieldingKeySetPlain {
			account: T::AccountId,
		},
		UserShieldingKeySet {
			account: AesOutput,
		},
		// link identity
		ChallengeCodeGeneratedPlain {
			account: T::AccountId,
			identity: Identity,
			code: ChallengeCode,
		},
		ChallengeCodeGenerated {
			account: AesOutput,
			identity: AesOutput,
			code: AesOutput,
		},
		IdentityLinkedPlain {
			account: T::AccountId,
			identity: Identity,
		},
		IdentityLinked {
			account: AesOutput,
			identity: AesOutput,
		},
		// unlink identity
		IdentityUnlinkedPlain {
			account: T::AccountId,
			identity: Identity,
		},
		IdentityUnlinked {
			account: AesOutput,
			identity: AesOutput,
		},
		// verify identity
		IdentityVerifiedPlain {
			account: T::AccountId,
			identity: Identity,
		},
		IdentityVerified {
			account: AesOutput,
			identity: AesOutput,
		},
		// some error happened during processing in TEE, we use string-like
		// parameters for more "generic" error event reporting
		// TODO: maybe use concrete errors instead of events when we are more sure
		// see also the comment at the beginning
		SomeError {
			func: Vec<u8>,
			error: Vec<u8>,
		},
	}

	/// These are the errors that are immediately emitted from this mock pallet
	#[pallet::error]
	pub enum Error<T> {
		/// caller is not in whitelist (therefore disallowed to call some extrinsics)
		CallerNotWhitelisted,
		/// Error when decrypting using TEE'shielding key
		ShieldingKeyDecryptionFailed,
		/// unexpected decoded type
		WrongDecodedType,
		/// identity already exists when linking an identity
		IdentityAlreadyExist,
		/// identity not exist when unlinking an identity
		IdentityNotExist,
		/// no shielding key for a given AccountId
		ShieldingKeyNotExist,
		/// a verification reqeust comes too early
		VerificationRequestTooEarly,
		/// a verification reqeust comes too late
		VerificationRequestTooLate,
		/// verify substrate signature failed
		VerifySubstrateSignatureFailed,
		/// recover substrate pubkey failed using ecdsa
		RecoverSubstratePubkeyFailed,
		/// verify evm signature failed
		VerifyEvmSignatureFailed,
		/// the linking request block is zero
		LinkingRequestBlockZero,
		/// the challenge code doesn't exist
		ChallengeCodeNotExist,
		/// wrong signature type
		WrongSignatureType,
		/// wrong web3 network type
		WrongWeb3NetworkType,
		/// wrong identity handle type
		WrongIdentityHanldeType,
		/// fail to recover evm address
		RecoverEvmAddressFailed,
		/// the message in validation data is unexpected
		UnexpectedMessage,
	}

	#[pallet::storage]
	#[pallet::getter(fn whitelisted_callers)]
	pub type WhitelistedCallers<T: Config> =
		StorageMap<_, Twox64Concat, T::AccountId, (), OptionQuery>;

	/// user shielding key is per Litentry account
	#[pallet::storage]
	#[pallet::getter(fn user_shielding_keys)]
	pub type UserShieldingKeys<T: Config> =
		StorageMap<_, Blake2_128Concat, T::AccountId, UserShieldingKeyType, OptionQuery>;

	/// challenge code is per Litentry account + identity
	#[pallet::storage]
	#[pallet::getter(fn challenge_codes)]
	pub type ChallengeCodes<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		T::AccountId,
		Blake2_128Concat,
		Identity,
		ChallengeCode,
		OptionQuery,
	>;

	/// ID graph is per Litentry account + identity
	#[pallet::storage]
	#[pallet::getter(fn id_graphs)]
	pub type IDGraphs<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		T::AccountId,
		Blake2_128Concat,
		Identity,
		IdentityContext<T>,
		OptionQuery,
	>;

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// add an account to the whitelist
		#[pallet::weight(195_000_000)]
		pub fn add_to_whitelist(origin: OriginFor<T>, account: T::AccountId) -> DispatchResult {
			let _ = T::ManageWhitelistOrigin::ensure_origin(origin)?;
			WhitelistedCallers::<T>::insert(account, ());
			Ok(())
		}

		/// remove an account from the whitelist
		#[pallet::weight(195_000_000)]
		pub fn remove_from_whitelist(
			origin: OriginFor<T>,
			account: T::AccountId,
		) -> DispatchResult {
			let _ = T::ManageWhitelistOrigin::ensure_origin(origin)?;
			WhitelistedCallers::<T>::remove(account);
			Ok(())
		}

		/// Set or update user's shielding key
		#[pallet::weight(195_000_000)]
		pub fn set_user_shielding_key(
			origin: OriginFor<T>,
			shard: ShardIdentifier,
			encrypted_key: Vec<u8>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			ensure!(WhitelistedCallers::<T>::contains_key(&who), Error::<T>::CallerNotWhitelisted);
			Self::deposit_event(Event::SetUserShieldingKeyRequested { shard });

			let decrypted_key = Self::decrypt_with_tee_shielding_key(&encrypted_key)?;
			let key = UserShieldingKeyType::decode(&mut decrypted_key.as_slice())
				.map_err(|_| Error::<T>::WrongDecodedType)?;
			UserShieldingKeys::<T>::insert(&who, &key);
			Self::deposit_event(Event::<T>::UserShieldingKeySetPlain { account: who.clone() });
			Self::deposit_event(Event::<T>::UserShieldingKeySet {
				account: aes_encrypt_default(&key, who.encode().as_slice()),
			});
			Ok(())
		}

		/// Link an identity
		#[pallet::weight(195_000_000)]
		pub fn link_identity(
			origin: OriginFor<T>,
			shard: ShardIdentifier,
			encrypted_identity: Vec<u8>,
			encrypted_metadata: Option<Vec<u8>>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			ensure!(WhitelistedCallers::<T>::contains_key(&who), Error::<T>::CallerNotWhitelisted);
			Self::deposit_event(Event::LinkIdentityRequested { shard });

			let decrypted_identitty = Self::decrypt_with_tee_shielding_key(&encrypted_identity)?;
			let identity = Identity::decode(&mut decrypted_identitty.as_slice())
				.map_err(|_| Error::<T>::WrongDecodedType)?;

			let metadata = match encrypted_metadata {
				None => None,
				Some(m) => {
					let decrypted_metadata = Self::decrypt_with_tee_shielding_key(&m)?;
					Some(
						Metadata::decode(&mut decrypted_metadata.as_slice())
							.map_err(|_| Error::<T>::WrongDecodedType)?,
					)
				},
			};

			ensure!(
				!IDGraphs::<T>::contains_key(&who, &identity),
				Error::<T>::IdentityAlreadyExist
			);
			let key = UserShieldingKeys::<T>::get(&who).ok_or(Error::<T>::ShieldingKeyNotExist)?;

			// emit the challenge code event, TODO: use randomness pallet
			let code = Self::get_mock_challenge_code();
			ChallengeCodes::<T>::insert(&who, &identity, &code);
			Self::deposit_event(Event::<T>::ChallengeCodeGeneratedPlain {
				account: who.clone(),
				identity: identity.clone(),
				code,
			});
			Self::deposit_event(Event::<T>::ChallengeCodeGenerated {
				account: aes_encrypt_default(&key, who.encode().as_slice()),
				identity: aes_encrypt_default(&key, identity.encode().as_slice()),
				code: aes_encrypt_default(&key, code.as_ref()),
			});

			// emit the IdentityLinked event
			let context = IdentityContext {
				metadata,
				linking_request_block: Some(<frame_system::Pallet<T>>::block_number()),
				verification_request_block: None,
				is_verified: false,
			};
			IDGraphs::<T>::insert(&who, &identity, context);
			Self::deposit_event(Event::<T>::IdentityLinkedPlain {
				account: who.clone(),
				identity: identity.clone(),
			});
			Self::deposit_event(Event::<T>::IdentityLinked {
				account: aes_encrypt_default(&key, who.encode().as_slice()),
				identity: aes_encrypt_default(&key, identity.encode().as_slice()),
			});
			Ok(())
		}

		/// Unlink an identity
		#[pallet::weight(195_000_000)]
		pub fn unlink_identity(
			origin: OriginFor<T>,
			shard: ShardIdentifier,
			encrypted_identity: Vec<u8>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			ensure!(WhitelistedCallers::<T>::contains_key(&who), Error::<T>::CallerNotWhitelisted);
			Self::deposit_event(Event::UnlinkIdentityRequested { shard });

			let decrypted_identitty = Self::decrypt_with_tee_shielding_key(&encrypted_identity)?;
			let identity = Identity::decode(&mut decrypted_identitty.as_slice())
				.map_err(|_| Error::<T>::WrongDecodedType)?;

			ensure!(IDGraphs::<T>::contains_key(&who, &identity), Error::<T>::IdentityNotExist);
			let key = UserShieldingKeys::<T>::get(&who).ok_or(Error::<T>::ShieldingKeyNotExist)?;

			// emit the IdentityUnlinked event
			IDGraphs::<T>::remove(&who, &identity);
			Self::deposit_event(Event::<T>::IdentityUnlinkedPlain {
				account: who.clone(),
				identity: identity.clone(),
			});
			Self::deposit_event(Event::<T>::IdentityUnlinked {
				account: aes_encrypt_default(&key, who.encode().as_slice()),
				identity: aes_encrypt_default(&key, identity.encode().as_slice()),
			});

			Ok(())
		}

		/// Verify a linked identity
		#[pallet::weight(195_000_000)]
		pub fn verify_identity(
			origin: OriginFor<T>,
			shard: ShardIdentifier,
			encrypted_identity: Vec<u8>,
			encrypted_validation_data: Vec<u8>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			ensure!(WhitelistedCallers::<T>::contains_key(&who), Error::<T>::CallerNotWhitelisted);
			Self::deposit_event(Event::VerifyIdentityRequested { shard });

			let now = <frame_system::Pallet<T>>::block_number();
			let decrypted_identitty = Self::decrypt_with_tee_shielding_key(&encrypted_identity)?;
			let identity = Identity::decode(&mut decrypted_identitty.as_slice())
				.map_err(|_| Error::<T>::WrongDecodedType)?;
			ensure!(IDGraphs::<T>::contains_key(&who, &identity), Error::<T>::IdentityNotExist);

			let decrypted_validation_data =
				Self::decrypt_with_tee_shielding_key(&encrypted_validation_data)?;
			let validation_data = ValidationData::decode(&mut decrypted_validation_data.as_slice())
				.map_err(|_| Error::<T>::WrongDecodedType)?;

			// Web3 signature verification, we can't do web2 here as it requires offchain operations
			if let ValidationData::Web3(web3_validation_data) = validation_data {
				match web3_validation_data {
					Web3ValidationData::Substrate(substrate_validation_data) => {
						Self::verify_substrate_signature(
							&who,
							&identity,
							&substrate_validation_data,
						)?;
					},
					Web3ValidationData::Evm(evm_validation_data) => {
						Self::verify_evm_signature(&who, &identity, &evm_validation_data)?;
					},
				}
			}

			let key = UserShieldingKeys::<T>::get(&who).ok_or(Error::<T>::ShieldingKeyNotExist)?;

			IDGraphs::<T>::try_mutate(&who, &identity, |context| -> DispatchResult {
				let mut c = context.take().ok_or(Error::<T>::IdentityNotExist)?;
				let linking_request_block =
					c.linking_request_block.ok_or(Error::<T>::LinkingRequestBlockZero)?;
				ensure!(linking_request_block <= now, Error::<T>::VerificationRequestTooEarly);
				ensure!(
					now - linking_request_block <= T::MaxVerificationDelay::get(),
					Error::<T>::VerificationRequestTooLate
				);
				c.is_verified = true;
				c.verification_request_block = Some(now);

				*context = Some(c);

				// remove challenge code
				ensure!(
					Self::challenge_codes(&who, &identity).is_some(),
					Error::<T>::ChallengeCodeNotExist
				);
				ChallengeCodes::<T>::remove(&who, &identity);

				// emit the IdentityVerified event
				Self::deposit_event(Event::<T>::IdentityVerifiedPlain {
					account: who.clone(),
					identity: identity.clone(),
				});
				Self::deposit_event(Event::<T>::IdentityVerified {
					account: aes_encrypt_default(&key, who.encode().as_slice()),
					identity: aes_encrypt_default(&key, identity.encode().as_slice()),
				});
				Ok(())
			})
		}

		// The following extrinsics are supposed to be called by TEE only
		#[pallet::weight(195_000_000)]
		pub fn user_shielding_key_set(
			origin: OriginFor<T>,
			account: AesOutput,
		) -> DispatchResultWithPostInfo {
			let _ = T::TEECallOrigin::ensure_origin(origin)?;
			Self::deposit_event(Event::UserShieldingKeySet { account });
			Ok(Pays::No.into())
		}

		#[pallet::weight(195_000_000)]
		pub fn challenge_code_generated(
			origin: OriginFor<T>,
			account: AesOutput,
			identity: AesOutput,
			code: AesOutput,
		) -> DispatchResultWithPostInfo {
			let _ = T::TEECallOrigin::ensure_origin(origin)?;
			Self::deposit_event(Event::ChallengeCodeGenerated { account, identity, code });
			Ok(Pays::No.into())
		}

		#[pallet::weight(195_000_000)]
		pub fn identity_linked(
			origin: OriginFor<T>,
			account: AesOutput,
			identity: AesOutput,
		) -> DispatchResultWithPostInfo {
			let _ = T::TEECallOrigin::ensure_origin(origin)?;
			Self::deposit_event(Event::IdentityLinked { account, identity });
			Ok(Pays::No.into())
		}

		#[pallet::weight(195_000_000)]
		pub fn identity_unlinked(
			origin: OriginFor<T>,
			account: AesOutput,
			identity: AesOutput,
		) -> DispatchResultWithPostInfo {
			let _ = T::TEECallOrigin::ensure_origin(origin)?;
			Self::deposit_event(Event::IdentityUnlinked { account, identity });
			Ok(Pays::No.into())
		}

		#[pallet::weight(195_000_000)]
		pub fn identity_verified(
			origin: OriginFor<T>,
			account: AesOutput,
			identity: AesOutput,
		) -> DispatchResultWithPostInfo {
			let _ = T::TEECallOrigin::ensure_origin(origin)?;
			Self::deposit_event(Event::IdentityVerified { account, identity });
			Ok(Pays::No.into())
		}

		#[pallet::weight(195_000_000)]
		pub fn some_error(
			origin: OriginFor<T>,
			func: Vec<u8>,
			error: Vec<u8>,
		) -> DispatchResultWithPostInfo {
			let _ = T::TEECallOrigin::ensure_origin(origin)?;
			Self::deposit_event(Event::SomeError { func, error });
			Ok(Pays::No.into())
		}
	}

	impl<T: Config> Pallet<T> {
		fn decrypt_with_tee_shielding_key(encrypted_data: &[u8]) -> Result<Vec<u8>, DispatchError> {
			let (_, private_key) = get_mock_tee_shielding_key();
			let decrypted_data = private_key
				.decrypt(PaddingScheme::new_oaep::<Sha256>(), encrypted_data)
				.map_err(|_| Error::<T>::ShieldingKeyDecryptionFailed)?;
			Ok(decrypted_data)
		}

		// TODO: maybe use randomness pallet
		fn get_mock_challenge_code() -> ChallengeCode {
			let now = <frame_system::Pallet<T>>::block_number();
			blake2_128(&now.encode())
		}

		fn verify_substrate_signature(
			who: &T::AccountId,
			identity: &Identity,
			validation_data: &Web3CommonValidationData,
		) -> DispatchResult {
			let code =
				Self::challenge_codes(who, identity).ok_or(Error::<T>::ChallengeCodeNotExist)?;
			let msg = Self::get_expected_payload(who, identity, &code)?;

			ensure!(
				msg.as_slice() == validation_data.message.as_slice(),
				Error::<T>::UnexpectedMessage
			);

			let substrate_address = match &identity.web_type {
				IdentityWebType::Web3(Web3Network::Substrate(_)) => match &identity.handle {
					IdentityHandle::Address32(handle) => handle,
					_ => return Err(Error::<T>::WrongIdentityHanldeType.into()),
				},
				_ => return Err(Error::<T>::WrongWeb3NetworkType.into()),
			};

			match &validation_data.signature {
				IdentityMultiSignature::Sr25519(sig) => {
					ensure!(
						sr25519_verify(sig, &msg, &sr25519::Public(*substrate_address)),
						Error::<T>::VerifySubstrateSignatureFailed
					);
				},
				IdentityMultiSignature::Ed25519(sig) => {
					ensure!(
						ed25519_verify(sig, &msg, &ed25519::Public(*substrate_address)),
						Error::<T>::VerifySubstrateSignatureFailed
					);
				},
				// We can' use `ecdsa_verify` directly we don't have the raw 33-bytes publick key
				// instead we only have AccountId which is blake2_256(pubkey)
				IdentityMultiSignature::Ecdsa(sig) => {
					// see https://github.com/paritytech/substrate/blob/493b58bd4a475080d428ce47193ee9ea9757a808/primitives/runtime/src/traits.rs#L132
					let digest = blake2_256(&msg);
					let recovered_substrate_pubkey =
						secp256k1_ecdsa_recover_compressed(&sig.0, &digest)
							.map_err(|_| Error::<T>::RecoverSubstratePubkeyFailed)?;
					ensure!(
						&blake2_256(&recovered_substrate_pubkey) == substrate_address,
						Error::<T>::VerifySubstrateSignatureFailed
					);
				},
				_ => return Err(Error::<T>::WrongSignatureType.into()),
			}
			Ok(())
		}

		fn verify_evm_signature(
			who: &T::AccountId,
			identity: &Identity,
			validation_data: &Web3CommonValidationData,
		) -> DispatchResult {
			let code =
				Self::challenge_codes(who, identity).ok_or(Error::<T>::ChallengeCodeNotExist)?;
			let msg = Self::get_expected_payload(who, identity, &code)?;
			let digest = Self::compute_evm_msg_digest(&msg);
			if let IdentityMultiSignature::Ethereum(sig) = &validation_data.signature {
				let recovered_evm_address = Self::recover_evm_address(&digest, sig.as_ref())
					.map_err(|_| Error::<T>::RecoverEvmAddressFailed)?;
				let evm_address = match &identity.web_type {
					IdentityWebType::Web3(Web3Network::Evm(_)) => match &identity.handle {
						IdentityHandle::Address20(handle) => handle,
						_ => return Err(Error::<T>::WrongIdentityHanldeType.into()),
					},
					_ => return Err(Error::<T>::WrongWeb3NetworkType.into()),
				};
				ensure!(
					&recovered_evm_address == evm_address,
					Error::<T>::VerifyEvmSignatureFailed
				);
			} else {
				return Err(Error::<T>::WrongSignatureType.into())
			}
			Ok(())
		}

		// Payload format: blake2_256(<challeng-code> + <litentry-AccountId32> + <Identity>), where
		// <> means SCALE-encoded. It applies to both web2 and web3 message
		pub fn get_expected_payload(
			who: &T::AccountId,
			identity: &Identity,
			code: &ChallengeCode,
		) -> Result<Vec<u8>, DispatchError> {
			let mut msg = code.encode();
			msg.append(&mut who.encode());
			msg.append(&mut identity.encode());
			Ok(blake2_256(&msg).to_vec())
		}

		// we use an EIP-191 message has computing
		pub fn compute_evm_msg_digest(message: &[u8]) -> [u8; 32] {
			let eip_191_message = [
				"\x19Ethereum Signed Message:\n".as_bytes(),
				message.len().to_string().as_bytes(),
				message,
			]
			.concat();
			keccak_256(&eip_191_message)
		}

		fn recover_evm_address(
			msg: &[u8; 32],
			sig: &[u8; 65],
		) -> Result<[u8; 20], sp_io::EcdsaVerifyError> {
			let pubkey = secp256k1_ecdsa_recover(sig, msg)?;
			let hashed_pk = keccak_256(&pubkey);

			let mut addr = [0u8; 20];
			addr[..20].copy_from_slice(&hashed_pk[12..32]);
			Ok(addr)
		}
	}
}
