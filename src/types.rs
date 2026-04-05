//! Shared types across multiple requests

#![allow(clippy::redundant_pub_crate, reason = "re-exported as pub lol")]

use chrono::{Datelike, Local, NaiveDate, NaiveDateTime, NaiveTime, DateTime, Utc, TimeDelta, Timelike};
use derive_more::{Display, FromStr};
use serde::de::Error;
use serde::{Deserialize, Deserializer};
use std::fmt::{Debug, Display, Formatter};
use std::num::{ParseFloatError, ParseIntError};
use std::ops::{Add, RangeInclusive};
use std::str::FromStr;
use std::ops::Not;
use thiserror::Error;
use crate::season::SeasonId;

/// The copyright at the top of every request
#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(from = "__CopyrightStruct")]
pub enum Copyright {
	/// Typical copyright format
	Typical {
		/// Year of the copyright, typically the current year.
		year: u32,
	},
	/// Unknown copyright format
	UnknownSpec(Box<str>),
}

#[derive(Deserialize)]
#[doc(hidden)]
struct __CopyrightStruct(String);

impl From<__CopyrightStruct> for Copyright {
	fn from(value: __CopyrightStruct) -> Self {
		let __CopyrightStruct(value) = value;
		if let Some(value) = value.strip_prefix("Copyright ") && let Some(value) = value.strip_suffix(" MLB Advanced Media, L.P.  Use of any content on this page acknowledges agreement to the terms posted here http://gdx.mlb.com/components/copyright.txt") && let Ok(year) = value.parse::<u32>() {
			Self::Typical { year }
		} else {
			Self::UnknownSpec(value.into_boxed_str())
		}
	}
}

impl Display for Copyright {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Typical { year } => write!(f, "Copyright {year} MLB Advanced Media, L.P.  Use of any content on this page acknowledges agreement to the terms posted here http://gdx.mlb.com/components/copyright.txt"),
			Self::UnknownSpec(copyright) => write!(f, "{copyright}"),
		}
	}
}

impl Default for Copyright {
	#[allow(clippy::cast_sign_loss, reason = "jesus is not alive")]
	fn default() -> Self {
		Self::Typical { year: Local::now().year() as _ }
	}
}

/// Try to deserialize a type using its [`FromStr`] implementation, fallback to `None` if it doesn't work.
/// # Errors
/// If a string cannot be parsed from the deserializer.
pub fn try_from_str<'de, D: Deserializer<'de>, T: FromStr>(deserializer: D) -> Result<Option<T>, D::Error> {
	Ok(String::deserialize(deserializer)?.parse::<T>().ok())
}

/// Deserializes a type using its [`FromStr`] implementation.
///
/// # Errors
/// 1. If a string cannot be parsed from the deserializer.
/// 2. If the type cannot be parsed from the string.
pub fn from_str<'de, D: Deserializer<'de>, T: FromStr>(deserializer: D) -> Result<T, D::Error>
where
	<T as FromStr>::Err: Debug,
{
	String::deserialize(deserializer)?.parse::<T>().map_err(|e| Error::custom(format!("{e:?}")))
}

/// Deserializes a `"Y"` or `"N"` into a `bool`
///
/// # Errors
/// If the type cannot be parsed into a Y or N string
pub fn from_yes_no<'de, D: Deserializer<'de>>(deserializer: D) -> Result<bool, D::Error> {
	#[derive(Deserialize)]
	#[repr(u8)]
	enum Boolean {
		#[serde(rename = "Y")]
		Yes = 1,
		#[serde(rename = "N")]
		No = 0,
	}

	Ok(match Boolean::deserialize(deserializer)? {
		Boolean::Yes => true,
		Boolean::No => false,
	})
}

/// Measurement of a person's height
///
/// Not using [`uom`] because we want feet and inches, not just one of the measurements.
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum HeightMeasurement {
	/// `{a: u8}' {b: u8}"`
	FeetAndInches { feet: u8, inches: u8 },
	/// '{x: u16} cm'
	Centimeters { cm: u16 },
}

impl FromStr for HeightMeasurement {
	type Err = HeightMeasurementParseError;

	/// Spec
	/// 1. `{x: u16} cm`
	/// 2. `{a: u8}' {b: u8}"`
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		if let Some((feet, Some((inches, "")))) = s.split_once("' ").map(|(feet, rest)| (feet, rest.split_once('"'))) {
			let feet = feet.parse::<u8>()?;
			let inches = inches.parse::<u8>()?;
			Ok(Self::FeetAndInches { feet, inches })
		} else if let Some((cm, "")) = s.split_once("cm") {
			let cm = cm.parse::<u16>()?;
			Ok(Self::Centimeters { cm })
		} else {
			Err(HeightMeasurementParseError::UnknownSpec(s.to_owned()))
		}
	}
}

