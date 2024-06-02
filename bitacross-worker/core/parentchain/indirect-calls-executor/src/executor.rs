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
//! Execute indirect calls, i.e. extrinsics extracted from parentchain blocks

#[cfg(all(not(feature = "std"), feature = "sgx"))]
use crate::sgx_reexport_prelude::*;

use crate::{
	error::{Error, Result},
	filter_metadata::{EventsFromMetadata, FilterIntoDataFrom},
	traits::{ExecuteIndirectCalls, IndirectDispatch},
};
use alloc::format;
use bc_relayer_registry::RelayerRegistryUpdater;
use bc_signer_registry::SignerRegistryUpdater;
use binary_merkle_tree::merkle_root;
use codec::{Decode, Encode};
use core::marker::PhantomData;
use itp_node_api::metadata::{
	pallet_teebag::TeebagCallIndexes, provider::AccessNodeMetadata, NodeMetadataTrait,
};
use itp_sgx_crypto::{key_repository::AccessKey, ShieldingCryptoDecrypt, ShieldingCryptoEncrypt};
use itp_stf_executor::traits::{StfEnclaveSigning, StfShardVaultQuery};
use itp_stf_primitives::{
	traits::{IndirectExecutor, TrustedCallSigning, TrustedCallVerification},
	types::AccountId,
};
use itp_top_pool_author::traits::AuthorApi;
use itp_types::{
	parentchain::{ExtrinsicStatus, FilterEvents, HandleParentchainEvents, ParentchainId},
	OpaqueCall, RsaRequest, ShardIdentifier, H256,
};
use log::*;
use sp_core::blake2_256;
use sp_runtime::traits::{Block as ParentchainBlockTrait, Header, Keccak256};
use std::{fmt::Debug, sync::Arc, vec::Vec};

pub struct IndirectCallsExecutor<
	ShieldingKeyRepository,
	StfEnclaveSigner,
	TopPoolAuthor,
	NodeMetadataProvider,
	IndirectCallsFilter,
	EventCreator,
	ParentchainEventHandler,
	TCS,
	G,
	RRU,
	SRU,
> where
	RRU: RelayerRegistryUpdater,
	SRU: SignerRegistryUpdater,
{
	pub(crate) shielding_key_repo: Arc<ShieldingKeyRepository>,
	pub stf_enclave_signer: Arc<StfEnclaveSigner>,
	pub(crate) top_pool_author: Arc<TopPoolAuthor>,
	pub(crate) node_meta_data_provider: Arc<NodeMetadataProvider>,
	pub parentchain_id: ParentchainId,
	pub relayer_registry_updater: Arc<RRU>,
	pub signer_registry_updater: Arc<SRU>,
	_phantom: PhantomData<(IndirectCallsFilter, EventCreator, ParentchainEventHandler, TCS, G)>,
}
impl<
		ShieldingKeyRepository,
		StfEnclaveSigner,
		TopPoolAuthor,
		NodeMetadataProvider,
		IndirectCallsFilter,
		EventCreator,
		ParentchainEventHandler,
		TCS,
		G,
		RRU,
		SRU,
	>
	IndirectCallsExecutor<
		ShieldingKeyRepository,
		StfEnclaveSigner,
		TopPoolAuthor,
		NodeMetadataProvider,
		IndirectCallsFilter,
		EventCreator,
		ParentchainEventHandler,
		TCS,
		G,
		RRU,
		SRU,
	> where
	RRU: RelayerRegistryUpdater,
	SRU: SignerRegistryUpdater,
{
	pub fn new(
		shielding_key_repo: Arc<ShieldingKeyRepository>,
		stf_enclave_signer: Arc<StfEnclaveSigner>,
		top_pool_author: Arc<TopPoolAuthor>,
		node_meta_data_provider: Arc<NodeMetadataProvider>,
		parentchain_id: ParentchainId,
		relayer_registry_updater: Arc<RRU>,
		signer_registry_updater: Arc<SRU>,
	) -> Self {
		IndirectCallsExecutor {
			shielding_key_repo,
			stf_enclave_signer,
			top_pool_author,
			node_meta_data_provider,
			parentchain_id,
			relayer_registry_updater,
			signer_registry_updater,
			_phantom: Default::default(),
		}
	}
}

