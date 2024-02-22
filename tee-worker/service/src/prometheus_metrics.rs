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

//! Service for prometheus metrics, hosted on a http server.

use crate::{
	account_funding::EnclaveAccountInfo,
	error::{Error, ServiceResult},
};
use async_trait::async_trait;
use codec::{Decode, Encode};
#[cfg(feature = "attesteer")]
use core::time::Duration;
use frame_support::scale_info::TypeInfo;
#[cfg(feature = "dcap")]
use itc_rest_client::{
	http_client::{DefaultSend, HttpClient},
	rest_client::{RestClient, Url as URL},
	RestGet, RestPath,
};
use itp_enclave_metrics::EnclaveMetric;
use lazy_static::lazy_static;
use lc_stf_task_sender::RequestType;
use litentry_primitives::{Assertion, Identity};
use log::*;
use prometheus::{
	proto::MetricFamily, register_counter_vec, register_histogram, register_histogram_vec,
	register_int_gauge, register_int_gauge_vec, CounterVec, Histogram, HistogramVec, IntGauge,
	IntGaugeVec,
};
use serde::{Deserialize, Serialize};
use std::{net::SocketAddr, sync::Arc};
use warp::{Filter, Rejection, Reply};

lazy_static! {
	/// Register all the prometheus metrics we want to monitor (aside from the default process ones).

	static ref ENCLAVE_ACCOUNT_FREE_BALANCE: IntGauge =
		register_int_gauge!("litentry_worker_enclave_account_free_balance", "Free balance of the enclave account")
			.unwrap();
	static ref ENCLAVE_SIDECHAIN_BLOCK_HEIGHT: IntGauge =
		register_int_gauge!("litentry_worker_enclave_sidechain_block_height", "Enclave sidechain block height")
			.unwrap();
	static ref ENCLAVE_SIDECHAIN_TOP_POOL_SIZE: IntGauge =
		register_int_gauge!("litentry_worker_enclave_sidechain_top_pool_size", "Enclave sidechain top pool size")
			.unwrap();
	static ref ENCLAVE_STF_TASKS: IntGaugeVec =
		register_int_gauge_vec!("litentry_worker_enclave_stf_total_tasks", "Litentry Stf Tasks", &["request_type", "variant"])
			.unwrap();
	static ref ENCLAVE_STF_TASKS_EXECUTION: HistogramVec =
		register_histogram_vec!("litentry_worker_enclave_stf_tasks_execution_times", "Litentry Stf Tasks Exeuction Time", &["request_type", "variant"])
			.unwrap();
	static ref ENCLAVE_SUCCESSFUL_TRUSTED_OPERATION: CounterVec =
		register_counter_vec!("litentry_worker_enclave_successful_trusted_operation", "Litentry Successful Trusted Operation", &["call"])
			.unwrap();
	static ref ENCLAVE_FAILED_TRUSTED_OPERATION: CounterVec =
		register_counter_vec!("litentry_worker_enclave_failed_trusted_operation", "Litentry Failed Trusted Operation", &["call"])
			.unwrap();
	static ref ENCLAVE_PARENTCHAIN_BLOCK_IMPORT_TIME: Histogram =
		register_histogram!("litentry_worker_enclave_parentchain_block_import_time", "Time taken to import parentchain block")
			.unwrap();
	static ref ENCLAVE_SIDECHAIN_BLOCK_IMPORT_TIME: Histogram =
		register_histogram!("litentry_worker_enclave_sidechain_block_import_time", "Time taken to import sidechain block")
			.unwrap();
	static ref ENCLAVE_SIDECHAIN_SLOT_PREPARE_TIME: Histogram =
		register_histogram!("litentry_worker_enclave_sidechain_slot_prepare_time", "Time taken to prepare sidechain extrinsics for execution")
			.unwrap();
	static ref ENCLAVE_SIDECHAIN_SLOT_STF_EXECUTION_TIME: Histogram =
		register_histogram!("litentry_worker_enclave_sidechain_slot_stf_execution_time", "Time taken to execute sidechain extrinsics")
			.unwrap();
	static ref ENCLAVE_SIDECHAIN_SLOT_BLOCK_COMPOSITION_TIME: Histogram =
		register_histogram!("litentry_worker_enclave_sidechain_slot_block_composition_time", "Time taken to compose sidechain block")
			.unwrap();
	static ref ENCLAVE_SIDECHAIN_BLOCK_BROADCASTING_TIME: Histogram =
		register_histogram!("litentry_worker_enclave_sidechain_block_broadcasting_time", "Time taken to broadcast sidechain block")
			.unwrap();

}

pub async fn start_metrics_server<MetricsHandler>(
	metrics_handler: Arc<MetricsHandler>,
	port: u16,
) -> ServiceResult<()>
where
	MetricsHandler: HandleMetrics + Send + Sync + 'static,
{
	let metrics_route = warp::path!("metrics").and_then(move || {
		let handler_clone = metrics_handler.clone();
		async move { handler_clone.handle_metrics().await }
	});
	let socket_addr: SocketAddr = ([0, 0, 0, 0], port).into();

	info!("Running prometheus metrics server on: {:?}", socket_addr);
	warp::serve(metrics_route).run(socket_addr).await;

	info!("Prometheus metrics server shut down");
	Ok(())
}

