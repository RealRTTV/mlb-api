#![warn(clippy::pedantic, clippy::nursery, clippy::complexity, clippy::cargo, clippy::perf, clippy::style)]
#![allow(clippy::multiple_crate_versions, clippy::ignore_without_reason)]

/// Generates stat structs to be used in requests.
///
/// These are commonly associated with [`person_hydrations`] to create a [`person::PersonRequest`].
///
/// The list of [`StatType`]s can be found on its file (if the static feature is enabled), or as impls of [`stats::StatTypeStats`].
///
/// The list of [`StatGroup`]s can be found on its type.
///
/// # Examples
/// ```rs
/// stats! {
///     pub struct MyStats {
///         [Season, Career] = [Hitting, Pitching]
///     }
/// }
///
/// ---
///
/// pub struct BasicStats {
///     season: BasicStatsSeasonSplit,
///     career: BasicStatsCareerSplit,
/// }
///
/// pub struct BasicStatsSeasonSplit {
///     hitting: Box<<SeasonStats as StatTypeStats>::Hitting>, // Box<Season<HittingStats>>
///     pitching: Box<<SeasonStats as StatTypeStats>::Pitching>, // Box<Season<PitchingStats>>
/// }
/// 
/// pub struct BasicStatsCareerSplit {
///     hitting: Box<<CareerStats as StatTypeStats>::Hitting>, // Box<HittingStats>
///     pitching: Box<<CareerStats as StatTypeStats>::Pitching>, // Box<PitchingStats>
/// }
/// ```
#[macro_export]
macro_rules! stats {
    ($($t:tt)*) => {
        ::mlb_api_proc::stats! {
            $crate $($t)*
        }
    };
}

pub mod ids;
pub mod hydrations;
pub mod request;
pub mod types;
pub mod cache;
mod requests;

pub use requests::*;

#[cfg(test)]
pub const TEST_YEAR: u32 = 2025;

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
            panic!("{}", serde_json::from_slice::<'_, types::StatsAPIError>(&bytes).map(request::Error::StatsAPI).map_or_else(|_| e.to_string(), |e| e.to_string()));
        }
    }
}
