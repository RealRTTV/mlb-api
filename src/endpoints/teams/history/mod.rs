use crate::gen_params;
use crate::seasons::season::SeasonId;
use crate::teams::team::TeamId;
use crate::teams::TeamsResponse;
use crate::StatsAPIEndpointUrl;
use bon::Builder;
use std::fmt::{Display, Formatter};

/// History of a [`TeamId`] throughout the years.
/// For example, the team history of the Los Angeles Dodgers would include those of the Brooklyn Dodgers.
#[derive(Builder)]
#[builder(derive(Into))]
pub struct TeamHistoryEndpoint {
	#[builder(into)]
	team_id: TeamId,
	#[builder(into)]
	start_season: Option<SeasonId>,
	#[builder(into)]
	end_season: Option<SeasonId>,
}

impl<S: team_history_endpoint_builder::State> crate::endpoints::links::StatsAPIEndpointUrlBuilderExt for TeamHistoryEndpointBuilder<S> where S: team_history_endpoint_builder::IsComplete {
	type Built = TeamHistoryEndpoint;
}

impl Display for TeamHistoryEndpoint {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "http://statsapi.mlb.com/api/v1/teams/{}/history{params}", self.team_id, params = gen_params! { "startSeason"?: self.start_season, "endSeason"?: self.end_season })
	}
}

impl StatsAPIEndpointUrl for TeamHistoryEndpoint {
	type Response = TeamsResponse;
}

#[cfg(test)]
mod tests {
	use crate::teams::history::TeamHistoryEndpoint;
	use crate::teams::TeamsEndpoint;
	use crate::StatsAPIEndpointUrlBuilderExt;

	#[tokio::test]
	async fn all_mlb_teams() {
		for team in TeamsEndpoint::builder().build_and_get().await.unwrap().teams {
			let _history = TeamHistoryEndpoint::builder().team_id(team.id).build_and_get().await.unwrap();
		}
	}
}
