use crate::StatsAPIEndpointUrl;
use crate::league::League;
use crate::sports::Sport;
use crate::{gen_params, rwlock_const_new, RwLock};
use crate::types::Copyright;
use derive_more::{Deref, DerefMut, Display, From};
use serde::Deserialize;
use std::fmt::{Display, Formatter};
use std::ops::{Deref, DerefMut};
use mlb_api_proc::{EnumTryAs, EnumTryAsMut, EnumTryInto};
use crate::cache::{HydratedCacheTable, EndpointEntryCache};

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ConferencesResponse {
	pub copyright: Copyright,
	pub conferences: Vec<Conference>,
}

#[derive(Debug, Deserialize, Deref, DerefMut, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HydratedConference {
	pub abbreviation: String,
	#[serde(rename = "nameShort")]
	pub short_name: String,
	pub has_wildcard: bool,
	pub league: League,
	pub sport: Sport,

	#[deref]
	#[deref_mut]
	#[serde(flatten)]
	inner: NamedConference,
}

#[derive(Debug, Deserialize, Deref, DerefMut, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NamedConference {
	pub name: String,

	#[deref]
	#[deref_mut]
	#[serde(flatten)]
	inner: IdentifiableConference,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
pub struct IdentifiableConference {
	pub id: ConferenceId,
}

#[repr(transparent)]
#[derive(Debug, Deserialize, Deref, Display, PartialEq, Eq, Copy, Clone, Hash, From)]
pub struct ConferenceId(u32);

#[derive(Debug, Deserialize, Eq, Clone, From, EnumTryAs, EnumTryAsMut, EnumTryInto)]
#[serde(untagged)]
pub enum Conference {
	Hydrated(HydratedConference),
	Named(NamedConference),
	Identifiable(IdentifiableConference),
}

impl Deref for Conference {
	type Target = IdentifiableConference;

	fn deref(&self) -> &Self::Target {
		match self {
			Self::Hydrated(inner) => inner,
			Self::Named(inner) => inner,
			Self::Identifiable(inner) => inner,
		}
	}
}

impl DerefMut for Conference {
	fn deref_mut(&mut self) -> &mut Self::Target {
		match self {
			Self::Hydrated(inner) => inner,
			Self::Named(inner) => inner,
			Self::Identifiable(inner) => inner,
		}
	}
}

impl PartialEq for Conference {
	fn eq(&self, other: &Self) -> bool {
		self.id == other.id
	}
}

#[derive(Default)]
pub struct ConferencesEndpoint {
	pub conference_id: Option<ConferenceId>,
	pub season: Option<u16>,
}

impl Display for ConferencesEndpoint {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "http://statsapi.mlb.com/api/v1/conferences{}", gen_params! { "conferenceId"?: self.conference_id, "season"?: self.season })
	}
}

impl StatsAPIEndpointUrl for ConferencesEndpoint {
	type Response = ConferencesResponse;
}

static CACHE: RwLock<HydratedCacheTable<Conference>> = rwlock_const_new(HydratedCacheTable::new());

impl EndpointEntryCache for Conference {
	type HydratedVariant = HydratedConference;
	type Identifier = ConferenceId;
	type URL = ConferencesEndpoint;

	fn into_hydrated_variant(self) -> Option<Self::HydratedVariant> {
		self.try_into_hydrated()
	}

	fn id(&self) -> &Self::Identifier {
		&self.id
	}

	fn url_for_id(id: &Self::Identifier) -> Self::URL {
		ConferencesEndpoint {
			conference_id: Some(id.clone()),
			season: None,
		}
	}

	fn get_entries(response: <Self::URL as StatsAPIEndpointUrl>::Response) -> impl IntoIterator<Item=Self>
	where
		Self: Sized
	{
		response.conferences
	}

	fn get_hydrated_cache_table() -> &'static RwLock<HydratedCacheTable<Self>>
	where
		Self: Sized
	{
		&CACHE
	}
}

#[cfg(test)]
mod tests {
	use crate::StatsAPIEndpointUrl;
	use crate::conferences::ConferencesEndpoint;

	#[tokio::test]
	async fn parse_all_conferences() {
		let _response = ConferencesEndpoint { ..ConferencesEndpoint::default() }.get().await.unwrap();
	}
}
