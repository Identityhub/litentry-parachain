// Copyright 2020-2024 Trust Computing GmbH.
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

#![allow(opaque_hidden_inferred_bound)]

use lc_data_providers::geniidata::{GeniidataResponse, ResponseData, ResponseItem};
use warp::{http::Response, path::FullPath, Filter};

pub(crate) fn query() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	warp::get().and(warp::path::full()).map(move |p: FullPath| {
		if p.as_str() == "/" {
            println!("geniidata query");
			let res = GeniidataResponse {
				code: 0,
				message: "success".to_string(),
				data: ResponseData {
					count: 16435,
					limit: "20".to_string(),
					offset: "0".to_string(),
					list: vec![ResponseItem {
						tick: "orid".to_string(),
						address: "bc1pmkk62aua2pghenz4nps5jgllfaer29ulgpmjm4p5wlc4ewjx3p3ql260rj".to_string(),
						overall_balance: "0.732086440000000000".to_string(),
						transferable_balance: "0.000000000000000000".to_string(),
						available_balance: "0.732086440000000000".to_string(),
					}],
				},
			};
			Response::builder().body(serde_json::to_string(&res).unwrap())
		} else {
			Response::builder().status(400).body(String::from("Error query"))
		}
	})
}
