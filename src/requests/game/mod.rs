//! The thing you're most likely here for.
//!
//! This module itself acts like [`crate::types`] but for misc game-specific types as there are many.

#![allow(unused_imports, reason = "usage of children modules")]

use std::fmt::{Display, Formatter};
use std::marker::PhantomData;
use std::ops::{ControlFlow, Sub};
use std::time::{Duration, Instant};
use bon::Builder;
use chrono::{NaiveDate, NaiveDateTime, NaiveTime, Utc};
use derive_more::{Deref, DerefMut, Display, From, Not};
use fxhash::FxHashMap;
use serde::{Deserialize, Deserializer};
use serde::de::{DeserializeOwned, Error, IgnoredAny, MapAccess, Visitor};
use serde_with::{serde_as, DisplayFromStr};
use crate::person::{Ballplayer, JerseyNumber, NamedPerson, PersonId};
use crate::meta::{DayNight, NamedPosition};
use crate::request::RequestURLBuilderExt;
use crate::team::TeamId;
use crate::team::roster::RosterStatus;
use crate::{DayHalf, HomeAway, ResourceUsage};
use crate::meta::WindDirectionId;
use crate::request;

mod boxscore; // done
mod changes;
mod content;
mod context_metrics;
mod diff;
mod linescore; // done
mod pace; // done
mod plays; // done
mod timestamps; // done
mod uniforms;
mod win_probability;
mod live_feed; // done

pub use boxscore::*;
pub use changes::*;
pub use content::*;
pub use context_metrics::*;
pub use diff::*;
pub use linescore::*;
pub use pace::*;
pub use plays::*;
pub use timestamps::*;
pub use uniforms::*;
pub use win_probability::*;
pub use live_feed::*;

id!(#[doc = "A [`u32`] representing a baseball game. [Sport](crate::sport)-independent"] GameId { gamePk: u32 });

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
#[doc(hidden)]
#[cfg_attr(feature = "_debug", serde(deny_unknown_fields))]
struct __GameDateTimeStruct {
	#[serde(rename = "dateTime", deserialize_with = "crate::deserialize_datetime")]
	datetime: NaiveDateTime,
	original_date: NaiveDate,
	official_date: NaiveDate,
	#[serde(rename = "dayNight")]
	sky: DayNight,
	time: NaiveTime,
	ampm: DayHalf,
}

/// Date & Time of the game. Note that the time is typically rounded to the hour and the :07, :05 on the hour is for the first pitch, which is a different timestamp.
#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(from = "__GameDateTimeStruct")]
pub struct GameDateTime {
	pub datetime: NaiveDateTime,
	pub original_date: NaiveDate,
	pub official_date: NaiveDate,
	pub sky: DayNight,
}

impl From<__GameDateTimeStruct> for GameDateTime {
	fn from(value: __GameDateTimeStruct) -> Self {
		let date = value.datetime.date();
		let time = value.ampm.into_24_hour_time(value.time);
		Self {
			datetime: NaiveDateTime::new(date, time),
			original_date: value.original_date,
			official_date: value.official_date,
			sky: value.sky,
		}
	}
}

/// General weather conditions, temperature, wind, etc.
#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(try_from = "__WeatherConditionsStruct")]
pub struct WeatherConditions {
	pub condition: String,
	pub temp: uom::si::f64::ThermodynamicTemperature,
	pub wind_speed: uom::si::f64::Velocity,
	pub wind_direction: WindDirectionId,
}

#[serde_as]
#[derive(Deserialize)]
#[doc(hidden)]
#[cfg_attr(feature = "_debug", serde(deny_unknown_fields))]
struct __WeatherConditionsStruct {
	condition: String,
	#[serde_as(as = "DisplayFromStr")]
	temp: i32,
	wind: String,
}

impl TryFrom<__WeatherConditionsStruct> for WeatherConditions {
	type Error = &'static str;

	fn try_from(value: __WeatherConditionsStruct) -> Result<Self, Self::Error> {
		let (speed, direction) = value.wind.split_once(" mph, ").ok_or("invalid wind format")?;
		let speed = speed.parse::<i32>().map_err(|_| "invalid wind speed")?;
		Ok(Self {
			condition: value.condition,
			temp: uom::si::f64::ThermodynamicTemperature::new::<uom::si::thermodynamic_temperature::degree_fahrenheit>(value.temp as f64),
			wind_speed: uom::si::f64::Velocity::new::<uom::si::velocity::mile_per_hour>(speed as f64),
			wind_direction: WindDirectionId::new(direction),
		})
	}
}

/// Misc
#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "_debug", serde(deny_unknown_fields))]
pub struct GameInfo {
	pub attendance: Option<u32>,
	#[serde(deserialize_with = "crate::deserialize_datetime")]
	pub first_pitch: NaiveDateTime,
	/// Measured in minutes,
	#[serde(rename = "gameDurationMinutes")]
	pub game_duration: Option<u32>,
	/// Durationg of the game delay; measured in minutes.
	#[serde(rename = "delayDurationMinutes")]
	pub delay_duration: Option<u32>,
}

/// Review usage for each team and if the game supports challenges.
#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "_debug", serde(deny_unknown_fields))]
pub struct TeamReviewData {
	pub has_challenges: bool,
	#[serde(flatten)]
	pub teams: HomeAway<ResourceUsage>,
}

/// Tags about a game, such as a perfect game in progress, no-hitter, etc.
#[allow(clippy::struct_excessive_bools, reason = "")]
#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "_debug", serde(deny_unknown_fields))]
pub struct GameTags {
	no_hitter: bool,
	perfect_game: bool,

	away_team_no_hitter: bool,
	away_team_perfect_game: bool,

	home_team_no_hitter: bool,
	home_team_perfect_game: bool,
}

/// Double-header information.
#[derive(Debug, Deserialize, PartialEq, Eq, Copy, Clone)]
pub enum DoubleHeaderKind {
	#[serde(rename = "N")]
	/// Not a doubleheader
	Not,

	#[serde(rename = "Y")]
	/// First game in a double-header
	FirstGame,

	#[serde(rename = "S")]
	/// Second game in a double-header.
	SecondGame,
}

impl DoubleHeaderKind {
	#[must_use]
	pub const fn is_double_header(self) -> bool {
		matches!(self, Self::FirstGame | Self::SecondGame)
	}
}

#[derive(Debug, Deserialize, Copy, Clone, PartialEq, Eq, Deref, DerefMut, From)]
pub struct Inning(usize);

impl Display for Inning {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		crate::write_nth(self.0, f)
	}
}

