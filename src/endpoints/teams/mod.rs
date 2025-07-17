pub mod affiliates;
pub mod history;
pub mod stats;
pub mod team;

use crate::endpoints::StatsAPIUrl;
use crate::endpoints::sports::SportId;
use crate::endpoints::teams::team::Team;
use crate::gen_params;
use crate::types::Copyright;
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

pub struct TeamsEndpointUrl {
	pub sport_id: Option<SportId>,
	pub season: Option<u16>,
}

impl Display for TeamsEndpointUrl {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "http://statsapi.mlb.com/api/v1/teams{}", gen_params! { "sportId"?: self.sport_id, "season"?: self.season })
	}
}

impl StatsAPIUrl for TeamsEndpointUrl {
	type Response = TeamsResponse;
}

#[cfg(test)]
mod tests {
	use super::*;
	use chrono::{Datelike, Local};

	#[tokio::test]
	#[cfg_attr(not(feature = "_heavy_tests"), ignore)]
	async fn parse_all_teams_all_seasons() {
		// let json = reqwest::get(TeamsEndpointUrl { sport_id: None, season: Some(2009) }.to_string()).await.unwrap().bytes().await.unwrap();
		// let mut de = serde_json::Deserializer::from_slice(&json);
		// let _response: TeamsResponse = serde_path_to_error::deserialize(&mut de).unwrap();
		for season in 1871..=Local::now().year() as _ {
			let _response = TeamsEndpointUrl { sport_id: None, season: Some(season) }.get().await.unwrap();
		}
	}

	#[tokio::test]
	async fn parse_all_mlb_teams_this_season() {
		let _response = TeamsEndpointUrl {
			sport_id: Some(SportId::default()),
			season: None,
		}
		.get()
		.await
		.unwrap();
	}
}