impl<
		ShieldingKeyRepository,
		StfEnclaveSigner,
		TopPoolAuthor,
		NodeMetadataProvider,
		FilterIndirectCalls,
		EventCreator,
		ParentchainEventHandler,
		TCS,
		G,
		RRU,
		SRU,
	> ExecuteIndirectCalls
	for IndirectCallsExecutor<
		ShieldingKeyRepository,
		StfEnclaveSigner,
		TopPoolAuthor,
		NodeMetadataProvider,
		FilterIndirectCalls,
		EventCreator,
		ParentchainEventHandler,
		TCS,
		G,
		RRU,
		SRU,
	> where
	ShieldingKeyRepository: AccessKey,
	<ShieldingKeyRepository as AccessKey>::KeyType: ShieldingCryptoDecrypt<Error = itp_sgx_crypto::Error>
		+ ShieldingCryptoEncrypt<Error = itp_sgx_crypto::Error>,
	StfEnclaveSigner: StfEnclaveSigning<TCS> + StfShardVaultQuery,
	TopPoolAuthor: AuthorApi<H256, H256, TCS, G> + Send + Sync + 'static,
	NodeMetadataProvider: AccessNodeMetadata,
	FilterIndirectCalls: FilterIntoDataFrom<NodeMetadataProvider::MetadataType>,
	NodeMetadataProvider::MetadataType: NodeMetadataTrait + Clone,
	FilterIndirectCalls::Output: IndirectDispatch<Self, TCS, RRU, SRU, Args = ()> + Encode + Debug,
	EventCreator: EventsFromMetadata<NodeMetadataProvider::MetadataType>,
	ParentchainEventHandler: HandleParentchainEvents<Self, TCS, Error, RRU, SRU>,
	TCS: PartialEq + Encode + Decode + Debug + Clone + Send + Sync + TrustedCallVerification,
	G: PartialEq + Encode + Decode + Debug + Clone + Send + Sync,
	RRU: RelayerRegistryUpdater,
	SRU: SignerRegistryUpdater,
{
	fn execute_indirect_calls_in_extrinsics<ParentchainBlock>(
		&self,
		block: &ParentchainBlock,
		events: &[u8],
	) -> Result<Option<OpaqueCall>>
	where
		ParentchainBlock: ParentchainBlockTrait<Hash = H256>,
	{
		let block_number = *block.header().number();
		let block_hash = block.hash();

		trace!("Scanning block {:?} for relevant xt", block_number);
		let mut executed_calls = Vec::<H256>::new();

		let events = self
			.node_meta_data_provider
			.get_from_metadata(|metadata| {
				EventCreator::create_from_metadata(metadata.clone(), block_hash, events)
			})?
			.ok_or_else(|| Error::Other("Could not create events from metadata".into()))?;

		let xt_statuses = events.get_extrinsic_statuses().map_err(|e| {
			Error::Other(format!("Error when shielding for privacy sidechain {:?}", e).into())
		})?;
		trace!("xt_statuses:: {:?}", xt_statuses);

		let shard = self.get_default_shard();
		if let Ok((vault, _parentchain_id)) = self.stf_enclave_signer.get_shard_vault(&shard) {
			ParentchainEventHandler::handle_events(self, events, &vault)?;
		}

		// This would be catastrophic but should never happen
		if xt_statuses.len() != block.extrinsics().len() {
			return Err(Error::Other("Extrinsic Status and Extrinsic count not equal".into()))
		}

		for (xt_opaque, xt_status) in block.extrinsics().iter().zip(xt_statuses.iter()) {
			let encoded_xt_opaque = xt_opaque.encode();

			let maybe_call = self.node_meta_data_provider.get_from_metadata(|metadata| {
				FilterIndirectCalls::filter_into_from_metadata(&encoded_xt_opaque, metadata)
			})?;

			let call = match maybe_call {
				Some(c) => c,
				None => continue,
			};

			if let ExtrinsicStatus::Failed = xt_status {
				warn!("Parentchain Extrinsic Failed, {:?} wont be dispatched", call);
				continue
			}

			if let Err(e) = call.dispatch(self, ()) {
				warn!("Error executing the indirect call: {:?}. Error {:?}", call, e);
			} else {
				executed_calls.push(hash_of(&call));
			}
		}
		debug!("successfully processed {} indirect invocations", executed_calls.len());
		if self.parentchain_id == ParentchainId::Litentry {
			// Include a processed parentchain block confirmation for each block.
			Ok(Some(self.create_processed_parentchain_block_call::<ParentchainBlock>(
				block_hash,
				executed_calls,
				block_number,
			)?))
		} else {
			// fixme: send other type of confirmation here:  https://github.com/integritee-network/worker/issues/1567
			Ok(None)
		}
	}

	fn create_processed_parentchain_block_call<ParentchainBlock>(
		&self,
		block_hash: H256,
		extrinsics: Vec<H256>,
		block_number: <<ParentchainBlock as ParentchainBlockTrait>::Header as Header>::Number,
	) -> Result<OpaqueCall>
	where
		ParentchainBlock: ParentchainBlockTrait<Hash = H256>,
	{
		let call = self.node_meta_data_provider.get_from_metadata(|meta_data| {
			meta_data.parentchain_block_processed_call_indexes()
		})??;
		let root: H256 = merkle_root::<Keccak256, _>(extrinsics);
		trace!("prepared parentchain_block_processed() call for block {:?} with index {:?} and merkle root {}", block_number, call, root);
		// Litentry: we don't include `shard` in the extrinsic parameter to be backwards compatible,
		//           however, we should not forget it in case we need it later
		Ok(OpaqueCall::from_tuple(&(call, block_hash, block_number, root)))
	}
}

