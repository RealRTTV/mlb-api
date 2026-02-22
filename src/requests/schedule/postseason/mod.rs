//! Postseason schedule info and endpoints.

pub mod series;

use crate::schedule::ScheduleResponse;
use crate::season::SeasonId;
use crate::team::TeamId;
use bon::Builder;
use itertools::Itertools;
use std::fmt::{Display, Formatter};
use crate::meta::GameType;
use crate::request::RequestURL;
use crate::sport::SportId;

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

impl<S: schedule_postseason_request_builder::State + schedule_postseason_request_builder::IsComplete> crate::request::RequestURLBuilderExt for SchedulePostseasonRequestBuilder<S> {
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

impl RequestURL for SchedulePostseasonRequest {
    type Response = ScheduleResponse;
}

#[cfg(test)]
mod tests {
    use crate::request::RequestURLBuilderExt;
    use crate::schedule::postseason::SchedulePostseasonRequest;
	use crate::TEST_YEAR;

	#[tokio::test]
    async fn test_one_season() {
        let _ = SchedulePostseasonRequest::builder().season(TEST_YEAR).build_and_get().await.unwrap();
    }

    #[tokio::test]
    #[cfg_attr(not(feature = "_heavy_tests"), ignore)]
    async fn test_all_seasons() {
        for season in 1876..=TEST_YEAR {
            let _ = SchedulePostseasonRequest::builder().season(season).build_and_get().await.unwrap();
        }
    }
}

