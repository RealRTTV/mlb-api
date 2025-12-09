use crate::game::Game;
use crate::gen_params;
use crate::league::LeagueId;
use crate::seasons::season::SeasonId;
use crate::teams::team::TeamId;
use crate::types::{Copyright, HomeAwaySplits, MLB_API_DATE_FORMAT};
use crate::{GameType, StatsAPIRequestUrl};
use bon::Builder;
use chrono::{Datelike, Local, NaiveDate, NaiveDateTime};
use either::Either;
use serde::Deserialize;
use std::cmp::Ordering;
use std::fmt::{Debug, Display, Formatter};
use std::iter::Sum;
use std::ops::Add;

/// Within regards to attendance, the term frequently used is "Opening" over "Game"; this is for reasons including but not limited to: single ticket double headers, and partially cancelled/rescheduled games.
///
/// Averages are calculated with respect to the # of openings on the sample, not the number of games the team played as either "home" or "away".
///
/// Since the 2020 season had 0 total attendance, the 'peak attendance game' has its default value of [`NaiveDate::MIN`]
#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(from = "AttendanceResponseStruct")]
pub struct AttendanceResponse {
	pub copyright: Copyright,
	#[serde(rename = "records")]
	pub annual_records: Vec<AttendanceRecord>,
}

impl AttendanceResponse {
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

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(from = "AnnualRecordStruct")]
pub struct AttendanceRecord {
	pub total_openings: HomeAwaySplits<u32>,
	pub total_openings_lost: u32,
	pub total_games: HomeAwaySplits<u32>,
	pub season: SeasonId,
	pub attendance_totals: HomeAwaySplits<u32>,
	pub single_opening_high: DatedAttendance,
	pub single_opening_low: DatedAttendance,
	pub game_type: GameType,
}

impl Add for AttendanceRecord {
	type Output = Self;

	/// Since the [`AttendanceRecord::default()`] value has some "worse"-er defaults (high and low attendance records have the epoch start time as their dates), we always take the later values in case of ties.
	fn add(self, rhs: Self) -> Self::Output {
		Self {
			total_openings: HomeAwaySplits {
				home: self.total_openings.home + rhs.total_openings.home,
				away: self.total_openings.away + rhs.total_openings.away,
			},
			total_openings_lost: self.total_openings_lost + rhs.total_openings_lost,
			total_games: HomeAwaySplits {
				home: self.total_games.home + rhs.total_games.home,
				away: self.total_games.away + rhs.total_games.away,
			},
			season: SeasonId::max(self.season, rhs.season),
			attendance_totals: HomeAwaySplits {
				home: self.attendance_totals.home + rhs.attendance_totals.home,
				away: self.attendance_totals.away + rhs.attendance_totals.away,
			},
			single_opening_high: self.single_opening_high.max(rhs.single_opening_high), // ties go to rhs
			single_opening_low: self.single_opening_low.min(rhs.single_opening_low),    // ties go to rhs
			game_type: rhs.game_type,
		}
	}
}

