use serde_with::DisplayFromStr;
use std::fmt::{Display, Formatter};
use std::num::ParseIntError;
use std::str::FromStr;
use derive_more::TryFrom;
use serde::Deserialize;
use serde_with::serde_as;
use thiserror::Error;
use crate::stats::Stat;
use crate::types::{RGBAColor, SimpleTemperature};

#[derive(Debug, Deserialize, PartialEq, Eq, Copy, Clone, TryFrom)]
#[serde(try_from = "u8")]
#[try_from(repr)]
#[repr(u8)]
pub enum StrikeZoneSection {
	TopLeft = 1,
	TopMiddle = 2,
	TopRight = 3,
	MiddleLeft = 4,
	MiddleMiddle = 5,
	MiddleRight = 6,
	BottomLeft = 7,
	BottomMiddle = 8,
	BottomRight = 9,

	OutOfZoneTopLeft = 11,
	OutOfZoneTopRight = 12,
	OutOfZoneBottomLeft = 13,
	OutOfZoneBottomRight = 14,
}

impl Display for StrikeZoneSection {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", *self as u8)
	}
}

#[derive(Debug, Error)]
pub enum StrikeZoneSectionFromStrError {
	#[error(transparent)]
	Integer(#[from] ParseIntError),
	#[error(transparent)]
	Section(#[from] <StrikeZoneSection as TryFrom<u8>>::Error),
}

impl FromStr for StrikeZoneSection {
	type Err = StrikeZoneSectionFromStrError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		Ok(Self::try_from(s.parse::<u8>()?)?)
	}
}

#[serde_as]
#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(rename = "camelCase")]
pub struct HotColdZone {
	pub zone: StrikeZoneSection,
	pub color: RGBAColor,
	#[serde(rename = "temp")]
	pub temperature: SimpleTemperature,
	#[serde_as(as = "DisplayFromStr")]
	pub value: f64,
}

impl Default for HotColdZone {
	fn default() -> Self {
		Self {
			zone: StrikeZoneSection::MiddleMiddle,
			color: RGBAColor::default(),
			temperature: SimpleTemperature::Lukewarm,
			value: 0.0,
		}
	}
}

impl Eq for HotColdZone {}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone, Default)]
#[serde(try_from = "__HotColdZonesStruct")]
pub struct HotColdZones {
	pub z01: HotColdZone,
	pub z02: HotColdZone,
	pub z03: HotColdZone,
	pub z04: HotColdZone,
	pub z05: HotColdZone,
	pub z06: HotColdZone,
	pub z07: HotColdZone,
	pub z08: HotColdZone,
	pub z09: HotColdZone,
	pub z11: HotColdZone,
	pub z12: HotColdZone,
	pub z13: HotColdZone,
	pub z14: HotColdZone,
}

#[derive(Deserialize)]
#[doc(hidden)]
struct __HotColdZonesStruct {
	zones: Vec<HotColdZone>,
}

impl TryFrom<__HotColdZonesStruct> for HotColdZones {
	type Error = &'static str;

	// the only way to make this look less ugly is with a macro or something else smart, so if you want...
	fn try_from(value: __HotColdZonesStruct) -> Result<Self, Self::Error> {
		let __HotColdZonesStruct { zones } = value;
		let mut z01: Option<HotColdZone> = None;
		let mut z02: Option<HotColdZone> = None;
		let mut z03: Option<HotColdZone> = None;
		let mut z04: Option<HotColdZone> = None;
		let mut z05: Option<HotColdZone> = None;
		let mut z06: Option<HotColdZone> = None;
		let mut z07: Option<HotColdZone> = None;
		let mut z08: Option<HotColdZone> = None;
		let mut z09: Option<HotColdZone> = None;
		let mut z11: Option<HotColdZone> = None;
		let mut z12: Option<HotColdZone> = None;
		let mut z13: Option<HotColdZone> = None;
		let mut z14: Option<HotColdZone> = None;

		for zone in zones {
			let slot = match zone.zone {
				StrikeZoneSection::TopLeft => &mut z01,
				StrikeZoneSection::TopMiddle => &mut z02,
				StrikeZoneSection::TopRight => &mut z03,
				StrikeZoneSection::MiddleLeft => &mut z04,
				StrikeZoneSection::MiddleMiddle => &mut z05,
				StrikeZoneSection::MiddleRight => &mut z06,
				StrikeZoneSection::BottomLeft => &mut z07,
				StrikeZoneSection::BottomMiddle => &mut z08,
				StrikeZoneSection::BottomRight => &mut z09,
				StrikeZoneSection::OutOfZoneTopLeft => &mut z11,
				StrikeZoneSection::OutOfZoneTopRight => &mut z12,
				StrikeZoneSection::OutOfZoneBottomLeft => &mut z13,
				StrikeZoneSection::OutOfZoneBottomRight => &mut z14,
			};

			if slot.is_some() {
				return Err("duplicate zone found")
			}

			*slot = Some(zone);
		}

		Ok(Self {
			z01: z01.ok_or("zone 'z01' not found")?,
			z02: z02.ok_or("zone 'z02' not found")?,
			z03: z03.ok_or("zone 'z03' not found")?,
			z04: z04.ok_or("zone 'z04' not found")?,
			z05: z05.ok_or("zone 'z05' not found")?,
			z06: z06.ok_or("zone 'z06' not found")?,
			z07: z07.ok_or("zone 'z07' not found")?,
			z08: z08.ok_or("zone 'z08' not found")?,
			z09: z09.ok_or("zone 'z09' not found")?,
			z11: z11.ok_or("zone 'z11' not found")?,
			z12: z12.ok_or("zone 'z12' not found")?,
			z13: z13.ok_or("zone 'z13' not found")?,
			z14: z14.ok_or("zone 'z14' not found")?
		})
	}
}

