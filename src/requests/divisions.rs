use crate::cache::{CacheTable, RequestEntryCache};
use crate::league::LeagueId;
use crate::request::StatsAPIRequestUrl;
use crate::season::SeasonId;
use crate::sport::SportId;
use crate::types::Copyright;
use crate::{rwlock_const_new, RwLock};
use bon::Builder;
use derive_more::{Deref, DerefMut};
use serde::Deserialize;
use std::fmt::{Display, Formatter};

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DivisionsResponse {
	pub copyright: Copyright,
	pub divisions: Vec<Division>,
}

id!(DivisionId { id: u32 });

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NamedDivision {
	pub name: String,
	#[serde(flatten)]
	pub id: DivisionId,
}

#[derive(Debug, Deserialize, Deref, DerefMut, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Division {
	#[serde(rename = "nameShort")]
	pub short_name: String,
	pub season: SeasonId,
	pub abbreviation: String,
	pub league: LeagueId,
	pub sport: SportId,
	pub has_wildcard: bool,
	pub num_playoff_teams: Option<u8>,
	pub active: bool,

	#[deref]
	#[deref_mut]
	#[serde(flatten)]
	inner: NamedDivision,
}

id_only_eq_impl!(Division, id);
id_only_eq_impl!(NamedDivision, id);

#[derive(Builder)]
#[builder(derive(Into))]
pub struct DivisionsRequest {
	#[builder(into)]
	division_id: Option<DivisionId>,
	#[builder(into)]
	league_id: Option<LeagueId>,
	#[builder(into)]
	sport_id: Option<SportId>,
	#[builder(into)]
	season: Option<SeasonId>,
}

impl<S: divisions_request_builder::State + divisions_request_builder::IsComplete> crate::request::StatsAPIRequestUrlBuilderExt for DivisionsRequestBuilder<S> {
    type Built = DivisionsRequest;
}

impl Display for DivisionsRequest {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"http://statsapi.mlb.com/api/v1/divisions{}",
			gen_params! { "divisionId"?: self.division_id, "leagueId"?: self.league_id, "sportId"?: self.sport_id, "season"?: self.season }
		)
	}
}

impl StatsAPIRequestUrl for DivisionsRequest {
	type Response = DivisionsResponse;
}

static CACHE: RwLock<CacheTable<Division>> = rwlock_const_new(CacheTable::new());

impl RequestEntryCache for Division {
	type Identifier = DivisionId;
	type URL = DivisionsRequest;

	fn id(&self) -> &Self::Identifier {
		&self.id
	}

	#[cfg(feature = "aggressive_cache")]
	fn url_for_id(_id: &Self::Identifier) -> Self::URL {
		DivisionsRequest::builder().build()
	}

	#[cfg(not(feature = "aggressive_cache"))]
	fn url_for_id(id: &Self::Identifier) -> Self::URL {
		DivisionsRequest::builder().division_id(*id).build()
	}

	fn get_entries(response: <Self::URL as StatsAPIRequestUrl>::Response) -> impl IntoIterator<Item=Self>
	where
		Self: Sized
	{
		response.divisions
	}

	fn get_cache_table() -> &'static RwLock<CacheTable<Self>>
	where
		Self: Sized
	{
		&CACHE
	}
}

entrypoint!(DivisionId => Division);
entrypoint!(NamedDivision.id => Division);
entrypoint!(Division.id => Division);

#[cfg(test)]
mod tests {
	use crate::divisions::DivisionsRequest;
	use crate::request::StatsAPIRequestUrlBuilderExt;

	#[tokio::test]
	async fn all_divisions_this_season() {
		let _response = DivisionsRequest::builder().build_and_get().await.unwrap();
	}
}