impl<'de> Deserialize<'de> for HeightMeasurement {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>
	{
		String::deserialize(deserializer)?.parse().map_err(D::Error::custom)
	}
}

/// Error for [`<HeightMeasurement as FromStr>::from_str`]
#[derive(Debug, Error)]
pub enum HeightMeasurementParseError {
	/// Failed to parse an integer in the height measurement
	#[error(transparent)]
	ParseIntError(#[from] ParseIntError),
	/// Was neither `{a}' {b}"` or `{x} cm`
	#[error("Unknown height '{0}'")]
	UnknownSpec(String),
}

/// General filter for players in requests
#[derive(Debug, Display, PartialEq, Eq, Copy, Clone, Default)]
pub enum PlayerPool {
	/// All players (no filter)
	#[default]
	#[display("ALL")]
	All,
	/// Qualified PAs or IP for a season, can be checked manually via [`QualificationMultipliers`](crate::season::QualificationMultipliers)
	#[display("QUALIFIED")]
	Qualified,
	/// Rookie season
	#[display("ROOKIES")]
	Rookies,
	/// Qualified && Rookie
	#[display("QUALIFIED_ROOKIES")]
	QualifiedAndRookies,
	/// ?
	#[display("ORGANIZATION")]
	Organization,
	/// ?
	#[display("ORGANIZATION_NO_MLB")]
	OrganizationNotMlb,
	/// Active Player (?)
	#[display("CURRENT")]
	Current,
	/// ?
	#[display("ALL_CURRENT")]
	AllCurrent,
	/// Qualified && Current
	#[display("QUALIFIED_CURRENT")]
	QualifiedAndCurrent,
}

/// Gender
///
/// Used on [`Ballplayer`](crate::person::Ballplayer)
#[derive(Debug, Deserialize, PartialEq, Eq, Copy, Clone, Default)]
pub enum Gender {
	#[serde(rename = "M")]
	Male,
	#[serde(rename = "F")]
	Female,
	#[default]
	#[serde(other)]
	Other,
}

/// Handedness
///
/// Either for batting or pitching
#[derive(Debug, Deserialize, PartialEq, Eq, Copy, Clone, Display)]
#[serde(try_from = "__HandednessStruct")]
pub enum Handedness {
	#[display("L")]
	Left,
	#[display("R")]
	Right,
	#[display("S")]
	Switch,
}

#[derive(Deserialize)]
#[doc(hidden)]
struct __HandednessStruct {
	code: String,
}

/// Error for handedness parsing
#[derive(Debug, Error)]
pub enum HandednessParseError {
	/// Did not match any of the known handedness variants
	#[error("Invalid handedness '{0}'")]
	InvalidHandedness(String),
}

impl TryFrom<__HandednessStruct> for Handedness {
	type Error = HandednessParseError;

	fn try_from(value: __HandednessStruct) -> Result<Self, Self::Error> {
		Ok(match &*value.code {
			"L" => Self::Left,
			"R" => Self::Right,
			"S" => Self::Switch,
			_ => return Err(HandednessParseError::InvalidHandedness(value.code)),
		})
	}
}

/// Represents a range from one date to another (inclusive on both ends)
///
/// # Examples
/// ```
/// let range: NaiveDateRange = NaiveDate::from_ymd(1, 1, 2025)..=NaiveDate::from_ymd(12, 31, 2025);
/// ```
pub type NaiveDateRange = RangeInclusive<NaiveDate>;

pub(crate) const MLB_API_DATE_FORMAT: &str = "%m/%d/%Y";

/// # Errors
/// 1. If a string cannot be deserialized
/// 2. If the data does not appear in the format `%Y-%m-%dT%H:%M:%SZ(%#z)?`. Why the MLB removes the +00:00 or -00:00 sometimes? I have no clue.
pub(crate) fn deserialize_datetime<'de, D: Deserializer<'de>>(deserializer: D) -> Result<DateTime<Utc>, D::Error> {
	let string = String::deserialize(deserializer)?;
	let fmt = match (string.ends_with('Z'), string.contains('.')) {
		(false, false) => "%FT%TZ%#z",
		(false, true) => "%FT%TZ%.3f%#z",
		(true, false) => "%FT%TZ",
		(true, true) => "%FT%T%.3fZ",
	};
	NaiveDateTime::parse_from_str(&string, fmt).map(|x| x.and_utc()).map_err(D::Error::custom)
}

/// # Errors
/// 1. If a string cannot be deserialized
/// 2. If the data does not appear in the format of `/(?:<t parser here>,)*<t parser here>?/g`
pub(crate) fn deserialize_comma_separated_vec<'de, D: Deserializer<'de>, T: FromStr>(deserializer: D) -> Result<Vec<T>, D::Error>
where
	<T as FromStr>::Err: Debug,
{
	String::deserialize(deserializer)?
		.split(", ")
		.map(|entry| T::from_str(entry))
		.collect::<Result<Vec<T>, <T as FromStr>::Err>>()
		.map_err(|e| Error::custom(format!("{e:?}")))
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Default)]
pub enum TeamSide {
	#[default]
	Home,
	Away,
}

