// todo: refactor all endpoints to be at `crate::*` depth
pub mod endpoints;
pub mod request;
pub mod types;
mod cache;

#[cfg(feature = "reqwest")]
pub(crate) type RwLock<T> = tokio::sync::RwLock<T>;

#[cfg(feature = "reqwest")]
pub(crate) const fn rwlock_const_new<T>(t: T) -> RwLock<T> {
    RwLock::const_new(t)
}

#[cfg(feature = "ureq")]
pub(crate) type RwLock<T> = parking_lot::RwLock<T>;

#[cfg(feature = "ureq")]
pub(crate) const fn rwlock_const_new<T>(t: T) -> RwLock<T> {
    parking_lot::const_rwlock(t)
}
