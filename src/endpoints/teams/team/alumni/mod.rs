use crate::gen_params;
use crate::people::PeopleResponse;
use crate::seasons::season::SeasonId;
use crate::teams::team::TeamId;
use crate::StatsAPIEndpointUrl;
use bon::Builder;
use std::fmt::{Display, Formatter};

#[derive(Builder)]
#[builder(derive(Into))]
pub struct AlumniEndpoint {
	#[builder(into)]
	team_id: TeamId,
	#[builder(into)]
	season: Option<SeasonId>,
}

impl<S: alumni_endpoint_builder::State> crate::endpoints::links::StatsAPIEndpointUrlBuilderExt for AlumniEndpointBuilder<S> where S: alumni_endpoint_builder::IsComplete {
	type Built = AlumniEndpoint;
}

impl Display for AlumniEndpoint {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "http://statsapi.mlb.com/api/v1/teams/{}/alumni{}", self.team_id, gen_params! { "season"?: self.season })
	}
}

impl StatsAPIEndpointUrl for AlumniEndpoint {
	type Response = PeopleResponse<()>;
}

#[cfg(test)]
mod tests {
	use crate::teams::team::alumni::AlumniEndpoint;
	use crate::teams::TeamsEndpoint;
	use crate::StatsAPIEndpointUrlBuilderExt;
	use chrono::{Datelike, Local};

	#[tokio::test]
	#[cfg_attr(not(feature = "_heavy_tests"), ignore)]
	async fn test_heavy() {
		let season = Local::now().year() as u32;
		let teams = TeamsEndpoint::builder().season(season).build_and_get().await.unwrap();
		for team in teams.teams {
			let _ = crate::serde_path_to_error_parse(AlumniEndpoint::builder().team_id(team.id).season(season).build());
		}
	}
}
