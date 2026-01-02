use crate::league::LeagueId;
use crate::season::SeasonId;
use crate::sport::SportId;
use crate::types::Copyright;
use crate::request::StatsAPIRequestUrl;
use bon::Builder;
use derive_more::{Deref, DerefMut};
use serde::Deserialize;
use std::fmt::{Display, Formatter};
use crate::cache::{CacheTable, RequestEntryCache};
use crate::{rwlock_const_new, RwLock};

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ConferencesResponse {
	pub copyright: Copyright,
	pub conferences: Vec<Conference>,
}

#[derive(Debug, Deserialize, Deref, DerefMut, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Conference {
	pub abbreviation: String,
	#[serde(rename = "nameShort")]
	pub short_name: String,
	pub has_wildcard: bool,
	pub league: LeagueId,
	pub sport: SportId,

	#[deref]
	#[deref_mut]
	#[serde(flatten)]
	inner: NamedConference,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NamedConference {
	pub name: String,
	#[serde(flatten)]
	pub id: ConferenceId,
}

id!(ConferenceId { id: u32 });

id_only_eq_impl!(Conference, id);
id_only_eq_impl!(NamedConference, id);

#[derive(Builder)]
#[builder(derive(Into))]
pub struct ConferencesRequest {
	#[builder(into)]
	conference_id: Option<ConferenceId>,
	#[builder(into)]
	season: Option<SeasonId>,
}

impl<S: conferences_request_builder::State + conferences_request_builder::IsComplete> crate::request::StatsAPIRequestUrlBuilderExt for ConferencesRequestBuilder<S> {
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

static CACHE: RwLock<CacheTable<Conference>> = rwlock_const_new(CacheTable::new());

impl RequestEntryCache for Conference {
	type Identifier = ConferenceId;
	type URL = ConferencesRequest;

	fn id(&self) -> &Self::Identifier {
		&self.id
	}

	#[cfg(feature = "aggressive_cache")]
	fn url_for_id(_id: &Self::Identifier) -> Self::URL {
		ConferencesRequest::builder().build()
	}

	#[cfg(not(feature = "aggressive_cache"))]
	fn url_for_id(id: &Self::Identifier) -> Self::URL {
		ConferencesRequest::builder().conference_id(*id).build()
	}

	fn get_entries(response: <Self::URL as StatsAPIRequestUrl>::Response) -> impl IntoIterator<Item=Self>
	where
		Self: Sized
	{
		response.conferences
	}

	fn get_cache_table() -> &'static RwLock<CacheTable<Self>>
	where
		Self: Sized
	{
		&CACHE
	}
}

entrypoint!(Conference.id => Conference);
entrypoint!(NamedConference.id => Conference);
entrypoint!(ConferenceId => Conference);

#[cfg(test)]
mod tests {
	use crate::conferences::ConferencesRequest;
	use crate::request::StatsAPIRequestUrlBuilderExt;

	#[tokio::test]
	async fn parse_all_conferences() {
		let _response = ConferencesRequest::builder().build_and_get().await.unwrap();
	}
}
