use crate::endpoints::StatsAPIUrl;
use crate::endpoints::teams::team::TeamId;
use crate::gen_params;
use std::fmt::{Display, Formatter};
use crate::endpoints::people::PeopleResponse;

pub struct AlumniEndpointUrl {
	team_id: TeamId,
	season: Option<u16>,
}

impl Display for AlumniEndpointUrl {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "http://statsapi.mlb.com/api/v1/teams/{}/alumni{}", self.team_id, gen_params! { "season"?: self.season })
	}
}

impl StatsAPIUrl for AlumniEndpointUrl {
	type Response = PeopleResponse;
}

#[cfg(test)]
mod tests {
	use crate::endpoints::StatsAPIUrl;
	use crate::endpoints::teams::TeamsEndpointUrl;
	use crate::endpoints::teams::team::alumni::AlumniEndpointUrl;
	use chrono::{Datelike, Local};

	#[tokio::test]
	#[cfg_attr(not(feature = "_heavy_tests"), ignore)]
	async fn test_heavy() {
		let season = Local::now().year() as _;
		let teams = TeamsEndpointUrl { sport_id: None, season: Some(season) }.get().await.unwrap();
		for team in teams.teams {
			let json = reqwest::get(AlumniEndpointUrl { team_id: team.id, season: Some(season) }.to_string()).await.unwrap().bytes().await.unwrap();
			let mut de = serde_json::Deserializer::from_slice(&json);
			let result: Result<<AlumniEndpointUrl as StatsAPIUrl>::Response, serde_path_to_error::Error<_>> = serde_path_to_error::deserialize(&mut de);
			match result {
				Ok(_) => {}
				Err(e) if format!("{:?}", e.inner()).contains("missing field `copyright`") => {}
				Err(e) => panic!("Err: {:?}", e),
			}
		}
	}
}
