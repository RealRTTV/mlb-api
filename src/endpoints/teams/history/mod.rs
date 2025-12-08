use std::fmt::{Display, Formatter};
use crate::StatsAPIEndpointUrl;
use crate::teams::team::TeamId;
use crate::teams::TeamsResponse;
use crate::gen_params;

/// History of a [`TeamId`] throughout the years.
/// For example, the team history of the Los Angeles Dodgers would include those of the Brooklyn Dodgers.
pub struct TeamHistoryEndpoint {
	pub team_id: TeamId,
	pub start_season: Option<u16>,
	pub end_season: Option<u16>,
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
	use crate::sports::SportId;
	use crate::StatsAPIEndpointUrl;
	use crate::teams::history::TeamHistoryEndpoint;
	use crate::teams::TeamsEndpoint;

	#[tokio::test]
	async fn all_mlb_teams() {
		for team in (TeamsEndpoint { sport_id: Some(SportId::MLB), season: None }).get().await.unwrap().teams {
			let _history = TeamHistoryEndpoint { team_id: team.id, start_season: None, end_season: None }.get().await.unwrap();
		}
	}
}