/// Half of the inning.
#[derive(Debug, Deserialize, Copy, Clone, PartialEq, Eq, Not)]
pub enum InningHalf {
	#[serde(rename = "Top", alias = "top")]
	Top,
	#[serde(rename = "Bottom", alias = "bottom")]
	Bottom,
}

impl InningHalf {
	/// A unicode character representing an up or down arrow.
	#[must_use]
	pub const fn unicode_char_filled(self) -> char {
		match self {
			Self::Top => '▲',
			Self::Bottom => '▼',
		}
	}
	
	/// A hollow character representing the inning half
	#[must_use]
	pub const fn unicode_char_empty(self) -> char {
		match self {
			Self::Top => '△',
			Self::Bottom => '▽',
		}
	}

	#[must_use]
	pub const fn three_char(self) -> &'static str {
		match self {
			Self::Top => "Top",
			Self::Bottom => "Bot",
		}
	}
}

/// The balls and strikes in a given at bat. Along with the number of outs (this technically can change during the AB due to pickoffs etc)
#[derive(Debug, Deserialize, PartialEq, Eq, Copy, Clone, Display)]
#[display("{balls}-{strikes} ({outs} out)")]
pub struct AtBatCount {
	pub balls: u8,
	pub strikes: u8,
	pub outs: u8,
}

/// The classic "R | H | E" and LOB in a scoreboard.
#[derive(Debug, Deserialize, PartialEq, Eq, Copy, Clone)]
#[serde(from = "__RHEStruct")]
#[cfg_attr(feature = "_debug", serde(deny_unknown_fields))]
pub struct RHE {
	pub runs: usize,
	pub hits: usize,
	pub errors: usize,
	pub left_on_base: usize,
	/// Ex: Home team wins and doesn't need to play Bot 9.
	pub was_inning_half_played: bool,
}

