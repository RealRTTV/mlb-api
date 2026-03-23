//! The thing you're most likely here for.
//!
//! This module itself acts like [`crate::types`] but for misc game-specific types as there are many.

#![allow(unused_imports, reason = "usage of children modules")]

use std::fmt::{Display, Formatter};
use std::marker::PhantomData;
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use derive_more::{Deref, DerefMut, Display, From, Not};
use fxhash::FxHashMap;
use serde::{Deserialize, Deserializer};
use serde::de::{DeserializeOwned, Error, IgnoredAny, MapAccess};
use serde_with::{serde_as, DisplayFromStr};
use crate::person::{Ballplayer, JerseyNumber, NamedPerson, PersonId};
use crate::meta::{DayNight, NamedPosition};
use crate::team::TeamId;
use crate::team::roster::RosterStatus;
use crate::{DayHalf, HomeAwaySplit, ResourceUsage};
use crate::meta::WindDirectionId;

mod boxscore;
mod changes;
mod color;
mod content;
mod context_metrics;
mod diff;
mod linescore;
mod pace;
mod plays;
mod timestamps;
mod uniforms;
mod win_probability;
mod live_feed;

pub use boxscore::*;
pub use changes::*;
pub use color::*;
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
#[cfg_attr(test, serde(deny_unknown_fields))]
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
#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(from = "__GameDateTimeStruct")]
pub struct GameDateTime {
	datetime: NaiveDateTime,
	original_date: NaiveDate,
	official_date: NaiveDate,
	sky: DayNight,
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
#[cfg_attr(test, serde(deny_unknown_fields))]
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
#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct GameInfo {
	pub attendance: u32,
	#[serde(deserialize_with = "crate::deserialize_datetime")]
	pub first_pitch: NaiveDateTime,
	/// Measured in minutes,
	#[serde(rename = "gameDurationMinutes")]
	pub game_duration: u32,
	/// Durationg of the game delay; measured in minutes.
	#[serde(rename = "delayDurationMinutes")]
	pub delay_duration: Option<u32>,
}

/// Review usage for each team and if the game supports challenges.
#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct ReviewData {
	pub has_challenges: bool,
	#[serde(flatten)]
	pub teams: HomeAwaySplit<ResourceUsage>,
}

/// Tags about a game, such as a perfect game in progress, no-hitter, etc.
#[allow(clippy::struct_excessive_bools, reason = "")]
#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct GameTags {
	no_hitter: bool,
	perfect_game: bool,

	away_team_no_hitter: bool,
	away_team_perfect_game: bool,

	home_team_no_hitter: bool,
	home_team_perfect_game: bool,
}

/// Double-header information.
#[derive(Debug, Deserialize, PartialEq, Copy, Clone)]
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

#[derive(Debug, Deserialize, Copy, Clone, PartialEq, Deref, DerefMut, From)]
pub struct Inning(usize);

impl Display for Inning {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		crate::write_nth(self.0, f)
	}
}

/// Half of the inning.
#[derive(Debug, Deserialize, Copy, Clone, PartialEq, Not)]
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
}

/// The balls and strikes in a given at bat. Along with the number of outs (this technically can change during the AB due to pickoffs etc)
#[derive(Debug, Deserialize, PartialEq, Copy, Clone)]
pub struct AtBatCount {
	pub balls: u8,
	pub strikes: u8,
	pub outs: u8,
}

/// The classic "R | H | E" and LOB in a scoreboard.
#[derive(Debug, Deserialize, PartialEq, Copy, Clone)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct RHE {
	/// Sometimes not present if the inning half isn't played. But weirdly hits and errors are present fields? Gonna make this default to 0
	#[serde(default)]
    pub runs: usize,
    pub hits: usize,
    pub errors: usize,
    pub left_on_base: usize,
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
#[derive(Debug, Deserialize, PartialEq, Clone)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct LabelledValue {
	pub label: String,
	#[serde(default)]
	pub value: String,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct SectionedLabelledValues {
	#[serde(rename = "title")]
	pub section: String,
	#[serde(rename = "fieldList")]
	pub values: Vec<LabelledValue>,
}

/// Various flags about the player in the current game
#[allow(clippy::struct_excessive_bools, reason = "not what's happening here")]
#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct PlayerGameStatusFlags {
	pub is_current_batter: bool,
	pub is_current_pitcher: bool,
	pub is_on_bench: bool,
	pub is_substitute: bool,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct Official {
	pub official: NamedPerson,
	pub official_type: OfficialType,
}

#[derive(Debug, Deserialize, PartialEq, Copy, Clone)]
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
#[derive(Debug, PartialEq, Copy, Clone)]
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
#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, serde(deny_unknown_fields))]
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
#[cfg_attr(test, serde(deny_unknown_fields))]
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

#[derive(Debug, Deserialize, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
pub enum Base {
	#[serde(rename = "1B")]
	First,
	#[serde(rename = "2B")]
	Second,
	#[serde(rename = "3B")]
	Third,
	#[serde(rename = "score")]
	Home,
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
