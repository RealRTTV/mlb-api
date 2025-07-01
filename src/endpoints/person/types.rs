use std::ops::{Deref, DerefMut};
use chrono::NaiveDate;
use derive_more::{Deref, Display, DerefMut};
use serde::Deserialize;
use crate::types::{Gender, Handedness, HeightMeasurement, Position};

#[derive(Debug, Deref, DerefMut, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Ballplayer {
    #[deref]
    #[deref_mut]
    #[serde(flatten)]
    inner: HydratedPerson,

    #[serde(deserialize_with = "crate::types::from_str")]
    pub primary_number: u8,
    pub current_age: u8,
    #[serde(flatten)]
    pub birth_data: BirthData,
    #[serde(flatten)]
    pub body_measurements: BodyMeasurements,
    pub gender: Gender,
    pub draft_year: u16,
    #[serde(rename = "mlbDebutDate")] pub mlb_debut: NaiveDate,
    pub bat_side: Handedness,
    pub pitch_hand: Handedness,
    #[serde(flatten)]
    pub strike_zone: StrikeZone,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BirthData {
    pub birth_date: NaiveDate,
    pub birth_city: String,
    #[serde(rename = "birthStateProvince")] pub birth_state_or_province: String,
    pub birth_country: String,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BodyMeasurements {
    #[serde(deserialize_with = "crate::types::from_str")]
    pub height: HeightMeasurement,
    pub weight: u16,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StrikeZone {
    pub strike_zone_top: f64,
    pub strike_zone_bottom: f64,
}

impl Eq for StrikeZone {}

#[derive(Debug, Deref, DerefMut, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HydratedPerson {
    #[deref]
    #[deref_mut]
    #[serde(flatten)]
    inner: UnhydratedPerson,
    
    pub primary_position: Position,
    // '? ? Brown' in 1920 does not have a first name or a middle name, rather than dealing with Option and making everyone hate this API, the better approach is an empty String.
    #[serde(default)]
    pub first_name: String,
    pub middle_name: Option<String>,
    #[serde(default)]
    pub last_name: String,
    #[serde(default)]
    #[serde(rename = "useName")] pub use_first_name: String,
    #[serde(default)]
    pub use_last_name: String,
    #[serde(default)]
    pub boxscore_name: String,
    
    pub is_player: bool,
    pub is_verified: bool,
    pub active: bool,
}

impl HydratedPerson {
    #[must_use]
    pub fn name_first_last(&self) -> String {
        format!("{0} {1}", self.first_name, self.last_name)
    }

    #[must_use]
    pub fn name_last_first(&self) -> String {
        format!("{1}, {0}", self.first_name, self.last_name)
    }

    #[must_use]
    pub fn name_last_first_initial(&self) -> String {
        if let Some(char) = self.first_name.chars().next() {
            format!("{1}, {0}", char, self.last_name)
        } else {
            self.last_name.clone()
        }
    }

    #[must_use]
    pub fn name_first_initial_last(&self) -> String {
        if let Some(char) = self.first_name.chars().next() {
            format!("{0} {1}", char, self.last_name)
        } else {
            self.last_name.clone()
        }
    }

    #[must_use]
    pub fn name_fml(&self) -> String {
        if let Some(middle) = &self.middle_name {
            format!("{0} {1} {2}", self.first_name, middle, self.last_name)
        } else {
            format!("{0} {1}", self.first_name, self.last_name)
        }
    }

    #[must_use]
    pub fn name_lfm(&self) -> String {
        if let Some(middle) = &self.middle_name {
            format!("{2}, {0} {1}", self.first_name, middle, self.last_name)
        } else {
            format!("{1}, {0}", self.first_name, self.last_name)
        }
    }
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UnhydratedPerson {
    pub id: PersonId,
    pub full_name: String,
}

#[repr(transparent)]
#[derive(Debug, Deref, Display, Deserialize, PartialEq, Eq, Clone)]
pub struct PersonId(u32);

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(untagged)]
pub enum Person {
    Ballplayer(Ballplayer),
    Hydrated(HydratedPerson),
    Unhydrated(UnhydratedPerson),
}

impl Deref for Person {
    type Target = UnhydratedPerson;

    fn deref(&self) -> &Self::Target {
        match self {
            Self::Ballplayer(inner) => inner,
            Self::Hydrated(inner) => inner,
            Self::Unhydrated(inner) => inner,
        }
    }
}

impl DerefMut for Person {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            Self::Ballplayer(inner) => inner,
            Self::Hydrated(inner) => inner,
            Self::Unhydrated(inner) => inner,
        }
    }
}
