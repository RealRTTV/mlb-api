pub mod season;

use crate::endpoints::StatsAPIEndpointUrl;
use crate::endpoints::seasons::season::Season;
use crate::endpoints::sports::SportId;
use crate::gen_params;
use crate::types::Copyright;
use serde::Deserialize;
use std::fmt::{Display, Formatter};

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SeasonsResponse {
	pub copyright: Copyright,
	pub seasons: Vec<Season>,
}

pub struct SeasonsEndpoint {
	sport_id: SportId,
	season: Option<u16>,
}

impl Display for SeasonsEndpoint {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "http://statsapi.mlb.com/api/v1/seasons{}", gen_params! { "sportId": self.sport_id, "season"?: self.season })
	}
}

impl StatsAPIEndpointUrl for SeasonsEndpoint {
	type Response = SeasonsResponse;
}

#[cfg(test)]
mod tests {
	use crate::endpoints::StatsAPIEndpointUrl;
	use crate::endpoints::seasons::SeasonsEndpoint;
	use crate::endpoints::sports::{SportId, SportsEndpoint};
	use chrono::{Datelike, Local};

	#[tokio::test]
	#[cfg_attr(not(feature = "_heavy_tests"), ignore)]
	async fn parses_all_seasons() {
		let all_sport_ids = SportsEndpoint { id: None }.get().await.unwrap().sports.into_iter().map(|sport| sport.id).collect::<Vec<_>>();

		for season in 1871..=Local::now().year() as _ {
			for id in all_sport_ids.iter().copied() {
				let _response = SeasonsEndpoint { sport_id: id, season: Some(season) }.get().await.unwrap();
			}
		}
	}

	#[tokio::test]
	async fn parse_this_season_mlb() {
		let _response = SeasonsEndpoint { sport_id: SportId::default(), season: None }.get().await.unwrap();
	}
}
