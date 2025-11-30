pub mod series;
pub mod tune_in;

use std::fmt::{Display, Formatter};
use chrono::{Datelike, Local};
use itertools::Itertools;
use crate::endpoints::{GameType, StatsAPIUrl};
use crate::endpoints::schedule::ScheduleResponse;
use crate::endpoints::sports::SportId;
use crate::endpoints::teams::team::TeamId;
use crate::gen_params;

pub struct SchedulePostseasonEndpointUrl {
    pub season: u32,
    pub sport_id: SportId,
    pub team_id: Option<TeamId>,
    pub game_types: Option<Vec<GameType>>,
    pub series_number: Option<u32>,
}

impl Default for SchedulePostseasonEndpointUrl {
    fn default() -> Self {
        Self {
            season: Local::now().year() as _,
            sport_id: SportId::MLB,
            team_id: None,
            game_types: None,
            series_number: None,
        }
    }
}

impl Display for SchedulePostseasonEndpointUrl {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "http://statsapi.mlb.com/api/v1/schedule/games/tied{params}", params = gen_params! {
            "season": self.season,
            "sportId": self.sport_id,
            "teamId"?: self.team_id,
            "gameTypes"?: self.game_types.as_ref().map(|x| x.iter().map(|g| format!("{g:?}")).join(",")),
        })
    }
}

impl StatsAPIUrl for SchedulePostseasonEndpointUrl {
    type Response = ScheduleResponse;
}

#[cfg(test)]
mod tests {
    use chrono::{Datelike, Local};
    use crate::endpoints::schedule::postseason::SchedulePostseasonEndpointUrl;
    use crate::endpoints::StatsAPIUrl;

    #[tokio::test]
    async fn test_one_season() {
        let request = SchedulePostseasonEndpointUrl {
            season: 2025,
            ..Default::default()
        };
        let _ = request.get().await.unwrap();
    }

    #[tokio::test]
    #[cfg_attr(not(feature = "_heavy_tests"), ignore)]
    async fn test_all_seasons() {
        for season in 1876..=Local::now().year() as _ {
            let request = SchedulePostseasonEndpointUrl {
                season,
                ..Default::default()
            };
            let _ = request.get().await.unwrap();
        }
    }
}

