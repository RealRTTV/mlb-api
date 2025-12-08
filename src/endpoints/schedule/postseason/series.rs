use std::fmt::{Display, Formatter};
use chrono::{Datelike, Local};
use itertools::Itertools;
use serde::{Deserialize, Deserializer};
use serde::de::Error;
use crate::{GameType, StatsAPIEndpointUrl};
use crate::schedule::ScheduleGame;
use crate::seasons::season::SeasonId;
use crate::sports::SportId;
use crate::teams::team::TeamId;
use crate::gen_params;
use crate::types::Copyright;

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

pub fn series_number_from_id<'de, D: Deserializer<'de>>(deserializer: D) -> Result<u32, D::Error> {
    let str = String::deserialize(deserializer)?;
    let (_game_type, series_number) = str.split_once('_').ok_or_else(|| D::Error::custom("Malformed id, expected format '{game_type}_{series_number}'"))?;
    let series_number: u32 = series_number.parse().map_err(|e| D::Error::custom(e))?;
    Ok(series_number)
}

pub struct SchedulePostseasonSeriesEndpoint {
    pub season: SeasonId,
    pub sport_id: Option<SportId>,
    pub team_id: Option<TeamId>,
    pub game_types: Option<Vec<GameType>>,
    pub series_number: Option<u32>,
}

impl Default for SchedulePostseasonSeriesEndpoint {
    fn default() -> Self {
        Self {
            season: (Local::now().year() as u32).into(),
            sport_id: None,
            team_id: None,
            game_types: None,
            series_number: None,
        }
    }
}

impl Display for SchedulePostseasonSeriesEndpoint {
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

impl StatsAPIEndpointUrl for SchedulePostseasonSeriesEndpoint {
    type Response = SchedulePostseasonSeriesResponse;
}
