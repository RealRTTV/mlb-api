use crate::person::people::PeopleResponse;
use crate::season::SeasonId;
use crate::request::RequestURL;
use bon::Builder;
use std::fmt::{Display, Formatter};
use crate::sport::SportId;

#[derive(Builder)]
#[builder(derive(Into))]
pub struct PlayersRequest {
	#[builder(into)]
	#[builder(default)]
	sport_id: SportId,
	#[builder(into)]
	season: Option<SeasonId>,
}

impl<S: players_request_builder::State + players_request_builder::IsComplete> crate::request::RequestURLBuilderExt for PlayersRequestBuilder<S> {
	type Built = PlayersRequest;
}

impl Display for PlayersRequest {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "http://statsapi.mlb.com/api/v1/sports/{}/players{}", self.sport_id, gen_params! { "sportId": self.sport_id, "season"?: self.season })
	}
}

impl RequestURL for PlayersRequest {
	type Response = PeopleResponse<()>;
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::TEST_YEAR;
	use crate::request::RequestURLBuilderExt;

	#[tokio::test]
	#[cfg_attr(not(feature = "_heavy_tests"), ignore)]
	async fn parse_all_players_all_seasons_mlb() {
		for season in 1876..=TEST_YEAR {
			let _response = PlayersRequest::builder().sport_id(SportId::MLB).season(season).build_and_get().await.unwrap();
		}
	}

	#[tokio::test]
	async fn parse_all_players_mlb() {
		let _response = PlayersRequest::builder().sport_id(SportId::MLB).build_and_get().await.unwrap();
	}
}
