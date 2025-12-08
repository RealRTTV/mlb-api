
use crate::endpoints::StatsAPIEndpointUrl;
use crate::endpoints::teams::team::TeamId;
use crate::gen_params;
use std::fmt::{Display, Formatter};
use chrono::NaiveDate;
use crate::endpoints::people::PeopleResponse;
use crate::types::MLB_API_DATE_FORMAT;

pub struct CoachesEndpoint {
    team_id: TeamId,
    season: Option<u16>,
    date: Option<NaiveDate>,
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
    use crate::endpoints::StatsAPIEndpointUrl;
    use crate::endpoints::teams::TeamsEndpoint;
    use crate::endpoints::teams::team::coaches::CoachesEndpoint;
    use chrono::{Datelike, Local};

    #[tokio::test]
    #[cfg_attr(not(feature = "_heavy_tests"), ignore)]
    async fn test_heavy() {
        let season = Local::now().year() as _;
        let teams = TeamsEndpoint { sport_id: None, season: Some(season) }.get().await.unwrap();
        for team in teams.teams {
            let _ = crate::serde_path_to_error_parse(CoachesEndpoint { team_id: team.id, season: Some(season), date: None }).await;
        }
    }
}