impl Not for TeamSide {
	type Output = Self;

	fn not(self) -> Self::Output {
		match self {
			Self::Home => Self::Away,
			Self::Away => Self::Home,
		}
	}
}

impl TeamSide {
	#[must_use]
	pub const fn is_home(self) -> bool {
		matches!(self, Self::Home)
	}

	#[must_use]
	pub const fn is_away(self) -> bool {
		matches!(self, Self::Away)
	}
}

pub fn deserialize_team_side_from_is_home<'de, D: Deserializer<'de>>(deserializer: D) -> Result<TeamSide, D::Error> {
	Ok(if bool::deserialize(deserializer)? { TeamSide::Home } else { TeamSide::Away })
}

/// General type that represents two fields where one is home and one is away
///
/// Examples:
/// ```json
/// {
///     "home": { "name": "New York Yankees", "id": ... },
///     "away": { "name": "Boston Red Sox", "id": ... }
/// }
/// ```
#[derive(Debug, Deserialize, PartialEq, Eq, Copy, Clone, Default)]
pub struct HomeAway<T> {
	pub home: T,
	pub away: T,
}

impl<T> HomeAway<T> {
	/// Constructs a new [`HomeAwaySplit`]
	#[must_use]
	pub const fn new(home: T, away: T) -> Self {
		Self { home, away }
	}

	/// Choose the value depending on the [`TeamSide`]
	#[must_use]
	pub fn choose(self, side: TeamSide) -> T {
		match side {
			TeamSide::Home => self.home,
			TeamSide::Away => self.away,
		}
	}

	#[must_use]
	pub const fn as_ref(&self) -> HomeAway<&T> {
		HomeAway {
			home: &self.home,
			away: &self.away,
		}
	}

	#[must_use]
	pub const fn as_mut(&mut self) -> HomeAway<&mut T> {
		HomeAway {
			home: &mut self.home,
			away: &mut self.away,
		}
	}

	#[must_use]
	pub fn map<U, F: FnMut(T) -> U>(self, mut f: F) -> HomeAway<U> {
		HomeAway {
			home: f(self.home),
			away: f(self.away),
		}
	}

	/// Switches the home and away sides
	#[must_use]
	pub fn swap(self) -> Self {
		Self {
			home: self.away,
			away: self.home,
		}
	}

	#[must_use]
	pub fn zip<U>(self, other: HomeAway<U>) -> HomeAway<(T, U)> {
		HomeAway {
			home: (self.home, other.home),
			away: (self.away, other.away),
		}
	}
	
	#[must_use]
	pub fn zip_with<U, V, F: FnMut(T, U) -> V>(self, other: HomeAway<U>, mut f: F) -> HomeAway<V> {
		HomeAway {
			home: f(self.home, other.home),
			away: f(self.away, other.away),
		}
	}

	#[must_use]
	pub fn combine<U, F: FnOnce(T, T) -> U>(self, f: F) -> U {
		f(self.home, self.away)
	}

	/// Adds home and away values (typically used in stats)
	#[must_use]
	pub fn added(self) -> <T as Add>::Output where T: Add {
		self.home + self.away
	}

	#[must_use]
	pub fn both(self, mut f: impl FnMut(T) -> bool) -> bool {
		f(self.home) && f(self.away)
	}

	#[must_use]
	pub fn either(self, mut f: impl FnMut(T) -> bool) -> bool {
		f(self.home) || f(self.away)
	}
}

