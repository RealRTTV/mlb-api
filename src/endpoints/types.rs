use std::fmt::Debug;
use std::num::ParseIntError;
use std::str::FromStr;
use chrono::{Datelike, Local};
use serde::{Deserialize, Deserializer};
use serde::de::Error;
use thiserror::Error;

/// Shared types across multiple endpoints
#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
pub struct Copyright(pub String);

impl Default for Copyright {
    fn default() -> Self {
        let year = Local::now().year();
        Self(format!("Copyright {year} MLB Advanced Media, L.P.  Use of any content on this page acknowledges agreement to the terms posted here http://gdx.mlb.com/components/copyright.txt"))
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Position {
    Unknown = 0,
    Pitcher = 1,
    Catcher = 2,
    FirstBaseman = 3,
    SecondBaseman = 4,
    ThirdBaseman = 5,
    Shortstop = 6,
    Leftfielder = 7,
    Centerfielder = 8,
    Rightfielder = 9,
    Outfielder = b'0' as _,
    DesignatedHitter = 10,
    TwoWayPlayer = b'Y' as _,
}

impl<'de> Deserialize<'de> for Position {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>
    {
        #[derive(Deserialize)]
        struct PositionStruct {
            code: String,
            abbreviation: String,
        }
        
        let PositionStruct { code, abbreviation } = PositionStruct::deserialize(deserializer)?;
        Ok(match &*abbreviation {
            "X" => Self::Unknown,
            "P" => Self::Pitcher,
            "C" => Self::Catcher,
            "1B" => Self::FirstBaseman,
            "2B" => Self::SecondBaseman,
            "3B" => Self::ThirdBaseman,
            "SS" => Self::Shortstop,
            "LF" => Self::Leftfielder,
            "CF" => Self::Centerfielder,
            "RF" => Self::Rightfielder,
            "OF" => Self::Outfielder,
            "DH" => Self::DesignatedHitter,
            "TWP" => Self::TwoWayPlayer,
            pos => return Err(Error::custom(format!("Invalid player position '{pos}' (code = {code})"))),
        })
    }
}

pub fn try_from_str<'de, D: Deserializer<'de>, T: FromStr>(deserializer: D) -> Result<Option<T>, D::Error> {
    Ok(String::deserialize(deserializer)?.parse::<T>().ok())
}

pub fn from_str<'de, D: Deserializer<'de>, T: FromStr>(deserializer: D) -> Result<T, D::Error> where <T as FromStr>::Err: Debug {
    String::deserialize(deserializer)?.parse::<T>().map_err(|e| Error::custom(format!("{e:?}")))
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum HeightMeasurement {
    FeetAndInches {
        feet: u8,
        inches: u8,
    },
    Centimeters {
        cm: u16,
    }
}

impl FromStr for HeightMeasurement {
    type Err = HeightMeasurementParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_once("' ").map(|(feet, rest)| (feet, rest.split_once(r#"""#))) {
            Some((feet, Some((inches, "")))) => {
                let feet = feet.parse::<u8>()?;
                let inches = inches.parse::<u8>()?;
                Ok(Self::FeetAndInches { feet, inches })
            },
            _ => {
                match s.split_once("cm") {
                    Some((cm, "")) => {
                        let cm = cm.parse::<u16>()?;
                        Ok(Self::Centimeters { cm })
                    },
                    _ => Err(HeightMeasurementParseError::UnknownSpec(s.to_owned())),
                }
            }
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

#[derive(Debug, Deserialize, PartialEq, Eq, Copy, Clone)]
pub enum Gender {
    #[serde(rename = "M")]
    Male,
    #[serde(rename = "F")]
    Female,
    #[serde(other)]
    Other,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Handedness {
    Left,
    Right,
}

impl<'de> Deserialize<'de> for Handedness {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>
    {
        #[derive(Deserialize)]
        struct HandednessStruct {
            code: String,
        }
        
        let HandednessStruct { code } = HandednessStruct::deserialize(deserializer)?;
        Ok(match &*code {
            "L" => Self::Left,
            "R" => Self::Right,
            _ => return Err(Error::custom("Invalid handedness")),
        })
    }
}
