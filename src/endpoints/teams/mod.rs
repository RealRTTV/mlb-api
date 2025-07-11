pub mod affiliates;
pub mod history;
pub mod stats;
pub mod team;

use crate::endpoints::Url;
use crate::endpoints::sports::SportId;
use crate::endpoints::teams::team::Team;
use crate::gen_params;
use crate::types::Copyright;
use serde::Deserialize;
use std::fmt::{Display, Formatter};

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
		write!(f, "http://statsapi.mlb.com/api/v1/teams{params}", params = gen_params! { "sportId"?: self.sport_id, "season"?: self.season })
	}
}

impl Url<TeamsResponse> for TeamsEndpointUrl {}

#[cfg(test)]
mod tests {
	use super::*;
	use chrono::{Datelike, Local};

	#[tokio::test]
	async fn parse_all_teams() {
		// let json = reqwest::get(TeamsEndpointUrl { sport_id: None, season: Some(2009) }.to_string()).await.unwrap().bytes().await.unwrap();
		// let mut de = serde_json::Deserializer::from_slice(&json);
		// let _response: TeamsResponse = serde_path_to_error::deserialize(&mut de).unwrap();
		for season in 1871..=Local::now().year() as _ {
			let _response = TeamsEndpointUrl { sport_id: None, season: Some(season) }.get().await.unwrap();
		}
	}
}