#[async_trait]
pub trait HandleMetrics {
	type ReplyType: Reply;

	async fn handle_metrics(&self) -> Result<Self::ReplyType, Rejection>;
}

/// Metrics handler implementation.
pub struct MetricsHandler<Wallet> {
	enclave_wallet: Arc<Wallet>,
}

#[async_trait]
impl<Wallet> HandleMetrics for MetricsHandler<Wallet>
where
	Wallet: EnclaveAccountInfo + Send + Sync,
{
	type ReplyType = String;

	async fn handle_metrics(&self) -> Result<Self::ReplyType, Rejection> {
		self.update_metrics().await;

		let default_metrics = match gather_metrics_into_reply(&prometheus::gather()) {
			Ok(r) => r,
			Err(e) => {
				error!("Failed to gather prometheus metrics: {:?}", e);
				String::default()
			},
		};

		Ok(default_metrics)
	}
}

impl<Wallet> MetricsHandler<Wallet>
where
	Wallet: EnclaveAccountInfo + Send + Sync,
{
	pub fn new(enclave_wallet: Arc<Wallet>) -> Self {
		MetricsHandler { enclave_wallet }
	}

	async fn update_metrics(&self) {
		match self.enclave_wallet.free_balance() {
			Ok(b) => {
				ENCLAVE_ACCOUNT_FREE_BALANCE.set(b as i64);
			},
			Err(e) => {
				error!("Failed to fetch free balance metric, value will not be updated: {:?}", e);
			},
		}
	}
}

fn gather_metrics_into_reply(metrics: &[MetricFamily]) -> ServiceResult<String> {
	use prometheus::Encoder;
	let encoder = prometheus::TextEncoder::new();

	let mut buffer = Vec::new();
	encoder.encode(metrics, &mut buffer).map_err(|e| {
		Error::Custom(format!("Failed to encode prometheus metrics: {:?}", e).into())
	})?;

	let result_string = String::from_utf8(buffer).map_err(|e| {
		Error::Custom(
			format!("Failed to convert Prometheus encoded metrics to UTF8: {:?}", e).into(),
		)
	})?;

	Ok(result_string)
}

/// Trait to receive metric updates from inside the enclave.
pub trait ReceiveEnclaveMetrics {
	fn receive_enclave_metric(&self, metric: EnclaveMetric) -> ServiceResult<()>;
}

pub struct EnclaveMetricsReceiver;

impl ReceiveEnclaveMetrics for EnclaveMetricsReceiver {
	fn receive_enclave_metric(&self, metric: EnclaveMetric) -> ServiceResult<()> {
		match metric {
			EnclaveMetric::SetSidechainBlockHeight(h) => {
				ENCLAVE_SIDECHAIN_BLOCK_HEIGHT.set(h as i64);
			},
			EnclaveMetric::TopPoolSizeSet(pool_size) => {
				ENCLAVE_SIDECHAIN_TOP_POOL_SIZE.set(pool_size as i64);
			},
			EnclaveMetric::TopPoolSizeIncrement => {
				ENCLAVE_SIDECHAIN_TOP_POOL_SIZE.inc();
			},
			EnclaveMetric::TopPoolSizeDecrement => {
				ENCLAVE_SIDECHAIN_TOP_POOL_SIZE.dec();
			},
			EnclaveMetric::StfTaskExecutionTime(req, time) => {
				handle_stf_call_request(*req, time);
			},
			EnclaveMetric::SuccessfulTrustedOperationIncrement(metric_name) => {
				ENCLAVE_SUCCESSFUL_TRUSTED_OPERATION.with_label_values(&[&metric_name]).inc();
			},
			EnclaveMetric::FailedTrustedOperationIncrement(metric_name) => {
				ENCLAVE_FAILED_TRUSTED_OPERATION.with_label_values(&[&metric_name]).inc();
			},
			EnclaveMetric::ParentchainBlockImportTime(time) =>
				ENCLAVE_PARENTCHAIN_BLOCK_IMPORT_TIME.observe(time.as_secs_f64()),
			EnclaveMetric::SidechainBlockImportTime(time) =>
				ENCLAVE_SIDECHAIN_BLOCK_IMPORT_TIME.observe(time.as_secs_f64()),
			EnclaveMetric::SidechainSlotPrepareTime(time) =>
				ENCLAVE_SIDECHAIN_SLOT_PREPARE_TIME.observe(time.as_secs_f64()),
			EnclaveMetric::SidechainSlotStfExecutionTime(time) =>
				ENCLAVE_SIDECHAIN_SLOT_STF_EXECUTION_TIME.observe(time.as_secs_f64()),
			EnclaveMetric::SidechainSlotBlockCompositionTime(time) =>
				ENCLAVE_SIDECHAIN_SLOT_BLOCK_COMPOSITION_TIME.observe(time.as_secs_f64()),
			EnclaveMetric::SidechainBlockBroadcastingTime(time) =>
				ENCLAVE_SIDECHAIN_BLOCK_BROADCASTING_TIME.observe(time.as_secs_f64()),
		}
		Ok(())
	}
}

