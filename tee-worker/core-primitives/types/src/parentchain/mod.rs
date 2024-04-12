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

pub mod events;

use crate::OpaqueCall;
use alloc::vec::Vec;
use codec::{Decode, Encode};
use core::fmt::Debug;
use events::{
	ActivateIdentityRequested, DeactivateIdentityRequested, LinkIdentityRequested,
	OpaqueTaskPosted, ScheduledEnclaveRemoved, ScheduledEnclaveSet, VCRequested,
};
use itp_stf_primitives::traits::{IndirectExecutor, TrustedCallVerification};
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use sp_core::{bounded::alloc, H256};
use sp_runtime::{generic::Header as HeaderG, traits::BlakeTwo256, MultiAddress, MultiSignature};

pub type StorageProof = Vec<Vec<u8>>;

// Basic Types.
pub type Index = u32;
pub type Balance = u128;
pub type Hash = sp_core::H256;

// Account Types.
pub type AccountId = sp_core::crypto::AccountId32;
pub type AccountData = pallet_balances::AccountData<Balance>;
pub type AccountInfo = frame_system::AccountInfo<Index, AccountData>;
pub type Address = MultiAddress<AccountId, ()>;

// todo! make generic
/// The type used to represent the kinds of proxying allowed.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Encode, Decode, Debug)]
pub enum ProxyType {
	Any,
	NonTransfer,
	Governance,
	Staking,
}

// Block Types
pub type BlockNumber = u32;
pub type Header = HeaderG<BlockNumber, BlakeTwo256>;
pub type BlockHash = sp_core::H256;

/// Alias to 512-bit hash when used in the context of a transaction signature on the chain.
pub type Signature = MultiSignature;

#[derive(Encode, Decode, Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum ParentchainId {
	/// The Litentry Parentchain, the trust root of the enclave and serving finality to sidechains.
	#[codec(index = 0)]
	Litentry,
	/// A target chain containing custom business logic.
	#[codec(index = 1)]
	TargetA,
	/// Another target chain containing custom business logic.
	#[codec(index = 2)]
	TargetB,
}

#[cfg(feature = "std")]
impl std::fmt::Display for ParentchainId {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		let message = match self {
			ParentchainId::Litentry => "Litentry",
			ParentchainId::TargetA => "TargetA",
			ParentchainId::TargetB => "TargetB",
		};
		write!(f, "{}", message)
	}
}

pub trait IdentifyParentchain {
	fn parentchain_id(&self) -> ParentchainId;
}

pub trait FilterEvents {
	type Error: From<ParentchainEventProcessingError> + core::fmt::Debug;
	fn get_extrinsic_statuses(&self) -> core::result::Result<Vec<ExtrinsicStatus>, Self::Error>;

	fn get_link_identity_events(
		&self,
	) -> core::result::Result<Vec<LinkIdentityRequested>, Self::Error>;

	fn get_vc_requested_events(&self) -> core::result::Result<Vec<VCRequested>, Self::Error>;

	fn get_deactivate_identity_events(
		&self,
	) -> core::result::Result<Vec<DeactivateIdentityRequested>, Self::Error>;

	fn get_activate_identity_events(
		&self,
	) -> core::result::Result<Vec<ActivateIdentityRequested>, Self::Error>;

	fn get_scheduled_enclave_set_events(
		&self,
	) -> core::result::Result<Vec<ScheduledEnclaveSet>, Self::Error>;

	fn get_scheduled_enclave_removed_events(
		&self,
	) -> core::result::Result<Vec<ScheduledEnclaveRemoved>, Self::Error>;

	fn get_opaque_task_posted_events(
		&self,
	) -> core::result::Result<Vec<OpaqueTaskPosted>, Self::Error>;
}

#[derive(Debug)]
pub enum ExtrinsicStatus {
	Success,
	Failed,
}

pub trait HandleParentchainEvents<Executor, TCS, Error>
where
	Executor: IndirectExecutor<TCS, Error>,
	TCS: PartialEq + Encode + Decode + Debug + Clone + Send + Sync + TrustedCallVerification,
{
	fn handle_events(
		executor: &Executor,
		events: impl FilterEvents,
	) -> core::result::Result<Vec<H256>, Error>;
}

#[derive(Debug)]
pub enum ParentchainEventProcessingError {
	FunctionalityDisabled,
	LinkIdentityFailure,
	DeactivateIdentityFailure,
	ActivateIdentityFailure,
	VCRequestedFailure,
	ScheduledEnclaveSetFailure,
	ScheduledEnclaveRemovedFailure,
	OpaqueTaskPostedFailure,
}

impl core::fmt::Display for ParentchainEventProcessingError {
	fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
		let message = match &self {
			ParentchainEventProcessingError::FunctionalityDisabled =>
				"Parentchain Event Processing Error: FunctionalityDisabled",
			ParentchainEventProcessingError::LinkIdentityFailure =>
				"Parentchain Event Processing Error: LinkIdentityFailure",
			ParentchainEventProcessingError::DeactivateIdentityFailure =>
				"Parentchain Event Processing Error: DeactivateIdentityFailure",
			ParentchainEventProcessingError::ActivateIdentityFailure =>
				"Parentchain Event Processing Error: ActivateIdentityFailure",
			ParentchainEventProcessingError::VCRequestedFailure =>
				"Parentchain Event Processing Error: VCRequestedFailure",
			ParentchainEventProcessingError::ScheduledEnclaveSetFailure =>
				"Parentchain Event Processing Error: ScheduledEnclaveSetFailure",
			ParentchainEventProcessingError::ScheduledEnclaveRemovedFailure =>
				"Parentchain Event Processing Error: ScheduledEnclaveRemovedFailure",
			ParentchainEventProcessingError::OpaqueTaskPostedFailure =>
				"Parentchain Event Processing Error: OpaqueTaskPostedFailure",
		};
		write!(f, "{}", message)
	}
}

impl From<ParentchainEventProcessingError> for () {
	fn from(_: ParentchainEventProcessingError) -> Self {}
}

/// a wrapper to target calls to specific parentchains
#[derive(Encode, Debug, Clone, PartialEq, Eq)]
pub enum ParentchainCall {
	Litentry(OpaqueCall),
	TargetA(OpaqueCall),
	TargetB(OpaqueCall),
}

impl ParentchainCall {
	pub fn as_litentry(&self) -> Option<OpaqueCall> {
		if let Self::Litentry(call) = self {
			Some(call.clone())
		} else {
			None
		}
	}
	pub fn as_target_a(&self) -> Option<OpaqueCall> {
		if let Self::TargetA(call) = self {
			Some(call.clone())
		} else {
			None
		}
	}
	pub fn as_target_b(&self) -> Option<OpaqueCall> {
		if let Self::TargetB(call) = self {
			Some(call.clone())
		} else {
			None
		}
	}
	pub fn as_opaque_call_for(&self, parentchain_id: ParentchainId) -> Option<OpaqueCall> {
		match parentchain_id {
			ParentchainId::Litentry =>
				if let Self::Litentry(call) = self {
					Some(call.clone())
				} else {
					None
				},
			ParentchainId::TargetA =>
				if let Self::TargetA(call) = self {
					Some(call.clone())
				} else {
					None
				},
			ParentchainId::TargetB =>
				if let Self::TargetB(call) = self {
					Some(call.clone())
				} else {
					None
				},
		}
	}
}
