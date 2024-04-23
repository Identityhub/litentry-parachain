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

#[cfg(all(not(feature = "std"), feature = "sgx"))]
extern crate sgx_tstd as std;

#[cfg(all(not(feature = "std"), feature = "sgx"))]
use crate::sgx_reexport_prelude::*;

#[cfg(all(feature = "std", feature = "sgx"))]
compile_error!("feature \"std\" and feature \"sgx\" cannot be enabled at the same time");

use crate::{ensure, Error, Result};
use itp_sgx_crypto::ShieldingCryptoDecrypt;
use lc_data_providers::{
	discord_official::{DiscordMessage, DiscordOfficialClient},
	twitter_official::{Tweet, TwitterOfficialClient},
	vec_to_string, DataProviderConfig, UserInfo,
};
use litentry_primitives::{
	DiscordValidationData, ErrorDetail, Identity, IntoErrorDetail, TwitterValidationData,
	Web2ValidationData,
};
use log::*;
use std::{string::ToString, vec::Vec};

pub trait DecryptionVerificationPayload<K: ShieldingCryptoDecrypt> {
	fn decrypt_ciphertext(&self, key: K) -> Result<Vec<u8>>;
}

fn payload_from_tweet(tweet: &Tweet) -> Result<Vec<u8>> {
	hex::decode(tweet.text.strip_prefix("0x").unwrap_or(tweet.text.as_str()))
		.map_err(|_| Error::LinkIdentityFailed(ErrorDetail::ParseError))
}

fn payload_from_discord(discord: &DiscordMessage) -> Result<Vec<u8>> {
	let data = &discord.content;
	hex::decode(data.strip_prefix("0x").unwrap_or(data.as_str()))
		.map_err(|_| Error::LinkIdentityFailed(ErrorDetail::ParseError))
}

pub fn verify(
	who: &Identity,
	identity: &Identity,
	raw_msg: &[u8],
	validation_data: &Web2ValidationData,
	config: &DataProviderConfig,
) -> Result<()> {
	debug!("verify web2 identity, who: {:?}", who);

	let username = match validation_data {
		Web2ValidationData::Twitter(data) => match data {
			TwitterValidationData::PublicTweet { ref tweet_id } => {
				let mut client = TwitterOfficialClient::v2(
					config.twitter_official_url.as_str(),
					config.twitter_auth_token_v2.as_str(),
				);
				let tweet: Tweet = client
					.query_tweet(tweet_id.to_vec())
					.map_err(|e| Error::LinkIdentityFailed(e.into_error_detail()))?;

				let user_id = tweet
					.get_user_id()
					.ok_or(Error::LinkIdentityFailed(ErrorDetail::WrongWeb2Handle))?;
				let user = client
					.query_user_by_id(user_id.into_bytes())
					.map_err(|e| Error::LinkIdentityFailed(e.into_error_detail()))?;

				let payload = payload_from_tweet(&tweet)?;
				ensure!(
					payload.as_slice() == raw_msg,
					Error::LinkIdentityFailed(ErrorDetail::UnexpectedMessage)
				);

				Ok(user.username)
			},
			TwitterValidationData::OAuth2 { access_token } => {
				let user_access_token = vec_to_string(access_token.to_vec())
					.map_err(|e| Error::LinkIdentityFailed(e.into_error_detail()))?;
				let user_authorization = std::format!("Bearer {}", user_access_token);
				let mut user_client =
					TwitterOfficialClient::v2(&config.twitter_official_url, &user_authorization);
				let user = user_client
					.query_user_by_id("me".to_string().into_bytes())
					.map_err(|e| Error::LinkIdentityFailed(e.into_error_detail()))?;

				Ok(user.username)
			},
		},
		Web2ValidationData::Discord(DiscordValidationData {
			ref channel_id,
			ref message_id,
			..
		}) => {
			let mut client = DiscordOfficialClient::new(config);
			let message: DiscordMessage = client
				.query_message(channel_id.to_vec(), message_id.to_vec())
				.map_err(|e| Error::LinkIdentityFailed(e.into_error_detail()))?;

			let user = client
				.get_user_info(message.author.id.clone())
				.map_err(|e| Error::LinkIdentityFailed(e.into_error_detail()))?;

			let mut username = message.author.username.clone();
			// if discord user's username is upgraded complete, the discriminator value from api will be "0".
			if user.discriminator != "0" {
				username.push_str(&'#'.to_string());
				username.push_str(&user.discriminator);
			}
			let payload = payload_from_discord(&message)?;
			ensure!(
				payload.as_slice() == raw_msg,
				Error::LinkIdentityFailed(ErrorDetail::UnexpectedMessage)
			);

			Ok(username)
		},
	}?;

	// compare the username:
	// - twitter's username is case insensitive
	// - discord's username is case sensitive
	match identity {
		Identity::Twitter(address) => {
			let handle = std::str::from_utf8(address.inner_ref())
				.map_err(|_| Error::LinkIdentityFailed(ErrorDetail::ParseError))?;
			ensure!(
				username.to_ascii_lowercase().eq(&handle.to_string().to_ascii_lowercase()),
				Error::LinkIdentityFailed(ErrorDetail::WrongWeb2Handle)
			);
		},
		Identity::Discord(address) => {
			let handle = std::str::from_utf8(address.inner_ref())
				.map_err(|_| Error::LinkIdentityFailed(ErrorDetail::ParseError))?;
			ensure!(username.eq(handle), Error::LinkIdentityFailed(ErrorDetail::WrongWeb2Handle));
		},
		_ => return Err(Error::LinkIdentityFailed(ErrorDetail::InvalidIdentity)),
	}

	Ok(())
}
