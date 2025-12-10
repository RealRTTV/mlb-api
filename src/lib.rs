#![warn(clippy::pedantic, clippy::nursery, clippy::complexity, clippy::cargo, clippy::perf, clippy::style)]
#![allow(clippy::multiple_crate_versions)]

#[macro_export]
macro_rules! stats {
    ($($t:tt)*) => {
        ::mlb_api_proc::stats! {
            $crate $($t)*
        }
    };
}

pub mod hydrations;
pub mod request;
pub mod types;
pub mod cache;
mod requests;
pub use requests::*;

#[cfg(feature = "reqwest")]
pub(crate) type RwLock<T> = tokio::sync::RwLock<T>;

#[cfg(feature = "ureq")]
pub(crate) type RwLock<T> = parking_lot::RwLock<T>;

#[cfg(feature = "reqwest")]
pub(crate) const fn rwlock_const_new<T>(t: T) -> RwLock<T> {
    RwLock::const_new(t)
}

#[cfg(feature = "ureq")]
pub(crate) const fn rwlock_const_new<T>(t: T) -> RwLock<T> {
    parking_lot::const_rwlock(t)
}

#[cfg(test)]
pub(crate) async fn serde_path_to_error_parse<T: StatsAPIRequestUrl>(url: T) -> T::Response {
    let url = url.to_string();
    let bytes = reqwest::get(url).await.unwrap().bytes().await.unwrap();
    let mut de = serde_json::Deserializer::from_slice(&bytes);
    let result: Result<T::Response, serde_path_to_error::Error<_>> = serde_path_to_error::deserialize(&mut de);
    match result {
        Ok(x) => x,
        Err(e) => {
            if let Ok(e) = serde_json::from_slice::<'_, types::StatsAPIError>(&bytes).map(request::Error::StatsAPI) {
                panic!("{e}");
            } else {
                panic!("{e}");
            }
        }
    }
}
