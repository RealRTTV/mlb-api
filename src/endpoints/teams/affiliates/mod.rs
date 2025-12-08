use crate::endpoints::teams::team::TeamId;
use crate::endpoints::teams::TeamsResponse;
use crate::endpoints::StatsAPIEndpointUrl;
use crate::gen_params;
use std::fmt::{Display, Formatter};

pub struct TeamAffiliatesEndpoint {
	pub id: TeamId,
	pub season: Option<u16>,
}

impl Display for TeamAffiliatesEndpoint {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "http://statsapi.mlb.com/api/v1/teams/{}/affiliates{params}", self.id, params = gen_params! { "season"?: self.season })
	}
}

impl StatsAPIEndpointUrl for TeamAffiliatesEndpoint {
	type Response = TeamsResponse;
}

#[cfg(test)]
mod tests {
	use crate::endpoints::sports::SportId;
	use crate::endpoints::teams::affiliates::TeamAffiliatesEndpoint;
	use crate::endpoints::teams::TeamsEndpoint;
	use crate::endpoints::StatsAPIEndpointUrl;
	use crate::request::Error as EndpointError;
	use chrono::{Datelike, Local};

	#[tokio::test]
	async fn all_mlb_teams() {
		for team in (TeamsEndpoint { sport_id: Some(SportId::MLB), season: None }).get().await.unwrap().teams {
			let _affiliates = TeamAffiliatesEndpoint { id: team.id, season: None }.get().await.unwrap();
		}
	}

	#[tokio::test]
	#[cfg_attr(not(feature = "_heavy_tests"), ignore)]
	async fn all_mlb_teams_all_seasons() {
		for season in 1876..=Local::now().year() as _ {
			for team in (TeamsEndpoint { sport_id: Some(SportId::MLB), season: Some(season) }).get().await.unwrap().teams {
				dbg!(team.id);
				dbg!(&*team.try_as_named_ref().unwrap().name);
				let affiliates_result = TeamAffiliatesEndpoint { id: team.id, season: Some(season) }.get().await;
				match affiliates_result {
					Ok(_) => {}
					Err(EndpointError::StatsAPI(_)) => {},
					Err(e) => panic!("{e}"),
				}
			}
		}
	}
}
