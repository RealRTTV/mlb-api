//! Handles the request portion of the code

use serde::de::DeserializeOwned;
use crate::MLBError;

/// # Errors
/// See variants of [`Error`]
#[cfg(not(feature = "_debug"))]
pub async fn get<T: DeserializeOwned>(url: String) -> Result<T> {
	let bytes = reqwest::Client::builder().build()?.get(url).send().await?.bytes().await?;
	let e = match serde_json::from_slice::<'_, T>(&bytes) {
		Ok(t) => return Ok(t),
		Err(e) => Error::Serde(e),
	};
	Err(Error::MLB(serde_json::from_slice::<'_, MLBError>(&bytes).map_err(|_| e)?))
}

/// # Errors
/// See variants of [`Error`]
#[cfg(feature = "_debug")]
pub async fn get<T: DeserializeOwned>(url: String) -> Result<T> {
	let bytes = reqwest::Client::builder().build()?.get(url).send().await?.bytes().await?;
	let mut de = serde_json::Deserializer::from_slice(&bytes);
	let result: std::result::Result<T, serde_path_to_error::Error<_>> = serde_path_to_error::deserialize(&mut de);
	let e = match result {
		Ok(t) => return Ok(t),
		Err(e) => Error::Serde(e),
	};
	Err(Error::MLB(serde_json::from_slice::<'_, MLBError>(&bytes).map_err(|_| e)?))
}

/// Error variant
pub type Result<T, E = Error> = std::result::Result<T, E>;

/// Represents the error returned when making an HTTP request for stats
#[derive(Debug, thiserror::Error)]
pub enum Error {
	#[error(transparent)]
	Network(#[from] ::reqwest::Error),
	/// Error from [`serde`], likely deserialization error.
	#[cfg(not(feature = "_debug"))]
	#[error(transparent)]
	Serde(#[from] serde_json::Error),
	/// Error from [`serde`], likely deserialization error.
	#[cfg(feature = "_debug")]
	#[error(transparent)]
	Serde(#[from] serde_path_to_error::Error<serde_json::Error>),
	/// Error from MLB, likely bad payload.
	#[error(transparent)]
	MLB(#[from] MLBError),
}

/// A type in which you can request the response from its URL
pub trait RequestURL: ToString {
	type Response: DeserializeOwned;

	/// Get the response from the URL
	fn get(&self) -> impl Future<Output = Result<Self::Response>>
	where
		Self: Sized,
	{
		let url = self.to_string();
		get::<Self::Response>(url)
	}
}

/// Extension for request URLs such that `build_and_get` exists.
pub trait RequestURLBuilderExt where Self: Sized {
	/// Built type; [`PersonRequest`](crate::person::PersonRequest) for [`PersonRequestBuilder`](crate::person::PersonRequestBuilder)
	type Built: RequestURL + From<Self>;

	fn build_and_get(self) -> impl Future<Output = Result<<Self::Built as RequestURL>::Response>> {
		async {
			let built = Self::Built::from(self);
			let url = built.to_string();
			if cfg!(all(feature = "_debug", test)) {
				println!("url = {url}");
			}
			get::<<Self::Built as RequestURL>::Response>(url).await
		}
	}
}

