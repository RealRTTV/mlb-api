use crate::gen_params;
use crate::requests::person::people::PeopleResponse;
use crate::seasons::season::SeasonId;
use crate::teams::team::TeamId;
use crate::StatsAPIRequestUrl;
use bon::Builder;
use std::fmt::{Display, Formatter};

#[derive(Builder)]
#[builder(derive(Into))]
pub struct AlumniRequest {
	#[builder(into)]
	team_id: TeamId,
	#[builder(into)]
	season: Option<SeasonId>,
}

impl<S: alumni_request_builder::State + alumni_request_builder::IsComplete> crate::requests::links::StatsAPIRequestUrlBuilderExt for AlumniRequestBuilder<S> {
	type Built = AlumniRequest;
}

impl Display for AlumniRequest {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "http://statsapi.mlb.com/api/v1/teams/{}/alumni{}", self.team_id, gen_params! { "season"?: self.season })
	}
}

impl StatsAPIRequestUrl for AlumniRequest {
	type Response = PeopleResponse<()>;
}

#[cfg(test)]
mod tests {
	use crate::teams::team::alumni::AlumniRequest;
	use crate::teams::TeamsRequest;
	use crate::StatsAPIRequestUrlBuilderExt;
	use chrono::{Datelike, Local};

	#[tokio::test]
	#[cfg_attr(not(feature = "_heavy_tests"), ignore)]
	async fn test_heavy() {
		let season = Local::now().year() as u32;
		let teams = TeamsRequest::builder().season(season).build_and_get().await.unwrap();
		for team in teams.teams {
			let _ = crate::serde_path_to_error_parse(AlumniRequest::builder().team_id(team.id).season(season).build());
		}
	}
}
