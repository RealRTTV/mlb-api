use crate::cache::{RequestEntryCache, HydratedCacheTable};
use crate::league::{IdentifiableLeague, LeagueId};
use crate::seasons::season::SeasonId;
use crate::sports::{IdentifiableSport, SportId};
use crate::types::Copyright;
use crate::StatsAPIRequestUrl;
use crate::{gen_params, rwlock_const_new, RwLock};
use bon::Builder;
use derive_more::{Deref, DerefMut, Display, From};
use mlb_api_proc::{EnumTryAs, EnumTryAsMut, EnumTryInto};
use serde::Deserialize;
use std::fmt::{Display, Formatter};
use std::ops::{Deref, DerefMut};

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DivisionsResponse {
	pub copyright: Copyright,
	pub divisions: Vec<Division>,
}
#[derive(Debug, Deserialize, PartialEq, Eq, Copy, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IdentifiableDivision {
	pub id: DivisionId,
}

#[derive(Debug, Deserialize, Deref, DerefMut, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NamedDivision {
	pub name: String,

	#[deref]
	#[deref_mut]
	#[serde(flatten)]
	inner: IdentifiableDivision,
}

#[derive(Debug, Deserialize, Deref, DerefMut, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HydratedDivision {
	#[serde(rename = "nameShort")]
	pub short_name: String,
	pub season: SeasonId,
	pub abbreviation: String,
	pub league: IdentifiableLeague,
	pub sport: IdentifiableSport,
	pub has_wildcard: bool,
	pub num_playoff_teams: Option<u8>,
	pub active: bool,

	#[deref]
	#[deref_mut]
	#[serde(flatten)]
	inner: NamedDivision,
}

#[derive(Debug, Deserialize, Eq, Clone, From, EnumTryAs, EnumTryAsMut, EnumTryInto)]
#[serde(untagged)]
pub enum Division {
	Hydrated(HydratedDivision),
	Named(NamedDivision),
	Identifiable(IdentifiableDivision),
}

impl PartialEq for Division {
	fn eq(&self, other: &Self) -> bool {
		self.id == other.id
	}
}

impl Deref for Division {
	type Target = IdentifiableDivision;

	fn deref(&self) -> &Self::Target {
		match self {
			Self::Hydrated(inner) => inner,
			Self::Named(inner) => inner,
			Self::Identifiable(inner) => inner,
		}
	}
}

impl DerefMut for Division {
	fn deref_mut(&mut self) -> &mut Self::Target {
		match self {
			Self::Hydrated(inner) => inner,
			Self::Named(inner) => inner,
			Self::Identifiable(inner) => inner,
		}
	}
}

#[repr(transparent)]
#[derive(Debug, Deserialize, Deref, Display, PartialEq, Eq, Copy, Clone, Hash, From)]
pub struct DivisionId(u32);

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

impl<S: divisions_request_builder::State> crate::requests::links::StatsAPIRequestUrlBuilderExt for DivisionsRequestBuilder<S> where S: divisions_request_builder::IsComplete {
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

static CACHE: RwLock<HydratedCacheTable<Division>> = rwlock_const_new(HydratedCacheTable::new());

impl RequestEntryCache for Division {
	type HydratedVariant = HydratedDivision;
	type Identifier = DivisionId;
	type URL = DivisionsRequest;

	fn into_hydrated_variant(self) -> Option<Self::HydratedVariant> {
		self.try_into_hydrated()
	}

	fn id(&self) -> &Self::Identifier {
		&self.id
	}

	fn url_for_id(id: &Self::Identifier) -> Self::URL {
		DivisionsRequest::builder().division_id(id.clone()).build()
	}

	fn get_entries(response: <Self::URL as StatsAPIRequestUrl>::Response) -> impl IntoIterator<Item=Self>
	where
		Self: Sized
	{
		response.divisions
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
	use crate::divisions::DivisionsRequest;
	use crate::StatsAPIRequestUrlBuilderExt;

	#[tokio::test]
	async fn all_divisions_this_season() {
		let _response = DivisionsRequest::builder().build_and_get().await.unwrap();
	}
}
