//! The thing you're most likely here for.

use std::fmt::{Display, Formatter};
use bon::Builder;
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use derive_more::{Deref, DerefMut};
use fxhash::FxHashMap;
use serde::{Deserialize, Deserializer};
use serde::de::{Error, MapAccess};
use serde_with::{serde_as, DisplayFromStr};
use crate::meta::{GameStatus, GameType};
use crate::meta::LogicalEventId;
use crate::person::{Ballplayer, NamedPerson, PersonId};
use crate::request::RequestURL;
use crate::season::SeasonId;
use crate::meta::DayNight;
use crate::team::Team;
use crate::{Copyright, HomeAwaySplit, DayHalf};
use crate::venue::{Venue, VenueId};
use crate::meta::WindDirectionId;

pub mod boxscore;
pub mod changes;
pub mod color;
pub mod content;
pub mod context_metrics;
pub mod diff;
pub mod linescore;
pub mod pace;
pub mod pbp;
pub mod timestamps;
pub mod uniforms;
pub mod win_probability;

id!(GameId { gamePk: u32 });

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GameResponse {
	pub copyright: Copyright,
	#[serde(rename = "gamePk")]
	pub id: GameId,
	#[serde(rename = "metaData")]
	pub meta: GameMetadata,
	#[serde(rename = "gameData")]
	pub data: GameData,
	#[serde(rename = "liveData")]
	pub live: GameLiveData,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GameMetadata {
	pub wait: u32,
	// pub timestamp: String, // todo
	pub game_events: Vec<String>, // todo: what is this type
	pub logical_events: Vec<LogicalEventId>,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone, Deref, DerefMut)]
#[serde(rename_all = "camelCase")]
pub struct GameData {
	#[deref]
	#[deref_mut]
	#[serde(rename = "game")]
	game: GameDataMeta,
	pub datetime: GameDateTime,
	pub status: GameStatus,
	pub teams: HomeAwaySplit<Team>,
	#[serde(deserialize_with = "deserialize_players_cache")]
	pub players: FxHashMap<PersonId, Ballplayer<()>>,
	pub venue: Venue,
	pub official_venue: VenueId,
	pub weather: GameWeather,
	#[serde(rename = "gameInfo")]
	pub info: GameInfo,
	pub review: GameReview,
	#[serde(rename = "flags")]
	pub live_tags: GameLiveTags,
	// pub alerts: Vec<()>, // todo: type?
	pub probable_pitchers: HomeAwaySplit<NamedPerson>,
	pub official_scorer: NamedPerson,
	pub primary_datacaster: NamedPerson,
	pub mound_visits: HomeAwaySplit<ResourceUsage>,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GameDataMeta {
	#[serde(rename = "pk")]
	pub id: GameId,
	#[serde(rename = "type")]
	pub game_type: GameType,
	pub double_header: DoubleHeaderKind,
	/// Will state `P` for [`GameType::Playoffs`] games rather than what playoff series it is.
	pub gameday_type: GameType,
	#[serde(deserialize_with = "crate::from_yes_no")]
	pub tiebreaker: bool,
	/// No clue what this means
	pub game_number: u32,
	pub season: SeasonId,
	#[serde(rename = "seasonDisplay")]
	pub displayed_season: SeasonId,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
#[doc(hidden)]
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

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
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

#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(try_from = "__GameWeatherStruct")]
pub struct GameWeather {
	pub condition: String,
	pub temp: uom::si::f64::ThermodynamicTemperature,
	pub wind_speed: uom::si::f64::Velocity,
	pub wind_direction: WindDirectionId,
}

impl Eq for GameWeather {}

#[serde_as]
#[derive(Deserialize)]
#[doc(hidden)]
struct __GameWeatherStruct {
	condition: String,
	#[serde_as(as = "DisplayFromStr")]
	temp: i32,
	wind: String,
}

impl TryFrom<__GameWeatherStruct> for GameWeather {
	type Error = &'static str;

	fn try_from(value: __GameWeatherStruct) -> Result<Self, Self::Error> {
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

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GameInfo {
	pub attendance: u32,
	#[serde(deserialize_with = "crate::deserialize_datetime")]
	pub first_pitch: NaiveDateTime,
	/// Measured in minutes,
	#[serde(rename = "gameDurationMinutes")]
	pub game_duration: u32,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GameReview {
	pub has_challenges: bool,
	#[serde(flatten)]
	pub teams: HomeAwaySplit<ResourceUsage>,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ResourceUsage {
	used: u32,
	remaining: u32,
}

#[allow(clippy::struct_excessive_bools, reason = "")]
#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GameLiveTags {
	no_hitter: bool,
	perfect_game: bool,

	away_team_no_hitter: bool,
	away_team_perfect_game: bool,

	home_team_no_hitter: bool,
	home_team_perfect_game: bool,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
pub struct GameLiveData {}

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

fn deserialize_players_cache<'de, D: Deserializer<'de>>(deserializer: D) -> Result<FxHashMap<PersonId, Ballplayer<()>>, D::Error> {
	struct PlayersCacheVisitor;

	impl<'de2> serde::de::Visitor<'de2> for PlayersCacheVisitor {
		type Value = FxHashMap<PersonId, Ballplayer<()>>;

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

	deserializer.deserialize_map(PlayersCacheVisitor)
}

#[derive(Builder)]
#[builder(derive(Into))]
pub struct GameRequest {
	#[builder(into)]
	id: GameId,
}

impl<S: game_request_builder::State + game_request_builder::IsComplete> crate::request::RequestURLBuilderExt for GameRequestBuilder<S> {
	type Built = GameRequest;
}

impl Display for GameRequest {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "http://statsapi.mlb.com/api/v1.1/game/{}/feed/live", self.id)
	}
}

impl RequestURL for GameRequest {
	type Response = GameResponse;
}

#[cfg(test)]
mod tests {
	use crate::game::GameRequest;
	use crate::request::RequestURLBuilderExt;

	#[tokio::test]
	async fn ws_gm7_2025() {
		dbg!(GameRequest::builder().id(813024).build().to_string());
		let response = GameRequest::builder().id(813024).build_and_get().await.unwrap();
		dbg!(response);
	}
}