impl<T> HomeAway<Option<T>> {
	#[must_use]
	pub fn flatten(self) -> Option<HomeAway<T>> {
		Some(HomeAway {
			home: self.home?,
			away: self.away?,
		})
	}
}

impl<T> From<(T, T)> for HomeAway<T> {
	fn from((home, away): (T, T)) -> Self {
		Self {
			home,
			away
		}
	}
}

/// Street address, city, etc.
///
/// Pretty much nothing *has* to be supplied so you either get an address, phone number, everything, or just a country.
#[derive(Debug, Deserialize, PartialEq, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Location {
	pub address_line_1: Option<String>,
	pub address_line_2: Option<String>,
	pub address_line_3: Option<String>,
	pub address_line_4: Option<String>,
	pub attention: Option<String>,
	#[serde(alias = "phone")]
	pub phone_number: Option<String>,
	pub city: Option<String>,
	#[serde(alias = "province")]
	pub state: Option<String>,
	pub country: Option<String>,
	#[serde(rename = "stateAbbrev")] pub state_abbreviation: Option<String>,
	pub postal_code: Option<String>,
	pub latitude: Option<f64>,
	pub longitude: Option<f64>,
	pub azimuth_angle: Option<f64>,
	pub elevation: Option<u32>,
}

/// Various information regarding a field.
#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FieldInfo {
	pub capacity: u32,
	pub turf_type: TurfType,
	pub roof_type: RoofType,
	pub left_line: Option<u32>,
	pub left: Option<u32>,
	pub left_center: Option<u32>,
	pub center: Option<u32>,
	pub right_center: Option<u32>,
	pub right: Option<u32>,
	pub right_line: Option<u32>,
}

/// Different types of turf.
#[derive(Debug, Deserialize, PartialEq, Eq, Clone, Display)]
pub enum TurfType {
	#[serde(rename = "Artificial Turf")]
	#[display("Artificial Turf")]
	ArtificialTurf,

	#[serde(rename = "Grass")]
	#[display("Grass")]
	Grass,
}

/// Different types of roof setups.
#[derive(Debug, Deserialize, PartialEq, Eq, Clone, Display)]
pub enum RoofType {
	#[serde(rename = "Retractable")]
	#[display("Retractable")]
	Retractable,

	#[serde(rename = "Open")]
	#[display("Open")]
	Open,

	#[serde(rename = "Dome")]
	#[display("Dome")]
	Dome,
}

/// Data regarding a timezone, uses [`chrono_tz`].
#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TimeZoneData {
	#[serde(rename = "id")]
	pub timezone: chrono_tz::Tz,
	pub offset: i32,
	pub offset_at_game_time: i32,
}

/// More generalized than social media, includes retrosheet, fangraphs, (+ some socials), etc.
#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
pub struct ExternalReference {
	#[serde(rename = "xrefId")]
	pub id: String,
	#[serde(rename = "xrefType")]
	pub xref_type: String,
	pub season: Option<SeasonId>,
}

/// Tracking equipment, Hawk-Eye, `PitchFx`, etc.
#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TrackingSystem {
	pub id: TrackingSystemVendorId,
	pub description: String,
	pub pitch_vendor: Option<TrackingSystemVendor>,
	pub hit_vendor: Option<TrackingSystemVendor>,
	pub player_vendor: Option<TrackingSystemVendor>,
	pub skeletal_vendor: Option<TrackingSystemVendor>,
	pub bat_vendor: Option<TrackingSystemVendor>,
	pub biomechanics_vendor: Option<TrackingSystemVendor>,
}

id!(TrackingSystemVendorId { id: u32 });

/// A vendor for specific tracking concepts, such as Hawk-Eye for skeletal data.
#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
pub struct TrackingSystemVendor {
	pub id: TrackingSystemVendorId,
	pub description: String,
	#[serde(rename = "version")]
	pub details: String,
}

/// Stat that is either an integer or float.
///
/// Used in [`StatLeader`](crate::stats::leaders::StatLeader)
#[derive(Debug, Copy, Clone)]
pub enum IntegerOrFloatStat {
	/// [`integer`](i64) stat, likely counting
	Integer(i64),
	/// [`float`](f64) stat, likely rate
	Float(f64),
}

