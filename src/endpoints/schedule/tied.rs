use std::fmt::{Display, Formatter};
use chrono::{Datelike, Local};
use itertools::Itertools;
use crate::endpoints::{GameType, StatsAPIEndpointUrl};
use crate::endpoints::schedule::ScheduleResponse;
use crate::gen_params;

pub struct ScheduleTiedEndpoint {
    pub season: u32,
    pub game_types: Option<Vec<GameType>>,
}

impl Default for ScheduleTiedEndpoint {
    fn default() -> Self {
        Self {
            season: Local::now().year() as _,
            game_types: None,
        }
    }
}

impl Display for ScheduleTiedEndpoint {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "http://statsapi.mlb.com/api/v1/schedule/games/tied{params}", params = gen_params! {
            "season": self.season,
            "gameTypes"?: self.game_types.as_ref().map(|x| x.iter().map(|g| format!("{g:?}")).join(",")),
        })
    }
}

impl StatsAPIEndpointUrl for ScheduleTiedEndpoint {
    type Response = ScheduleResponse;
}

#[cfg(test)]
mod tests {
    use chrono::{Datelike, Local};
    use crate::endpoints::schedule::tied::ScheduleTiedEndpoint;
    use crate::endpoints::StatsAPIEndpointUrl;

    #[tokio::test]
    async fn test_one_season() {
        let request = ScheduleTiedEndpoint {
            season: 1961,
            game_types: None,
        };
        let _ = request.get().await.unwrap();
    }

    #[tokio::test]
    #[cfg_attr(not(feature = "_heavy_tests"), ignore)]
    async fn test_all_seasons() {
        for season in 1876..=Local::now().year() as _ {
            let request = ScheduleTiedEndpoint {
                season,
                game_types: None,
            };
            let _ = request.get().await.unwrap();
        }
    }
}
