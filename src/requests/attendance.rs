//! Attendance for games and seasons.
//!
//! Typically only seasonal [`AttendanceRecord`]s are accessible so some extra work is needed to get a specific game's attendance.
//!
//! Within regards to attendance, the term frequently used is "Opening" over "Game";
//! this is for reasons including but not limited to: single ticket double headers,
//! and rescheduled games.
//!
//! Averages are calculated with respect to the # of openings on the sample, not the number of games the team played as either "home" or "away".
//!
//! Since the 2020 season had 0 total attendance, the 'peak attendance game' has its default value of [`NaiveDate::MIN`]

use crate::league::LeagueId;
use crate::season::SeasonId;
use crate::team::TeamId;
use crate::{Copyright, HomeAwaySplit, MLB_API_DATE_FORMAT};
use bon::Builder;
use chrono::{Datelike, Local, NaiveDate, NaiveDateTime};
use either::Either;
use serde::de::Error;
use serde::{Deserialize, Deserializer};
use std::cmp::Ordering;
use std::fmt::{Debug, Display, Formatter};
use std::iter::Sum;
use std::num::NonZeroU32;
use std::ops::Add;
use crate::game::GameId;
use crate::meta::GameType;
use crate::request::RequestURL;

/// Response from the `attendance` endpoint.
/// Returns a [`Vec`] of [`AttendanceRecord`].
///
/// Example: <http://statsapi.mlb.com/api/v1/attendance?teamId=141>
#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(from = "AttendanceResponseStruct")]
pub struct AttendanceResponse {
	pub copyright: Copyright,
	#[serde(rename = "records")]
	pub annual_records: Vec<AttendanceRecord>,
}

impl AttendanceResponse {
	/// Combines all the attendance records into one for all the recorded openings.
	#[must_use]
	pub fn into_aggregate(self) -> AttendanceRecord {
		self.annual_records.into_iter().sum()
	}
}

#[derive(Deserialize)]
struct AttendanceResponseStruct {
	copyright: Copyright,
	records: Vec<AttendanceRecord>,
}

impl From<AttendanceResponseStruct> for AttendanceResponse {
	fn from(value: AttendanceResponseStruct) -> Self {
		let AttendanceResponseStruct { copyright, records } = value;
		Self { copyright, annual_records: records }
	}
}

/// A record of attendance.
///
/// Does not represent a single opening, those opening-by-opening requests require a little more MacGyver-ing with the date.
///
/// Represents a full season of attendance data (segmented by [`GameType`]).
#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(from = "AnnualRecordStruct")]
pub struct AttendanceRecord {
	pub total_openings: HomeAwaySplit<u32>,
	pub total_openings_lost: u32,
	pub total_games: HomeAwaySplit<u32>,
	pub season: SeasonWithMinorId,
	pub attendance_totals: HomeAwaySplit<u32>,
	/// Minimum at an opening, then maximum at an opening
	pub single_opening_min_max: Option<(DatedAttendance, DatedAttendance)>,
	pub game_type: GameType,
}

impl Add for AttendanceRecord {
	type Output = Self;

	/// Since the [`AttendanceRecord::default()`] value has some "worse"-er defaults (high and low attendance records have the epoch start time as their dates), we always take the later values in case of ties.
	fn add(self, rhs: Self) -> Self::Output {
		Self {
			total_openings: HomeAwaySplit {
				home: self.total_openings.home + rhs.total_openings.home,
				away: self.total_openings.away + rhs.total_openings.away,
			},
			total_openings_lost: self.total_openings_lost + rhs.total_openings_lost,
			total_games: HomeAwaySplit {
				home: self.total_games.home + rhs.total_games.home,
				away: self.total_games.away + rhs.total_games.away,
			},
			season: SeasonWithMinorId::max(self.season, rhs.season),
			attendance_totals: HomeAwaySplit {
				home: self.attendance_totals.home + rhs.attendance_totals.home,
				away: self.attendance_totals.away + rhs.attendance_totals.away,
			},
			single_opening_min_max: match (self.single_opening_min_max, rhs.single_opening_min_max) {
				(None, None) => None,
				(Some(min_max), None) | (None, Some(min_max)) => Some(min_max),
				// ties go to rhs in `min` and `max` calls
				(Some((a_min, a_max)), Some((b_min, b_max))) => Some((b_min.min(a_min), a_max.max(b_max))),
			},
			game_type: rhs.game_type,
		}
	}
}

