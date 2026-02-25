//! Returns a list of personnel associated with a [`Team`](super::Team); [`PeopleResponse`]

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
pub struct PersonnelRequest<H: PersonHydrations = ()> {
    #[builder(into)]
    team_id: TeamId,
    #[builder(into)]
    season: Option<SeasonId>,
    date: Option<NaiveDate>,
    hydrations: H::RequestData,
}

impl PersonnelRequest {
    pub fn for_team(team_id: impl Into<TeamId>) -> PersonnelRequestBuilder<(), personnel_request_builder::SetHydrations<personnel_request_builder::SetTeamId>> {
        Self::builder()
            .team_id(team_id)
            .hydrations(<() as Hydrations>::RequestData::default())
    }
}

impl<H: PersonHydrations, S: personnel_request_builder::State + personnel_request_builder::IsComplete> crate::request::RequestURLBuilderExt for PersonnelRequestBuilder<H, S> {
    type Built = PersonnelRequest<H>;
}

impl<H: PersonHydrations> Display for PersonnelRequest<H> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let hydrations = Some(H::hydration_text(&self.hydrations)).filter(|h| !h.is_empty());
        write!(f, "http://statsapi.mlb.com/api/v1/teams/{}/personnel{}", self.team_id, gen_params! { "season"?: self.season, "date"?: self.date.as_ref().map(|date| date.format(MLB_API_DATE_FORMAT)), "hydrate"?: hydrations })
    }
}

impl<H: PersonHydrations> RequestURL for PersonnelRequest<H> {
    type Response = PeopleResponse<H>;
}

#[cfg(test)]
mod tests {
    use crate::request::RequestURLBuilderExt;
    use crate::team::personnel::PersonnelRequest;
	use crate::team::TeamsRequest;
	use crate::TEST_YEAR;

	#[tokio::test]
    #[cfg_attr(not(feature = "_heavy_tests"), ignore)]
    async fn test_heavy() {
        let season = TEST_YEAR;
        let teams = TeamsRequest::mlb_teams().season(season).build_and_get().await.unwrap();
        for team in teams.teams {
            let _ = PersonnelRequest::<()>::for_team(team.id).season(season).build_and_get().await.unwrap();
        }
    }
}
