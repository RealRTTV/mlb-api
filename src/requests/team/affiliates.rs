use crate::season::SeasonId;
use crate::requests::team::TeamId;
use crate::requests::team::teams::TeamsResponse;
use crate::request::StatsAPIRequestUrl;
use bon::Builder;
use std::fmt::{Display, Formatter};

#[derive(Builder)]
#[builder(derive(Into))]
pub struct TeamAffiliatesRequest {
	#[builder(into)]
	team_id: TeamId,
	#[builder(into)]
	season: Option<SeasonId>,
}

impl<S: team_affiliates_request_builder::State + team_affiliates_request_builder::IsComplete> crate::request::StatsAPIRequestUrlBuilderExt for TeamAffiliatesRequestBuilder<S> {
	type Built = TeamAffiliatesRequest;
}

impl Display for TeamAffiliatesRequest {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "http://statsapi.mlb.com/api/v1/teams/{}/affiliates{params}", self.team_id, params = gen_params! { "season"?: self.season })
	}
}

impl StatsAPIRequestUrl for TeamAffiliatesRequest {
	type Response = TeamsResponse;
}

#[cfg(test)]
mod tests {
	use crate::request::{Error as RequestError, StatsAPIRequestUrlBuilderExt};
	use crate::requests::team::affiliates::TeamAffiliatesRequest;
	use crate::requests::team::teams::TeamsRequest;
	use crate::TEST_YEAR;

	#[tokio::test]
	async fn all_mlb_teams() {
		for team in TeamsRequest::mlb_teams().build_and_get().await.unwrap().teams {
			let _affiliates = TeamAffiliatesRequest::builder().team_id(team.id).build_and_get().await.unwrap();
		}
	}

	#[tokio::test]
	#[cfg_attr(not(feature = "_heavy_tests"), ignore)]
	async fn all_mlb_teams_all_seasons() {
		for season in 1876..=TEST_YEAR {
			for team in TeamsRequest::mlb_teams().build_and_get().await.unwrap().teams {
				let affiliates_result = TeamAffiliatesRequest::builder().team_id(team.id).season(season).build_and_get().await;
				match affiliates_result {
					Ok(_) | Err(RequestError::StatsAPI(_)) => {},
					Err(e) => panic!("{e}"),
				}
			}
		}
	}
}
