use std::fmt::{Display, Formatter};
use crate::endpoints::sports::SportId;
use crate::endpoints::Url;
use crate::gen_params;
use serde::Deserialize;
use crate::endpoints::person::HydratedPerson;
use crate::types::Copyright;

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SportsPlayersResponse {
    pub copyright: Copyright,
    pub people: Vec<HydratedPerson>,
}


#[derive(Default)]
pub struct SportsPlayersEndpointUrl {
    pub id: SportId,
    pub season: Option<u16>,
}

impl Display for SportsPlayersEndpointUrl {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "http://statsapi.mlb.com/api/v1/sports/{id}/players{params}", id = self.id, params = gen_params! { "sportId": self.id, "season"?: self.season })
    }
}

impl Url<SportsPlayersResponse> for SportsPlayersEndpointUrl {}

#[cfg(test)]
mod tests {
    use chrono::{Datelike, Local};
    use crate::endpoints::sports::SportId;
    use crate::endpoints::Url;
    use super::*;

    #[tokio::test]
    async fn parse_all_players_mlb() {
        for season in 1876..=Local::now().year() as _ {
            let _response = SportsPlayersEndpointUrl { id: SportId::default(), season: Some(season) }.get().await.unwrap();
        }
    }
}