impl Default for AttendanceRecord {
	#[allow(clippy::cast_sign_loss, reason = "jesus is not alive")]
	fn default() -> Self {
		Self {
			total_openings: HomeAwaySplit::new(0, 0),
			total_openings_lost: 0,
			total_games: HomeAwaySplit::new(0, 0),
			season: (Local::now().year() as u32).into(),
			attendance_totals: HomeAwaySplit::new(0, 0),
			single_opening_min_max: None,
			game_type: GameType::default(),
		}
	}
}

impl Sum for AttendanceRecord {
	fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
		iter.fold(Self::default(), |acc, x| acc + x)
	}
}

impl AttendanceRecord {
	/// Calculates the average attendance.
	///
	/// # Examples
	/// ```
	/// use mlb_api::attendance::AttendanceRecord;
	///
	/// assert_eq!(AttendanceRecord {
	///     total_openings: (2, 2).into(),
	///     attendance_totals: (200, 200).into(),
	///     ..Default::default(),
	/// }.average_attendance(), (100, 100).into());
	/// ```
	#[must_use]
	pub const fn average_attendance(&self) -> HomeAwaySplit<u32> {
		let HomeAwaySplit { home, away } = self.attendance_totals;
		let HomeAwaySplit { home: num_at_home, away: num_at_away } = self.total_openings;
		HomeAwaySplit::new((home + num_at_home / 2) / num_at_home, (away + num_at_away / 2) / num_at_away)
	}
}

/// Season with an optional minor part
///
/// Some seasons are duplicated since there might be multiple in the same year, because of that we get stuff like `2018.2`.
#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Hash)]
pub struct SeasonWithMinorId {
	season: SeasonId,
	minor: Option<NonZeroU32>,
}

impl From<SeasonId> for SeasonWithMinorId {
	fn from(value: SeasonId) -> Self {
		Self { season: value, minor: None }
	}
}

impl From<u32> for SeasonWithMinorId {
	fn from(value: u32) -> Self {
		Self { season: value.into(), minor: None }
	}
}

impl<'de> Deserialize<'de> for SeasonWithMinorId {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>
	{
		struct Visitor;

		impl serde::de::Visitor<'_> for Visitor {
			type Value = SeasonWithMinorId;

			fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
				formatter.write_str("a season id, or a string with a . denoting the minor")
			}

			fn visit_u32<E>(self, value: u32) -> Result<Self::Value, E>
			where
				E: Error
			{
				Ok(SeasonWithMinorId { season: SeasonId::from(value), minor: None })
			}

			fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
			where
				E: Error,
			{
				if let Some((season, minor)) = v.split_once('.') {
					let season = season.parse::<u32>().map_err(Error::custom)?;
					let minor = minor.parse::<u32>().map_err(Error::custom)?;
					let minor = NonZeroU32::try_from(minor).map_err(Error::custom)?;
					Ok(SeasonWithMinorId { season: SeasonId::from(season), minor: Some(minor) })
				} else {
					Ok(v.parse::<u32>().map(|season| SeasonWithMinorId { season: SeasonId::from(season), minor: None }).map_err(Error::custom)?)
				}
			}
		}

		deserializer.deserialize_any(Visitor)
	}
}

impl Display for SeasonWithMinorId {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.season)?;
		if let Some(minor) = self.minor {
			write!(f, ".{minor}")?;
		}
		Ok(())
	}
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
struct AnnualRecordStruct {
	// openings_total: u32,
	openings_total_away: u32,
	openings_total_home: u32,
	openings_total_lost: u32,
	// games_total: u32,
	games_away_total: u32,
	games_home_total: u32,
	year: SeasonWithMinorId,
	// attendance_average_away: u32,
	// attendance_average_home: u32,
	// attendance_average_ytd: u32,
	attendance_high: Option<u32>,
	attendance_high_date: Option<NaiveDateTime>,
	attendance_high_game: Option<GameId>,
	attendance_low: Option<u32>,
	attendance_low_date: Option<NaiveDateTime>,
	attendance_low_game: Option<GameId>,
	// attendance_opening_average: u32,
	// attendance_total: u32,
	attendance_total_away: Option<u32>,
	attendance_total_home: Option<u32>,
	game_type: GameType,
	// team: Team,
}