impl PartialEq for IntegerOrFloatStat {
	fn eq(&self, other: &Self) -> bool {
		match (*self, *other) {
			(Self::Integer(lhs), Self::Integer(rhs)) => lhs == rhs,
			(Self::Float(lhs), Self::Float(rhs)) => lhs == rhs,

			#[allow(clippy::cast_precision_loss, reason = "we checked if it's perfectly representable")]
			#[allow(clippy::cast_possible_truncation, reason = "we checked if it's perfectly representable")]
			(Self::Integer(int), Self::Float(float)) | (Self::Float(float), Self::Integer(int)) => {
				// fast way to check if the float is representable perfectly as an integer and if it's within range of `i64`
				// we inline the f64 casts of i64::MIN and i64::MAX, and change the upper bound to be < as i64::MAX is not perfectly representable.
				if float.is_normal() && float.floor() == float && (i64::MIN as f64..-(i64::MIN as f64)).contains(&float) {
					float as i64 == int
				} else {
					false
				}
			},
		}
	}
}

impl<'de> Deserialize<'de> for IntegerOrFloatStat {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>
	{
		struct Visitor;

		impl serde::de::Visitor<'_> for Visitor {
			type Value = IntegerOrFloatStat;

			fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
				formatter.write_str("integer, float, or string that can be parsed to either")
			}

			fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
			where
				E: Error,
			{
				Ok(IntegerOrFloatStat::Integer(v))
			}

			fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
			where
				E: Error,
			{
				Ok(IntegerOrFloatStat::Float(v))
			}

			fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
			where
				E: Error,
			{
				if v == "-.--" || v == ".---" {
					Ok(IntegerOrFloatStat::Float(0.0))
				} else if let Ok(i) = v.parse::<i64>() {
					Ok(IntegerOrFloatStat::Integer(i))
				} else if let Ok(f) = v.parse::<f64>() {
					Ok(IntegerOrFloatStat::Float(f))
				} else {
					Err(E::invalid_value(serde::de::Unexpected::Str(v), &self))
				}
			}
		}

		deserializer.deserialize_any(Visitor)
	}
}

/// Represents an error parsing an HTTP request
///
/// Not a reqwest error, this typically happens from a bad payload like asking for games at a date in BCE.
#[derive(Debug, Deserialize, Display)]
#[display("An error occurred parsing the statsapi http request: {message}")]
pub struct MLBError {
	pub(crate) message: String,
}

impl std::error::Error for MLBError {}

/// `rgba({red}, {green}, {blue})` into a type
#[derive(Debug, Deserialize, PartialEq, Eq, Copy, Clone, Default)]
#[serde(try_from = "&str")]
pub struct RGBAColor {
	pub red: u8,
	pub green: u8,
	pub blue: u8,
	pub alpha: u8,
}

impl Display for RGBAColor {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "0x{:02x}{:02x}{:02x}{:02x}", self.alpha, self.red, self.green, self.blue)
	}
}

