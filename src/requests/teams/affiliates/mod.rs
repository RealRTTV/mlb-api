use crate::gen_params;
use crate::seasons::season::SeasonId;
use crate::teams::team::TeamId;
use crate::teams::TeamsResponse;
use crate::StatsAPIRequestUrl;
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

impl<S: team_affiliates_request_builder::State> crate::requests::links::StatsAPIRequestUrlBuilderExt for TeamAffiliatesRequestBuilder<S> where S: team_affiliates_request_builder::IsComplete {
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
	use crate::request::Error as RequestError;
	use crate::teams::affiliates::TeamAffiliatesRequest;
	use crate::teams::TeamsRequest;
	use crate::StatsAPIRequestUrlBuilderExt;
	use chrono::{Datelike, Local};

	#[tokio::test]
	async fn all_mlb_teams() {
		for team in TeamsRequest::builder().build_and_get().await.unwrap().teams {
			let _affiliates = TeamAffiliatesRequest::builder().team_id(team.id).build_and_get().await.unwrap();
		}
	}

	#[tokio::test]
	#[cfg_attr(not(feature = "_heavy_tests"), ignore)]
	async fn all_mlb_teams_all_seasons() {
		for season in 1876..=Local::now().year() as _ {
			for team in TeamsRequest::builder().build_and_get().await.unwrap().teams {
				dbg!(team.id);
				dbg!(&*team.try_as_named().unwrap().name);
				let affiliates_result = TeamAffiliatesRequest::builder().team_id(team.id).season(season).build_and_get().await;
				match affiliates_result {
					Ok(_) => {}
					Err(RequestError::StatsAPI(_)) => {},
					Err(e) => panic!("{e}"),
				}
			}
		}
	}
}