// Function to increment STF calls with labels
fn inc_stf_calls(category: &str, label: &str) {
	ENCLAVE_STF_TASKS.with_label_values(&[category, label]).inc();
}

// Function to observe STF call execution time with labels
fn observe_execution_time(category: &str, label: &str, time: f64) {
	ENCLAVE_STF_TASKS_EXECUTION.with_label_values(&[category, label]).observe(time);
}

// Handle STF call request and increment metrics
fn handle_stf_call_request(req: RequestType, time: f64) {
	// Determine the category based on the request type
	let category = match req {
		RequestType::IdentityVerification(_) => "link_identity",
		RequestType::AssertionVerification(_) => "request_vc",
	};

	let label = match req {
		RequestType::IdentityVerification(request) => match request.identity {
			Identity::Twitter(_) => "Twitter",
			Identity::Discord(_) => "Discord",
			Identity::Github(_) => "Github",
			Identity::Substrate(_) => "Substrate",
			Identity::Evm(_) => "Evm",
			Identity::Bitcoin(_) => "Bitcoin",
		},
		RequestType::AssertionVerification(request) => match request.assertion {
			Assertion::A1 => "A1",
			Assertion::A2(_) => "A2",
			Assertion::A3(..) => "A3",
			Assertion::A4(_) => "A4",
			Assertion::A6 => "A6",
			Assertion::A7(_) => "A7",
			Assertion::A8(_) => "A8",
			Assertion::A10(_) => "A10",
			Assertion::A11(_) => "A11",
			Assertion::A13(_) => "A13",
			Assertion::A14 => "A14",
			Assertion::A20 => "A20",
			Assertion::Achainable(..) => "Achainable",
			Assertion::Oneblock(..) => "Oneblock",
			Assertion::BnbDomainHolding => "BnbDomainHolding",
			Assertion::BnbDigitDomainClub(..) => "BnbDigitDomainClub",
			Assertion::GenericDiscordRole(_) => "GenericDiscordRole",
			Assertion::VIP3MembershipCard(..) => "VIP3MembershipCard",
			Assertion::WeirdoGhostGangHolder => "WeirdoGhostGangHolder",
			Assertion::LITStaking => "LITStaking",
			Assertion::EVMAmountHolding(_) => "EVMAmountHolding",
			Assertion::BRC20AmountHolder => "BRC20AmountHolder",
			Assertion::CryptoSummary => "CryptoSummary",
			Assertion::TokenHoldingAmount(_) => "TokenHoldingAmount",
			Assertion::PlatformUser(_) => "PlatformUser",
			Assertion::NftHolder(_) => "NftHolder",
			Assertion::Dynamic(_) => "Dynamic",
		},
	};
	inc_stf_calls(category, label);
	observe_execution_time(category, label, time)
}

#[derive(Serialize, Deserialize, Debug)]
struct PrometheusMarblerunEvents(pub Vec<PrometheusMarblerunEvent>);

#[cfg(feature = "attesteer")]
impl RestPath<&str> for PrometheusMarblerunEvents {
	fn get_path(path: &str) -> Result<String, itc_rest_client::error::Error> {
		Ok(format!("{}", path))
	}
}

#[cfg(feature = "attesteer")]
pub fn fetch_marblerun_events(base_url: &str) -> Result<Vec<PrometheusMarblerunEvent>, Error> {
	let base_url = URL::parse(&base_url).map_err(|e| {
		Error::Custom(
			format!("Failed to parse marblerun prometheus endpoint base URL: {:?}", e).into(),
		)
	})?;
	let timeout = 3u64;
	let http_client =
		HttpClient::new(DefaultSend {}, true, Some(Duration::from_secs(timeout)), None, None);

	let mut rest_client = RestClient::new(http_client, base_url.clone());
	let events: PrometheusMarblerunEvents = rest_client.get("events").map_err(|e| {
		Error::Custom(
			format!("Failed to fetch marblerun prometheus events from: {}, error: {}", base_url, e)
				.into(),
		)
	})?;

	Ok(events.0)
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Encode, Decode, TypeInfo)]
pub struct PrometheusMarblerunEvent {
	pub time: String,
	pub activation: PrometheusMarblerunEventActivation,
}

#[cfg(feature = "attesteer")]
impl PrometheusMarblerunEvent {
	pub fn get_quote_without_prepended_bytes(&self) -> &[u8] {
		let marblerun_magic_prepended_header_size = 16usize;
		&self.activation.quote.as_bytes()[marblerun_magic_prepended_header_size..]
	}
}
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Encode, Decode, TypeInfo)]
#[serde(rename_all = "camelCase")]
pub struct PrometheusMarblerunEventActivation {
	pub marble_type: String,
	pub uuid: String,
	pub quote: String,
}
