use crate::season::{Season, SeasonId};
use crate::types::Copyright;
use crate::request::StatsAPIRequestUrl;
use bon::Builder;
use serde::Deserialize;
use std::fmt::{Display, Formatter};
use crate::sports::SportId;

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SeasonsResponse {
    pub copyright: Copyright,
    pub seasons: Vec<Season>,
}

#[derive(Builder)]
#[builder(derive(Into))]
pub struct SeasonsRequest {
    #[builder(into)]
    #[builder(default)]
    sport_id: SportId,
    #[builder(into)]
    season: Option<SeasonId>,
}

impl<S: seasons_request_builder::State + seasons_request_builder::IsComplete> crate::request::StatsAPIRequestUrlBuilderExt for SeasonsRequestBuilder<S> {
    type Built = SeasonsRequest;
}

impl Display for SeasonsRequest {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "http://statsapi.mlb.com/api/v1/seasons{}", gen_params! { "sportId": self.sport_id, "season"?: self.season })
    }
}

impl StatsAPIRequestUrl for SeasonsRequest {
    type Response = SeasonsResponse;
}

#[cfg(test)]
mod tests {
    use crate::season::SeasonsRequest;
    use crate::sports::SportsRequest;
    use crate::TEST_YEAR;
    use crate::request::StatsAPIRequestUrlBuilderExt;

    #[tokio::test]
    #[cfg_attr(not(feature = "_heavy_tests"), ignore)]
    async fn parses_all_seasons() {
        let all_sport_ids = SportsRequest::builder().build_and_get().await.unwrap().sports.into_iter().map(|sport| sport.id).collect::<Vec<_>>();

        for season in 1871..=TEST_YEAR {
            for id in all_sport_ids.iter().copied() {
                let _response = SeasonsRequest::builder().sport_id(id).season(season).build_and_get().await.unwrap();
            }
        }
    }

    #[tokio::test]
    async fn parse_this_season_mlb() {
        let _response = SeasonsRequest::builder().build_and_get().await.unwrap();
    }
}
