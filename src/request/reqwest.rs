use crate::request::{Error, Result};
use crate::types::StatsAPIError;
use serde::de::DeserializeOwned;

/// # Errors
/// See variants of [`Error`]
pub async fn get<T: DeserializeOwned>(url: String) -> Result<T> {
	let bytes = reqwest::Client::builder().build()?.get(url).send().await?.bytes().await?;
	let e = match serde_json::from_slice::<'_, T>(&bytes) {
		Ok(t) => return Ok(t),
		Err(e) => Error::Serde(e),
	};
	Err(Error::StatsAPI(serde_json::from_slice::<'_, StatsAPIError>(&bytes).map_err(|_| e)?))
}