#[doc(hidden)]
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct __RHEStruct {
	pub runs: Option<usize>,
    pub hits: usize,
    pub errors: usize,
    pub left_on_base: usize,

    // only sometimes present, regardless of whether a game is won
    #[doc(hidden)]
    #[serde(rename = "isWinner", default)]
    pub __is_winner: IgnoredAny,
}

impl From<__RHEStruct> for RHE {
	fn from(__RHEStruct { runs, hits, errors, left_on_base, .. }: __RHEStruct) -> Self {
		Self {
			runs: runs.unwrap_or(0),
			hits,
			errors,
			left_on_base,
			was_inning_half_played: runs.is_some(),
		}
	}
}

/// Unparsed miscellaneous data.
///
/// Some of these values might be handwritten per game so parsing them would prove rather difficult.
/// 
/// ## Examples
/// | Name          | Value     |
/// |---------------|-----------|
/// | First pitch   | 8:10 PM.  |
/// | Weather       | 68 degrees, Roof Closed |
/// | Att           | 44,713.   |
#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "_debug", serde(deny_unknown_fields))]
pub struct LabelledValue {
	pub label: String,
	#[serde(default)]
	pub value: String,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "_debug", serde(deny_unknown_fields))]
pub struct SectionedLabelledValues {
	#[serde(rename = "title")]
	pub section: String,
	#[serde(rename = "fieldList")]
	pub values: Vec<LabelledValue>,
}

/// Various flags about the player in the current game
#[allow(clippy::struct_excessive_bools, reason = "not what's happening here")]
#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "_debug", serde(deny_unknown_fields))]
pub struct PlayerGameStatusFlags {
	pub is_current_batter: bool,
	pub is_current_pitcher: bool,
	pub is_on_bench: bool,
	pub is_substitute: bool,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "_debug", serde(deny_unknown_fields))]
pub struct Official {
	pub official: NamedPerson,
	pub official_type: OfficialType,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Copy, Clone)]
pub enum OfficialType {
	#[serde(rename = "Home Plate")]
	HomePlate,
	#[serde(rename = "First Base")]
	FirstBase,
	#[serde(rename = "Second Base")]
	SecondBase,
	#[serde(rename = "Third Base")]
	ThirdBase,
	#[serde(rename = "Left Field")]
	LeftField,
	#[serde(rename = "Right Field")]
	RightField,
}

/// A position in the batting order, 1st, 2nd, 3rd, 4th, etc.
///
/// Note that this number is split in two, the general batting order position is the `major` while if there is a lineup movement then the player would have an increased `minor` since they replace an existing batting order position.
///
/// Example:
/// Alice bats 1st (major = 1, minor = 0)
/// Bob pinch hits and bats 1st for Alice (major = 1, minor = 1)
/// Alice somehow hits again (major = 1, minor = 0)
/// Charlie pinch runs and takes over from then on (major = 1, minor = 2)
///
/// Note: These minors are [`Display`]ed incremented one more than is done internally, so (major = 1, minor = 1) displays as `1st (2)`.
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct BattingOrderIndex {
	pub major: usize,
	pub minor: usize,
}

impl<'de> Deserialize<'de> for BattingOrderIndex {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
	    D: Deserializer<'de>
	{
		let v: usize = String::deserialize(deserializer)?.parse().map_err(D::Error::custom)?;
		Ok(Self {
			major: v / 100,
			minor: v % 100,
		})
	}
}

impl Display for BattingOrderIndex {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		crate::write_nth(self.major, f)?;
		if self.minor > 0 {
			write!(f, " ({})", self.minor + 1)?;
		}
		Ok(())
	}
}

/// Decisions of winner & loser (and potentially the save)
#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "_debug", serde(deny_unknown_fields))]
pub struct Decisions {
	pub winner: Option<NamedPerson>,
	pub loser: Option<NamedPerson>,
	pub save: Option<NamedPerson>,
}

/// Game records in stats like exit velocity, hit distance, etc.
///
/// Currently unable to actually get data for these though
#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "_debug", serde(deny_unknown_fields))]
pub struct GameStatLeaders {
	#[doc(hidden)]
	#[serde(rename = "hitDistance", default)]
	pub __distance: IgnoredAny,
	#[doc(hidden)]
	#[serde(rename = "hitSpeed", default)]
	pub __exit_velocity: IgnoredAny,
	#[doc(hidden)]
	#[serde(rename = "pitchSpeed", default)]
	pub __velocity: IgnoredAny,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Display)]
