use crate::types::{Copyright, NaiveDateRange};
use chrono::NaiveDate;
use derive_more::{Deref, Display, From};
use serde::{Deserialize, Deserializer};
use std::fmt::{Display, Formatter};
use bon::Builder;
use crate::request::StatsAPIRequestUrl;
use crate::sports::SportId;

integer_id!(#[derive(Debug, Default, Deref, Display, From, PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Hash)] SeasonId);

impl<'de> Deserialize<'de> for SeasonId {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>
	{
		struct Visitor;

		impl serde::de::Visitor<'_> for Visitor {
			type Value = SeasonId;

			fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
				formatter.write_str("int or string")
			}

			fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E> {
				Ok(SeasonId(v))
			}

			fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: serde::de::Error {
				v.parse::<u32>().map(SeasonId).map_err(E::custom)
			}
		}

		deserializer.deserialize_any(Visitor)
	}
}

#[derive(Deserialize)]
struct SeasonRaw {
	#[serde(alias = "season", alias = "seasonId")]
	pub id: SeasonId,

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
	pub season_start: Option<NaiveDate>,
	#[serde(rename = "regularSeasonStartDate")]
	pub regular_season_start: Option<NaiveDate>,
	#[serde(rename = "lastDate1stHalf")]
	pub first_half_end: Option<NaiveDate>,
	#[serde(rename = "allStarDate")]
	pub all_star: Option<NaiveDate>,
	#[serde(rename = "firstDate2ndHalf")]
	pub second_half_start: Option<NaiveDate>,
	#[serde(rename = "regularSeasonEndDate")]
	pub regular_season_end: Option<NaiveDate>,
	#[serde(rename = "postSeasonStartDate")]
	pub postseason_start: Option<NaiveDate>,
	#[serde(rename = "postSeasonEndDate")]
	pub postseason_end: Option<NaiveDate>,
	#[serde(rename = "seasonEndDate")]
	pub season_end: Option<NaiveDate>,
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
			season: season_start.unwrap_or(preseason_start)..=season_end.unwrap_or(offseason_end),
			regular_season: regular_season_start.or(season_start).unwrap_or(preseason_start)..=regular_season_end.or(season_end).unwrap_or(offseason_end),
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
	pub id: SeasonId,
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
	#[serde(rename = "inseason")]
	Inseason,
	#[serde(rename = "offseason")]
	Offseason,
	#[serde(rename = "preseason")]
	Preseason,
}

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
