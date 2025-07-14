use crate::endpoints::StatsAPIUrl;
use crate::endpoints::person::HydratedPerson;
use crate::endpoints::sports::SportId;
use crate::gen_params;
use crate::types::Copyright;
use serde::Deserialize;
use std::fmt::{Display, Formatter};

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SportsPlayersResponse {
	pub copyright: Copyright,
	pub people: Vec<HydratedPerson>,
}

#[derive(Default)]
pub struct SportsPlayersEndpointUrl {
	pub id: SportId,
	pub season: Option<u16>,
}

impl Display for SportsPlayersEndpointUrl {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "http://statsapi.mlb.com/api/v1/sports/{id}/players{params}", id = self.id, params = gen_params! { "sportId": self.id, "season"?: self.season })
	}
}

impl StatsAPIUrl<SportsPlayersResponse> for SportsPlayersEndpointUrl {}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::endpoints::StatsAPIUrl;
	use crate::endpoints::sports::SportId;
	use chrono::{Datelike, Local};

	#[tokio::test]
	#[cfg_attr(not(feature = "_heavy_tests"), ignore)]
	async fn parse_all_players_all_seasons_mlb() {
		for season in 1876..=Local::now().year() as _ {
			let _response = SportsPlayersEndpointUrl { id: SportId::default(), season: Some(season) }.get().await.unwrap();
		}
	}
	
	#[tokio::test]
	async fn parse_all_players_mlb() {
		let _response = SportsPlayersEndpointUrl { id: SportId::default(), season: None }.get().await.unwrap();
	}
}
