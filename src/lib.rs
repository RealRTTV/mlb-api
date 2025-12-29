#![warn(clippy::pedantic, clippy::nursery, clippy::complexity, clippy::cargo, clippy::perf, clippy::style)]
#![allow(clippy::multiple_crate_versions, clippy::ignore_without_reason)]

pub use requests::*;

pub mod hydrations;
pub mod request;
pub mod types;
pub mod cache;
mod requests;

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
