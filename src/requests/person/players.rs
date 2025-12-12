use crate::gen_params;
use crate::requests::person::people::PeopleResponse;
use crate::seasons::season::SeasonId;
use crate::sports::SportId;
use crate::StatsAPIRequestUrl;
use bon::Builder;
use std::fmt::{Display, Formatter};

#[derive(Builder)]
#[builder(derive(Into))]
pub struct PlayersRequest {
	#[builder(into)]
	#[builder(default)]
	id: SportId,
	#[builder(into)]
	season: Option<SeasonId>,
}

impl<S: players_request_builder::State + players_request_builder::IsComplete> crate::requests::links::StatsAPIRequestUrlBuilderExt for PlayersRequestBuilder<S> {
	type Built = PlayersRequest;
}

impl Display for PlayersRequest {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "http://statsapi.mlb.com/api/v1/sports/{}/players{}", self.id, gen_params! { "sportId": self.id, "season"?: self.season })
	}
}

impl StatsAPIRequestUrl for PlayersRequest {
	type Response = PeopleResponse<()>;
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{StatsAPIRequestUrlBuilderExt, TEST_YEAR};

	#[tokio::test]
	#[cfg_attr(not(feature = "_heavy_tests"), ignore)]
	async fn parse_all_players_all_seasons_mlb() {
		for season in 1876..=TEST_YEAR {
			let _response = PlayersRequest::builder().season(season).build_and_get().await.unwrap();
		}
	}

	#[tokio::test]
	async fn parse_all_players_mlb() {
		let _response = PlayersRequest::builder().build_and_get().await.unwrap();
	}
}
