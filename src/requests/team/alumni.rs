//! Returns a list of alumni associated with a team; [`PeopleResponse`].

use crate::person::people::PeopleResponse;
use crate::season::SeasonId;
use crate::team::TeamId;
use crate::request::RequestURL;
use bon::Builder;
use std::fmt::{Display, Formatter};
use crate::hydrations::Hydrations;
use crate::person::PersonHydrations;

#[derive(Builder)]
#[builder(derive(Into))]
pub struct AlumniRequest<H: PersonHydrations = ()> {
	#[builder(into)]
	team_id: TeamId,
	#[builder(into)]
	season: Option<SeasonId>,
	#[builder(into)]
	hydrations: H::RequestData,
}

impl AlumniRequest {
	pub fn for_team(team_id: impl Into<TeamId>) -> AlumniRequestBuilder<(), alumni_request_builder::SetHydrations<alumni_request_builder::SetTeamId>> {
		Self::builder()
			.team_id(team_id.into())
			.hydrations(<() as Hydrations>::RequestData::default())
	}
}

impl<S: alumni_request_builder::State + alumni_request_builder::IsComplete, H: PersonHydrations> crate::request::RequestURLBuilderExt for AlumniRequestBuilder<H, S> {
	type Built = AlumniRequest<H>;
}

impl<H: PersonHydrations> Display for AlumniRequest<H> {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		let hydrations = Some(H::hydration_text(&self.hydrations)).filter(|s| !s.is_empty());
		write!(f, "http://statsapi.mlb.com/api/v1/teams/{}/alumni{}", self.team_id, gen_params! { "season"?: self.season, "hydrate"?: hydrations })
	}
}

impl<H: PersonHydrations> RequestURL for AlumniRequest<H> {
	type Response = PeopleResponse<H>;
}

#[cfg(test)]
mod tests {
	use crate::request::RequestURLBuilderExt;
	use crate::team::alumni::AlumniRequest;
	use crate::team::TeamsRequest;
	use crate::TEST_YEAR;

	#[tokio::test]
	#[cfg_attr(not(feature = "_heavy_tests"), ignore)]
	async fn test_heavy() {
		let season = TEST_YEAR;
		let teams = TeamsRequest::mlb_teams().season(season).build_and_get().await.unwrap();
		for team in teams.teams {
			let _ = AlumniRequest::<()>::for_team(team.id).season(season).build_and_get().await.unwrap();
		}
	}
}
