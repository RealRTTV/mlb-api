use crate::types::NaiveDateRange;
use chrono::NaiveDate;
use serde::Deserialize;

#[derive(Deserialize)]
struct SeasonRaw {
    #[serde(deserialize_with = "crate::types::from_str")]
    #[serde(alias = "season", alias = "seasonId")] pub id: u16,

    #[serde(default)] // will be overwriten if not present because of bad league schedule schema
    #[serde(rename = "hasWildcard")]
    pub has_wildcard: bool,

    #[serde(rename = "preSeasonStartDate")]
    pub preseason_start: NaiveDate,
    #[serde(rename = "preSeasonEndDate")]
    pub preseason_end: Option<NaiveDate>,
    #[serde(rename = "springStartDate")]
    pub spring_start: Option<NaiveDate>,
    #[serde(rename = "springEndDate")]
    pub spring_end: Option<NaiveDate>,
    #[serde(rename = "seasonStartDate")]
    pub season_start: NaiveDate,
    #[serde(rename = "regularSeasonStartDate")]
    pub regular_season_start: NaiveDate,
    #[serde(rename = "lastDate1stHalf")]
    pub first_half_end: Option<NaiveDate>,
    #[serde(rename = "allStarDate")]
    pub all_star: Option<NaiveDate>,
    #[serde(rename = "firstDate2ndHalf")]
    pub second_half_start: Option<NaiveDate>,
    #[serde(rename = "regularSeasonEndDate")]
    pub regular_season_end: NaiveDate,
    #[serde(rename = "postSeasonStartDate")]
    pub postseason_start: Option<NaiveDate>,
    #[serde(rename = "postSeasonEndDate")]
    pub postseason_end: Option<NaiveDate>,
    #[serde(rename = "seasonEndDate")]
    pub season_end: NaiveDate,
    #[serde(rename = "offseasonStartDate")]
    pub offseason_start: Option<NaiveDate>,
    #[serde(rename = "offSeasonEndDate")]
    pub offseason_end: NaiveDate,
    #[serde(flatten)]
    pub qualification_multipliers: Option<QualificationMultipliers>,
}

impl From<SeasonRaw> for Season {
    fn from(value: SeasonRaw) -> Self {
        let SeasonRaw {
            id,
            has_wildcard,
            preseason_start,
            preseason_end,
            spring_start,
            spring_end,
            season_start,
            regular_season_start,
            first_half_end,
            all_star,
            second_half_start,
            regular_season_end,
            postseason_start,
            postseason_end,
            season_end,
            offseason_start,
            offseason_end,
            qualification_multipliers,
        } = value;

        Self {
            id,
            has_wildcard,
            preseason: preseason_start..=preseason_end.unwrap_or(preseason_start),
            spring: spring_start.and_then(|start| spring_end.map(|end| start..=end)),
            season: season_start..=season_end,
            regular_season: regular_season_start..=regular_season_end,
            first_half_end,
            all_star,
            second_half_start,
            postseason: postseason_start.and_then(|start| postseason_end.map(|end| start..=end)),
            offseason: offseason_start.unwrap_or(offseason_end)..=offseason_end,
            qualification_multipliers,
        }
    }
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(from = "SeasonRaw")]
pub struct Season {
    pub id: u16,
    pub has_wildcard: bool,
    pub preseason: NaiveDateRange,
    pub spring: Option<NaiveDateRange>,
    pub season: NaiveDateRange,
    pub regular_season: NaiveDateRange,
    pub first_half_end: Option<NaiveDate>,
    pub all_star: Option<NaiveDate>,
    pub second_half_start: Option<NaiveDate>,
    pub postseason: Option<NaiveDateRange>,
    pub offseason: NaiveDateRange,
    pub qualification_multipliers: Option<QualificationMultipliers>,
    // opt<(season_level_gameday_type, game_level_gameday_type)>
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct QualificationMultipliers {
    #[serde(rename = "qualifierPlateAppearances")]
    pub plate_appearances_per_game: f64,
    #[serde(rename = "qualifierOutsPitched")]
    pub outs_pitched_per_game: f64,
}

impl Eq for QualificationMultipliers {}

#[derive(Debug, Deserialize, PartialEq, Eq, Copy, Clone)]
pub enum SeasonState {
    #[serde(rename = "inseason")] Inseason,
    #[serde(rename = "offseason")] Offseason,
    #[serde(rename = "preseason")] Preseason,
}
