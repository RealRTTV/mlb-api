pub mod series;

use std::fmt::{Display, Formatter};
use chrono::{Datelike, Local};
use itertools::Itertools;
use crate::{GameType, StatsAPIEndpointUrl};
use crate::schedule::ScheduleResponse;
use crate::seasons::season::SeasonId;
use crate::sports::SportId;
use crate::teams::team::TeamId;
use crate::gen_params;

pub struct SchedulePostseasonEndpoint {
    pub season: SeasonId,
    pub sport_id: SportId,
    pub team_id: Option<TeamId>,
    pub game_types: Option<Vec<GameType>>,
    pub series_number: Option<u32>,
}

impl Default for SchedulePostseasonEndpoint {
    fn default() -> Self {
        Self {
            season: (Local::now().year() as u32).into(),
            sport_id: SportId::MLB,
            team_id: None,
            game_types: None,
            series_number: None,
        }
    }
}

impl Display for SchedulePostseasonEndpoint {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "http://statsapi.mlb.com/api/v1/schedule/games/tied{params}", params = gen_params! {
            "season": self.season,
            "sportId": self.sport_id,
            "teamId"?: self.team_id,
            "gameTypes"?: self.game_types.as_ref().map(|x| x.iter().map(|g| format!("{g:?}")).join(",")),
        })
    }
}

impl StatsAPIEndpointUrl for SchedulePostseasonEndpoint {
    type Response = ScheduleResponse;
}

#[cfg(test)]
mod tests {
    use chrono::{Datelike, Local};
    use crate::schedule::postseason::SchedulePostseasonEndpoint;
    use crate::StatsAPIEndpointUrl;

    #[tokio::test]
    async fn test_one_season() {
        let request = SchedulePostseasonEndpoint {
            season: 2025.into(),
            ..Default::default()
        };
        let _ = request.get().await.unwrap();
    }

    #[tokio::test]
    #[cfg_attr(not(feature = "_heavy_tests"), ignore)]
    async fn test_all_seasons() {
        for season in 1876..=Local::now().year() as _ {
            let request = SchedulePostseasonEndpoint {
                season: season.into(),
                ..Default::default()
            };
            let _ = request.get().await.unwrap();
        }
    }
}

