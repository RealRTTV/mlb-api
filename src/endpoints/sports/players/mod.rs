mod types;
pub use types::*;

use std::fmt::{Display, Formatter};
use crate::endpoints::sports::SportId;
use crate::endpoints::Url;
use crate::gen_params;

#[derive(Default)]
pub struct SportsPlayersResponseUrl {
    pub id: SportId,
    pub season: Option<u16>,
}

impl Display for SportsPlayersResponseUrl {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "https://statsapi.mlb.com/api/v1/sports/{id}/players{params}", id = self.id, params = gen_params! { "sportId": self.id, "season"?: self.season })
    }
}

impl Url<SportsPlayersResponse> for SportsPlayersResponseUrl {}

#[cfg(test)]
mod tests {
    use chrono::{Datelike, Local};
    use crate::endpoints::sports::SportId;
    use crate::endpoints::Url;
    use super::*;

    #[tokio::test]
    async fn parse_all_players() {
        for season in 1876..=Local::now().year() as _ {
            let _response = SportsPlayersResponseUrl { id: SportId::MLB, season: Some(season) }.get().await.unwrap();
        }
    }
}
