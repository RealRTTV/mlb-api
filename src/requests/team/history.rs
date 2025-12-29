use crate::season::SeasonId;
use crate::team::TeamId;
use crate::team::teams::TeamsResponse;
use crate::request::StatsAPIRequestUrl;
use bon::Builder;
use std::fmt::{Display, Formatter};

/// History of a [`TeamId`] throughout the years.
/// For example, the team history of the Los Angeles Dodgers would include those of the Brooklyn Dodgers.
#[derive(Builder)]
#[builder(derive(Into))]
pub struct TeamHistoryRequest {
	#[builder(into)]
	team_id: TeamId,
	#[builder(into)]
	start_season: Option<SeasonId>,
	#[builder(into)]
	end_season: Option<SeasonId>,
}

impl<S: team_history_request_builder::State + team_history_request_builder::IsComplete> crate::request::StatsAPIRequestUrlBuilderExt for TeamHistoryRequestBuilder<S> {
	type Built = TeamHistoryRequest;
}

impl Display for TeamHistoryRequest {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "http://statsapi.mlb.com/api/v1/teams/{}/history{params}", self.team_id, params = gen_params! { "startSeason"?: self.start_season, "endSeason"?: self.end_season })
	}
}

impl StatsAPIRequestUrl for TeamHistoryRequest {
	type Response = TeamsResponse;
}

#[cfg(test)]
mod tests {
	use crate::team::history::TeamHistoryRequest;
	use crate::team::teams::TeamsRequest;
	use crate::request::StatsAPIRequestUrlBuilderExt;

	#[tokio::test]
	async fn all_mlb_teams() {
		for team in TeamsRequest::mlb_teams().build_and_get().await.unwrap().teams {
			let _history = TeamHistoryRequest::builder().team_id(team.id).build_and_get().await.unwrap();
		}
	}
}
