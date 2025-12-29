use serde::de::DeserializeOwned;
use crate::types::StatsAPIError;

#[cfg(feature = "ureq")]
mod ureq;

#[cfg(feature = "ureq")]
pub use ureq::*;

#[cfg(feature = "reqwest")]
mod reqwest;
#[cfg(feature = "reqwest")]
pub use reqwest::*;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
	#[cfg(feature = "ureq")]
	#[error(transparent)]
	Network(#[from] ::ureq::Error),
	#[cfg(feature = "reqwest")]
	#[error(transparent)]
	Network(#[from] ::reqwest::Error),
	#[cfg(not(all(test, debug_assertions)))]
	#[error(transparent)]
	Serde(#[from] serde_json::Error),
	#[cfg(all(test, debug_assertions))]
	#[error(transparent)]
	Serde(#[from] serde_path_to_error::Error<serde_json::Error>),
	#[error(transparent)]
	StatsAPI(#[from] StatsAPIError),
}

#[cfg(all(feature = "reqwest", feature = "ureq"))]
compile_error!("Only one http backend is allowed!");

pub trait StatsAPIRequestUrl: ToString {
	type Response: DeserializeOwned;

	#[cfg(feature = "ureq")]
	fn get(&self) -> Result<Self::Response>
	where
		Self: Sized,
	{
		let url = self.to_string();
		get::<Self::Response>(url)
	}

	#[cfg(feature = "reqwest")]
	fn get(&self) -> impl Future<Output = Result<Self::Response>>
	where
		Self: Sized,
	{
		let url = self.to_string();
		get::<Self::Response>(url)
	}
}

pub trait StatsAPIRequestUrlBuilderExt where Self: Sized {
	type Built: StatsAPIRequestUrl + From<Self>;

	#[cfg(feature = "ureq")]
	fn build_and_get(self) -> Result<<Self::Built as StatsAPIRequestUrl>::Response> {
		let built = Self::Built::from(self);
		let url = built.to_string();
		get::<<Self::Built as StatsAPIRequestUrl>::Response>(url)
	}

	#[cfg(feature = "reqwest")]
	fn build_and_get(self) -> impl Future<Output = Result<<Self::Built as StatsAPIRequestUrl>::Response>> {
		async {
			let built = Self::Built::from(self);
			let url = built.to_string();
			get::<<Self::Built as StatsAPIRequestUrl>::Response>(url).await
		}
	}
}

