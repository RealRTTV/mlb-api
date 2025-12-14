use crate::schedule::ScheduleGame;
use crate::season::SeasonId;
use crate::teams::team::TeamId;
use crate::types::Copyright;
use bon::Builder;
use itertools::Itertools;
use serde::de::Error;
use serde::{Deserialize, Deserializer};
use std::fmt::{Display, Formatter};
use crate::game_types::GameType;
use crate::request::StatsAPIRequestUrl;
use crate::sports::SportId;

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SchedulePostseasonSeriesResponse {
    pub copyright: Copyright,
    pub series: Vec<ScheduleSeries>,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ScheduleSeries {
    pub games: Vec<ScheduleGame>,
    #[serde(rename = "series")]
    pub data: SeriesData,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SeriesData {
    #[serde(rename = "id", deserialize_with = "series_number_from_id")]
    pub series_number: u32,
    pub is_default: bool,
    pub game_type: GameType,
}

/// # Errors
/// 1. Cannot deserialize into string
/// 2. Not in format `{game_type}_{series_number}`
/// 3. Not a valid `u32`
pub fn series_number_from_id<'de, D: Deserializer<'de>>(deserializer: D) -> Result<u32, D::Error> {
    let str = String::deserialize(deserializer)?;
    let (_game_type, series_number) = str.split_once('_').ok_or_else(|| D::Error::custom("Malformed id, expected format '{game_type}_{series_number}'"))?;
    let series_number: u32 = series_number.parse().map_err(D::Error::custom)?;
    Ok(series_number)
}

#[derive(Builder)]
#[builder(derive(Into))]
pub struct SchedulePostseasonSeriesRequest {
    #[builder(into)]
    season: SeasonId,
    #[builder(into)]
    sport_id: Option<SportId>,
    #[builder(into)]
    team_id: Option<TeamId>,
    game_types: Option<Vec<GameType>>,
    series_number: Option<u32>,
}

impl<S: schedule_postseason_series_request_builder::State + schedule_postseason_series_request_builder::IsComplete> crate::request::StatsAPIRequestUrlBuilderExt for SchedulePostseasonSeriesRequestBuilder<S> {
    type Built = SchedulePostseasonSeriesRequest;
}

impl Display for SchedulePostseasonSeriesRequest {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "http://statsapi.mlb.com/api/v1/schedule/postseason/series{params}", params = gen_params! {
            "season": self.season,
            "sportId"?: self.sport_id,
            "teamId"?: self.team_id,
            "gameTypes"?: self.game_types.as_ref().map(|x| x.iter().map(|x| format!("{x:?}")).join(",")),
            "seriesNumber"?: self.series_number,
        })
    }
}

impl StatsAPIRequestUrl for SchedulePostseasonSeriesRequest {
    type Response = SchedulePostseasonSeriesResponse;
}
