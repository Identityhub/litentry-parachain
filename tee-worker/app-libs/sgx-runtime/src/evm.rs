//! Adds the `pallet-evm` support for the `sgx-runtime.

// Import types from the crate root including the ones generated by the `construct_runtime!` macro.
use crate::{Balances, Runtime, RuntimeEvent, Timestamp, NORMAL_DISPATCH_RATIO};
use frame_support::{
	pallet_prelude::Weight, parameter_types, weights::constants::WEIGHT_REF_TIME_PER_SECOND,
};
use sp_core::{H160, U256};
use sp_runtime::traits::BlakeTwo256;

pub use pallet_evm::{
	AddressMapping, Call as EvmCall, EnsureAddressTruncated, FeeCalculator, GasWeightMapping,
	HashedAddressMapping as GenericHashedAddressMapping, SubstrateBlockHashMapping,
};

pub type HashedAddressMapping = GenericHashedAddressMapping<BlakeTwo256>;

/// Maximum weight per block
pub const MAXIMUM_BLOCK_WEIGHT: Weight =
	Weight::from_parts(WEIGHT_REF_TIME_PER_SECOND.saturating_div(2), u64::MAX);

// FIXME: For now just a random value.
pub struct FixedGasPrice;
impl FeeCalculator for FixedGasPrice {
	fn min_gas_price() -> (U256, Weight) {
		(1.into(), Weight::from_parts(1, 0u64))
	}
}

/// Current approximation of the gas/s consumption considering
/// EVM execution over compiled WASM (on 4.4Ghz CPU).
/// Given the 500ms Weight, from which 75% only are used for transactions,
/// the total EVM execution gas limit is: GAS_PER_SECOND * 0.500 * 0.75 ~= 15_000_000.
pub const GAS_PER_SECOND: u64 = 40_000_000;

/// Approximate ratio of the amount of Weight per Gas.
/// u64 works for approximations because Weight is a very small unit compared to gas.
pub const WEIGHT_PER_GAS: u64 = WEIGHT_REF_TIME_PER_SECOND / GAS_PER_SECOND;

pub struct FixedGasWeightMapping;

impl GasWeightMapping for FixedGasWeightMapping {
	fn gas_to_weight(gas: u64, _without_base_weight: bool) -> Weight {
		Weight::from_parts(gas.saturating_mul(WEIGHT_PER_GAS), 0u64)
	}
	fn weight_to_gas(weight: Weight) -> u64 {
		weight.ref_time().wrapping_div(WEIGHT_PER_GAS)
	}
}

/// An ipmlementation of Frontier's AddressMapping trait for Sgx Accounts.
/// This is basically identical to Frontier's own IdentityAddressMapping, but it works for any type
/// that is Into<H160> like AccountId20 for example.
pub struct IntoAddressMapping;

impl<T: From<H160>> AddressMapping<T> for IntoAddressMapping {
	fn into_account_id(address: H160) -> T {
		address.into()
	}
}

parameter_types! {
	pub const ChainId: u64 = 42;
	pub BlockGasLimit: U256 = U256::from(NORMAL_DISPATCH_RATIO * MAXIMUM_BLOCK_WEIGHT.ref_time() / WEIGHT_PER_GAS);
	pub const GasLimitPovSizeRatio: u64 = 150_000_000 / (5 * 1024 * 1024);
	//pub PrecompilesValue: FrontierPrecompiles<Runtime> = FrontierPrecompiles::<_>::new();
}

impl pallet_evm::Config for Runtime {
	type FeeCalculator = FixedGasPrice;
	type GasWeightMapping = FixedGasWeightMapping;
	type BlockHashMapping = SubstrateBlockHashMapping<Self>;
	type CallOrigin = EnsureAddressTruncated;
	type WithdrawOrigin = EnsureAddressTruncated;
	type AddressMapping = HashedAddressMapping;
	type Currency = Balances;
	type RuntimeEvent = RuntimeEvent;
	type Runner = pallet_evm::runner::stack::Runner<Self>;
	type PrecompilesType = ();
	type PrecompilesValue = ();
	type ChainId = ChainId;
	type OnChargeTransaction = ();
	type BlockGasLimit = BlockGasLimit;
	type FindAuthor = (); // Currently not available. Would need some more thoughts how prioritisation fees could be handled.
					  // BlockGasLimit / MAX_POV_SIZE
	type GasLimitPovSizeRatio = GasLimitPovSizeRatio;
	type WeightPerGas = ();
	type OnCreate = ();
	type Timestamp = Timestamp;
	type WeightInfo = ();
}
