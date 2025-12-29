use crate::person::people::PeopleResponse;
use crate::season::SeasonId;
use crate::team::TeamId;
use crate::types::MLB_API_DATE_FORMAT;
use crate::request::StatsAPIRequestUrl;
use bon::Builder;
use chrono::NaiveDate;
use std::fmt::{Display, Formatter};

#[derive(Builder)]
#[builder(derive(Into))]
pub struct CoachesRequest {
    #[builder(into)]
    team_id: TeamId,
    #[builder(into)]
    season: Option<SeasonId>,
    date: Option<NaiveDate>,
}

impl<S: coaches_request_builder::State + coaches_request_builder::IsComplete> crate::request::StatsAPIRequestUrlBuilderExt for CoachesRequestBuilder<S> {
    type Built = CoachesRequest;
}

impl Display for CoachesRequest {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "http://statsapi.mlb.com/api/v1/teams/{}/coaches{}", self.team_id, gen_params! { "season"?: self.season, "date"?: self.date.as_ref().map(|date| date.format(MLB_API_DATE_FORMAT)) })
    }
}

impl StatsAPIRequestUrl for CoachesRequest {
    type Response = PeopleResponse<()>;
}

#[cfg(test)]
mod tests {
    use crate::request::StatsAPIRequestUrlBuilderExt;
    use crate::team::coaches::CoachesRequest;
	use crate::team::teams::TeamsRequest;
    use crate::TEST_YEAR;

    #[tokio::test]
    #[cfg_attr(not(feature = "_heavy_tests"), ignore)]
    async fn test_heavy() {
        let season = TEST_YEAR;
        let teams = TeamsRequest::mlb_teams().season(season).build_and_get().await.unwrap();
        for team in teams.teams {
            let _ = CoachesRequest::builder().team_id(team.id).season(season).build_and_get().await.unwrap();
        }
    }
}
