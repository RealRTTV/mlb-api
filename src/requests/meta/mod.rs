macro_rules! meta_kind_impl {
	($endpoint:literal => $name:ty) => {
		impl $crate::requests::meta::MetaKind for $name {
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
				let _response = $crate::requests::meta::MetaRequest::<$name>::new().get().await.unwrap();
			}
		}
	};
}

/*
id_only_eq_impl!($name, $id_field);
meta_kind_impl!($endpoint => $name);
tiered_request_entry_cache_impl!($name => $hydrated_name; $id_field: $id);
test_impl!($name);
*/

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
pub mod situation_codes;
pub mod sky;
pub mod standings_types;
pub mod stat_groups;
pub mod stat_types;
pub mod wind_direction;

use crate::request::StatsAPIRequestUrl;
use derive_more::{Deref, DerefMut};
use serde::de::{Error, MapAccess, SeqAccess};
use serde::{de, Deserialize, Deserializer};
use std::fmt::{Debug, Display, Formatter};
use std::marker::PhantomData;
use crate::cache::RequestEntryCache;

pub trait MetaKind: RequestEntryCache {
	const ENDPOINT_NAME: &'static str;
}

#[derive(Debug, Deref, DerefMut, PartialEq, Eq, Clone)]
pub struct MetaResponse<T: MetaKind> {
	pub entries: Vec<T>,
}

impl<'de, T: MetaKind> Deserialize<'de> for MetaResponse<T> {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		struct Visitor<T>(PhantomData<T>);

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
				while let Some(element) = seq.next_element::<T>()? {
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
						let entries = map.next_value::<Vec<T>>()?;
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
	pub fn new() -> Self {
		Self::default()
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
