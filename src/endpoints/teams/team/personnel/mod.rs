
use crate::endpoints::StatsAPIUrl;
use crate::endpoints::teams::team::TeamId;
use crate::gen_params;
use std::fmt::{Display, Formatter};
use chrono::NaiveDate;
use crate::endpoints::people::PeopleResponse;
use crate::types::MLB_API_DATE_FORMAT;

pub struct PersonnelEndpointUrl {
    team_id: TeamId,
    season: Option<u16>,
    date: Option<NaiveDate>,
}

impl Display for PersonnelEndpointUrl {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "http://statsapi.mlb.com/api/v1/teams/{}/personnel{}", self.team_id, gen_params! { "season"?: self.season, "date"?: self.date.as_ref().map(|date| date.format(MLB_API_DATE_FORMAT)) })
    }
}

impl StatsAPIUrl for PersonnelEndpointUrl {
    type Response = PeopleResponse;
}

#[cfg(test)]
mod tests {
    use crate::endpoints::StatsAPIUrl;
    use crate::endpoints::teams::TeamsEndpointUrl;
    use crate::endpoints::teams::team::personnel::PersonnelEndpointUrl;
    use chrono::{Datelike, Local};

    #[tokio::test]
    #[cfg_attr(not(feature = "_heavy_tests"), ignore)]
    async fn test_heavy() {
        let season = Local::now().year() as _;
        let teams = TeamsEndpointUrl { sport_id: None, season: Some(season) }.get().await.unwrap();
        for team in teams.teams {
            let json = reqwest::get(PersonnelEndpointUrl { team_id: team.id, season: Some(season), date: None }.to_string()).await.unwrap().bytes().await.unwrap();
            let mut de = serde_json::Deserializer::from_slice(&json);
            let result: Result<<PersonnelEndpointUrl as StatsAPIUrl>::Response, serde_path_to_error::Error<_>> = serde_path_to_error::deserialize(&mut de);
            match result {
                Ok(_) => {}
                Err(e) if format!("{:?}", e.inner()).contains("missing field `copyright`") => {}
                Err(e) => panic!("Err: {:?}", e),
            }
        }
    }
}
