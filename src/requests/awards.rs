use crate::league::{League, LeagueId};
use crate::request::StatsAPIRequestUrl;
use crate::season::SeasonId;
use crate::sports::{Sport, SportId};
use crate::types::Copyright;
use bon::Builder;
use derive_more::{Deref, DerefMut, From};
use mlb_api_proc::{EnumDeref, EnumDerefMut, EnumTryAs, EnumTryAsMut, EnumTryInto};
use serde::Deserialize;
use std::fmt::{Display, Formatter};

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

#[derive(Debug, Deserialize, Eq, Clone, From, EnumTryAs, EnumTryAsMut, EnumTryInto, EnumDeref, EnumDerefMut)]
#[serde(untagged)]
pub enum Award {
	Hydrated(Box<HydratedAward>),
	Identifiable(IdentifiableAward),
}

id_only_eq_impl!(Award, id);

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

impl<S: award_request_builder::State + award_request_builder::IsComplete> crate::request::StatsAPIRequestUrlBuilderExt for AwardRequestBuilder<S> {
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

tiered_request_entry_cache_impl!(AwardRequest => |id: AwardId| { AwardRequest::builder().award_id(id.clone()).build() }.awards => Award => Box<HydratedAward>);

#[cfg(test)]
mod tests {
	use crate::awards::AwardRequest;
	use crate::request::StatsAPIRequestUrlBuilderExt;

	#[tokio::test]
	async fn parse_this_season() {
		let _response = AwardRequest::builder().build_and_get().await.unwrap();
	}
}