pub enum Base {
	#[display("1B")]
	First,
	#[display("2B")]
	Second,
	#[display("3B")]
	Third,
	#[display("HP")]
	Home,
}

impl<'de> Deserialize<'de> for Base {
	#[allow(clippy::too_many_lines, reason = "Visitor impl takes up the bulk, is properly scoped")]
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>
	{
		struct BaseVisitor;

		impl Visitor<'_> for BaseVisitor {
			type Value = Base;

			fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
				write!(f, "a string or integer representing the base")
			}

			fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
			where
				E: Error,
			{
				Ok(match v {
					"1B" | "1" => Base::First,
					"2B" | "2" => Base::Second,
					"3B" | "3" => Base::Third,
					"score" | "HP" | "4B" | "4" => Base::Home,
					_ => return Err(E::unknown_variant(v, &["1B", "1", "2B" , "2", "3B", "3", "score", "HP", "4B", "4"]))
				})
			}

			fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
			where
				E: Error,
			{
				Ok(match v {
					1 => Base::First,
					2 => Base::Second,
					3 => Base::Third,
					4 => Base::Home,
					_ => return Err(E::unknown_variant("[a number]", &["1", "2", "3", "4"]))
				})
			}
		}

		deserializer.deserialize_any(BaseVisitor)
	}
}

#[derive(Debug, Deserialize, PartialEq, Eq, Copy, Clone)]
pub enum ContactHardness {
	#[serde(rename = "soft")]
	Soft,
	#[serde(rename = "medium")]
	Medium,
	#[serde(rename = "hard")]
	Hard,
}

pub(crate) fn deserialize_players_cache<'de, T: DeserializeOwned, D: Deserializer<'de>>(deserializer: D) -> Result<FxHashMap<PersonId, T>, D::Error> {
	struct PlayersCacheVisitor<T2: DeserializeOwned>(PhantomData<T2>);

	impl<'de2, T2: DeserializeOwned> serde::de::Visitor<'de2> for PlayersCacheVisitor<T2> {
		type Value = FxHashMap<PersonId, T2>;

		fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
			formatter.write_str("a map")
		}

		fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
		where
			A: MapAccess<'de2>,
		{
			let mut values = FxHashMap::default();

			while let Some((key, value)) = map.next_entry()? {
				let key: String = key;
				let key = PersonId::new(key.strip_prefix("ID").ok_or_else(|| A::Error::custom("invalid id format"))?.parse::<u32>().map_err(A::Error::custom)?);
				values.insert(key, value);
			}

			Ok(values)
		}
	}

	deserializer.deserialize_map(PlayersCacheVisitor::<T>(PhantomData))
}

/// Meant for active & live games, gives a streamable version of the plays in a game.
///
/// The [`PlayStream`] is meant to be for consistently polling the MLB API for live play-by-play updates.
/// The list of events can be seen on [`PlayStreamEvent`]
/// 
/// ## Examples
/// ```no_run
/// PlayStream::new(/* game id */).run(|event: PlayStreamEvent, meta: &LiveFeedMetadata, data: &LiveFeedData| {
///     match event {
///         PlayStreamEvent::GameStart => println!("Game Start"),
///         PlayStreamEvent::StartPlay(play) => println!("{} vs. {}", play.matchup.batter.full_name, play.matchup.pitcher.full_name),
///         PlayStreamEvent::PlayEvent(play_event) => {
///             match play_event {
///                 PlayEvent::Action { details, .. } => println!("{}", details.description),
///                 PlayEvent::Pitch { details, common, .. } => println!("{} -> {}", details.call, common.count),
///                 PlayEvent::Stepoff { .. } => println!("Stepoff"),
///                 PlayEvent::NoPitch { .. } => println!("No Pitch"),
///                 PlayEvent::Pickoff { .. } => println!("Pickoff"),
///             }
///         },
///         PlayStreamEvent::PlayEventReviewStart(review) => println!("PlayEventReviewStart; {}", review.review_type),
///         PlayStreamEvent::PlayEventReviewEnd(review) => println!("PlayEventReviewEnd; {}", review.review_type),
///         PlayStreamEvent::PlayReviewStart(review) => println!("PlayReviewStart; {}", review.review_type),
///         PlayStreamEvent::PlayReviewEnd(review) => println!("PlayReviewEnd; {}", review.review_type),
///         PlayStreamEvent::EndPlay(play) => println!("{}", play.result.completed_play_details.as_ref().expect("Completed play").description),
///         PlayStreamEvent::GameEnd(_, _, _, _) => println!("GameEnd"),
///     }
/// }).await?;
/// ```
#[derive(Debug)]
pub struct PlayStream {
	game_id: GameId,

