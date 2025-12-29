use crate::request::{Error, Result};
use crate::types::StatsAPIError;
use serde::de::DeserializeOwned;

/// # Errors
/// See variants of [`Error`]
#[cfg(not(all(test, debug_assertions)))]
pub async fn get<T: DeserializeOwned>(url: String) -> Result<T> {
	let bytes = reqwest::Client::builder().build()?.get(url).send().await?.bytes().await?;
	let e = match serde_json::from_slice::<'_, T>(&bytes) {
		Ok(t) => return Ok(t),
		Err(e) => Error::Serde(e),
	};
	Err(Error::StatsAPI(serde_json::from_slice::<'_, StatsAPIError>(&bytes).map_err(|_| e)?))
}

/// # Errors
/// See variants of [`Error`]
#[cfg(all(test, debug_assertions))]
pub async fn get<T: DeserializeOwned>(url: String) -> Result<T> {
	let bytes = reqwest::Client::builder().build()?.get(url).send().await?.bytes().await?;
	let mut de = serde_json::Deserializer::from_slice(&bytes);
	let result: std::result::Result<T, serde_path_to_error::Error<_>> = serde_path_to_error::deserialize(&mut de);
	let e = match result {
		Ok(t) => return Ok(t),
		Err(e) => Error::Serde(e),
	};
	Err(Error::StatsAPI(serde_json::from_slice::<'_, StatsAPIError>(&bytes).map_err(|_| e)?))
}