impl<
		ShieldingKeyRepository,
		StfEnclaveSigner,
		TopPoolAuthor,
		NodeMetadataProvider,
		FilterIndirectCalls,
		EventFilter,
		PrivacySidechain,
		TCS,
		G,
		RRU,
		SRU,
	> IndirectExecutor<TCS, Error, RRU, SRU>
	for IndirectCallsExecutor<
		ShieldingKeyRepository,
		StfEnclaveSigner,
		TopPoolAuthor,
		NodeMetadataProvider,
		FilterIndirectCalls,
		EventFilter,
		PrivacySidechain,
		TCS,
		G,
		RRU,
		SRU,
	> where
	ShieldingKeyRepository: AccessKey,
	<ShieldingKeyRepository as AccessKey>::KeyType: ShieldingCryptoDecrypt<Error = itp_sgx_crypto::Error>
		+ ShieldingCryptoEncrypt<Error = itp_sgx_crypto::Error>,
	StfEnclaveSigner: StfEnclaveSigning<TCS> + StfShardVaultQuery,
	TopPoolAuthor: AuthorApi<H256, H256, TCS, G> + Send + Sync + 'static,
	TCS: PartialEq + Encode + Decode + Debug + Clone + Send + Sync + TrustedCallVerification,
	G: PartialEq + Encode + Decode + Debug + Clone + Send + Sync,
	RRU: RelayerRegistryUpdater,
	SRU: SignerRegistryUpdater,
{
	fn submit_trusted_call(&self, shard: ShardIdentifier, encrypted_trusted_call: Vec<u8>) {
		if let Err(e) = futures::executor::block_on(
			self.top_pool_author.submit_top(RsaRequest::new(shard, encrypted_trusted_call)),
		) {
			error!("Error adding indirect trusted call to TOP pool: {:?}", e);
		}
	}

	fn decrypt(&self, encrypted: &[u8]) -> Result<Vec<u8>> {
		let key = self.shielding_key_repo.retrieve_key()?;
		Ok(key.decrypt(encrypted)?)
	}

	fn encrypt(&self, value: &[u8]) -> Result<Vec<u8>> {
		let key = self.shielding_key_repo.retrieve_key()?;
		Ok(key.encrypt(value)?)
	}

	fn get_enclave_account(&self) -> Result<AccountId> {
		Ok(self.stf_enclave_signer.get_enclave_account()?)
	}

	fn get_default_shard(&self) -> ShardIdentifier {
		self.top_pool_author.list_handled_shards().first().copied().unwrap_or_default()
	}

	fn sign_call_with_self<TC: Encode + Debug + TrustedCallSigning<TCS>>(
		&self,
		trusted_call: &TC,
		shard: &ShardIdentifier,
	) -> Result<TCS> {
		Ok(self.stf_enclave_signer.sign_call_with_self(trusted_call, shard)?)
	}

	fn get_relayer_registry_updater(&self) -> &RRU {
		self.relayer_registry_updater.as_ref()
	}

	fn get_signer_registry_updater(&self) -> &SRU {
		self.signer_registry_updater.as_ref()
	}
}

