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
    Ureq(#[from] ::ureq::Error),
    #[cfg(feature = "reqwest")]
    #[error(transparent)]
    Reqwest(#[from] ::reqwest::Error),
    #[error(transparent)]
    Serde(#[from] serde_json::Error),
    #[error(transparent)]
    StatsAPI(#[from] StatsAPIError),
}
