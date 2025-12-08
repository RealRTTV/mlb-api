use serde_with::DefaultOnError;
use crate::endpoints::game::{DoubleHeaderKind, GameId};
use crate::endpoints::league::LeagueId;
use crate::endpoints::sports::SportId;
use crate::endpoints::teams::team::{Team, TeamId};
use crate::endpoints::venue::{Venue, VenueId};
use crate::endpoints::{GameStatus, GameType, Sky, StatsAPIEndpointUrl};
use crate::gen_params;
use crate::types::{Copyright, HomeAwaySplits, MLB_API_DATE_FORMAT, NaiveDateRange};
use chrono::{NaiveDate, Utc};
use itertools::Itertools;
use serde::Deserialize;
use serde_with::DisplayFromStr;
use serde_with::serde_as;
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

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(from = "__ScheduleGameStruct")]
pub struct ScheduleGame {
	pub game_id: GameId,
	pub game_guid: Uuid,
	pub game_type: GameType,
	pub season: u32,
	// pub game_date: NaiveDateTime,
	/// Different from `game_date` in cases such as a rescheduled/postponed game (ex: Toronto @ Boston June 26, 2024)
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
	pub displayed_season: u32,
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
	#[serde_as(as = "DisplayFromStr")]
	season: u32,
	// game_date: NaiveDateTime,
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
	#[serde_as(as = "DisplayFromStr")]
	#[serde(rename = "seasonDisplay")]
	displayed_season: u32,
	day_night: Sky,
	description: Option<String>,
	scheduled_innings: u32,
	reverse_home_away_status: bool,
	inning_break_length: Option<u32>,
	#[serde(flatten)]
	series_data: Option<SeriesData>,
}

impl From<__ScheduleGameStruct> for ScheduleGame {
	fn from(
		__ScheduleGameStruct {
			game_id,
			game_guid,
			game_type,
			season,
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

	// todo: replace with PercentageStat
	#[must_use]
	pub fn pct(self) -> f64 {
		self.wins as f64 / self.games_played() as f64
	}
}

pub struct ScheduleEndpoint {
	pub sport_id: SportId,
	pub game_ids: Option<Vec<GameId>>,
	pub team_id: Option<TeamId>,
	pub league_id: Option<LeagueId>,
	pub venue_ids: Option<Vec<VenueId>>,
	pub date: Result<NaiveDate, NaiveDateRange>,
	pub opponent_id: Option<TeamId>,
	pub season: Option<u32>,
}

impl Default for ScheduleEndpoint {
	fn default() -> Self {
		Self {
			sport_id: SportId::MLB,
			game_ids: None,
			team_id: None,
			league_id: None,
			venue_ids: None,
			date: Ok(Utc::now().date_naive()),
			opponent_id: None,
			season: None,
		}
	}
}

impl Display for ScheduleEndpoint {
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
				"date"?: self.date.as_ref().ok().map(|x| x.format(MLB_API_DATE_FORMAT)),
				"startDate"?: self.date.as_ref().err().map(|range| range.start().format(MLB_API_DATE_FORMAT)),
				"endDate"?: self.date.as_ref().err().map(|range| range.end().format(MLB_API_DATE_FORMAT)),
				"opponentId"?; self.opponent_id,
				"season"?: self.season,
			}
		)
	}
}

impl StatsAPIEndpointUrl for ScheduleEndpoint {
	type Response = ScheduleResponse;
}

#[cfg(test)]
mod tests {
	use crate::endpoints::StatsAPIEndpointUrl;
	use crate::endpoints::schedule::ScheduleEndpoint;
	use chrono::{Datelike, Local, NaiveDate};

	#[tokio::test]
	async fn test_one_date() {
		let date = NaiveDate::from_ymd_opt(2020, 8, 2).expect("Valid date");
		let _ = ScheduleEndpoint { date: Ok(date), ..Default::default() }.get().await.unwrap();
	}

	#[tokio::test]
	async fn test_all_dates_current_year() {
		let current_date = Local::now().naive_local().date();
		let request = ScheduleEndpoint {
			date: Err(current_date.with_ordinal0(0).unwrap()..=current_date.with_month(12).unwrap().with_day(31).unwrap()),
			..Default::default()
		};
		dbg!(request.to_string());
		let _ = request.get().await.unwrap();
	}

	#[tokio::test]
	#[cfg_attr(not(feature = "_heavy_tests"), ignore)]
	async fn test_all_dates_all_years() {
		for year in 1876..=Local::now().year() as _ {
			dbg!(year);
			let _ = ScheduleEndpoint {
				date: Err(NaiveDate::from_ymd_opt(year, 1, 1).unwrap()..=NaiveDate::from_ymd_opt(year, 12, 31).unwrap()),
				..Default::default()
			}
			.get()
			.await
			.unwrap();
		}
	}
}
