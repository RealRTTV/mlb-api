#![warn(clippy::pedantic, clippy::nursery, clippy::complexity, clippy::cargo, clippy::perf, clippy::style)]
#![warn(clippy::allow_attributes_without_reason, clippy::ignore_without_reason)]
#![allow(clippy::multiple_crate_versions, clippy::cast_lossless, reason = "deemed unnecessary")]

macro_rules! id {
    ($(#[$meta:meta])* $name:ident { $id_field:ident: String }) => {
		$(#[$meta])*
		#[derive(::core::fmt::Debug, ::derive_more::Deref, ::derive_more::Display, ::core::cmp::PartialEq, ::core::cmp::Eq, ::core::clone::Clone, ::core::hash::Hash, ::derive_more::From)]
		#[repr(transparent)]
		pub struct $name(String);

		impl<'de> ::serde::Deserialize<'de> for $name {
			#[allow(non_snake_case, reason = "is camel case because serde deserializes that from the API")]
			fn deserialize<D: ::serde::de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
				#[derive(::serde::Deserialize)]
				#[serde(untagged)]
				enum Repr {
					Wrapped { $id_field: String },
					Inline(String),
				}

				let (Repr::Wrapped { $id_field } | Repr::Inline($id_field)) = Repr::deserialize(deserializer)?;
				Ok($name($id_field))
			}
		}

		impl $name {
			#[must_use]
			pub fn new(id: impl Into<String>) -> Self {
				Self(id.into())
			}
		}
	};
    ($(#[$meta:meta])* $name:ident { $id_field:ident: u32 }) => {
		$(#[$meta])*
		#[derive(::core::fmt::Debug, ::derive_more::Deref, ::derive_more::Display, ::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::hash::Hash, ::derive_more::From)]
		#[repr(transparent)]
		pub struct $name(u32);

		impl<'de> ::serde::Deserialize<'de> for $name {
			#[allow(non_snake_case, reason = "is camel case because serde deserializes that from the API")]
			fn deserialize<D: ::serde::de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
				#[derive(::serde::Deserialize)]
				#[serde(untagged)]
				enum Repr {
					Wrapped { $id_field: u32 },
					Inline(u32),
				}

				let (Repr::Wrapped { $id_field } | Repr::Inline($id_field)) = Repr::deserialize(deserializer)?;
				Ok($name($id_field))
			}
		}

		impl $name {
			#[must_use]
			pub const fn new(id: u32) -> Self {
				Self(id)
			}
		}
	};
}

// todo: add macro to lookup one stat type under one stat group for a player and return the type directly.

pub mod hydrations;
pub mod request;
mod types;
pub mod cache;
mod requests;

pub use requests::*;
pub use types::*;

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
