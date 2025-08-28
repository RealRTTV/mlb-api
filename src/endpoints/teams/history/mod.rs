use std::fmt::{Display, Formatter};
use crate::endpoints::StatsAPIUrl;
use crate::endpoints::teams::team::TeamId;
use crate::endpoints::teams::TeamsResponse;
use crate::gen_params;

/// History of a [`TeamId`] throughout the years.
/// For example, the team history of the Los Angeles Dodgers would include those of the Brooklyn Dodgers.
pub struct TeamHistoryEndpointUrl {
	pub team_id: TeamId,
	pub start_season: Option<u16>,
	pub end_season: Option<u16>,
}

impl Display for TeamHistoryEndpointUrl {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "http://statsapi.mlb.com/api/v1/teams/{}/history{params}", self.team_id, params = gen_params! { "startSeason"?: self.start_season, "endSeason"?: self.end_season })
	}
}

impl StatsAPIUrl for TeamHistoryEndpointUrl {
	type Response = TeamsResponse;
}

#[cfg(test)]
mod tests {
	use crate::endpoints::sports::SportId;
	use crate::endpoints::StatsAPIUrl;
	use crate::endpoints::teams::history::TeamHistoryEndpointUrl;
	use crate::endpoints::teams::TeamsEndpointUrl;

	#[tokio::test]
	async fn all_mlb_teams() {
		for team in (TeamsEndpointUrl { sport_id: Some(SportId::MLB), season: None }).get().await.unwrap().teams {
			let _history = TeamHistoryEndpointUrl { team_id: team.id, start_season: None, end_season: None }.get().await.unwrap();
		}
	}
}
