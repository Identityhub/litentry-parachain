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

#[cfg(all(not(feature = "std"), feature = "sgx"))]
use chrono::{offset::Utc as TzUtc, TimeZone};

#[cfg(feature = "std")]
use chrono::{offset::Utc as TzUtc, TimeZone};

use codec::{Decode, Encode, MaxEncodedLen};
use scale_info::TypeInfo;
use sp_runtime::{traits::ConstU32, BoundedVec};
use std::{format, string::String};

type Balance = u128;
type MaxStringLength = ConstU32<64>;
pub type ParameterString = BoundedVec<u8, MaxStringLength>;

#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq, TypeInfo, MaxEncodedLen)]
pub enum Assertion {
	A1,
	A2(ParameterString, ParameterString), // (guild_id, user_id)
	A3(ParameterString, ParameterString), // (guild_id, user_id)
	A4(Balance, ParameterString),         // (LIT_amount, timestamp, e.g.: 2022-10-16T00:00:00Z)
	A5(ParameterString, ParameterString), // (twitter_account, tweet_id)
	A6,
	A7(Balance, u32), // (DOT_amount, year)
	A8,
	A9,
	A10(Balance, u32), // (WBTC_amount, year)
	A11(Balance, u32), // (ETH_amount, year)
	A13(u32),          // (Karma_amount) - TODO: unsupported
}

pub fn year_to_date(year: u32) -> String {
	#[cfg(feature = "std")]
	let dt1 = TzUtc.with_ymd_and_hms(year as i32, 1, 1, 0, 0, 0);
	#[cfg(all(not(feature = "std"), feature = "sgx"))]
	let dt1 = TzUtc.ymd(year as i32, 1, 1).and_hms(0, 0, 0);
	format!("{:?}", dt1)
}
