use crate::gen_params;
use crate::people::PeopleResponse;
use crate::seasons::season::SeasonId;
use crate::teams::team::TeamId;
use crate::types::MLB_API_DATE_FORMAT;
use crate::StatsAPIEndpointUrl;
use bon::Builder;
use chrono::NaiveDate;
use std::fmt::{Display, Formatter};

#[derive(Builder)]
#[builder(derive(Into))]
pub struct CoachesEndpoint {
    #[builder(into)]
    team_id: TeamId,
    #[builder(into)]
    season: Option<SeasonId>,
    date: Option<NaiveDate>,
}

impl<S: coaches_endpoint_builder::State> crate::endpoints::links::StatsAPIEndpointUrlBuilderExt for CoachesEndpointBuilder<S> where S: coaches_endpoint_builder::IsComplete {
    type Built = CoachesEndpoint;
}

impl Display for CoachesEndpoint {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "http://statsapi.mlb.com/api/v1/teams/{}/coaches{}", self.team_id, gen_params! { "season"?: self.season, "date"?: self.date.as_ref().map(|date| date.format(MLB_API_DATE_FORMAT)) })
    }
}

impl StatsAPIEndpointUrl for CoachesEndpoint {
    type Response = PeopleResponse<()>;
}

#[cfg(test)]
mod tests {
    use crate::teams::team::coaches::CoachesEndpoint;
    use crate::teams::TeamsEndpoint;
    use crate::StatsAPIEndpointUrlBuilderExt;
    use chrono::{Datelike, Local};

    #[tokio::test]
    #[cfg_attr(not(feature = "_heavy_tests"), ignore)]
    async fn test_heavy() {
        let season = Local::now().year() as u32;
        let teams = TeamsEndpoint::builder().season(season).build_and_get().await.unwrap();
        for team in teams.teams {
            let _ = crate::serde_path_to_error_parse(CoachesEndpoint::builder().team_id(team.id).season(season).build()).await;
        }
    }
}
