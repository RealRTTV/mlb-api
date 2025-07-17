use crate::request::{Error, Result};
use crate::types::StatsAPIError;
use serde::de::DeserializeOwned;

pub async fn get<T: DeserializeOwned>(url: &impl ToString) -> Result<T> {
	let bytes = reqwest::Client::builder().build()?.get(url.to_string()).send().await?.bytes().await?;
	let e = match serde_json::from_slice::<'_, T>(&bytes) {
		Ok(t) => return Ok(t),
		Err(e) => Error::Serde(e),
	};
	Err(Error::StatsAPI(serde_json::from_slice::<'_, StatsAPIError>(&bytes).map_err(|_| e)?))
}
