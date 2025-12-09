pub mod series;

use crate::gen_params;
use crate::schedule::ScheduleResponse;
use crate::seasons::season::SeasonId;
use crate::sports::SportId;
use crate::teams::team::TeamId;
use crate::{GameType, StatsAPIRequestUrl};
use bon::Builder;
use itertools::Itertools;
use std::fmt::{Display, Formatter};

#[derive(Builder)]
#[builder(derive(Into))]
pub struct SchedulePostseasonRequest {
    #[builder(into)]
    season: SeasonId,
    #[builder(into)]
    #[builder(default)]
    sport_id: SportId,
    #[builder(into)]
    team_id: Option<TeamId>,
    game_types: Option<Vec<GameType>>,
    series_number: Option<u32>,
}

impl<S: schedule_postseason_request_builder::State + schedule_postseason_request_builder::IsComplete> crate::requests::links::StatsAPIRequestUrlBuilderExt for SchedulePostseasonRequestBuilder<S> {
    type Built = SchedulePostseasonRequest;
}

impl Display for SchedulePostseasonRequest {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "http://statsapi.mlb.com/api/v1/schedule/games/tied{params}", params = gen_params! {
            "season": self.season,
            "sportId": self.sport_id,
            "teamId"?: self.team_id,
            "gameTypes"?: self.game_types.as_ref().map(|x| x.iter().map(|g| format!("{g:?}")).join(",")),
            "seriesNumber"?: self.series_number,
        })
    }
}

impl StatsAPIRequestUrl for SchedulePostseasonRequest {
    type Response = ScheduleResponse;
}

#[cfg(test)]
mod tests {
    use crate::schedule::postseason::SchedulePostseasonRequest;
    use crate::StatsAPIRequestUrlBuilderExt;
    use chrono::{Datelike, Local};

    #[tokio::test]
    async fn test_one_season() {
        let _ = SchedulePostseasonRequest::builder().season(2025).build_and_get().await.unwrap();
    }

    #[tokio::test]
    #[cfg_attr(not(feature = "_heavy_tests"), ignore)]
    async fn test_all_seasons() {
        for season in 1876..=Local::now().year() as _ {
            let _ = SchedulePostseasonRequest::builder().season(season).build_and_get().await.unwrap();
        }
    }
}