pub fn hash_of<T: Encode>(xt: &T) -> H256 {
	blake2_256(&xt.encode()).into()
}

#[cfg(test)]
mod test {
	use super::*;
	use crate::mock::*;
	use bc_relayer_registry::RelayerRegistry;
	use bc_signer_registry::SignerRegistry;
	use codec::Encode;
	use itc_parentchain_test::ParentchainBlockBuilder;
	use itp_node_api::{
		api_client::{
			ExtrinsicParams, ParentchainAdditionalParams, ParentchainExtrinsicParams,
			ParentchainUncheckedExtrinsic,
		},
		metadata::{metadata_mocks::NodeMetadataMock, provider::NodeMetadataRepository},
	};
	use itp_sgx_crypto::mocks::KeyRepositoryMock;
	use itp_stf_executor::mocks::StfEnclaveSignerMock;
	use itp_test::mock::{
		shielding_crypto_mock::ShieldingCryptoMock,
		stf_mock::{GetterMock, TrustedCallSignedMock},
	};
	use itp_top_pool_author::mocks::AuthorApiMock;
	use itp_types::{Block, PostOpaqueTaskFn, RsaRequest, ShardIdentifier};
	use sp_core::{ed25519, Pair};
	use sp_runtime::{MultiAddress, MultiSignature, OpaqueExtrinsic};

	type TestShieldingKeyRepo = KeyRepositoryMock<ShieldingCryptoMock>;
	type TestStfEnclaveSigner = StfEnclaveSignerMock;
	type TestTopPoolAuthor = AuthorApiMock<H256, H256, TrustedCallSignedMock, GetterMock>;
	type TestNodeMetadataRepository = NodeMetadataRepository<NodeMetadataMock>;
	type TestIndirectCallExecutor = IndirectCallsExecutor<
		TestShieldingKeyRepo,
		TestStfEnclaveSigner,
		TestTopPoolAuthor,
		TestNodeMetadataRepository,
		MockExtrinsicFilter<MockParentchainExtrinsicParser>,
		TestEventCreator,
		MockParentchainEventHandler,
		TrustedCallSignedMock,
		GetterMock,
		RelayerRegistry,
		SignerRegistry,
	>;

	type Seed = [u8; 32];

	const TEST_SEED: Seed = *b"12345678901234567890123456789012";

	#[test]
	fn indirect_call_can_be_added_to_pool_successfully() {
		let _ = env_logger::builder().is_test(true).try_init();

		let (indirect_calls_executor, top_pool_author, _) =
			test_fixtures([0u8; 32], NodeMetadataMock::new());

		let opaque_extrinsic =
			OpaqueExtrinsic::from_bytes(invoke_unchecked_extrinsic().encode().as_slice()).unwrap();

		let parentchain_block = ParentchainBlockBuilder::default()
			.with_extrinsics(vec![opaque_extrinsic])
			.build();

		indirect_calls_executor
			.execute_indirect_calls_in_extrinsics(&parentchain_block, &Vec::new())
			.unwrap();

		assert_eq!(1, top_pool_author.pending_tops(shard_id()).unwrap().len());
	}

