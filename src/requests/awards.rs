use crate::cache::{HydratedCacheTable, RequestEntryCache};
use crate::league::{League, LeagueId};
use crate::seasons::season::SeasonId;
use crate::sports::{Sport, SportId};
use crate::types::Copyright;
use crate::{gen_params, rwlock_const_new, RwLock};
use crate::{string_id, StatsAPIRequestUrl};
use bon::Builder;
use derive_more::{Deref, DerefMut, From};
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

string_id!(AwardId);

#[derive(Debug, Deserialize, Eq, Clone, From, EnumTryAs, EnumTryAsMut, EnumTryInto)]
#[serde(untagged)]
pub enum Award {
	Hydrated(Box<HydratedAward>),
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
pub struct AwardRequest {
	#[builder(into)]
	award_id: Option<AwardId>,
	#[builder(into)]
	sport_id: Option<SportId>,
	#[builder(into)]
	league_id: Option<LeagueId>,
	#[builder(into)]
	season: Option<SeasonId>,
}

impl<S: award_request_builder::State + award_request_builder::IsComplete> crate::requests::links::StatsAPIRequestUrlBuilderExt for AwardRequestBuilder<S> {
    type Built = AwardRequest;
}

impl Display for AwardRequest {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"http://statsapi.mlb.com/api/v1/awards{}",
			gen_params! { "awardId"?: self.award_id.as_ref(), "sportId"?: self.sport_id, "leagueId"?: self.league_id, "season"?: self.season }
		)
	}
}

impl StatsAPIRequestUrl for AwardRequest {
	type Response = AwardsResponse;
}

static CACHE: RwLock<HydratedCacheTable<Award>> = rwlock_const_new(HydratedCacheTable::new());

impl RequestEntryCache for Award {
	type HydratedVariant = Box<HydratedAward>;
	type Identifier = AwardId;
	type URL = AwardRequest;

	fn into_hydrated_variant(self) -> Option<Self::HydratedVariant> {
		self.try_into_hydrated()
	}

	fn id(&self) -> &Self::Identifier {
		&self.id
	}

	fn url_for_id(id: &Self::Identifier) -> Self::URL {
		AwardRequest::builder().award_id(id.clone()).build()
	}

	fn get_entries(response: <Self::URL as StatsAPIRequestUrl>::Response) -> impl IntoIterator<Item=Self>
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
	use crate::awards::AwardRequest;
	use crate::StatsAPIRequestUrlBuilderExt;

	#[tokio::test]
	async fn parse_this_season() {
		let _response = AwardRequest::builder().build_and_get().await.unwrap();
	}
}
