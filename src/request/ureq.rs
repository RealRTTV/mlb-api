use crate::request::Error;
use crate::types::StatsAPIError;
use serde::de::DeserializeOwned;

pub fn get<T: DeserializeOwned>(url: &impl ToString) -> Result<T> {
	let bytes = ureq::get(url.to_string()).call()?.into_body().read_to_vec()?;
	let e = match serde_json::from_slice::<'_, T>(&bytes) {
		Ok(t) => return Ok(t),
		Err(e) => Error::Serde(e),
	};
	Err(Error::StatsAPI(serde_json::from_slice::<'_, StatsAPIError>(&bytes).map_err(|_| e)?))
}
