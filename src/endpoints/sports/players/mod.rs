use crate::StatsAPIEndpointUrl;
use crate::sports::SportId;
use crate::gen_params;
use std::fmt::{Display, Formatter};
use crate::people::PeopleResponse;

#[derive(Default)]
pub struct SportsPlayersEndpoint {
	pub id: SportId,
	pub season: Option<u16>,
}

impl Display for SportsPlayersEndpoint {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "http://statsapi.mlb.com/api/v1/sports/{}/players{}", self.id, gen_params! { "sportId": self.id, "season"?: self.season })
	}
}

impl StatsAPIEndpointUrl for SportsPlayersEndpoint {
	type Response = PeopleResponse<()>;
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::StatsAPIEndpointUrl;
	use crate::sports::SportId;
	use chrono::{Datelike, Local};

	#[tokio::test]
	#[cfg_attr(not(feature = "_heavy_tests"), ignore)]
	async fn parse_all_players_all_seasons_mlb() {
		for season in 1876..=Local::now().year() as _ {
			let _response = SportsPlayersEndpoint { id: SportId::default(), season: Some(season) }.get().await.unwrap();
		}
	}

	#[tokio::test]
	async fn parse_all_players_mlb() {
		let _response = SportsPlayersEndpoint { id: SportId::default(), season: None }.get().await.unwrap();
	}
}
