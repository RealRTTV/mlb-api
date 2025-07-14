use serde::de::DeserializeOwned;
use crate::request::{Error, Result};
use crate::types::StatsAPIError;

pub async fn get<T: DeserializeOwned>(url: &impl ToString) -> Result<T> {
	let bytes = reqwest::Client::builder().build()?.get(url.to_string()).send().await?.bytes().await?;
	match serde_json::from_slice::<'_, T>(&bytes) {
		Ok(t) => return Ok(t),
		Err(e) if e.is_data() => {}
		Err(e) => return Err(Error::Serde(e)),
	}
	Err(Error::StatsAPI(serde_json::from_slice::<'_, StatsAPIError>(&bytes)?))
}
