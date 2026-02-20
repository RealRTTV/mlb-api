//! Data about important dates in a season for a specific [`SportId`].
//!
//! When spring training starts, ends. Regular season dates, Postseason dates, ASG, etc.

use crate::{Copyright, NaiveDateRange};
use chrono::{Datelike, NaiveDate, Utc};
use derive_more::{Deref, Display, From};
use serde::{Deserialize, Deserializer};
use std::fmt::{Display, Formatter};
use bon::Builder;
use serde::de::Error;
use crate::request::RequestURL;
use crate::sport::SportId;

#[derive(Debug, Default, Deref, Display, PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Hash, From)]
#[repr(transparent)]
pub struct SeasonId(u32);

impl<'de> Deserialize<'de> for SeasonId {
	fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
		#[derive(::serde::Deserialize)]
		#[serde(untagged)]
		enum Repr {
			Wrapped { id: u32 },
			Inline(u32),
			String(String),
		}

		let id = match Repr::deserialize(deserializer)? {
			Repr::Wrapped { id } | Repr::Inline(id) => id,
			Repr::String(id) => id.parse::<u32>().map_err(D::Error::custom)?,
		};
		Ok(Self(id))
	}
}

impl SeasonId {
	#[must_use]
	pub const fn new(id: u32) -> Self {
		Self(id)
	}

	#[allow(clippy::cast_sign_loss, reason = "jesus is not alive")]
	#[must_use]
	pub fn current_season() -> Self {
		Self::new(Utc::now().year() as _)
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

/// A season and it's info - dependent on [`SportId`].
///
/// These fields are arranged in a chronological order but the specification makes no guarantees that this order remain consistent.
#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(from = "SeasonRaw")]
pub struct Season {
	pub id: SeasonId,
	/// If the season has a wildcard system
	pub has_wildcard: bool,
	/// Preseason date range
	pub preseason: NaiveDateRange,
	/// Spring Training date range
	pub spring: Option<NaiveDateRange>,
	/// Full Season date range
	pub season: NaiveDateRange,
	/// Regular Season date range
	pub regular_season: NaiveDateRange,
	/// End of the first half of the season (if the season halves are defined)
	pub first_half_end: Option<NaiveDate>,
	/// When the ASG is
	pub all_star: Option<NaiveDate>,
	/// Start of the second half of the season (if the season halves are defined)
	pub second_half_start: Option<NaiveDate>,
	/// When the postseason happens
	pub postseason: Option<NaiveDateRange>,
	/// When the offseason is active (different from preseason)
	pub offseason: NaiveDateRange,
	/// [`QualificationMultipliers`]
	pub qualification_multipliers: Option<QualificationMultipliers>,
	// opt<(season_level_gameday_type, game_level_gameday_type)>
}

// Coefficients for the qualified player cutoffs.
#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct QualificationMultipliers {
	/// Amount of plate appearances needed per game your (current?) team has played to be considered qualified
	#[serde(rename = "qualifierPlateAppearances")]
	pub plate_appearances_per_game: f64,
	// Amount of outs pitched per game your (current?) team has played to be considered qualified
	#[serde(rename = "qualifierOutsPitched")]
	pub outs_pitched_per_game: f64,
}

impl Eq for QualificationMultipliers {}

/// Current state of the season
#[derive(Debug, Deserialize, PartialEq, Eq, Copy, Clone)]
pub enum SeasonState {
	#[serde(rename = "inseason")]
	Inseason,
	#[serde(rename = "offseason")]
	Offseason,
	#[serde(rename = "preseason")]
	Preseason,
}

/// Returns a [`Vec`] of [`Season`]s.
#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SeasonsResponse {
	pub copyright: Copyright,
	pub seasons: Vec<Season>,
}

/// Returns a [`SeasonsResponse`]
#[derive(Builder)]
#[builder(derive(Into))]
pub struct SeasonsRequest {
	#[builder(into)]
	#[builder(default)]
	sport_id: SportId,
	#[builder(into)]
	season: Option<SeasonId>,
}

impl<S: seasons_request_builder::State + seasons_request_builder::IsComplete> crate::request::RequestURLBuilderExt for SeasonsRequestBuilder<S> {
	type Built = SeasonsRequest;
}

impl Display for SeasonsRequest {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "http://statsapi.mlb.com/api/v1/seasons{}", gen_params! { "sportId": self.sport_id, "season"?: self.season })
	}
}

impl RequestURL for SeasonsRequest {
	type Response = SeasonsResponse;
}

#[cfg(test)]
mod tests {
	use crate::season::SeasonsRequest;
	use crate::sport::SportsRequest;
	use crate::TEST_YEAR;
	use crate::request::RequestURLBuilderExt;

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
