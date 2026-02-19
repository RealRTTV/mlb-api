//! An abstraction over endpoints that contain fixed data, such as [`GameType`]s, [`JobType`]s, etc.
//!
//! For types in which quick accessibility is important (and there are not tons of variants), they will be represented as an enum ([`GameType`]).
//!
//! For types which have many variants and ones that would be constantly updating, they are represented as `Vec<struct>`s ([`SituationCode`]).
//!
//! These types implement [`MetaKind`] and [`Requestable`] for ease of use.
//!
//! Some of these types will have id-only variants in most responses, in which you can request details using [`Requestable`]; [`Position`].
//!
//! [`Requestable`]: crate::cache::Requestable

macro_rules! meta_kind_impl {
	($endpoint:literal => $name:ty) => {
		impl $crate::meta::MetaKind for $name {
			type Complete = $name;

			const ENDPOINT_NAME: &'static str = $endpoint;
		}
	};
}

macro_rules! test_impl {
    ($name:ty) => {
		#[cfg(test)]
		mod tests {
			use super::*;
			use crate::request::RequestURL;

			#[tokio::test]
			async fn parse_meta() {
				let _response = $crate::meta::MetaRequest::<$name>::new().get().await.unwrap();
			}
		}
	};
}

macro_rules! tiered_request_entry_cache_impl {
	($complete:ident.$id_field:ident: $id:ident) => {
		tiered_request_entry_cache_impl!([$complete].$id_field: $id);
	};
	([$complete:ident $(, $($others:ident)*)?].$id_field:ident: $id:ident) => {
		#[cfg(feature = "cache")]
		static CACHE: $crate::RwLock<$crate::cache::CacheTable<$complete>> = $crate::rwlock_const_new($crate::cache::CacheTable::new());

		impl $crate::cache::Requestable for $complete {
			type Identifier = $id;
			type URL = $crate::meta::MetaRequest<Self>;

			fn id(&self) -> &Self::Identifier {
				&self.$id_field
			}

			fn url_for_id(_id: &Self::Identifier) -> Self::URL {
				$crate::meta::MetaRequest::new()
			}

			fn get_entries(response: <Self::URL as $crate::request::RequestURL>::Response) -> impl IntoIterator<Item=Self>
			where
				Self: Sized
			{
				response.entries
			}

			#[cfg(feature = "cache")]
			fn get_cache_table() -> &'static $crate::RwLock<$crate::cache::CacheTable<Self>>
			where
				Self: Sized
			{
				&CACHE
			}
		}

		entrypoint!($complete.$id_field => $complete);
		$($(
		entrypoint!($others.$id_field => $complete);
		)*)?
		entrypoint!($id => $complete);
	};
}

macro_rules! static_request_entry_cache_impl {
    ($name:ident) => {
	    #[cfg(feature = "cache")]
		static CACHE: $crate::RwLock<$crate::cache::CacheTable<$name>> = $crate::rwlock_const_new($crate::cache::CacheTable::new());

		impl $crate::cache::Requestable for $name {
			type Identifier = Self;
			type URL = $crate::meta::MetaRequest<Self>;

			fn id(&self) -> &Self::Identifier {
				self
			}

			fn url_for_id(_id: &Self::Identifier) -> Self::URL {
				$crate::meta::MetaRequest::new()
			}

			fn get_entries(response: <Self::URL as $crate::request::RequestURL>::Response) -> impl IntoIterator<Item=Self>
			where
				Self: Sized
			{
				response.entries
			}

			#[cfg(feature = "cache")]
			fn get_cache_table() -> &'static $crate::RwLock<$crate::cache::CacheTable<Self>>
			where
				Self: Sized
			{
				&CACHE
			}
		}
		
		entrypoint!($name => $name);
	};
}

mod baseball_stats;
mod event_types;
mod game_status;
mod game_types;
mod hit_trajectories;
mod job_types;
mod languages;
mod league_leader_types;
mod logical_events;
mod metrics;
mod pitch_codes;
mod pitch_types;
mod platforms;
mod positions;
mod review_reasons;
mod roster_types;
mod schedule_event_types;
mod situations;
mod sky;
mod standings_types;
mod stat_groups;
mod stat_types;
mod wind_direction;

pub use baseball_stats::*;
pub use event_types::*;
pub use game_status::*;
pub use game_types::*;
pub use hit_trajectories::*;
pub use job_types::*;
pub use languages::*;
pub use league_leader_types::*;
pub use logical_events::*;
pub use metrics::*;
pub use pitch_codes::*;
pub use pitch_types::*;
pub use platforms::*;
pub use positions::*;
pub use review_reasons::*;
pub use roster_types::*;
pub use schedule_event_types::*;
pub use situations::*;
pub use sky::*;
pub use standings_types::*;
pub use stat_groups::*;
pub use stat_types::*;
pub use wind_direction::*;

use crate::request::RequestURL;
use derive_more::{Deref, DerefMut};
use serde::de::{DeserializeOwned, Error, MapAccess, SeqAccess};
use serde::{de, Deserialize, Deserializer};
use std::fmt::{Debug, Display, Formatter};
use std::marker::PhantomData;

/// Represents a type that is metadata.
pub trait MetaKind {
	type Complete: Debug + DeserializeOwned + Eq + Clone;

	const ENDPOINT_NAME: &'static str;
}

/// Generalized response for the meta endpoints.
#[derive(Debug, Deref, DerefMut, PartialEq, Eq, Clone)]
pub struct MetaResponse<T: MetaKind> {
	pub entries: Vec<<T as MetaKind>::Complete>,
}

impl<'de, T: MetaKind> Deserialize<'de> for MetaResponse<T> {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		struct Visitor<T: MetaKind>(PhantomData<T>);

		impl<'de, T: MetaKind> de::Visitor<'de> for Visitor<T> {
			type Value = MetaResponse<T>;

			fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
				formatter.write_str("either copyright and other entry, or just raw list")
			}

			fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
			where
				A: SeqAccess<'de>,
			{
				let mut entries = vec![];
				while let Some(element) = seq.next_element::<<T as MetaKind>::Complete>()? {
					entries.push(element);
				}
				Ok(MetaResponse { entries })
			}

			fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
			where
				A: MapAccess<'de>,
			{
				while let Some(key) = map.next_key::<String>()? {
					if key != "copyright" {
						let entries = map.next_value::<Vec<<T as MetaKind>::Complete>>()?;
						return Ok(MetaResponse { entries });
					}
				}
				Err(Error::custom("Could not find a field that deserializes to the entries"))
			}
		}

		deserializer.deserialize_any(Visitor(PhantomData))
	}
}

/// Returns [`MetaResponse<T>`].
pub struct MetaRequest<T: MetaKind> {
	_marker: PhantomData<T>,
}

impl<T: MetaKind> Default for MetaRequest<T> {
	fn default() -> Self {
		Self { _marker: PhantomData }
	}
}

impl<T: MetaKind> MetaRequest<T> {
	#[must_use]
	pub const fn new() -> Self {
		Self {
			_marker: PhantomData
		}
	}
}

impl<T: MetaKind> Display for MetaRequest<T> {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "http://statsapi.mlb.com/api/v1/{}", T::ENDPOINT_NAME)
	}
}

impl<T: MetaKind> RequestURL for MetaRequest<T> {
	type Response = MetaResponse<T>;
}
