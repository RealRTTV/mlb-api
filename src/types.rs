use chrono::{Datelike, Local, NaiveDate, NaiveDateTime};
use compact_str::CompactString;
use derive_more::{Display, FromStr};
use serde::de::Error;
use serde::{Deserialize, Deserializer};
use std::fmt::{Debug, Display, Formatter};
use std::num::{ParseFloatError, ParseIntError};
use std::ops::{Add, RangeInclusive};
use std::str::FromStr;
use thiserror::Error;

/// Shared types across multiple requests
#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(from = "__CopyrightStruct")]
pub enum Copyright {
	Typical {
		year: u32,
	},
	UnknownSpec(CompactString),
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
			Self::UnknownSpec(CompactString::from(value))
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

/// # Errors
/// If a string cannot be parsed from the deserializer.
pub fn try_from_str<'de, D: Deserializer<'de>, T: FromStr>(deserializer: D) -> Result<Option<T>, D::Error> {
	Ok(String::deserialize(deserializer)?.parse::<T>().ok())
}

/// # Errors
/// 1. If a string cannot be parsed from the deserializer.
/// 2. If the type cannot be parsed from the string.
pub fn from_str<'de, D: Deserializer<'de>, T: FromStr>(deserializer: D) -> Result<T, D::Error>
where
	<T as FromStr>::Err: Debug,
{
	String::deserialize(deserializer)?.parse::<T>().map_err(|e| Error::custom(format!("{e:?}")))
}

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

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum HeightMeasurement {
	FeetAndInches { feet: u8, inches: u8 },
	Centimeters { cm: u16 },
}

impl FromStr for HeightMeasurement {
	type Err = HeightMeasurementParseError;

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

#[derive(Debug, Error)]
pub enum HeightMeasurementParseError {
	#[error(transparent)]
	ParseIntError(#[from] ParseIntError),
	#[error("Unknown height '{0}'")]
	UnknownSpec(String),
}

#[derive(Debug, Display, PartialEq, Eq, Copy, Clone, Default)]
pub enum PlayerPool {
	#[default]
	#[display("ALL")]
	All,
	#[display("QUALIFIED")]
	Qualified,
	#[display("ROOKIES")]
	Rookies,
	#[display("QUALIFIED_ROOKIES")]
	QualifiedAndRookies,
	#[display("ORGANIZATION")]
	Organization,
	#[display("ORGANIZATION_NO_MLB")]
	OrganizationNotMlb,
	#[display("CURRENT")]
	Current,
	#[display("ALL_CURRENT")]
	AllCurrent,
	#[display("QUALIFIED_CURRENT")]
	QualifiedAndCurrent,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Copy, Clone)]
pub enum Gender {
	#[serde(rename = "M")]
	Male,
	#[serde(rename = "F")]
	Female,
	#[serde(other)]
	Other,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Copy, Clone)]
#[serde(try_from = "__HandednessStruct")]
pub enum Handedness {
	Left,
	Right,
	Switch,
}

#[derive(Deserialize)]
#[doc(hidden)]
struct __HandednessStruct {
	code: String,
}

#[derive(Debug, Error)]
pub enum HandednessParseError {
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

pub type NaiveDateRange = RangeInclusive<NaiveDate>;

pub(crate) const MLB_API_DATE_FORMAT: &str = "%m/%d/%Y";

/// # Errors
/// 1. If a string cannot be deserialized
/// 2. If the data does not appear in the format `%Y-%m-%dT%H:%M:%SZ(%#z)?`. Why the MLB removes the +00:00 or -00:00 sometimes? I have no clue.
pub fn deserialize_datetime<'de, D: Deserializer<'de>>(deserializer: D) -> Result<NaiveDateTime, D::Error> {
	let string = String::deserialize(deserializer)?;
	if string.ends_with('Z') {
		NaiveDateTime::parse_from_str(&string, "%FT%TZ").map_err(D::Error::custom)
	} else {
		NaiveDateTime::parse_from_str(&string, "%FT%TZ%#z").map_err(D::Error::custom)
	}
}

/// # Errors
/// 1. If a string cannot be deserialized
/// 2. If the data does not appear in the format of `/(?:<t parser here>,)*<t parser here>?/g`
pub fn deserialize_comma_separated_vec<'de, D: Deserializer<'de>, T: FromStr>(deserializer: D) -> Result<Vec<T>, D::Error>
where
	<T as FromStr>::Err: Debug,
{
	String::deserialize(deserializer)?
		.split(", ")
		.map(|entry| T::from_str(entry))
		.collect::<Result<Vec<T>, <T as FromStr>::Err>>()
		.map_err(|e| Error::custom(format!("{e:?}")))
}

#[derive(Debug, Deserialize, PartialEq, Eq, Copy, Clone)]
pub struct HomeAwaySplits<T> {
	pub home: T,
	pub away: T,
}

impl<T> HomeAwaySplits<T> {
	#[must_use]
	pub const fn new(home: T, away: T) -> Self {
		Self { home, away }
	}
}

impl<T: Add> HomeAwaySplits<T> {
	#[must_use]
	pub fn combined(self) -> <T as Add>::Output {
		self.home + self.away
	}
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Location {
	pub address_line_1: Option<String>,
	pub address_line_2: Option<String>,
	pub address_line_3: Option<String>,
	pub address_line_4: Option<String>,
	pub attention: Option<String>,
	pub phone_number: Option<String>,
	pub city: Option<String>,
	pub state: Option<String>,
	pub country: Option<String>,
	#[serde(rename = "stateAbbrev")] pub state_abbreviation: Option<String>,
	pub postal_code: Option<String>,
	pub latitude: Option<f64>,
	pub longitude: Option<f64>,
	pub azimuth_angle: Option<f64>,
	pub elevation: Option<u32>,
}

impl Eq for Location {}

// todo: replace these with stat types like percentage, two decimal place, three decimal place, etc.
#[derive(Debug, Copy, Clone)]
pub enum IntegerOrFloatStat {
	Integer(i64),
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
				if float.is_normal() && float.floor() == float && (-9_223_372_036_854_775_808.0..9_223_372_036_854_775_808.0).contains(&float) {
					float as i64 == int
				} else {
					false
				}
			},
		}
	}
}

impl Eq for IntegerOrFloatStat {}

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

#[derive(Debug, Deserialize, Display)]
#[display("An error occurred parsing the statsapi http request: {message}")]
pub struct StatsAPIError {
	message: String,
}

impl std::error::Error for StatsAPIError {}

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

#[derive(Debug, Deserialize, PartialEq, Eq, Copy, Clone, Display, FromStr)]
#[serde(try_from = "&str")]
pub enum SimpleTemperature {
	Hot,
	Warm,
	Lukewarm,
	Cool,
	Cold,
}

impl<'a> TryFrom<&'a str> for SimpleTemperature {
	type Error = <Self as FromStr>::Err;

	fn try_from(value: &'a str) -> Result<Self, Self::Error> {
		<Self as FromStr>::from_str(value)
	}
}
