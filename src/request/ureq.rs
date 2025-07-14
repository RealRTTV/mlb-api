use serde::de::DeserializeOwned;
use crate::types::StatsAPIError;
use crate::request::{Error, Result};

pub fn get<T: DeserializeOwned>(url: &impl ToString) -> Result<T> {
	let bytes= ureq::get(url.to_string()).call()?.into_body().read_to_vec()?;
	match serde_json::from_slice::<'_, T>(&bytes) {
		Ok(t) => return Ok(t),
		Err(e) if e.is_data() => {}
		Err(e) => return Err(Error::Serde(e)),
	}
	Err(Error::StatsAPI(serde_json::from_slice::<'_, StatsAPIError>(&bytes)?))
}