/// Spec: `rgba({red}, {green}, {blue})`
#[derive(Debug, Error)]
pub enum RGBAColorFromStrError {
	#[error("Invalid spec")]
	InvalidFormat,
	#[error(transparent)]
	InvalidInt(#[from] ParseIntError),
	#[error(transparent)]
	InvalidFloat(#[from] ParseFloatError),
}

impl<'a> TryFrom<&'a str> for RGBAColor {
	type Error = <Self as FromStr>::Err;

	fn try_from(value: &'a str) -> Result<Self, Self::Error> {
		<Self as FromStr>::from_str(value)
	}
}

impl FromStr for RGBAColor {
	type Err = RGBAColorFromStrError;

	/// Spec: `rgba({red}, {green}, {blue})`
	#[allow(clippy::single_char_pattern, reason = "other patterns are strings, the choice to make that one a char does not denote any special case")]
	#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss, reason = "intended behaviour with alpha channel")]
	fn from_str(mut s: &str) -> Result<Self, Self::Err> {
		s = s.strip_suffix("rgba(").ok_or(Self::Err::InvalidFormat)?;
		let (red, s) = s.split_once(", ").ok_or(Self::Err::InvalidFormat)?;
		let red = red.parse::<u8>()?;
		let (green, s) = s.split_once(", ").ok_or(Self::Err::InvalidFormat)?;
		let green = green.parse::<u8>()?;
		let (blue, s) = s.split_once(", ").ok_or(Self::Err::InvalidFormat)?;
		let blue = blue.parse::<u8>()?;
		let (alpha, s) = s.split_once(")").ok_or(Self::Err::InvalidFormat)?;
		let alpha = (alpha.parse::<f32>()? * 255.0).round() as u8;
		if !s.is_empty() { return Err(Self::Err::InvalidFormat); }
		Ok(Self {
			red,
			green,
			blue,
			alpha
		})
	}
}

/// Used in [`HittingHotColdZones`](crate::stats::raw::HittingHotColdZones) and [`PitchingHotColdZones`](crate::stats::raw::PitchingHotColdZones).
#[derive(Debug, Deserialize, PartialEq, Eq, Copy, Clone, Display, FromStr)]
#[serde(try_from = "&str")]
pub enum HeatmapTemperature {
	Hot,
	Warm,
	Lukewarm,
	Cool,
	Cold,
}

impl<'a> TryFrom<&'a str> for HeatmapTemperature {
	type Error = <Self as FromStr>::Err;

	fn try_from(value: &'a str) -> Result<Self, Self::Error> {
		<Self as FromStr>::from_str(value)
	}
}

pub(crate) fn write_nth(n: usize, f: &mut Formatter<'_>) -> std::fmt::Result {
	write!(f, "{n}")?;
	let (tens, ones) = (n / 10, n % 10);
	let is_teen = (tens % 10) == 1;
	if is_teen {
		write!(f, "th")?;
	} else {
		write!(f, "{}", match ones {
			1 => "st",
			2 => "nd",
			3 => "rd",
			_ => "th",
		})?;
	}
	Ok(())
}

/// # Errors
/// 1. if format is not `"{hours}:{minutes}:{seconds}"`
pub fn deserialize_time_delta_from_hms<'de, D: Deserializer<'de>>(deserializer: D) -> Result<TimeDelta, D::Error> {
	let string = String::deserialize(deserializer)?;
	let (hour, rest) = string.split_once(':').ok_or_else(|| D::Error::custom("Unable to find `:`"))?;
	let (minute, second) = rest.split_once(':').ok_or_else(|| D::Error::custom("Unable to find `:`"))?;
	let hour = hour.parse::<u32>().map_err(D::Error::custom)?;
	let minute = minute.parse::<u32>().map_err(D::Error::custom)?;
	let second = second.parse::<u32>().map_err(D::Error::custom)?;

	TimeDelta::new(((hour * 24 + minute) * 60 + second) as _, 0).ok_or_else(|| D::Error::custom("Invalid time quantity, overflow."))
}

/// AM/PM
#[derive(Debug, Deserialize, PartialEq, Eq, Copy, Clone, Display, FromStr)]
#[serde(try_from = "&str")]
pub enum DayHalf {
	AM,
	PM,
}

impl DayHalf {
	/// Converts a 12-hour time into it's 24-hour version.
	#[must_use]
	pub fn into_24_hour_time(self, mut time: NaiveTime) -> NaiveTime {
		if (self == Self::PM) ^ (time.hour() == 12) {
			time += TimeDelta::hours(12);
		}

		time
	}
}

impl<'a> TryFrom<&'a str> for DayHalf {
	type Error = <Self as FromStr>::Err;

	fn try_from(value: &'a str) -> Result<Self, Self::Error> {
		<Self as FromStr>::from_str(value)
	}
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ResourceUsage {
	pub used: u32,
	pub remaining: u32,
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_ampm() {
		assert_eq!(NaiveTime::from_hms_opt(0, 0, 0).unwrap(), DayHalf::AM.into_24_hour_time(NaiveTime::from_hms_opt(12, 0, 0).unwrap()));
		assert_eq!(NaiveTime::from_hms_opt(12, 0, 0).unwrap(), DayHalf::PM.into_24_hour_time(NaiveTime::from_hms_opt(12, 0, 0).unwrap()));
		assert_eq!(NaiveTime::from_hms_opt(0, 1, 0).unwrap(), DayHalf::AM.into_24_hour_time(NaiveTime::from_hms_opt(12, 1, 0).unwrap()));
		assert_eq!(NaiveTime::from_hms_opt(12, 1, 0).unwrap(), DayHalf::PM.into_24_hour_time(NaiveTime::from_hms_opt(12, 1, 0).unwrap()));
		assert_eq!(NaiveTime::from_hms_opt(0, 1, 0).unwrap(), DayHalf::AM.into_24_hour_time(NaiveTime::from_hms_opt(12, 1, 0).unwrap()));
		assert_eq!(NaiveTime::from_hms_opt(23, 59, 0).unwrap(), DayHalf::PM.into_24_hour_time(NaiveTime::from_hms_opt(11, 59, 0).unwrap()));
		assert_eq!(NaiveTime::from_hms_opt(1, 1, 0).unwrap(), DayHalf::AM.into_24_hour_time(NaiveTime::from_hms_opt(1, 1, 0).unwrap()));
	}
}
