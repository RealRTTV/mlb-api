//! Returns a list of coaches affiliated with the team; [`PeopleResponse`]

use crate::person::PeopleResponse;
use crate::season::SeasonId;
use crate::team::TeamId;
use crate::MLB_API_DATE_FORMAT;
use crate::request::RequestURL;
use bon::Builder;
use chrono::NaiveDate;
use std::fmt::{Display, Formatter};
use crate::hydrations::Hydrations;
use crate::person::PersonHydrations;

#[derive(Builder)]
#[builder(derive(Into))]
pub struct CoachesRequest<H: PersonHydrations = ()> {
    #[builder(into)]
    team_id: TeamId,
    #[builder(into)]
    season: Option<SeasonId>,
    date: Option<NaiveDate>,
    #[builder(into)]
    hydrations: H::RequestData,
}

impl CoachesRequest {
    pub fn for_team(team_id: impl Into<TeamId>) -> CoachesRequestBuilder<(), coaches_request_builder::SetHydrations<coaches_request_builder::SetTeamId>> {
        Self::builder()
            .team_id(team_id.into())
            .hydrations(<() as Hydrations>::RequestData::default())
    }
}

impl<H: PersonHydrations, S: coaches_request_builder::State + coaches_request_builder::IsComplete> crate::request::RequestURLBuilderExt for CoachesRequestBuilder<H, S> {
    type Built = CoachesRequest<H>;
}

impl<H: PersonHydrations> Display for CoachesRequest<H> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let hydrations = Some(H::hydration_text(&self.hydrations)).filter(|s| !s.is_empty());
        write!(f, "http://statsapi.mlb.com/api/v1/teams/{}/coaches{}", self.team_id, gen_params! { "season"?: self.season, "date"?: self.date.as_ref().map(|date| date.format(MLB_API_DATE_FORMAT)), "hydrate"?: hydrations })
    }
}

impl<H: PersonHydrations> RequestURL for CoachesRequest<H> {
    type Response = PeopleResponse<H>;
}

#[cfg(test)]
mod tests {
    use crate::request::RequestURLBuilderExt;
    use crate::team::coaches::CoachesRequest;
	use crate::team::TeamsRequest;
    use crate::TEST_YEAR;

    #[tokio::test]
    #[cfg_attr(not(feature = "_heavy_tests"), ignore)]
    async fn test_heavy() {
        let season = TEST_YEAR;
        let teams = TeamsRequest::mlb_teams().season(season).build_and_get().await.unwrap();
        for team in teams.teams {
            let _ = CoachesRequest::<()>::for_team(team.id).season(season).build_and_get().await.unwrap();
        }
    }
}
