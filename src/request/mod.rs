//! Handles the request portion of the code; uses [`ureq`] or [`reqwest`] for blocking or non-blocking networking.

use serde::de::DeserializeOwned;
use crate::MLBError;

#[cfg(feature = "ureq")]
mod ureq;

#[cfg(feature = "ureq")]
pub use ureq::*;

#[cfg(feature = "reqwest")]
mod reqwest;
#[cfg(feature = "reqwest")]
pub use reqwest::*;

/// Error variant
pub type Result<T, E = Error> = std::result::Result<T, E>;

/// Represents the error returned when making an HTTP request for stats
#[derive(Debug, thiserror::Error)]
pub enum Error {
	/// Error from [`ureq`], likely network error.
	#[cfg(feature = "ureq")]
	#[error(transparent)]
	Network(#[from] ::ureq::Error),
	/// Error from [`reqwest`], likely network error.
	#[cfg(feature = "reqwest")]
	#[error(transparent)]
	Network(#[from] ::reqwest::Error),
	/// Error from [`serde`], likely deserialization error.
	#[cfg(not(all(test, debug_assertions)))]
	#[error(transparent)]
	Serde(#[from] serde_json::Error),
	/// Error from [`serde`], likely deserialization error.
	#[cfg(all(test, debug_assertions))]
	#[error(transparent)]
	Serde(#[from] serde_path_to_error::Error<serde_json::Error>),
	/// Error from MLB, likely bad payload.
	#[error(transparent)]
	MLB(#[from] MLBError),
}

#[cfg(all(feature = "reqwest", feature = "ureq"))]
compile_error!("Only one http backend is allowed!");

/// A type in which you can request the response from its URL
pub trait RequestURL: ToString {
	type Response: DeserializeOwned;

	/// Get the response from the URL
	#[cfg(feature = "ureq")]
	fn get(&self) -> Result<Self::Response>
	where
		Self: Sized,
	{
		let url = self.to_string();
		get::<Self::Response>(url)
	}

	/// Get the response from the URL
	#[cfg(feature = "reqwest")]
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

	#[cfg(feature = "ureq")]
	fn build_and_get(self) -> Result<<Self::Built as RequestURL>::Response> {
		let built = Self::Built::from(self);
		let url = built.to_string();
		get::<<Self::Built as RequestURL>::Response>(url)
	}

	#[cfg(feature = "reqwest")]
	fn build_and_get(self) -> impl Future<Output = Result<<Self::Built as RequestURL>::Response>> {
		async {
			let built = Self::Built::from(self);
			let url = built.to_string();
			get::<<Self::Built as RequestURL>::Response>(url).await
		}
	}
}

