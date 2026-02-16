use crate::schedule::ScheduleResponse;
use crate::season::SeasonId;
use bon::Builder;
use itertools::Itertools;
use std::fmt::{Display, Formatter};
use crate::game_types::GameType;
use crate::request::RequestURL;

#[derive(Builder)]
#[builder(derive(Into))]
pub struct ScheduleTiedRequest {
    #[builder(into)]
    season: SeasonId,
    game_types: Option<Vec<GameType>>,
}

impl<S: schedule_tied_request_builder::State + schedule_tied_request_builder::IsComplete> crate::request::RequestURLBuilderExt for ScheduleTiedRequestBuilder<S> {
    type Built = ScheduleTiedRequest;
}

impl Display for ScheduleTiedRequest {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "http://statsapi.mlb.com/api/v1/schedule/games/tied{params}", params = gen_params! {
            "season": self.season,
            "gameTypes"?: self.game_types.as_ref().map(|x| x.iter().map(|g| format!("{g:?}")).join(",")),
        })
    }
}

impl RequestURL for ScheduleTiedRequest {
    type Response = ScheduleResponse;
}

#[cfg(test)]
mod tests {
    use crate::request::RequestURLBuilderExt;
    use crate::schedule::tied::ScheduleTiedRequest;
	use crate::TEST_YEAR;

	#[tokio::test]
    async fn test_one_season() {
        let _ = ScheduleTiedRequest::builder().season(TEST_YEAR).build_and_get().await.unwrap();
    }

    #[tokio::test]
    #[cfg_attr(not(feature = "_heavy_tests"), ignore)]
    async fn test_all_seasons() {
        for season in 1876..=TEST_YEAR {
            let _ = ScheduleTiedRequest::builder()
                .season(season)
                .build_and_get()
                .await.unwrap();
        }
    }
}
