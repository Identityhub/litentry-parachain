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

use serde_json::json;
use warp::{http::Response, path::FullPath, Filter};


const RESPONSE_ONEBLOCK: &str = r#"
{"object":"list","results":[{"object":"block","id":"8b371d31-1e60-4060-9f63-3527fa8cfe0c","parent":{"type":"block_id","block_id":"e4068e6a-3262-4346-8f35-dcdc0c43f111"},"created_time":"2023-09-05T03:21:00.000Z","last_edited_time":"2023-09-05T04:11:00.000Z","created_by":{"object":"user","id":"03bab8a4-8794-44d8-8843-961ee4c33485"},"last_edited_by":{"object":"user","id":"c208e40c-306a-4b31-a249-49139ff24411"},"has_children":false,"archived":false,"type":"table_row","table_row":{"cells":[[{"type":"text","text":{"content":"学号","link":null},"annotations":{"bold":false,"italic":false,"strikethrough":false,"underline":false,"code":false,"color":"default"},"plain_text":"学号","href":null}],[{"type":"text","text":{"content":"姓名","link":null},"annotations":{"bold":false,"italic":false,"strikethrough":false,"underline":false,"code":false,"color":"default"},"plain_text":"姓名","href":null}],[{"type":"text","text":{"content":"substrate地址","link":null},"annotations":{"bold":false,"italic":false,"strikethrough":false,"underline":false,"code":false,"color":"default"},"plain_text":"substrate地址","href":null}],[{"type":"text","text":{"content":"课程观看进度","link":null},"annotations":{"bold":false,"italic":false,"strikethrough":false,"underline":false,"code":false,"color":"default"},"plain_text":"课程观看进度","href":null}],[{"type":"text","text":{"content":"是否毕业","link":null},"annotations":{"bold":false,"italic":false,"strikethrough":false,"underline":false,"code":false,"color":"default"},"plain_text":"是否毕业","href":null}],[{"type":"text","text":{"content":"是否优秀毕业","link":null},"annotations":{"bold":false,"italic":false,"strikethrough":false,"underline":false,"code":false,"color":"default"},"plain_text":"是否优秀毕业","href":null}]]}},{"object":"block","id":"00d4a220-db1f-4d24-8b4f-4d3815dab3c5","parent":{"type":"block_id","block_id":"e4068e6a-3262-4346-8f35-dcdc0c43f111"},"created_time":"2023-09-05T03:21:00.000Z","last_edited_time":"2023-09-05T04:11:00.000Z","created_by":{"object":"user","id":"03bab8a4-8794-44d8-8843-961ee4c33485"},"last_edited_by":{"object":"user","id":"c208e40c-306a-4b31-a249-49139ff24411"},"has_children":false,"archived":false,"type":"table_row","table_row":{"cells":[[{"type":"text","text":{"content":"Team1","link":null},"annotations":{"bold":false,"italic":false,"strikethrough":false,"underline":false,"code":false,"color":"default"},"plain_text":"Team1","href":null}],[],[],[],[],[]]}},{"object":"block","id":"71130f91-0326-468a-8c42-c64c17e779af","parent":{"type":"block_id","block_id":"e4068e6a-3262-4346-8f35-dcdc0c43f111"},"created_time":"2023-09-05T03:21:00.000Z","last_edited_time":"2023-09-05T06:25:00.000Z","created_by":{"object":"user","id":"03bab8a4-8794-44d8-8843-961ee4c33485"},"last_edited_by":{"object":"user","id":"c208e40c-306a-4b31-a249-49139ff24411"},"has_children":false,"archived":false,"type":"table_row","table_row":{"cells":[[{"type":"text","text":{"content":"1264","link":null},"annotations":{"bold":false,"italic":false,"strikethrough":false,"underline":false,"code":false,"color":"default"},"plain_text":"1264","href":null}],[{"type":"text","text":{"content":"Student1","link":null},"annotations":{"bold":false,"italic":false,"strikethrough":false,"underline":false,"code":false,"color":"default"},"plain_text":"Student1","href":null}],[{"type":"text","text":{"content":"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQQ","link":null},"annotations":{"bold":false,"italic":false,"strikethrough":false,"underline":false,"code":false,"color":"default"},"plain_text":"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQQ","href":null}],[{"type":"text","text":{"content":"第一课","link":null},"annotations":{"bold":false,"italic":false,"strikethrough":false,"underline":false,"code":false,"color":"default"},"plain_text":"第一课","href":null}],[{"type":"text","text":{"content":"yes","link":null},"annotations":{"bold":false,"italic":false,"strikethrough":false,"underline":false,"code":false,"color":"default"},"plain_text":"yes","href":null}],[{"type":"text","text":{"content":"yes","link":null},"annotations":{"bold":false,"italic":false,"strikethrough":false,"underline":false,"code":false,"color":"default"},"plain_text":"yes","href":null}]]}},{"object":"block","id":"72cdaf64-9dc5-4e71-8285-bba8d03abd99","parent":{"type":"block_id","block_id":"e4068e6a-3262-4346-8f35-dcdc0c43f111"},"created_time":"2023-09-05T03:21:00.000Z","last_edited_time":"2023-09-05T04:11:00.000Z","created_by":{"object":"user","id":"03bab8a4-8794-44d8-8843-961ee4c33485"},"last_edited_by":{"object":"user","id":"c208e40c-306a-4b31-a249-49139ff24411"},"has_children":false,"archived":false,"type":"table_row","table_row":{"cells":[[{"type":"text","text":{"content":"1263","link":null},"annotations":{"bold":false,"italic":false,"strikethrough":false,"underline":false,"code":false,"color":"default"},"plain_text":"1263","href":null}],[{"type":"text","text":{"content":"Clement Tam","link":null},"annotations":{"bold":false,"italic":false,"strikethrough":false,"underline":false,"code":false,"color":"default"},"plain_text":"Clement Tam","href":null}],[{"type":"text","text":{"content":"5HYaWcUJvX1xjNnduouJnD3F5q5X7uCpGxHV2yCRZEurymEE","link":null},"annotations":{"bold":false,"italic":false,"strikethrough":false,"underline":false,"code":false,"color":"default"},"plain_text":"5HYaWcUJvX1xjNnduouJnD3F5q5X7uCpGxHV2yCRZEurymEE","href":null}],[],[],[]]}},{"object":"block","id":"42d0ada7-1e3d-493a-bd2f-17821d2e54a2","parent":{"type":"block_id","block_id":"e4068e6a-3262-4346-8f35-dcdc0c43f111"},"created_time":"2023-09-05T03:21:00.000Z","last_edited_time":"2023-09-05T04:11:00.000Z","created_by":{"object":"user","id":"03bab8a4-8794-44d8-8843-961ee4c33485"},"last_edited_by":{"object":"user","id":"c208e40c-306a-4b31-a249-49139ff24411"},"has_children":false,"archived":false,"type":"table_row","table_row":{"cells":[[{"type":"text","text":{"content":"1262","link":null},"annotations":{"bold":false,"italic":false,"strikethrough":false,"underline":false,"code":false,"color":"default"},"plain_text":"1262","href":null}],[{"type":"text","text":{"content":"Student2","link":null},"annotations":{"bold":false,"italic":false,"strikethrough":false,"underline":false,"code":false,"color":"default"},"plain_text":"Student2","href":null}],[{"type":"text","text":{"content":"12zh1QyBrqddzgLbxBHjmoCnna8XsT9pTTMPCUt1f7WtC1f5","link":null},"annotations":{"bold":false,"italic":false,"strikethrough":false,"underline":false,"code":false,"color":"default"},"plain_text":"12zh1QyBrqddzgLbxBHjmoCnna8XsT9pTTMPCUt1f7WtC1f5","href":null}],[],[],[]]}}],"next_cursor":null,"has_more":false,"type":"block","block":{}}
"#;
pub(crate) fn query() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	warp::get().and(warp::path::full()).map(|p: FullPath| {

		// Better filtering?
		if p.as_str() == "/" {
			Response::builder().body(RESPONSE_ONEBLOCK.to_string())
		} else {
			Response::builder().status(400).body(String::from("Error query"))
		}
	})
}
