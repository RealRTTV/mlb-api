use crate::endpoints::Url;
use crate::endpoints::teams::team::TeamId;
use crate::gen_params;
use std::fmt::{Display, Formatter};
use serde::Deserialize;
use crate::endpoints::person::Ballplayer;
use crate::types::Copyright;

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AlumniResponse {
    pub copyright: Copyright,
    pub people: Vec<Ballplayer>,
}

pub struct AlumniEndpointUrl {
    team_id: TeamId,
    season: u16,
}

impl Display for AlumniEndpointUrl {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "http://statsapi.mlb.com/api/v1/teams/{}/alumni{params}",
            self.team_id,
            params = gen_params! { "season": self.season }
        )
    }
}

impl Url<AlumniResponse> for AlumniEndpointUrl {}

#[cfg(test)]
mod tests {
    use crate::endpoints::Url;
    use crate::endpoints::teams::TeamsEndpointUrl;
    use crate::endpoints::teams::team::alumni::{AlumniEndpointUrl, AlumniResponse};
    use chrono::{Datelike, Local};

    #[tokio::test]
    async fn test_all_players_this_year_all_teams() {
        let season = Local::now().year() as _;
        let teams = TeamsEndpointUrl {
            sport_id: None,
            season: Some(season),
        }
        .get()
        .await
        .unwrap();
        for team in teams.teams.into_iter() {
            dbg!(&*team.name);
            let json = reqwest::get(
                AlumniEndpointUrl {
                    team_id: team.id,
                    season,
                }
                .to_string(),
            )
            .await
            .unwrap()
            .bytes()
            .await
            .unwrap();
            let mut de = serde_json::Deserializer::from_slice(&json);
            let result: Result<AlumniResponse, serde_path_to_error::Error<_>> =
                serde_path_to_error::deserialize(&mut de);
            match result {
                Ok(_) => {}
                Err(e) if format!("{:?}", e.inner()).contains("missing field `copyright`") => {}
                Err(e) => panic!("Err: {:?}", e),
            }
        }
    }
}
