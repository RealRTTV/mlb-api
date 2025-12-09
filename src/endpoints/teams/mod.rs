pub mod affiliates;
pub mod history;
pub mod stats;
pub mod team;

use crate::gen_params;
use crate::seasons::season::SeasonId;
use crate::sports::SportId;
use crate::teams::team::Team;
use crate::types::Copyright;
use crate::StatsAPIEndpointUrl;
use bon::Builder;
use serde::Deserialize;
use std::fmt::{Display, Formatter};

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
#[builder(derive(Into))]
pub struct TeamsEndpoint {
	#[builder(into)]
	sport_id: Option<SportId>,
	#[builder(into)]
	season: Option<SeasonId>,
}

impl<S: teams_endpoint_builder::State> crate::endpoints::links::StatsAPIEndpointUrlBuilderExt for TeamsEndpointBuilder<S> where S: teams_endpoint_builder::IsComplete {
	type Built = TeamsEndpoint;
}

impl Display for TeamsEndpoint {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "http://statsapi.mlb.com/api/v1/teams{}", gen_params! { "sportId"?: self.sport_id, "season"?: self.season })
	}
}

impl StatsAPIEndpointUrl for TeamsEndpoint {
	type Response = TeamsResponse;
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::StatsAPIEndpointUrlBuilderExt;
	use chrono::{Datelike, Local};

	#[tokio::test]
	#[cfg_attr(not(feature = "_heavy_tests"), ignore)]
	async fn parse_all_teams_all_seasons() {
		for season in 1871..=Local::now().year() as _ {
			let _response = TeamsEndpoint::builder().season(season).build_and_get().await.unwrap();
		}
	}

	#[tokio::test]
	async fn parse_all_mlb_teams_this_season() {
		let _response = TeamsEndpoint::builder()
		.build_and_get()
		.await
		.unwrap();
	}
}
