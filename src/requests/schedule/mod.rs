#![allow(non_snake_case)]

use crate::game::{DoubleHeaderKind, GameId};
use crate::league::LeagueId;
use crate::season::SeasonId;
use crate::teams::team::{Team, TeamId};
use crate::types::{Copyright, HomeAwaySplits, NaiveDateRange, MLB_API_DATE_FORMAT};
use crate::venue::{Venue, VenueId};
use crate::game_status::GameStatus;
use crate::game_types::GameType;
use crate::request::StatsAPIRequestUrl;
use crate::sky::Sky;
use crate::sports::SportId;
use bon::Builder;
use chrono::{NaiveDate, NaiveDateTime, Utc};
use either::Either;
use itertools::Itertools;
use serde::Deserialize;
use serde_with::serde_as;
use serde_with::DefaultOnError;
use std::fmt::{Display, Formatter};
use uuid::Uuid;

pub mod postseason;
pub mod tied;

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ScheduleResponse {
	pub copyright: Copyright,
	pub dates: Vec<ScheduleDate>,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ScheduleDate {
	pub date: NaiveDate,
	pub games: Vec<ScheduleGame>,
}

#[allow(clippy::struct_excessive_bools, reason = "false positive")]
#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(from = "__ScheduleGameStruct")]
pub struct ScheduleGame {
	pub game_id: GameId,
	pub game_guid: Uuid,
	pub game_type: GameType,
	pub season: SeasonId,
	pub game_date: NaiveDateTime,
	/// Different from `game_date.date()` in cases such as a rescheduled/postponed game (ex: Toronto @ Boston June 26, 2024)
	pub official_date: NaiveDate,
	pub status: GameStatus,
	pub teams: HomeAwaySplits<TeamWithStandings>,
	pub venue: Venue,
	pub is_tie: bool,

	/// Refers to the ordinal in the day? (maybe season?).
	/// Starts at 1.
	pub game_ordinal: u32,
	pub is_public_facing: bool,
	pub double_header: DoubleHeaderKind,
	// #[serde(rename = "gamedayType")]
	// pub gameday_game_type: GamedayGameType,
	pub is_tiebreaker: bool,
	// pub calender_event_id: CalenderEventId,
	pub displayed_season: SeasonId,
	pub day_night: Sky,
	pub description: Option<String>,
	pub scheduled_innings: u32,
	pub reverse_home_away_status: bool,
	pub inning_break_length: uom::si::i32::Time,
	/// [`None`] if the current game is not of a series-format (ex: [Spring Training](`GameType::SpringTraining`))
	pub series_data: Option<SeriesData>,
}

#[serde_as]
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct __ScheduleGameStruct {
	#[serde(rename = "gamePk")]
	game_id: GameId,
	game_guid: Uuid,
	game_type: GameType,
	season: SeasonId,
	#[serde(deserialize_with = "crate::types::deserialize_datetime")]
	game_date: NaiveDateTime,
	official_date: NaiveDate,
	status: GameStatus,
	teams: HomeAwaySplits<TeamWithStandings>,
	#[serde_as(deserialize_as = "DefaultOnError")]
	venue: Option<Venue>,
	is_tie: Option<bool>,
	#[serde(rename = "gameNumber")]
	game_ordinal: u32,
	#[serde(rename = "publicFacing")]
	is_public_facing: bool,
	double_header: DoubleHeaderKind,
	// #[serde(rename = "gamedayType")]
	// gameday_game_type: GamedayGameType,
	#[serde(rename = "tiebreaker", deserialize_with = "crate::types::from_yes_no")]
	is_tiebreaker: bool,
	// calender_event_id: CalenderEventId,
	#[serde(rename = "seasonDisplay")]
	displayed_season: SeasonId,
	day_night: Sky,
	description: Option<String>,
	scheduled_innings: u32,
	reverse_home_away_status: bool,
	inning_break_length: Option<u32>,
	#[serde(flatten)]
	series_data: Option<SeriesData>,
}

impl From<__ScheduleGameStruct> for ScheduleGame {
	#[allow(clippy::cast_possible_wrap, reason = "not gonna happen")]
	fn from(
		__ScheduleGameStruct {
			game_id,
			game_guid,
			game_type,
			season,
			game_date,
			official_date,
			status,
			teams,
			venue,
			is_tie,
			game_ordinal,
			is_public_facing,
			double_header,
			is_tiebreaker,
			displayed_season,
			day_night,
			description,
			scheduled_innings,
			reverse_home_away_status,
			inning_break_length,
			series_data,
		}: __ScheduleGameStruct,
	) -> Self {
		Self {
			game_id,
			game_guid,
			game_type,
			season,
			game_date,
			official_date,
			status,
			teams,
			venue: venue.unwrap_or_else(Venue::unknown_venue),
			is_tie: is_tie.unwrap_or(false),
			game_ordinal,
			is_public_facing,
			double_header,
			is_tiebreaker,
			displayed_season,
			day_night,
			description,
			scheduled_innings,
			reverse_home_away_status,
			inning_break_length: uom::si::i32::Time::new::<uom::si::time::second>(inning_break_length.unwrap_or(120) as i32),
			series_data,
		}
	}
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SeriesData {
	pub games_in_series: u32,
	#[serde(rename = "seriesGameNumber")]
	pub game_in_series_ordinal: u32,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TeamWithStandings {
	pub team: Team,
	#[serde(rename = "leagueRecord")]
	pub standings: Standings,
	#[serde(flatten)]
	pub score: Option<TeamWithStandingsGameScore>,
	#[serde(rename = "splitSquad")]
	pub is_split_squad_game: bool,

	/// Refers to the ordinal of series, not within the current series.
	/// Starts at 1.
	/// [`None`] if the current game is not of a series-format (ex: [Spring Training](`GameType::SpringTraining`))
	#[serde(rename = "seriesNumber")]
	pub series_ordinal: Option<u32>,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TeamWithStandingsGameScore {
	#[serde(rename = "score")]
	pub runs_scored: u32,
	pub is_winner: bool,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Copy, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Standings {
	pub wins: u32,
	pub losses: u32,
}

impl Standings {
	#[must_use]
	pub const fn games_played(self) -> u32 {
		self.wins + self.losses
	}

	#[must_use]
	pub fn pct(self) -> f64 {
		f64::from(self.wins) / f64::from(self.games_played())
	}
}

#[allow(dead_code, reason = "rust analyzer says that opponent_id and season are dead, while being used in Display")]
#[derive(Builder)]
#[builder(derive(Into))]
pub struct ScheduleRequest {
	#[builder(into)]
	#[builder(default)]
	sport_id: SportId,
	#[builder(setters(vis = "", name = __game_ids_internal))]
	game_ids: Option<Vec<GameId>>,
	#[builder(into)]
	team_id: Option<TeamId>,
	#[builder(into)]
	league_id: Option<LeagueId>,
	#[builder(setters(vis = "", name = __venue_ids_internal))]
	venue_ids: Option<Vec<VenueId>>,
	#[builder(default = Either::Left(Utc::now().date_naive()))]
	#[builder(setters(vis = "", name = __date_internal))]
	date: Either<NaiveDate, NaiveDateRange>,
	#[builder(into)]
	opponent_id: Option<TeamId>,
	#[builder(into)]
	season: Option<SeasonId>,
}


impl<S: schedule_request_builder::State + schedule_request_builder::IsComplete> crate::request::StatsAPIRequestUrlBuilderExt for ScheduleRequestBuilder<S> {
    type Built = ScheduleRequest;
}

impl<S: schedule_request_builder::State> ScheduleRequestBuilder<S> {
	pub fn game_ids(self, game_ids: Vec<impl Into<GameId>>) -> ScheduleRequestBuilder<schedule_request_builder::SetGameIds<S>> where S::GameIds: schedule_request_builder::IsUnset {
		self.__game_ids_internal(game_ids.into_iter().map(Into::into).collect())
	}

	pub fn venue_ids(self, venue_ids: Vec<impl Into<VenueId>>) -> ScheduleRequestBuilder<schedule_request_builder::SetVenueIds<S>> where S::VenueIds: schedule_request_builder::IsUnset {
		self.__venue_ids_internal(venue_ids.into_iter().map(Into::into).collect())
	}

	pub fn date(self, date: NaiveDate) -> ScheduleRequestBuilder<schedule_request_builder::SetDate<S>> where S::Date: schedule_request_builder::IsUnset {
		self.__date_internal(Either::Left(date))
	}

	pub fn date_range(self, range: NaiveDateRange) -> ScheduleRequestBuilder<schedule_request_builder::SetDate<S>> where S::Date: schedule_request_builder::IsUnset {
		self.__date_internal(Either::Right(range))
	}
}

impl Display for ScheduleRequest {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"http://statsapi.mlb.com/api/v1/schedule{params}",
			params = gen_params! {
				"sportId": self.sport_id,
				"gamePks"?: self.game_ids.as_ref().map(|ids| ids.iter().map(ToString::to_string).join(",")),
				"teamId"?: self.team_id,
				"leagueId"?: self.league_id,
				"venueIds"?: self.venue_ids.as_ref().map(|ids| ids.iter().map(ToString::to_string).join(",")),
				"date"?: self.date.as_ref().left().map(|x| x.format(MLB_API_DATE_FORMAT)),
				"startDate"?: self.date.as_ref().right().map(|range| range.start().format(MLB_API_DATE_FORMAT)),
				"endDate"?: self.date.as_ref().right().map(|range| range.end().format(MLB_API_DATE_FORMAT)),
				"opponentId"?; self.opponent_id,
				"season"?: self.season,
			}
		)
	}
}

impl StatsAPIRequestUrl for ScheduleRequest {
	type Response = ScheduleResponse;
}

#[cfg(test)]
mod tests {
	use crate::schedule::ScheduleRequest;
	use crate::TEST_YEAR;
	use chrono::NaiveDate;
	use crate::request::StatsAPIRequestUrlBuilderExt;

	#[tokio::test]
	async fn test_one_date() {
		let date = NaiveDate::from_ymd_opt(2020, 8, 2).expect("Valid date");
		let _ = ScheduleRequest::builder().date(date).build_and_get().await.unwrap();
	}

	#[tokio::test]
	async fn test_all_dates_current_year() {
		let _ = ScheduleRequest::builder().date_range(NaiveDate::from_ymd_opt(TEST_YEAR.try_into().unwrap(), 1, 1).expect("Valid date")..=NaiveDate::from_ymd_opt(TEST_YEAR.try_into().unwrap(), 12, 31).expect("Valid date")).build_and_get().await.unwrap();
	}

	#[tokio::test]
	#[cfg_attr(not(feature = "_heavy_tests"), ignore)]
	async fn test_all_dates_all_years() {
		for year in 1876..=TEST_YEAR {
			let _ = ScheduleRequest::builder().date_range(NaiveDate::from_ymd_opt(year.try_into().unwrap(), 1, 1).unwrap()..=NaiveDate::from_ymd_opt(year.try_into().unwrap(), 12, 31).unwrap())
			.build_and_get()
			.await
			.unwrap();
		}
	}
}
