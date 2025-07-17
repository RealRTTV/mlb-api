pub mod stats;

use crate::endpoints::positions::Position;
use crate::types::{Gender, Handedness, HeightMeasurement};
use chrono::NaiveDate;
use derive_more::{Deref, DerefMut, Display, From};
use serde::Deserialize;
use serde_with::DisplayFromStr;
use serde_with::serde_as;
use std::ops::{Deref, DerefMut};
use strum::EnumTryAs;

#[serde_as]
#[derive(Debug, Deref, DerefMut, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Ballplayer {
	#[serde(deserialize_with = "crate::types::try_from_str")]
	#[serde(default)]
	pub primary_number: Option<u8>,
	pub current_age: u8,
	#[serde(flatten)]
	pub birth_data: BirthData,
	#[serde(flatten)]
	pub body_measurements: BodyMeasurements,
	pub gender: Gender,
	pub draft_year: Option<u16>,
	#[serde(rename = "mlbDebutDate")]
	pub mlb_debut: Option<NaiveDate>,
	pub bat_side: Handedness,
	pub pitch_hand: Handedness,
	#[serde(flatten)]
	pub strike_zone: StrikeZone,
	#[serde(rename = "nickName")]
	pub nickname: Option<String>,

	#[deref]
	#[deref_mut]
	#[serde(flatten)]
	inner: HydratedPerson,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BirthData {
	pub birth_date: NaiveDate,
	pub birth_city: String,
	#[serde(rename = "birthStateProvince")]
	pub birth_state_or_province: Option<String>,
	pub birth_country: String,
}

#[serde_as]
#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BodyMeasurements {
	#[serde_as(as = "DisplayFromStr")]
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
	pub primary_position: Position,
	// '? ? Brown' in 1920 does not have a first name or a middle name, rather than dealing with Option and making everyone hate this API, the better approach is an empty String.
	#[serde(default)]
	pub first_name: String,
	pub middle_name: Option<String>,
	#[serde(default)]
	pub last_name: String,
	#[serde(default)]
	#[serde(rename = "useName")]
	pub use_first_name: String,
	#[serde(default)]
	pub use_last_name: String,
	#[serde(default)]
	pub boxscore_name: String,

	pub is_player: bool,
	#[serde(default)]
	pub is_verified: bool,
	pub active: bool,

	#[deref]
	#[deref_mut]
	#[serde(flatten)]
	inner: NamedPerson,
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

#[derive(Debug, Deserialize, Deref, DerefMut, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NamedPerson {
	pub full_name: String,

	#[deref]
	#[deref_mut]
	#[serde(flatten)]
	inner: IdentifiablePerson,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IdentifiablePerson {
	pub id: PersonId,
}

#[repr(transparent)]
#[derive(Debug, Deserialize, Deref, Display, PartialEq, Eq, Copy, Clone)]
pub struct PersonId(u32);

impl PersonId {
	#[must_use]
	pub const fn new(id: u32) -> Self {
		Self(id)
	}
}

#[derive(Debug, Deserialize, Eq, Clone, From, EnumTryAs)]
#[serde(untagged)]
pub enum Person {
	Ballplayer(Ballplayer),
	Hydrated(HydratedPerson),
	Named(NamedPerson),
	Identifiable(IdentifiablePerson),
}

impl Person {
	#[must_use]
	pub(crate) const fn unknown_person() -> Self {
		Self::Identifiable(IdentifiablePerson { id: PersonId::new(0) })
	}
}

impl PartialEq for Person {
	fn eq(&self, other: &Self) -> bool {
		self.id == other.id
	}
}

impl Deref for Person {
	type Target = IdentifiablePerson;

	fn deref(&self) -> &Self::Target {
		match self {
			Self::Ballplayer(inner) => inner,
			Self::Hydrated(inner) => inner,
			Self::Named(inner) => inner,
			Self::Identifiable(inner) => inner,
		}
	}
}

impl DerefMut for Person {
	fn deref_mut(&mut self) -> &mut Self::Target {
		match self {
			Self::Ballplayer(inner) => inner,
			Self::Hydrated(inner) => inner,
			Self::Named(inner) => inner,
			Self::Identifiable(inner) => inner,
		}
	}
}
