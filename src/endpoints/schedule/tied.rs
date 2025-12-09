use crate::gen_params;
use crate::schedule::ScheduleResponse;
use crate::seasons::season::SeasonId;
use crate::{GameType, StatsAPIEndpointUrl};
use bon::Builder;
use itertools::Itertools;
use std::fmt::{Display, Formatter};

#[derive(Builder)]
#[builder(derive(Into))]
pub struct ScheduleTiedEndpoint {
    #[builder(into)]
    season: SeasonId,
    game_types: Option<Vec<GameType>>,
}

impl<S: schedule_tied_endpoint_builder::State> crate::endpoints::links::StatsAPIEndpointUrlBuilderExt for ScheduleTiedEndpointBuilder<S> where S: schedule_tied_endpoint_builder::IsComplete {
    type Built = ScheduleTiedEndpoint;
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
    use crate::schedule::tied::ScheduleTiedEndpoint;
    use crate::StatsAPIEndpointUrlBuilderExt;
    use chrono::{Datelike, Local};

    #[tokio::test]
    async fn test_one_season() {
        let _ = ScheduleTiedEndpoint::builder().season(2025).build_and_get().await.unwrap();
    }

    #[tokio::test]
    #[cfg_attr(not(feature = "_heavy_tests"), ignore)]
    async fn test_all_seasons() {
        for season in 1876..=Local::now().year() as _ {
            let _ = ScheduleTiedEndpoint::builder()
                .season(season)
                .build_and_get()
                .await.unwrap();
        }
    }
}
