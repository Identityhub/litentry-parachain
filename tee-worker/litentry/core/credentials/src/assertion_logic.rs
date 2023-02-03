// Copyright 2020-2023 Litentry Technologies GmbH.
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

// This file includes the predefined rulesets and the corresponding parameters
// when requesting VCs.

use codec::{Decode, Encode};
use scale_info::TypeInfo;
use serde::{Deserialize, Serialize};
use std::{
	boxed::Box,
	fmt::Debug,
	string::{String, ToString},
	vec,
	vec::Vec,
};

#[derive(Serialize, Deserialize, Encode, Decode, Debug, PartialEq, Eq, TypeInfo, Copy, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Op {
	#[serde(rename = ">")]
	GreaterThan,
	#[serde(rename = "<")]
	LessThan,
	#[serde(rename = ">=")]
	GreaterEq,
	#[serde(rename = "<=")]
	LessEq,
	#[serde(rename = "==")]
	Equal,
	#[serde(rename = "!=")]
	NotEq,
}

#[derive(Serialize, Deserialize, Encode, Decode, PartialEq, Eq, TypeInfo, Debug, Clone)]
#[serde(untagged)]
pub enum AssertionLogic {
	Item {
		src: String,
		op: Op,
		dsc: String,
	},
	And {
		#[serde(rename = "and")]
		items: Vec<Box<AssertionLogic>>,
	},
	Or {
		#[serde(rename = "or")]
		items: Vec<Box<AssertionLogic>>,
	},
}

impl AssertionLogic {
	pub fn new_and() -> Self {
		Self::And { items: vec![] }
	}

	pub fn new_or() -> Self {
		Self::Or { items: vec![] }
	}

	pub fn new_item<T: ToString>(src: T, op: Op, dsc: T) -> Self {
		Self::Item { src: src.to_string(), op, dsc: dsc.to_string() }
	}
	pub fn add_item(mut self, item: AssertionLogic) -> Self {
		match &mut self {
			Self::Item { .. } => unreachable!(),
			Self::Or { items } => items.push(Box::new(item)),
			Self::And { items } => items.push(Box::new(item)),
		}
		self
	}
}

pub trait Logic {
	fn eval(&self) -> bool;
}

impl Logic for AssertionLogic {
	fn eval(&self) -> bool {
		match self {
			Self::Item { src, op, dsc } => match op {
				Op::GreaterThan => src > dsc,
				Op::LessThan => src < dsc,
				Op::GreaterEq => src >= dsc,
				Op::LessEq => src <= dsc,
				Op::Equal => src == dsc,
				Op::NotEq => src != dsc,
			},
			Self::And { items } => items.iter().all(|item| item.eval()),
			Self::Or { items } => items.iter().any(|item| item.eval()),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn assertion_logic_works() {
		let a1 = r#"
            {
                "or":[
                    {
                        "src":"$web2_account_cnt",
                        "op":">=",
                        "dsc":"7"
                    },
                    {
                        "src":"$web3_account_cnt",
                        "op":">",
                        "dsc":"3"
                    }
                ]
            }
            "#;

		let a1_from_str: AssertionLogic = serde_json::from_str(a1).unwrap();

		let web2_item = AssertionLogic::new_item("$web2_account_cnt", Op::GreaterEq, "7");
		let web3_item = AssertionLogic::new_item("$web3_account_cnt", Op::GreaterThan, "3");

		let a1_from_struct = AssertionLogic::new_or().add_item(web2_item).add_item(web3_item);

		assert_eq!(a1_from_str, a1_from_struct);
	}

	#[test]
	fn assertion_a1_eval_works() {
		let web2_item = AssertionLogic::new_item("7", Op::GreaterEq, "7");
		let web3_item = AssertionLogic::new_item("7", Op::GreaterThan, "3");

		let a1 = AssertionLogic::new_or().add_item(web2_item).add_item(web3_item);
		assert_eq!(a1.eval(), true);
	}
}
