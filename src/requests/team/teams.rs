use crate::season::SeasonId;
use crate::types::Copyright;
use crate::request::StatsAPIRequestUrl;
use bon::Builder;
use serde::Deserialize;
use std::fmt::{Display, Formatter};
use crate::sport::SportId;
use crate::team::Team;

/// Hydrations:
/// * `previousSchedule`
/// * `nextSchedule`
/// * `venue`
/// * `springVenue`
/// * `social`
/// * `deviceProperties`
/// * `game(promotions)`
/// * `game(atBatPromotions)`
/// * `game(tickets)`
/// * `game(atBatTickets)`
/// * `game(sponsorships)`
/// * `league`
/// * `person`
/// * `sport`
/// * `standings`
/// * `division`
/// * `xrefId`
/// * `location`
#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TeamsResponse {
	pub copyright: Copyright,
	pub teams: Vec<Team>,
}

#[derive(Builder)]
#[builder(start_fn(vis = ""))]
#[builder(derive(Into))]
pub struct TeamsRequest {
	#[builder(setters(vis = "", name = "sport_id_internal"))]
	sport_id: Option<SportId>,
	#[builder(into)]
	season: Option<SeasonId>,
}

impl TeamsRequest {
	pub fn for_sport(sport_id: SportId) -> TeamsRequestBuilder<teams_request_builder::SetSportId> {
		Self::builder().sport_id_internal(sport_id)
	}

	pub fn mlb_teams() -> TeamsRequestBuilder<teams_request_builder::SetSportId> {
		Self::builder().sport_id_internal(SportId::MLB)
	}

	pub fn all_sports() -> TeamsRequestBuilder {
		Self::builder()
	}
}

impl<S: teams_request_builder::State + teams_request_builder::IsComplete> crate::request::StatsAPIRequestUrlBuilderExt for TeamsRequestBuilder<S> {
	type Built = TeamsRequest;
}

impl Display for TeamsRequest {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "http://statsapi.mlb.com/api/v1/teams{}", gen_params! { "sportId"?: self.sport_id, "season"?: self.season })
	}
}

impl StatsAPIRequestUrl for TeamsRequest {
	type Response = TeamsResponse;
}

#[cfg(test)]
mod tests {
	use crate::request::StatsAPIRequestUrlBuilderExt;
	use crate::TEST_YEAR;
	use super::*;

	#[tokio::test]
	#[cfg_attr(not(feature = "_heavy_tests"), ignore)]
	async fn parse_all_teams_all_seasons() {
		for season in 1871..=TEST_YEAR {
			let _response = TeamsRequest::all_sports().season(season).build_and_get().await.unwrap();
		}
	}

	#[tokio::test]
	async fn parse_all_mlb_teams_this_season() {
		let _ = TeamsRequest::mlb_teams().build_and_get().await.unwrap();
	}
}
