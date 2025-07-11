pub mod season;

use std::fmt::{Display, Formatter};
use crate::endpoints::sports::SportId;
use crate::endpoints::Url;
use crate::gen_params;
use serde::Deserialize;
use crate::endpoints::seasons::season::Season;
use crate::types::Copyright;

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SeasonsResponse {
    pub copyright: Copyright,
    pub seasons: Vec<Season>,
}

pub struct SeasonsEndpointUrl {
    sport_id: SportId,
    season: Option<u16>,
}

impl Display for SeasonsEndpointUrl {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "http://statsapi.mlb.com/api/v1/seasons{params}", params = gen_params! { "sportId": self.sport_id, "season"?: self.season })
    }
}

impl Url<SeasonsResponse> for SeasonsEndpointUrl {}

#[cfg(test)]
mod tests {
    use chrono::{Datelike, Local};
    use crate::endpoints::seasons::SeasonsEndpointUrl;
    use crate::endpoints::sports::SportsEndpointUrl;
    use crate::endpoints::Url;

    #[tokio::test]
    async fn parses_all_seasons() {
        let all_sport_ids = SportsEndpointUrl { id: None }.get().await.unwrap().sports.into_iter().map(|sport| sport.id).collect::<Vec<_>>();
        
        for season in 1871..=Local::now().year() as _ {
            for id in all_sport_ids.iter().copied() {
                dbg!(season, id);
                let _response = SeasonsEndpointUrl { sport_id: id, season: Some(season) }.get().await.unwrap();
            }
        }
    }
}