	current_play_idx: usize,
	in_progress_current_play: bool,
	current_play_review_idx: usize,
	in_progress_current_play_review: bool,
	
	current_play_event_idx: usize,
	current_play_event_review_idx: usize,
	in_progress_current_play_event_review: bool,
}

impl PlayStream {
	#[must_use]
	pub fn new(game_id: impl Into<GameId>) -> Self {
		Self {
			game_id: game_id.into(),
			
			current_play_idx: 0,
			in_progress_current_play: false,
			current_play_review_idx: 0,
			in_progress_current_play_review: false,
			
			current_play_event_idx: 0,
			current_play_event_review_idx: 0,
			in_progress_current_play_event_review: false,
		}
	}
}

/// An event in a game, such as the game starting, ending, a [`Play`] (At-Bat) starting, or a [`PlayEvent`] occuring, or a challenge on a play or play event.
#[derive(Debug, PartialEq, Clone)]
pub enum PlayStreamEvent<'a> {
	/// Sent at the beginning of a game
	GameStart,
	
	StartPlay(&'a Play),
	PlayReviewStart(&'a ReviewData, &'a Play),
	PlayReviewEnd(&'a ReviewData, &'a Play),
	EndPlay(&'a Play),
	
	PlayEvent(&'a PlayEvent, &'a Play),
	PlayEventReviewStart(&'a ReviewData, &'a PlayEvent, &'a Play),
	PlayEventReviewEnd(&'a ReviewData, &'a PlayEvent, &'a Play),
	
	GameEnd(&'a Decisions, &'a Linescore, &'a Boxscore, &'a GameStatLeaders),
}

impl PlayStream {
	/// Runs through plays until the game is over.
	///
	/// # Errors
	/// See [`request::Error`]
	pub async fn run<F: AsyncFnMut(PlayStreamEvent, &LiveFeedMetadata, &LiveFeedData) -> Result<ControlFlow<()>, request::Error>>(self, f: F) -> Result<(), request::Error> {
		self.run_with_custom_error::<request::Error, F>(f).await
	}

	/// Evaluation for the current play
	async fn run_current_play<E, F: AsyncFnMut(PlayStreamEvent, &LiveFeedMetadata, &LiveFeedData) -> Result<ControlFlow<()>, E>>(&self, mut f: F, current_play: &Play, meta: &LiveFeedMetadata, data: &LiveFeedData) -> Result<ControlFlow<()>, E> {
		macro_rules! flow_try {
			($($t:tt)*) => {
				match ($($t)*).await? {
					ControlFlow::Continue(()) => {},
					ControlFlow::Break(()) => return Ok(ControlFlow::Break(())),
				}
			};
		}
		
		if !self.in_progress_current_play {
			flow_try!(f(PlayStreamEvent::StartPlay(current_play), meta, data));
		}
		let mut play_events = current_play.play_events.iter().skip(self.current_play_event_idx);
		if let Some(current_play_event) = play_events.next() {
			flow_try!(f(PlayStreamEvent::PlayEvent(current_play_event, current_play), meta, data));
			let mut reviews = current_play_event.reviews.iter().skip(self.current_play_event_review_idx);
			if let Some(current_review) = reviews.next() {
				if !self.in_progress_current_play_event_review {
					flow_try!(f(PlayStreamEvent::PlayEventReviewStart(current_review, current_play_event, current_play), meta, data));
				}
				if !current_review.is_in_progress {
					flow_try!(f(PlayStreamEvent::PlayEventReviewEnd(current_review, current_play_event, current_play), meta, data));
				}
			}
			for review in reviews {
				flow_try!(f(PlayStreamEvent::PlayEventReviewStart(review, current_play_event, current_play), meta, data));
				if !review.is_in_progress {
					flow_try!(f(PlayStreamEvent::PlayEventReviewEnd(review, current_play_event, current_play), meta, data));
				}
			}
		}
		for play_event in play_events {
			flow_try!(f(PlayStreamEvent::PlayEvent(play_event, current_play), meta, data));
			for review in &play_event.reviews {
				flow_try!(f(PlayStreamEvent::PlayEventReviewStart(review, play_event, current_play), meta, data));
				if !review.is_in_progress {
					flow_try!(f(PlayStreamEvent::PlayEventReviewEnd(review, play_event, current_play), meta, data));
				}
			}
		}
		let mut reviews = current_play.reviews.iter().skip(self.current_play_review_idx);
		if let Some(current_review) = reviews.next() {
			if !self.in_progress_current_play_review {
				flow_try!(f(PlayStreamEvent::PlayReviewStart(current_review, current_play), meta, data));
			}
			if !current_review.is_in_progress {
				flow_try!(f(PlayStreamEvent::PlayReviewEnd(current_review, current_play), meta, data));
			}
		}
		
		for review in reviews {
			flow_try!(f(PlayStreamEvent::PlayReviewStart(review, current_play), meta, data));
			if !review.is_in_progress {
				flow_try!(f(PlayStreamEvent::PlayReviewEnd(review, current_play), meta, data));
			}
		}
		if current_play.about.is_complete {
			flow_try!(f(PlayStreamEvent::EndPlay(current_play), meta, data));
		}
		
		Ok(ControlFlow::Continue(()))
	}

	/// Evaluation for remaining plays
	async fn run_next_plays<E, F: AsyncFnMut(PlayStreamEvent, &LiveFeedMetadata, &LiveFeedData) -> Result<ControlFlow<()>, E>>(&self, mut f: F, plays: impl Iterator<Item=&Play>, meta: &LiveFeedMetadata, data: &LiveFeedData) -> Result<ControlFlow<()>, E> {
		macro_rules! flow_try {
			($($t:tt)*) => {
				match ($($t)*).await? {
					ControlFlow::Continue(()) => {},
					ControlFlow::Break(()) => return Ok(ControlFlow::Break(())),
				}
			};
		}
		
		for play in plays {
			flow_try!(f(PlayStreamEvent::StartPlay(play), meta, data));
			for play_event in &play.play_events {
				flow_try!(f(PlayStreamEvent::PlayEvent(play_event, play), meta, data));
				for review in &play_event.reviews {
					flow_try!(f(PlayStreamEvent::PlayEventReviewStart(review, play_event, play), meta, data));
					if !review.is_in_progress {
						flow_try!(f(PlayStreamEvent::PlayEventReviewEnd(review, play_event, play), meta, data));
					}
				}
			}
			for review in &play.reviews {
				flow_try!(f(PlayStreamEvent::PlayReviewStart(review, play), meta, data));
				if !review.is_in_progress {
					flow_try!(f(PlayStreamEvent::PlayReviewEnd(review, play), meta, data));
				}
			}
			if play.about.is_complete {
				flow_try!(f(PlayStreamEvent::EndPlay(play), meta, data));
			}
		}

		Ok(ControlFlow::Continue(()))
	}

	fn update_indices(&mut self, plays: &[Play]) {
		let latest_play = plays.last();

		self.in_progress_current_play = latest_play.is_some_and(|play| !play.about.is_complete);
		self.current_play_idx = if self.in_progress_current_play { plays.len() - 1 } else { plays.len() };

		let current_play = plays.get(self.current_play_idx);
		let current_play_event = current_play.and_then(|play| play.play_events.last());
		let current_play_review = current_play.and_then(|play| play.reviews.last());
		let current_play_event_review = current_play_event.and_then(|play_event| play_event.reviews.last());
		
		self.in_progress_current_play_review = current_play_review.is_some_and(|review| review.is_in_progress);
		self.current_play_review_idx = current_play.map_or(0, |play| if self.in_progress_current_play_review { play.reviews.len() - 1 } else { play.reviews.len() });

		self.current_play_event_idx = current_play.map_or(0, |play| play.play_events.len());

		self.in_progress_current_play_event_review = current_play_event_review.is_some_and(|review| review.is_in_progress);
		self.current_play_event_review_idx = current_play_event.map_or(0, |play_event| if self.in_progress_current_play_event_review { play_event.reviews.len() - 1 } else { play_event.reviews.len() });
	}

	/// Variant of the ``run`` function that allows for custom error types.
	///
	/// # Errors
	/// See [`request::Error`]
	pub async fn run_with_custom_error<E: From<request::Error>, F: AsyncFnMut(PlayStreamEvent, &LiveFeedMetadata, &LiveFeedData) -> Result<ControlFlow<()>, E>>(mut self, mut f: F) -> Result<(), E> {
		macro_rules! flow_try {
			($($t:tt)*) => {
				match ($($t)*).await? {
					ControlFlow::Continue(()) => {},
					ControlFlow::Break(()) => return Ok(()),
				}
			};
		}
		
		let mut feed = LiveFeedRequest::builder().id(self.game_id).build_and_get().await?;
		flow_try!(f(PlayStreamEvent::GameStart, &feed.meta, &feed.data));
		
		loop {
		    let since_last_request = Instant::now();
		    
			let LiveFeedResponse { meta, data, live, .. } = &feed;
			let LiveFeedLiveData { linescore, boxscore, decisions, leaders, plays } = live;
			let mut plays = plays.iter().skip(self.current_play_idx);

			if let Some(current_play) = plays.next() {
				flow_try!(self.run_current_play(&mut f, current_play, meta, data));
			}
			
			flow_try!(self.run_next_plays(&mut f, plays, meta, data));
			
			if data.status.abstract_game_code.is_finished() && let Some(decisions) = decisions {
				let _ = f(PlayStreamEvent::GameEnd(decisions, linescore, boxscore, leaders), meta, data).await?;
				return Ok(())
			}

			self.update_indices(&live.plays);

			let total_sleep_time = Duration::from_secs(meta.recommended_poll_rate as _);
			drop(feed);
			tokio::time::sleep(total_sleep_time.saturating_sub(since_last_request.elapsed())).await;
		    feed = LiveFeedRequest::builder().id(self.game_id).build_and_get().await?;
		}
	}
}
	
#[cfg(test)]
mod tests {
    use std::ops::ControlFlow;
    use crate::{cache::RequestableEntrypoint, game::{PlayEvent, PlayStream, PlayStreamEvent}};

	#[tokio::test]
	async fn test_play_stream() {
		PlayStream::new(822_834).run(async |event, _meta, _data| {
			match event {
				PlayStreamEvent::GameStart => println!("GameStart"),
				PlayStreamEvent::StartPlay(play) => println!("PlayStart; {} vs. {}", play.matchup.batter.full_name, play.matchup.pitcher.full_name),
				PlayStreamEvent::PlayEvent(play_event, _play) => {
					print!("PlayEvent; ");
					match play_event {
						PlayEvent::Action { details, .. } => println!("{}", details.description),
						PlayEvent::Pitch { details, common, .. } => println!("{} -> {}", details.call, common.count),
						PlayEvent::Stepoff { .. } => println!("Stepoff"),
						PlayEvent::NoPitch { .. } => println!("No Pitch"),
						PlayEvent::Pickoff { .. } => println!("Pickoff"),
					}
				},
				PlayStreamEvent::PlayEventReviewStart(review, _, _) => println!("PlayEventReviewStart; {}", review.review_type),
				PlayStreamEvent::PlayEventReviewEnd(review, _, _) => println!("PlayEventReviewEnd; {}", review.review_type),
				PlayStreamEvent::PlayReviewStart(review, _) => println!("PlayReviewStart; {}", review.review_type),
				PlayStreamEvent::PlayReviewEnd(review, _) => println!("PlayReviewEnd; {}", review.review_type),
				PlayStreamEvent::EndPlay(play) => println!("PlayEnd; {}", play.result.completed_play_details.as_ref().expect("Completed play").description),
				PlayStreamEvent::GameEnd(_, _, _, _) => println!("GameEnd"),
			}
			Ok(ControlFlow::Continue(()))
		}).await.unwrap();
	}
}
