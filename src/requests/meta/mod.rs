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
			use crate::request::StatsAPIRequestUrl;

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
		static CACHE: $crate::RwLock<$crate::cache::CacheTable<$complete>> = $crate::rwlock_const_new($crate::cache::CacheTable::new());

		impl $crate::cache::RequestEntryCache for $complete {
			type Identifier = $id;
			type URL = $crate::meta::MetaRequest<Self>;

			fn id(&self) -> &Self::Identifier {
				&self.$id_field
			}

			fn url_for_id(_id: &Self::Identifier) -> Self::URL {
				$crate::meta::MetaRequest::new()
			}

			fn get_entries(response: <Self::URL as $crate::request::StatsAPIRequestUrl>::Response) -> impl IntoIterator<Item=Self>
			where
				Self: Sized
			{
				response.entries
			}

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
		static CACHE: $crate::RwLock<$crate::cache::CacheTable<$name>> = $crate::rwlock_const_new($crate::cache::CacheTable::new());

		impl $crate::cache::RequestEntryCache for $name {
			type Identifier = Self;
			type URL = $crate::meta::MetaRequest<Self>;

			fn id(&self) -> &Self::Identifier {
				self
			}

			fn url_for_id(_id: &Self::Identifier) -> Self::URL {
				$crate::meta::MetaRequest::new()
			}

			fn get_entries(response: <Self::URL as $crate::request::StatsAPIRequestUrl>::Response) -> impl IntoIterator<Item=Self>
			where
				Self: Sized
			{
				response.entries
			}

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

pub mod baseball_stats;
pub mod event_types;
pub mod game_status;
pub mod game_types;
pub mod hit_trajectories;
pub mod job_types;
pub mod languages;
pub mod league_leader_types;
pub mod logical_events;
pub mod metrics;
pub mod pitch_codes;
pub mod pitch_types;
pub mod platforms;
pub mod positions;
pub mod review_reasons;
pub mod roster_types;
pub mod schedule_event_types;
pub mod situations;
pub mod sky;
pub mod standings_types;
pub mod stat_groups;
pub mod stat_types;
pub mod wind_direction;

use crate::request::StatsAPIRequestUrl;
use derive_more::{Deref, DerefMut};
use serde::de::{DeserializeOwned, Error, MapAccess, SeqAccess};
use serde::{de, Deserialize, Deserializer};
use std::fmt::{Debug, Display, Formatter};
use std::marker::PhantomData;

pub trait MetaKind {
	type Complete: Debug + DeserializeOwned + Eq + Clone;

	const ENDPOINT_NAME: &'static str;
}

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

impl<T: MetaKind> StatsAPIRequestUrl for MetaRequest<T> {
	type Response = MetaResponse<T>;
}