impl From<AnnualRecordStruct> for AttendanceRecord {
	fn from(value: AnnualRecordStruct) -> Self {
		let AnnualRecordStruct {
			// openings_total,
			openings_total_away,
			openings_total_home,
			openings_total_lost,
			// games_total,
			games_away_total,
			games_home_total,
			year,
			// attendance_average_away,
			// attendance_average_home,
			// attendance_average_ytd,
			attendance_high,
			attendance_high_date,
			attendance_high_game,
			attendance_low,
			attendance_low_date,
			attendance_low_game,
			// attendance_opening_average,
			// attendance_total,
			attendance_total_away,
			attendance_total_home,
			game_type,
			// team,
		} = value;
		let single_opening_min_max = if let Some(attendance_high) = attendance_high
			&& let Some(attendance_high_date) = attendance_high_date
			&& let Some(attendance_high_game) = attendance_high_game
		{
			let max = DatedAttendance {
				value: attendance_high,
				date: attendance_high_date.date(),
				game: attendance_high_game,
			};

			let min = {
				if let Some(attendance_low) = attendance_low
					&& let Some(attendance_low_date) = attendance_low_date
					&& let Some(attendance_low_game) = attendance_low_game
				{
					DatedAttendance {
						value: attendance_low,
						date: attendance_low_date.date(),
						game: attendance_low_game,
					}
				} else {
					max.clone()
				}
			};

			Some((min, max))
		} else {
			None
		};
		Self {
			total_openings: HomeAwaySplit {
				home: openings_total_home,
				away: openings_total_away,
			},
			total_openings_lost: openings_total_lost,
			total_games: HomeAwaySplit {
				home: games_home_total,
				away: games_away_total,
			},
			season: year,
			attendance_totals: HomeAwaySplit {
				home: attendance_total_home.unwrap_or(0),
				away: attendance_total_away.unwrap_or(0),
			},
			single_opening_min_max,
			game_type,
		}
	}
}

/// An attendance record of a single game.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct DatedAttendance {
	/// Attendance quantity
	pub value: u32,
	/// Date of attendance
	pub date: NaiveDate,
	/// Game in which people attended
	pub game: GameId,
}

impl PartialOrd<Self> for DatedAttendance {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl Ord for DatedAttendance {
	fn cmp(&self, other: &Self) -> Ordering {
		self.value.cmp(&other.value)
	}
}

/// Returns [`AttendanceResponse`]
#[derive(Builder)]
#[builder(derive(Into))]
pub struct AttendanceRequest {
	#[doc(hidden)]
	#[builder(setters(vis = "", name = __id_internal))]
	id: Either<TeamId, LeagueId>,
	#[builder(into)]
	season: Option<SeasonWithMinorId>,
	#[builder(into)]
	date: Option<NaiveDate>,
	#[builder(default)]
	game_type: GameType,
}

impl<S: attendance_request_builder::State + attendance_request_builder::IsComplete> crate::request::RequestURLBuilderExt for AttendanceRequestBuilder<S> {
    type Built = AttendanceRequest;
}

#[allow(dead_code, reason = "optionally used by the end user")]
impl<S: attendance_request_builder::State> AttendanceRequestBuilder<S> {
	#[doc = "_**Required.**_\n\n"]
	pub fn team_id(self, id: impl Into<TeamId>) -> AttendanceRequestBuilder<attendance_request_builder::SetId<S>>
	where
		S::Id: attendance_request_builder::IsUnset,
	{
		self.__id_internal(Either::Left(id.into()))
	}

	#[doc = "_**Required.**_\n\n"]
	pub fn league_id(self, id: impl Into<LeagueId>) -> AttendanceRequestBuilder<attendance_request_builder::SetId<S>>
	where
		S::Id: attendance_request_builder::IsUnset,
	{
		self.__id_internal(Either::Right(id.into()))
	}
}

impl Display for AttendanceRequest {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"http://statsapi.mlb.com/api/v1/attendance{}",
			gen_params! { "teamId"?: self.id.left(), "leagueId"?: self.id.right(), "season"?: self.season, "date"?: self.date.as_ref().map(|date| date.format(MLB_API_DATE_FORMAT)), "gameType": format!("{:?}", self.game_type) }
		)
	}
}

impl RequestURL for AttendanceRequest {
	type Response = AttendanceResponse;
}

#[cfg(test)]
mod tests {
	use crate::attendance::AttendanceRequest;
	use crate::request::{RequestURL, RequestURLBuilderExt};
	use crate::team::teams::TeamsRequest;
	use crate::TEST_YEAR;

	#[tokio::test]
	#[cfg_attr(not(feature = "_heavy_tests"), ignore)]
	async fn parse_all_teams_test_year() {
		let mlb_teams = TeamsRequest::all_sports()
			.season(TEST_YEAR)
			.build_and_get()
		.await
		.unwrap()
		.teams;
		for team in mlb_teams {
			let request = AttendanceRequest::builder()
				.team_id(team.id)
				.build();
			let _response = request.get()
				.await
				.unwrap();
		}
	}
}