	#[test]
	fn ensure_empty_extrinsic_vec_triggers_zero_filled_merkle_root() {
		// given
		let dummy_metadata = NodeMetadataMock::new();
		let (indirect_calls_executor, _, _) = test_fixtures([38u8; 32], dummy_metadata.clone());

		let block_hash = H256::from([1; 32]);
		let extrinsics = Vec::new();
		let parentchain_block_processed_call_indexes =
			dummy_metadata.parentchain_block_processed_call_indexes().unwrap();
		let expected_call =
			(parentchain_block_processed_call_indexes, block_hash, 1u32, H256::default()).encode();

		// when
		let call = indirect_calls_executor
			.create_processed_parentchain_block_call::<Block>(block_hash, extrinsics, 1u32)
			.unwrap();

		// then
		assert_eq!(call.0, expected_call);
	}

	#[test]
	fn ensure_non_empty_extrinsic_vec_triggers_non_zero_merkle_root() {
		// given
		let dummy_metadata = NodeMetadataMock::new();
		let (indirect_calls_executor, _, _) = test_fixtures([39u8; 32], dummy_metadata.clone());

		let block_hash = H256::from([1; 32]);
		let extrinsics = vec![H256::from([4; 32]), H256::from([9; 32])];
		let parentchain_block_processed_call_indexes =
			dummy_metadata.parentchain_block_processed_call_indexes().unwrap();

		let zero_root_call =
			(parentchain_block_processed_call_indexes, block_hash, 1u32, H256::default()).encode();

		// when
		let call = indirect_calls_executor
			.create_processed_parentchain_block_call::<Block>(block_hash, extrinsics, 1u32)
			.unwrap();

		// then
		assert_ne!(call.0, zero_root_call);
	}

	fn invoke_unchecked_extrinsic() -> ParentchainUncheckedExtrinsic<PostOpaqueTaskFn> {
		let request = RsaRequest::new(shard_id(), vec![1u8, 2u8]);
		let dummy_metadata = NodeMetadataMock::new();
		let call_worker_indexes = dummy_metadata.post_opaque_task_call_indexes().unwrap();

		ParentchainUncheckedExtrinsic::<PostOpaqueTaskFn>::new_signed(
			(call_worker_indexes, request),
			MultiAddress::Address32([1u8; 32]),
			MultiSignature::Ed25519(default_signature()),
			default_extrinsic_params().signed_extra(),
		)
	}

	fn default_signature() -> ed25519::Signature {
		signer().sign(&[0u8])
	}

	fn signer() -> ed25519::Pair {
		ed25519::Pair::from_seed(&TEST_SEED)
	}

	fn shard_id() -> ShardIdentifier {
		ShardIdentifier::default()
	}

	fn default_extrinsic_params() -> ParentchainExtrinsicParams {
		ParentchainExtrinsicParams::new(
			0,
			0,
			0,
			H256::default(),
			ParentchainAdditionalParams::default(),
		)
	}

	fn test_fixtures(
		mr_enclave: [u8; 32],
		metadata: NodeMetadataMock,
	) -> (TestIndirectCallExecutor, Arc<TestTopPoolAuthor>, Arc<TestShieldingKeyRepo>) {
		let shielding_key_repo = Arc::new(TestShieldingKeyRepo::default());
		let stf_enclave_signer = Arc::new(TestStfEnclaveSigner::new(mr_enclave));
		let top_pool_author = Arc::new(TestTopPoolAuthor::default());
		let node_metadata_repo = Arc::new(NodeMetadataRepository::new(metadata));
		let relayer_registry = Arc::new(RelayerRegistry::new(Default::default()));
		let signer_registry = Arc::new(SignerRegistry::new(Default::default()));

		let executor = IndirectCallsExecutor::new(
			shielding_key_repo.clone(),
			stf_enclave_signer,
			top_pool_author.clone(),
			node_metadata_repo,
			ParentchainId::Litentry,
			relayer_registry,
			signer_registry,
		);

		(executor, top_pool_author, shielding_key_repo)
	}
}
