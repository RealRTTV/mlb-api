use crate::cache::{HydratedCacheTable, RequestEntryCache};
use crate::league::League;
use crate::seasons::season::SeasonId;
use crate::sports::Sport;
use crate::types::Copyright;
use crate::{gen_params, rwlock_const_new, RwLock};
use crate::{integer_id, StatsAPIRequestUrl};
use bon::Builder;
use derive_more::{Deref, DerefMut, From};
use mlb_api_proc::{EnumTryAs, EnumTryAsMut, EnumTryInto};
use serde::Deserialize;
use std::fmt::{Display, Formatter};
use std::ops::{Deref, DerefMut};

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

integer_id!(ConferenceId);

#[derive(Debug, Deserialize, Eq, Clone, From, EnumTryAs, EnumTryAsMut, EnumTryInto)]
#[serde(untagged)]
pub enum Conference {
	Hydrated(Box<HydratedConference>),
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

#[derive(Builder)]
#[builder(derive(Into))]
pub struct ConferencesRequest {
	#[builder(into)]
	conference_id: Option<ConferenceId>,
	#[builder(into)]
	season: Option<SeasonId>,
}

impl<S: conferences_request_builder::State + conferences_request_builder::IsComplete> crate::requests::links::StatsAPIRequestUrlBuilderExt for ConferencesRequestBuilder<S> {
    type Built = ConferencesRequest;
}

impl Display for ConferencesRequest {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "http://statsapi.mlb.com/api/v1/conferences{}", gen_params! { "conferenceId"?: self.conference_id, "season"?: self.season })
	}
}

impl StatsAPIRequestUrl for ConferencesRequest {
	type Response = ConferencesResponse;
}

static CACHE: RwLock<HydratedCacheTable<Conference>> = rwlock_const_new(HydratedCacheTable::new());

impl RequestEntryCache for Conference {
	type HydratedVariant = Box<HydratedConference>;
	type Identifier = ConferenceId;
	type URL = ConferencesRequest;

	fn into_hydrated_variant(self) -> Option<Self::HydratedVariant> {
		self.try_into_hydrated()
	}

	fn id(&self) -> &Self::Identifier {
		&self.id
	}

	fn url_for_id(id: &Self::Identifier) -> Self::URL {
		ConferencesRequest::builder().conference_id(*id).build()
	}

	fn get_entries(response: <Self::URL as StatsAPIRequestUrl>::Response) -> impl IntoIterator<Item=Self>
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
	use crate::conferences::ConferencesRequest;
	use crate::StatsAPIRequestUrlBuilderExt;

	#[tokio::test]
	async fn parse_all_conferences() {
		let _response = ConferencesRequest::builder().build_and_get().await.unwrap();
	}
}
