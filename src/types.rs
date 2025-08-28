use chrono::{Datelike, Local, NaiveDate};
use derive_more::Display;
use serde::de::Error;
use serde::{Deserialize, Deserializer};
use std::fmt::{Debug, Display, Formatter};
use std::num::ParseIntError;
use std::ops::Add;
use std::str::FromStr;
use compact_str::CompactString;
use thiserror::Error;

/// Shared types across multiple endpoints
#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(from = "__CopyrightStruct")]
pub enum Copyright {
	Typical {
		year: u32,
	},
	UnknownSpec(CompactString),
}

#[derive(Deserialize)]
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
	fn default() -> Self {
		Self::Typical { year: Local::now().year() as _ }
	}
}

pub fn try_from_str<'de, D: Deserializer<'de>, T: FromStr>(deserializer: D) -> Result<Option<T>, D::Error> {
	Ok(String::deserialize(deserializer)?.parse::<T>().ok())
}

pub fn from_str<'de, D: Deserializer<'de>, T: FromStr>(deserializer: D) -> Result<T, D::Error>
where
	<T as FromStr>::Err: Debug,
{
	String::deserialize(deserializer)?.parse::<T>().map_err(|e| Error::custom(format!("{e:?}")))
}

pub fn from_yes_no<'de, D: Deserializer<'de>>(deserializer: D) -> Result<bool, D::Error> {
	#[derive(Deserialize)]
	enum Boolean {
		#[serde(rename = "Y")]
		Yes,
		#[serde(rename = "N")]
		No,
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
		match s.split_once("' ").map(|(feet, rest)| (feet, rest.split_once(r#"""#))) {
			Some((feet, Some((inches, "")))) => {
				let feet = feet.parse::<u8>()?;
				let inches = inches.parse::<u8>()?;
				Ok(Self::FeetAndInches { feet, inches })
			}
			_ => match s.split_once("cm") {
				Some((cm, "")) => {
					let cm = cm.parse::<u16>()?;
					Ok(Self::Centimeters { cm })
				}
				_ => Err(HeightMeasurementParseError::UnknownSpec(s.to_owned())),
			},
		}
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

pub type NaiveDateRange = std::ops::RangeInclusive<NaiveDate>;

pub(crate) const MLB_API_DATE_FORMAT: &str = "%m/%d/%Y";

pub fn deserialize_comma_seperated_vec<'de, D: Deserializer<'de>, T: FromStr>(deserializer: D) -> Result<Vec<T>, D::Error>
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

#[derive(Debug, Copy, Clone)]
pub enum IntegerOrFloat {
	Integer(i64),
	Float(f64),
}

impl PartialEq for IntegerOrFloat {
	fn eq(&self, other: &Self) -> bool {
		match (*self, *other) {
			(Self::Integer(lhs), Self::Integer(rhs)) => lhs == rhs,
			(Self::Float(lhs), Self::Float(rhs)) => lhs == rhs,

			(Self::Integer(int), Self::Float(float)) | (Self::Float(float), Self::Integer(int)) => {
				// fast way to check if the float is representable perfectly as an integer and if it's within range of `i64`
				if float.floor() == float && (i64::MIN as f64..=i64::MAX as f64).contains(&float) {
					float as i64 == int
				} else {
					false
				}
			},
		}
	}
}

impl Eq for IntegerOrFloat {}

impl<'de> Deserialize<'de> for IntegerOrFloat {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>
	{
		struct Visitor;
		
		impl<'de> serde::de::Visitor<'de> for Visitor {
			type Value = IntegerOrFloat;

			fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
				formatter.write_str("integer or float, or string that can be parsed to either")
			}

			fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
			where
				E: Error,
			{
				if let Ok(i) = v.parse::<i64>() {
					Ok(IntegerOrFloat::Integer(i))
				} else if let Ok(f) = v.parse::<f64>() {
					Ok(IntegerOrFloat::Float(f))
				} else {
					Err(E::invalid_value(serde::de::Unexpected::Str(v), &self))
				}
			}

			fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
			where
				E: Error,
			{
				Ok(IntegerOrFloat::Integer(v))
			}
			
			fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
			where
				E: Error,
			{
				Ok(IntegerOrFloat::Float(v))
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
