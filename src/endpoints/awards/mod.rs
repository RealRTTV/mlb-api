use crate::cache::{EndpointEntryCache, HydratedCacheTable};
use crate::league::{League, LeagueId};
use crate::seasons::season::SeasonId;
use crate::sports::{Sport, SportId};
use crate::types::Copyright;
use crate::StatsAPIEndpointUrl;
use crate::{gen_params, rwlock_const_new, RwLock};
use bon::Builder;
use derive_more::{Deref, DerefMut, Display, From};
use mlb_api_proc::{EnumTryAs, EnumTryAsMut, EnumTryInto};
use serde::Deserialize;
use std::fmt::{Display, Formatter};
use std::ops::Deref;

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
pub struct AwardsResponse {
	pub copyright: Copyright,
	pub awards: Vec<Award>,
}

#[derive(Debug, Deserialize, Deref, DerefMut, PartialEq, Eq, Clone)]
pub struct HydratedAward {
	pub name: String,
	pub description: Option<String>,
	pub sport: Option<Sport>,
	pub league: Option<League>,
	pub notes: Option<String>,

	#[deref]
	#[deref_mut]
	#[serde(flatten)]
	inner: IdentifiableAward,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
pub struct IdentifiableAward {
	pub id: AwardId,
}

#[repr(transparent)]
#[derive(Debug, Deserialize, Deref, Display, PartialEq, Eq, Clone, Hash, From)]
pub struct AwardId(String);

#[derive(Debug, Deserialize, Eq, Clone, From, EnumTryAs, EnumTryAsMut, EnumTryInto)]
#[serde(untagged)]
pub enum Award {
	Hydrated(HydratedAward),
	Identifiable(IdentifiableAward),
}

impl PartialEq for Award {
	fn eq(&self, other: &Self) -> bool {
		self.id == other.id
	}
}

impl Deref for Award {
	type Target = IdentifiableAward;

	fn deref(&self) -> &Self::Target {
		match self {
			Self::Hydrated(inner) => inner,
			Self::Identifiable(inner) => inner,
		}
	}
}

#[derive(Builder)]
#[builder(derive(Into))]
pub struct AwardEndpoint {
	#[builder(into)]
	award_id: Option<AwardId>,
	#[builder(into)]
	sport_id: Option<SportId>,
	#[builder(into)]
	league_id: Option<LeagueId>,
	#[builder(into)]
	season: Option<SeasonId>,
}

impl<S: award_endpoint_builder::State> crate::endpoints::links::StatsAPIEndpointUrlBuilderExt for AwardEndpointBuilder<S> where S: award_endpoint_builder::IsComplete {
    type Built = AwardEndpoint;
}

impl Display for AwardEndpoint {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"http://statsapi.mlb.com/api/v1/awards{}",
			gen_params! { "awardId"?: self.award_id.as_ref(), "sportId"?: self.sport_id, "leagueId"?: self.league_id, "season"?: self.season }
		)
	}
}

impl StatsAPIEndpointUrl for AwardEndpoint {
	type Response = AwardsResponse;
}

static CACHE: RwLock<HydratedCacheTable<Award>> = rwlock_const_new(HydratedCacheTable::new());

impl EndpointEntryCache for Award {
	type HydratedVariant = HydratedAward;
	type Identifier = AwardId;
	type URL = AwardEndpoint;

	fn into_hydrated_variant(self) -> Option<Self::HydratedVariant> {
		self.try_into_hydrated()
	}

	fn id(&self) -> &Self::Identifier {
		&self.id
	}

	fn url_for_id(id: &Self::Identifier) -> Self::URL {
		AwardEndpoint::builder().award_id(id.clone()).build()
	}

	fn get_entries(response: <Self::URL as StatsAPIEndpointUrl>::Response) -> impl IntoIterator<Item=Self>
	where
		Self: Sized
	{
		response.awards
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
	use crate::awards::AwardEndpoint;
	use crate::StatsAPIEndpointUrlBuilderExt;

	#[tokio::test]
	async fn parse_this_season() {
		let _response = AwardEndpoint::builder().build_and_get().await.unwrap();
	}
}
