use crate::request::{Result, Error};
use crate::types::MLBError;
use serde::de::DeserializeOwned;

/// # Errors
/// See variants of [`Error`]
#[cfg(not(all(test, debug_assertions)))]
pub fn get<T: DeserializeOwned>(url: impl ToString) -> Result<T> {
	let bytes = ureq::get(url.to_string()).call()?.into_body().read_to_vec()?;
	let e = match serde_json::from_slice::<'_, T>(&bytes) {
		Ok(t) => return Ok(t),
		Err(e) => Error::Serde(e),
	};
	Err(Error::MLB(serde_json::from_slice::<'_, MLBError>(&bytes).map_err(|_| e)?))
}

/// # Errors
/// See variants of [`Error`]
#[cfg(all(test, debug_assertions))]
pub fn get<T: DeserializeOwned>(url: impl ToString) -> Result<T> {
	let bytes = ureq::get(url.to_string()).call()?.into_body().read_to_vec()?;
	let mut de = serde_json::Deserializer::from_slice(&bytes);
	let result: std::result::Result<T, serde_path_to_error::Error<_>> = serde_path_to_error::deserialize(&mut de);
	let e = match result {
		Ok(t) => return Ok(t),
		Err(e) => Error::Serde(e),
	};
	Err(Error::MLB(serde_json::from_slice::<'_, MLBError>(&bytes).map_err(|_| e)?))
}
