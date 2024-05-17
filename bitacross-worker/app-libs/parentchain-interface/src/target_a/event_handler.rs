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

use codec::Encode;
pub use ita_sgx_runtime::{Balance, Index};

use ita_stf::{Getter, TrustedCall, TrustedCallSigned};
use itc_parentchain_indirect_calls_executor::error::Error;
use itp_stf_primitives::{traits::IndirectExecutor, types::TrustedOperation};
use itp_types::{
	parentchain::{
		AccountId, FilterEvents, HandleParentchainEvents, ParentchainEventProcessingError,
		ParentchainId,
	},
	H256,
};
use litentry_hex_utils::hex_encode;
use log::*;
use std::vec::Vec;

pub struct ParentchainEventHandler {}

impl<Executor> HandleParentchainEvents<Executor, TrustedCallSigned, Error>
	for ParentchainEventHandler
where
	Executor: IndirectExecutor<TrustedCallSigned, Error>,
{
	fn handle_events(
		_executor: &Executor,
		_events: impl FilterEvents,
		_vault_account: &AccountId,
	) -> Result<Vec<H256>, Error> {
		debug!("not handling any events for target a");
		Ok(Vec::new())
	}
}
