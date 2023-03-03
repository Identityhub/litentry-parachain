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

use crate::{error::Error, hash_of, ExecutionStatus, IndirectCallsExecutor};
use codec::{Decode, Encode, Error as CodecError};
use itp_node_api::{
	api_client::ParentchainUncheckedExtrinsic,
	metadata::{
		pallet_imp::IMPCallIndexes, pallet_teerex::TeerexCallIndexes, pallet_vcmp::VCMPCallIndexes,
		provider::AccessNodeMetadata, Error as MetadataError,
	},
};
use itp_types::{extrinsics::ParentchainUncheckedExtrinsicWithStatus, H256};

pub mod call_worker;
pub mod litentry;
pub mod shield_funds;

pub(crate) trait Executor<
	ShieldingKeyRepository,
	StfEnclaveSigner,
	TopPoolAuthor,
	NodeMetadataProvider,
> where
	NodeMetadataProvider: AccessNodeMetadata,
{
	type Call: Decode + Encode + Clone;

	fn call_index(&self, call: &Self::Call) -> [u8; 2];

	fn call_index_from_metadata(
		&self,
		metadata_type: &NodeMetadataProvider::MetadataType,
	) -> Result<[u8; 2], MetadataError>;

	fn is_target_call(&self, call: &Self::Call, node_metadata: &NodeMetadataProvider) -> bool {
		node_metadata
			.get_from_metadata(|m| match self.call_index_from_metadata(m) {
				Ok(call_index) => self.call_index(call) == call_index,
				Err(_e) => false,
			})
			.unwrap_or(false)
	}

	fn decode(
		&self,
		input: &mut &[u8],
	) -> Result<ParentchainUncheckedExtrinsicWithStatus<Self::Call>, CodecError> {
		ParentchainUncheckedExtrinsicWithStatus::<Self::Call>::decode(input)
	}

	/// extrinisc in this function should execute successfully on parentchain
	fn execute(
		&self,
		context: &IndirectCallsExecutor<
			ShieldingKeyRepository,
			StfEnclaveSigner,
			TopPoolAuthor,
			NodeMetadataProvider,
		>,
		extrinsic: ParentchainUncheckedExtrinsic<Self::Call>,
	) -> Result<(), Error>;
}

pub(crate) trait DecorateExecutor<
	ShieldingKeyRepository,
	StfEnclaveSigner,
	TopPoolAuthor,
	NodeMetadataProvider,
>
{
	fn decode_and_execute(
		&self,
		context: &IndirectCallsExecutor<
			ShieldingKeyRepository,
			StfEnclaveSigner,
			TopPoolAuthor,
			NodeMetadataProvider,
		>,
		input: &mut &[u8],
	) -> Result<ExecutionStatus<H256>, Error>;
}

impl<E, ShieldingKeyRepository, StfEnclaveSigner, TopPoolAuthor, NodeMetadataProvider>
	DecorateExecutor<ShieldingKeyRepository, StfEnclaveSigner, TopPoolAuthor, NodeMetadataProvider>
	for E
where
	E: Executor<ShieldingKeyRepository, StfEnclaveSigner, TopPoolAuthor, NodeMetadataProvider>,
	NodeMetadataProvider: AccessNodeMetadata,
	NodeMetadataProvider::MetadataType: IMPCallIndexes + TeerexCallIndexes + VCMPCallIndexes,
{
	fn decode_and_execute(
		&self,
		context: &IndirectCallsExecutor<
			ShieldingKeyRepository,
			StfEnclaveSigner,
			TopPoolAuthor,
			NodeMetadataProvider,
		>,
		input: &mut &[u8],
	) -> Result<ExecutionStatus<H256>, Error> {
		if let Ok(ParentchainUncheckedExtrinsicWithStatus { xt, status }) = self.decode(input) {
			if self.is_target_call(&xt.function, context.node_meta_data_provider.as_ref()) {
				if status {
					self.execute(context, xt.clone())
						.map(|_| ExecutionStatus::Success(hash_of(&xt)))
				} else {
					log::warn!(
						"extrinsic(call index: {:?}) fail to execute on parentchain.",
						self.call_index(&xt.function)
					);
					Ok(ExecutionStatus::Skip)
				}
			} else {
				Ok(ExecutionStatus::NextExecutor)
			}
		} else {
			Ok(ExecutionStatus::NextExecutor)
		}
	}
}