impl Default for AttendanceRecord {
	fn default() -> Self {
		Self {
			total_openings: HomeAwaySplits::new(0, 0),
			total_openings_lost: 0,
			total_games: HomeAwaySplits::new(0, 0),
			season: (Local::now().year() as u32).into(),
			attendance_totals: HomeAwaySplits::new(0, 0),
			single_opening_high: DatedAttendance::default(),
			single_opening_low: DatedAttendance::default(),
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
	#[must_use]
	pub fn average_attendance(&self) -> HomeAwaySplits<u32> {
		let HomeAwaySplits { home, away } = self.attendance_totals;
		let HomeAwaySplits { home: num_at_home, away: num_at_away } = self.total_openings;
		HomeAwaySplits::new((home + num_at_home / 2) / num_at_home, (away + num_at_away / 2) / num_at_away)
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
	year: SeasonId,
	// attendance_average_away: u32,
	// attendance_average_home: u32,
	// attendance_average_ytd: u32,
	attendance_high: u32,
	attendance_high_date: NaiveDateTime,
	attendance_high_game: Game,
	attendance_low: Option<u32>,
	attendance_low_date: Option<NaiveDateTime>,
	attendance_low_game: Option<Game>,
	// attendance_opening_average: u32,
	// attendance_total: u32,
	attendance_total_away: u32,
	attendance_total_home: u32,
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
		let high = DatedAttendance {
			value: attendance_high,
			date: attendance_high_date.date(),
			game: attendance_high_game,
		};
		Self {
			total_openings: HomeAwaySplits {
				home: openings_total_home,
				away: openings_total_away,
			},
			total_openings_lost: openings_total_lost,
			total_games: HomeAwaySplits {
				home: games_home_total,
				away: games_away_total,
			},
			season: year,
			attendance_totals: HomeAwaySplits {
				home: attendance_total_home,
				away: attendance_total_away,
			},
			single_opening_low: {
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
					high.clone()
				}
			},
			single_opening_high: high,
			game_type,
		}
	}
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct DatedAttendance {
	pub value: u32,
	pub date: NaiveDate,
	pub game: Game,
}

impl Default for DatedAttendance {
	fn default() -> Self {
		Self {
			value: 0,
			date: NaiveDate::default(),
			game: Game::default(),
		}
	}
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

#[derive(Builder)]
#[builder(derive(Into))]
pub struct AttendanceRequest {
	#[doc(hidden)]
	#[builder(setters(vis = "", name = __id_internal))]
	id: Either<TeamId, LeagueId>,
	#[builder(into)]
	season: Option<SeasonId>,
	#[builder(into)]
	date: Option<NaiveDate>,
	#[builder(default)]
	game_type: GameType,
}

impl<S: State> crate::requests::links::StatsAPIRequestUrlBuilderExt for AttendanceRequestBuilder<S> where S: attendance_request_builder::IsComplete {
    type Built = AttendanceRequest;
}

use attendance_request_builder::{IsUnset, SetId, State};

#[allow(dead_code)]
impl<S: State> AttendanceRequestBuilder<S> {
	#[doc = "_**Required.**_\n\n"]
	pub fn team_id(self, id: impl Into<TeamId>) -> AttendanceRequestBuilder<SetId<S>>
	where
		S::Id: IsUnset,
	{
		self.__id_internal(Either::Left(id.into()))
	}

	#[doc = "_**Required.**_\n\n"]
	pub fn league_id(self, id: impl Into<LeagueId>) -> AttendanceRequestBuilder<SetId<S>>
	where
		S::Id: IsUnset,
	{
		self.__id_internal(Either::Right(id.into()))
	}
}

impl Display for AttendanceRequest {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"http://statsapi.mlb.com/api/v1/attendance{}",
			gen_params! { "teamId"?: self.id.clone().left(), "leagueId"?: self.id.clone().right(), "season"?: self.season, "date"?: self.date.as_ref().map(|date| date.format(MLB_API_DATE_FORMAT)), "gameType": format!("{:?}", self.game_type) }
		)
	}
}

impl StatsAPIRequestUrl for AttendanceRequest {
	type Response = AttendanceResponse;
}

#[cfg(test)]
mod tests {
	use crate::attendance::AttendanceRequest;
	use crate::teams::TeamsRequest;
	use crate::StatsAPIRequestUrlBuilderExt;

	#[tokio::test]
	#[cfg_attr(not(feature = "_heavy_tests"), ignore)]
	async fn parse_all_mlb_teams_2025() {
		let mlb_teams = TeamsRequest::builder()
			.season(2025)
			.build_and_get()
		.await
		.unwrap()
		.teams;
		for team in mlb_teams {
			let _response = AttendanceRequest::builder()
				.team_id(team.id)
				.build_and_get()
				.await
				.unwrap();
		}
	}
}
