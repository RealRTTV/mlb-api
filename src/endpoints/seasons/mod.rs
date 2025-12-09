pub mod season;

use crate::gen_params;
use crate::seasons::season::{Season, SeasonId};
use crate::sports::SportId;
use crate::types::Copyright;
use crate::StatsAPIEndpointUrl;
use bon::Builder;
use serde::Deserialize;
use std::fmt::{Display, Formatter};

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SeasonsResponse {
	pub copyright: Copyright,
	pub seasons: Vec<Season>,
}

#[derive(Builder)]
#[builder(derive(Into))]
pub struct SeasonsEndpoint {
	#[builder(into)]
	#[builder(default)]
	sport_id: SportId,
	#[builder(into)]
	season: Option<SeasonId>,
}

impl<S: seasons_endpoint_builder::State> crate::endpoints::links::StatsAPIEndpointUrlBuilderExt for SeasonsEndpointBuilder<S> where S: seasons_endpoint_builder::IsComplete {
	type Built = SeasonsEndpoint;
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
	use crate::seasons::SeasonsEndpoint;
	use crate::sports::SportsEndpoint;
	use crate::StatsAPIEndpointUrlBuilderExt;
	use chrono::{Datelike, Local};

	#[tokio::test]
	#[cfg_attr(not(feature = "_heavy_tests"), ignore)]
	async fn parses_all_seasons() {
		let all_sport_ids = SportsEndpoint::builder().build_and_get().await.unwrap().sports.into_iter().map(|sport| sport.id).collect::<Vec<_>>();

		for season in 1871..=Local::now().year() as _ {
			for id in all_sport_ids.iter().copied() {
				let _response = SeasonsEndpoint::builder().sport_id(id).season(season).build_and_get().await.unwrap();
			}
		}
	}

	#[tokio::test]
	async fn parse_this_season_mlb() {
		let _response = SeasonsEndpoint::builder().build_and_get().await.unwrap();
	}
}
