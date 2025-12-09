use crate::gen_params;
use crate::people::PeopleResponse;
use crate::seasons::season::SeasonId;
use crate::sports::SportId;
use crate::StatsAPIRequestUrl;
use bon::Builder;
use std::fmt::{Display, Formatter};

#[derive(Builder)]
#[builder(derive(Into))]
pub struct SportsPlayersRequest {
	#[builder(into)]
	#[builder(default)]
	id: SportId,
	#[builder(into)]
	season: Option<SeasonId>,
}

impl<S: sports_players_request_builder::State> crate::requests::links::StatsAPIRequestUrlBuilderExt for SportsPlayersRequestBuilder<S> where S: sports_players_request_builder::IsComplete {
	type Built = SportsPlayersRequest;
}

impl Display for SportsPlayersRequest {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "http://statsapi.mlb.com/api/v1/sports/{}/players{}", self.id, gen_params! { "sportId": self.id, "season"?: self.season })
	}
}

impl StatsAPIRequestUrl for SportsPlayersRequest {
	type Response = PeopleResponse<()>;
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::StatsAPIRequestUrlBuilderExt;
	use chrono::{Datelike, Local};

	#[tokio::test]
	#[cfg_attr(not(feature = "_heavy_tests"), ignore)]
	async fn parse_all_players_all_seasons_mlb() {
		for season in 1876..=Local::now().year() as _ {
			let _response = SportsPlayersRequest::builder().season(season).build_and_get().await.unwrap();
		}
	}

	#[tokio::test]
	async fn parse_all_players_mlb() {
		let _response = SportsPlayersRequest::builder().build_and_get().await.unwrap();
	}
}