#[allow(non_snake_case, reason = "stats names")]
#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct HittingHotColdZones {
	pub OPS: HotColdZones,
	pub AVG: HotColdZones,
	pub OBP: HotColdZones,
	pub SLG: HotColdZones,
	pub avgEV: HotColdZones,
}

#[derive(Debug, Error)]
pub enum HotColdZonesFromSplitWrappedError {
	#[error("Missing {0}.")]
	Missing(&'static str),
	#[error("Duplicate {0} found.")]
	Duplicate(String),
	#[error("Unknown variant '{0}'")]
	Unknown(String),
}

impl Stat for HittingHotColdZones {
	type Split = __HotColdZonesEntryStruct;
	type TryFromSplitError = HotColdZonesFromSplitWrappedError;

	fn from_splits(splits: impl Iterator<Item=Self::Split>) -> Result<Self, Self::TryFromSplitError>
	where
		Self: Sized
	{
		use HotColdZonesFromSplitWrappedError as Error;

		let mut ops: Option<HotColdZones> = None;
		let mut avg: Option<HotColdZones> = None;
		let mut obp: Option<HotColdZones> = None;
		let mut slg: Option<HotColdZones> = None;
		let mut avg_ev: Option<HotColdZones> = None;

		for entry in splits {
			let slot = match &*entry.name {
				"battingAverage" => &mut avg,
				"onBasePercentage" => &mut obp,
				"sluggingPercentage" => &mut slg,
				"exitVelocity" => &mut avg_ev,
				"onBasePlusSlugging" => &mut ops,
				_ => return Err(Error::Unknown(entry.name.clone())),
			};

			if slot.is_some() {
				return Err(Error::Duplicate(entry.name))
			}

			*slot = Some(entry.zones);
		}

		Ok(Self {
			OPS: ops.ok_or(Error::Missing("OPS"))?,
			AVG: avg.ok_or(Error::Missing("AVG"))?,
			OBP: obp.ok_or(Error::Missing("OBP"))?,
			SLG: slg.ok_or(Error::Missing("SLG"))?,
			avgEV: avg_ev.ok_or(Error::Missing("avgEV"))?,
		})
	}
}

#[allow(non_snake_case, reason = "stats names")]
#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct PitchingHotColdZones {
	pub AVG: HotColdZones,
	pub OBP: HotColdZones,
	pub SLG: HotColdZones,
	pub numStrikes: HotColdZones,
	pub ERA: HotColdZones,
	pub numPitches: HotColdZones,
	pub OPS: HotColdZones,
}

impl Stat for PitchingHotColdZones {
	type Split = __HotColdZonesEntryStruct;
	type TryFromSplitError = HotColdZonesFromSplitWrappedError;

	fn from_splits(splits: impl Iterator<Item=Self::Split>) -> Result<Self, Self::TryFromSplitError>
	where
		Self: Sized
	{
		use HotColdZonesFromSplitWrappedError as Error;

		let mut avg: Option<HotColdZones> = None;
		let mut obp: Option<HotColdZones> = None;
		let mut slg: Option<HotColdZones> = None;
		let mut num_strikes: Option<HotColdZones> = None;
		let mut era: Option<HotColdZones> = None;
		let mut num_pitches: Option<HotColdZones> = None;
		let mut ops: Option<HotColdZones> = None;

		for split in splits {
			let slot = match &*split.name {
				"battingAverage" => &mut avg,
				"onBasePercentage" => &mut obp,
				"sluggingPercentage" => &mut slg,
				"numberOfStrikes" => &mut num_strikes,
				"earnedRunAverage" => &mut era,
				"numberOfPitches" => &mut num_pitches,
				"onBasePlusSlugging" => &mut ops,
				_ => return Err(Error::Unknown(split.name.clone())),
			};

			if slot.is_some() {
				return Err(Error::Duplicate(split.name))
			}

			*slot = Some(split.zones);
		}

		Ok(Self {
			AVG: avg.ok_or(Error::Missing("AVG"))?,
			OBP: obp.ok_or(Error::Missing("OBP"))?,
			SLG: slg.ok_or(Error::Missing("SLG"))?,
			numStrikes: num_strikes.ok_or(Error::Missing("numStrikes"))?,
			ERA: era.ok_or(Error::Missing("ERA"))?,
			numPitches: num_pitches.ok_or(Error::Missing("numStrikes"))?,
			OPS: ops.ok_or(Error::Missing("OPS"))?,
		})
	}
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
#[doc(hidden)]
pub struct __HotColdZonesEntryStruct {
	name: String,
	zones: HotColdZones,
}