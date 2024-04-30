use extism_pdk::*;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Deserialize, Serialize)]
struct Identity {
	identity_type: IdentityType,
	value: String,
	networks: Vec<Web3Network>,
}

impl Identity {
	fn is_web3(&self) -> bool {
		matches!(
			self.identity_type,
			IdentityType::Substrate
				| IdentityType::Evm
				| IdentityType::Bitcoin
				| IdentityType::Solana
		)
	}
}

#[derive(Deserialize, Serialize)]
pub enum IdentityType {
	// web2
	Twitter,
	Discord,
	Github,

	// web3
	Substrate,
	Evm,
	Bitcoin,
	Solana,
}

#[derive(Deserialize, Serialize)]
pub enum Web3Network {
	// substrate
	Polkadot,
	Kusama,
	Litentry,
	Litmus,
	LitentryRococo,
	Khala,
	SubstrateTestnet,

	// evm
	Ethereum,
	Bsc,

	// bitcoin
	BitcoinP2tr,
	BitcoinP2pkh,
	BitcoinP2sh,
	BitcoinP2wpkh,
	BitcoinP2wsh,

	// evm
	Polygon,
	Arbitrum,
	Solana,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AssertionResult {
	pub description: String,
	pub assertion_type: String,
	pub assertions: Vec<String>,
	pub schema_url: String,
	pub meet: bool,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EarlyBirdResponse {
	has_joined: bool,
}

#[plugin_fn]
pub fn execute(Json(identities): Json<Vec<Identity>>) -> FnResult<Json<AssertionResult>> {
	let description = "The user is an early bird user of the IdentityHub EVM version and has generated at least 1 credential during 2023 Aug 14th ~ Aug 21st.";
	let assertion_type = "IDHub EVM Version Early Bird";
	let assertions = vec![r#"{ "src": "$has_joined", "op": "==", "dst": "true" }"#.to_string()];
	let schema_url = "https://raw.githubusercontent.com/litentry/vc-jsonschema/main/dist/schemas/12-idhub-evm-version-early-bird/1-0-0.json";

	let mut result = false;

	for identity in identities {
		if identity.is_web3() {
			let req = HttpRequest {
				url: format!(
					"http://localhost:4000/events/does-user-joined-evm-campaign?account={}",
					identity.value
				),
				headers: BTreeMap::new(),
				method: Some("GET".to_string()),
			};
			let res = http::request::<()>(&req, None)?;
			let data: EarlyBirdResponse = res.json()?;
			if data.has_joined {
				result = true;
				break;
			}
		}
	}

	Ok(Json(AssertionResult {
		description: description.to_string(),
		assertion_type: assertion_type.to_string(),
		assertions,
		schema_url: schema_url.to_string(),
		meet: result,
	}))
}

#[cfg(test)]
mod tests {
	use super::*;
	use ::extism::{convert::Json, Manifest, Plugin, Wasm};
	use serde_json::json;
	use std::{collections::HashMap, thread};
	use warp::{http::Response, Filter};

	fn run_mock_server() -> Result<(), Box<dyn std::error::Error>> {
		let endpoint = warp::get()
			.and(warp::path!("events" / "does-user-joined-evm-campaign"))
			.and(warp::query::<HashMap<String, String>>())
			.map(|p: HashMap<String, String>| {
				let default = String::default();
				let account = p.get("account").unwrap_or(&default);
				if account == "0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d" {
					let body = r#"{ "hasJoined": true }"#;
					Response::builder().body(body.to_string())
				} else {
					let body = r#"{ "hasJoined": false }"#;
					Response::builder().body(body.to_string())
				}
			});
		thread::spawn(move || {
			let rt = tokio::runtime::Runtime::new().unwrap();
			rt.block_on(async {
				warp::serve(endpoint).run(([127, 0, 0, 1], 4000)).await;
			});
		});

		Ok(())
	}

	#[test]
	fn test_execute() {
		run_mock_server().unwrap();
		let identities = vec![
			Identity {
				identity_type: IdentityType::Substrate,
				value: "1FRMM8PEiWXYax7rpS6X4XZX1aAAxSWx1CrKTyrVYhV24fg".to_string(),
				networks: vec![Web3Network::Polkadot],
			},
			Identity {
				identity_type: IdentityType::Evm,
				value: "0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d"
					.to_string(),
				networks: vec![Web3Network::Ethereum],
			},
		];

		let byte_code = include_bytes!("../target/wasm32-unknown-unknown/release/a20_plugin.wasm");
		let wasm = Wasm::data(byte_code);
		let manifest = Manifest::new([wasm]).with_allowed_host("*".to_string());
		let mut plugin = Plugin::new(manifest, [], true).unwrap();
		let input = json!(identities).to_string();
		let Json(result) = plugin.call::<&str, Json<AssertionResult>>("execute", &input).unwrap();

		let expected = AssertionResult {
			description: "The user is an early bird user of the IdentityHub EVM version and has generated at least 1 credential during 2023 Aug 14th ~ Aug 21st."
				.to_string(),
			assertion_type: "IDHub EVM Version Early Bird".to_string(),
			assertions: vec!["{ \"src\": \"$has_joined\", \"op\": \"==\", \"dst\": \"true\" }".to_string()],
			schema_url: "https://raw.githubusercontent.com/litentry/vc-jsonschema/main/dist/schemas/12-idhub-evm-version-early-bird/1-0-0.json".to_string(),
			meet: true,
		};

		assert_eq!(result.description, expected.description);
		assert_eq!(result.assertion_type, expected.assertion_type);
		assert_eq!(result.assertions, expected.assertions);
		assert_eq!(result.schema_url, expected.schema_url);
		assert_eq!(result.meet, expected.meet);
	}
}
