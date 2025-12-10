use serde_with::DisplayFromStr;
use std::collections::hash_map::Entry;
use std::convert::Infallible;
use derive_more::{Deref, DerefMut, TryFrom};
use serde::{Deserialize, Deserializer};
use std::fmt::{Debug, Display, Formatter};
use std::num::ParseIntError;
use std::str::FromStr;
use chrono::NaiveDate;
use fxhash::FxHashMap;
use serde::de::{DeserializeOwned, Error, Visitor};
use serde_json::Value;
use serde_with::serde_as;
use thiserror::Error;
use crate::requests::{GameType, StatGroup, StatType};
use crate::requests::person::Person;
use crate::requests::stats::catching::CatchingStats;
use crate::requests::stats::fielding::{FieldingStats, SimplifiedGameLogFieldingStats};
use crate::requests::stats::hitting::{AdvancedHittingStats, HittingStats, SabermetricsHittingStats, SimplifiedGameLogHittingStats, VsPlayerHittingStats};
use crate::requests::stats::pitching::{AdvancedPitchingStats, PitchUsage, PitchingStats, SabermetricsPitchingStats, SimplifiedGameLogPitchingStats, VsPlayerPitchingStats};
use crate::requests::stats::units::PercentageStat;
use crate::requests::teams::team::Team;
use crate::types::{RGBAColor, SimpleTemperature};

pub mod pieces;
pub mod piece_impls;
pub mod leaders;
pub mod hitting;
pub mod pitching;
pub mod fielding;
pub mod catching;
pub mod units;

pub trait Stats: Debug + DeserializeOwned + PartialEq + Eq + Clone {
	fn request_text() -> &'static str;
}

impl Stats for () {
	fn request_text() -> &'static str {
		""
	}
}

pub trait Stat: Debug + Clone + PartialEq + Eq {
	type SplitWrappedVariant: DeserializeOwned;

	type TryFromSplitWrappedVariantError;

	fn from_split_wrapped_variant(split_wrapped: Vec<Self::SplitWrappedVariant>) -> Result<Self, Self::TryFromSplitWrappedVariantError> where Self: Sized;

	fn fallback() -> Option<Self> where Self: Sized { None }
}

pub trait BaseStat: Debug + DeserializeOwned + Clone + PartialEq + Eq {

}

impl<T: BaseStat> Stat for T {
	type SplitWrappedVariant = Self;

	type TryFromSplitWrappedVariantError = &'static str;

	fn from_split_wrapped_variant(split_wrapped: Vec<Self::SplitWrappedVariant>) -> Result<Self, Self::TryFromSplitWrappedVariantError>
	where
		Self: Sized
	{
		<Vec<Self> as TryInto<[Self; 1]>>::try_into(split_wrapped)
			.map_err(|_| "length of stat splits is is not 1, cannot convert to unit type.")
			.map(|[x]| x)
	}
}

impl BaseStat for HittingStats {}
impl BaseStat for VsPlayerHittingStats {}
impl BaseStat for SimplifiedGameLogHittingStats {}
impl BaseStat for AdvancedHittingStats {}
impl BaseStat for SabermetricsHittingStats {}

impl BaseStat for PitchingStats {}
impl BaseStat for VsPlayerPitchingStats {}
impl BaseStat for SimplifiedGameLogPitchingStats {}
impl BaseStat for SabermetricsPitchingStats {}
impl BaseStat for AdvancedPitchingStats {}

impl BaseStat for FieldingStats {}
impl BaseStat for SimplifiedGameLogFieldingStats {}

impl BaseStat for CatchingStats {}


pub trait StatTypeStats {
	type Hitting: Stat;

	type Pitching: Stat;

	type Fielding: Stat;

	type Catching: Stat;
}

#[derive(Deserialize)]
#[doc(hidden)]
struct __RawStats(Vec<__RawStatEntry>);

#[derive(Deserialize)]
#[serde(untagged)]
#[doc(hidden)]
enum __RawStatEntry {
	Depth0(__Depth0StatEntry),
	Depth1(__Depth1StatEntry),
}

pub type __Depth0StatEntry = __ParsedStatEntry;

#[derive(Deserialize)]
#[doc(hidden)]
struct __Depth1StatEntry {
	splits: Vec<__InlineStatEntry>
}

#[derive(Deserialize)]
#[doc(hidden)]
struct __InlineStatEntry {
	#[serde(rename = "type")]
	stat_type: StatType,
	#[serde(rename = "group")]
	stat_group: StatGroup,
	stat: Value,
}

impl From<__InlineStatEntry> for __ParsedStatEntry {
	fn from(value: __InlineStatEntry) -> Self {
		Self {
			stat_type: value.stat_type,
			stat_group: value.stat_group,
			splits: vec![value.stat],
		}
	}
}

impl From<__Depth1StatEntry> for Vec<__Depth0StatEntry> {
	fn from(value: __Depth1StatEntry) -> Self {
		value.splits.into_iter().map(|x| x.into()).collect()
	}
}

impl From<__RawStatEntry> for Vec<__ParsedStatEntry> {
	fn from(value: __RawStatEntry) -> Self {
		match value {
			__RawStatEntry::Depth0(x) => vec![x],
			__RawStatEntry::Depth1(x) => x.into(),
		}
	}
}

impl From<__RawStats> for __ParsedStats {
	fn from(value: __RawStats) -> Self {
		Self {
			entries: value.0.into_iter().flat_map::<Vec<__ParsedStatEntry>, _>(|entry| entry.into()).collect()
		}
	}
}

#[doc(hidden)]
#[derive(Deserialize, Debug)]
#[serde(from = "__RawStats")]
pub struct __ParsedStats {
	entries: Vec<__ParsedStatEntry>
}

#[doc(hidden)]
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct __ParsedStatEntry {
	#[serde(rename = "type")]
	stat_type: StatType,
	#[serde(rename = "group")]
	stat_group: StatGroup,
	splits: Vec<Value>,
}

#[doc(hidden)]
#[derive(Debug, Error)]
pub enum MakeStatSplitsError<S: Stat> {
	#[error("No matches found for {0} + {1}")]
	NoMatchFound(&'static str, StatGroup),
	#[error("Failed to deserialize json into split type ({name}): {0}", name = core::any::type_name::<S>())]
	FailedPartialDeserialize(serde_json::Error),
	// FailedPartialDeserialize(serde_path_to_error::Error<serde_json::Error>),
	#[error("Failed to deserialize splits into greater split type ({name}): {0}", name = core::any::type_name::<S>())]
	FailedFullDeserialize(S::TryFromSplitWrappedVariantError),
}

#[doc(hidden)]
pub fn make_stat_split<S: Stat>(stats: &mut __ParsedStats, target_stat_type_str: &'static str, target_stat_group: StatGroup) -> Result<S, MakeStatSplitsError<S>> {
	if let Some(idx) = stats.entries.iter().position(|entry| entry.stat_type.as_str().eq_ignore_ascii_case(target_stat_type_str) && entry.stat_group == target_stat_group) {
		let entry = stats.entries.remove(idx);
		let partially_deserialized = entry.splits
			.into_iter()
			.map(|split| {
				<<S as Stat>::SplitWrappedVariant as Deserialize>::deserialize(split)
				// serde_path_to_error::deserialize::<_, <S as Stat>::SplitWrappedVariant>(split)
			})
			.collect::<Result<Vec<S::SplitWrappedVariant>, _>>()
			.map_err(MakeStatSplitsError::FailedPartialDeserialize)?;
		let deserialized = <S as Stat>::from_split_wrapped_variant(partially_deserialized).map_err(MakeStatSplitsError::FailedFullDeserialize)?;
		Ok(deserialized)
	} else if let Some(fallback) = S::fallback() {
		Ok(fallback)
	} else {
		Err(MakeStatSplitsError::NoMatchFound(target_stat_type_str, target_stat_group))
	}
}

macro_rules! stat_type_stats {
    ($name:ident {
	    $hitting:ty,
	    $pitching:ty,
	    $fielding:ty,
	    $catching:ty $(,)?
    }) => {
	    pub struct $name;

	    impl StatTypeStats for $name {
		    type Hitting = $hitting;
		    type Pitching = $pitching;
		    type Fielding = $fielding;
		    type Catching = $catching;
	    }
    };
}

impl BaseStat for () {}

#[derive(Debug, PartialEq, Eq, Clone, Default, Deref, DerefMut)]
pub struct Multiple<T: Stat + DeserializeOwned> {
	pub entries: Vec<T>,
}

impl<T: Stat + DeserializeOwned> Stat for Multiple<T> {
	type SplitWrappedVariant = T;
	type TryFromSplitWrappedVariantError = Infallible;

	fn from_split_wrapped_variant(split_wrapped: Vec<Self::SplitWrappedVariant>) -> Result<Self, Self::TryFromSplitWrappedVariantError>
	where
		Self: Sized
	{
		Ok(Self { entries: split_wrapped })
	}

	fn fallback() -> Option<Self>
	where
		Self: Sized,
	{
		Some(Multiple { entries: Vec::new() })
	}
}

#[serde_as]
#[derive(Debug, Deserialize, PartialEq, Eq, Clone, Deref, DerefMut)]
#[serde(bound = "T: Stat + DeserializeOwned")]
pub struct Season<T: Stat + DeserializeOwned> {
	#[serde_as(as = "DisplayFromStr")]
	pub season: u32,
	#[deref]
	#[deref_mut]
	#[serde(rename = "stat")]
	pub stats: T,
}

impl<T: Stat + DeserializeOwned> BaseStat for Season<T> {

}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct MultipleSeasons<T: Stat + DeserializeOwned> {
	pub seasons: FxHashMap<u32, Season<T>>,
}

#[derive(Debug, Error)]
pub enum MultipleSeasonsFromSplitWrappedVariantError {
	#[error("Duplicate entry for season {season} found")]
	DuplicateEntry { season: u32 },
}

impl<T: Stat + DeserializeOwned> Stat for MultipleSeasons<T> {
	type SplitWrappedVariant = Season<T>;
	type TryFromSplitWrappedVariantError = MultipleSeasonsFromSplitWrappedVariantError;

	fn from_split_wrapped_variant(split_wrapped: Vec<Self::SplitWrappedVariant>) -> Result<Self, Self::TryFromSplitWrappedVariantError>
	where
		Self: Sized
	{
		let mut this = Self { seasons: Default::default() };
		for season in split_wrapped {
			match this.seasons.entry(season.season) {
				Entry::Occupied(_) => return Err (Self::TryFromSplitWrappedVariantError::DuplicateEntry { season: season.season }),
				Entry::Vacant(slot) => slot.insert(season),
			};
		}
		Ok(this)
	}

	fn fallback() -> Option<Self> {
		Some(Self { seasons: Default::default() })
	}
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone, Deref, DerefMut)]
#[serde(rename_all = "camelCase")]
#[serde(bound = "T: Stat + DeserializeOwned")]
pub struct Game<T: Stat + DeserializeOwned> {
	#[deref]
	#[deref_mut]
	#[serde(flatten)]
	stats: Season<T>,
	pub opponent: Team,
	pub date: NaiveDate,
	pub is_home: bool,
	pub game: super::game::Game,
}

impl<T: Stat + DeserializeOwned> BaseStat for Game<T> {}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone, Deref, DerefMut)]
#[serde(rename_all = "camelCase")]
#[serde(bound = "T: Stat + DeserializeOwned")]
pub struct Player<T: Stat + DeserializeOwned> {
	#[deref]
	#[deref_mut]
	#[serde(flatten)]
	stats: Season<T>,
	pub player: Person,
	pub game_type: GameType,
	pub rank: u32,
}

impl<T: Stat + DeserializeOwned> BaseStat for Player<T> {}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone, Deref, DerefMut)]
#[serde(rename_all = "camelCase")]
#[serde(bound = "T: Stat + DeserializeOwned")]
pub struct SingleMatchup<T: Stat + DeserializeOwned> {
	#[deref]
	#[deref_mut]
	#[serde(flatten)]
	stats: Game<T>,
	pub pitcher: Person,
	pub batter: Person,
}

impl<T: Stat + DeserializeOwned> BaseStat for SingleMatchup<T> {}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone, Deref, DerefMut)]
#[serde(rename_all = "camelCase")]
#[serde(bound = "T: Stat + DeserializeOwned")]
pub struct AccumulatedMatchup<T: Stat + DeserializeOwned> {
	#[deref]
	#[deref_mut]
	#[serde(rename = "stat")]
	pub stats: T,
	pub team: Team,
	#[serde(rename = "opponent")]
	pub opposing_team: Team,
	pub game_type: GameType,
}

impl<T: Stat + DeserializeOwned> BaseStat for AccumulatedMatchup<T> {}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone, Deref, DerefMut)]
#[serde(rename_all = "camelCase")]
#[serde(bound = "T: Stat + DeserializeOwned")]
pub struct AccumulatedVsPlayerMatchup<T: Stat + DeserializeOwned> {
	#[deref]
	#[deref_mut]
	#[serde(flatten)]
	stats: AccumulatedMatchup<T>,
	pub pitcher: Person,
	pub batter: Person,
}

impl<T: Stat + DeserializeOwned> BaseStat for AccumulatedVsPlayerMatchup<T> {}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone, Deref, DerefMut)]
#[serde(rename_all = "camelCase")]
#[serde(bound = "T: Stat + DeserializeOwned")]
pub struct AccumulatedVsTeamTotalMatchup<T: Stat + DeserializeOwned> {
	#[deref]
	#[deref_mut]
	#[serde(flatten)]
	stats: AccumulatedMatchup<T>,
	pub rank: usize,
	pub batter: Person,
}

impl<T: Stat + DeserializeOwned> BaseStat for AccumulatedVsTeamTotalMatchup<T> {}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone, Deref, DerefMut)]
#[serde(rename_all = "camelCase")]
#[serde(bound = "T: Stat + DeserializeOwned")]
pub struct AccumulatedVsTeamSeasonalPitcherSplit<T: Stat + DeserializeOwned> {
	#[deref]
	#[deref_mut]
	#[serde(flatten)]
	stats: AccumulatedMatchup<Season<T>>,
	pub rank: usize,
	pub pitcher: Person,
	pub batter: Person,
}

impl<T: Stat + DeserializeOwned> BaseStat for AccumulatedVsTeamSeasonalPitcherSplit<T> {}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FieldedMatchup {
	pub pitcher: Person,
	pub batter: Person,
	pub fielding_team: Team,
}

impl BaseStat for FieldedMatchup {}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone, Deref, DerefMut)]
#[serde(rename_all = "camelCase")]
#[serde(bound = "T: Stat + DeserializeOwned")]
pub struct Month<T: Stat + DeserializeOwned> {
	#[deref]
	#[deref_mut]
	#[serde(flatten)]
	stats: Season<T>,
	pub month: chrono::Month,
}

impl<T: Stat + DeserializeOwned> BaseStat for Month<T> {}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone, Deref, DerefMut)]
#[serde(rename_all = "camelCase")]
#[serde(bound = "T: Stat + DeserializeOwned")]
pub struct Weekday<T: Stat + DeserializeOwned> {
	#[deref]
	#[deref_mut]
	#[serde(flatten)]
	stats: Season<T>,
	#[serde(deserialize_with = "deserialize_day_of_week")]
	pub weekday: chrono::Weekday,
}

impl<T: Stat + DeserializeOwned> BaseStat for Weekday<T> {}

fn deserialize_day_of_week<'de, D: Deserializer<'de>>(deserializer: D) -> Result<chrono::Weekday, D::Error> {
	struct WeekdayVisitor;

	impl Visitor<'_> for WeekdayVisitor {
		type Value = chrono::Weekday;

		fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
			formatter.write_str("an integer between 0 and 6 representing the day of the week")
		}

		fn visit_i8<E>(self, v: i8) -> Result<Self::Value, E>
		where
			E: Error,
		{
			chrono::Weekday::try_from(v as u8 - 1).map_err(E::custom)
		}
	}

	deserializer.deserialize_any(WeekdayVisitor)
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(bound = "T: Stat + DeserializeOwned")]
#[doc(hidden)]
pub struct __HomeOrAwayStruct<T: Stat + DeserializeOwned> {
	#[serde(flatten)]
	stats: Season<T>,
	is_home: bool,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct HomeAndAway<T: Stat + DeserializeOwned> {
	pub home: Season<T>,
	pub away: Season<T>,
}

#[derive(Debug, Error)]
pub enum HomeAndAwayFromSplitWrappedVariantError {
	#[error("Did not find exactly two splits")]
	NotLen2,
	#[error("Found multiple home splits")]
	DuplicateHome,
	#[error("Found multiple away splits")]
	DuplicateAway,
}

impl<T: Stat + DeserializeOwned> Stat for HomeAndAway<T> {
	type SplitWrappedVariant = __HomeOrAwayStruct<T>;
	type TryFromSplitWrappedVariantError = HomeAndAwayFromSplitWrappedVariantError;

	fn from_split_wrapped_variant(split_wrapped: Vec<Self::SplitWrappedVariant>) -> Result<Self, Self::TryFromSplitWrappedVariantError>
	where
		Self: Sized
	{
		use HomeAndAwayFromSplitWrappedVariantError as Error;

		let [a, b] = <Vec<Self::SplitWrappedVariant> as TryInto<[Self::SplitWrappedVariant; 2]>>::try_into(split_wrapped).map_err(|_| Error::NotLen2)?;
		if a.is_home == b.is_home {
			return Err(if a.is_home { Error::DuplicateHome } else { Error::DuplicateAway })
		}

		if a.is_home {
			Ok(Self {
				home: a.stats,
				away: b.stats,
			})
		} else {
			Ok(Self {
				home: b.stats,
				away: a.stats,
			})
		}
	}
}


#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(bound = "T: Stat + DeserializeOwned")]
#[doc(hidden)]
pub struct __WinOrLossStruct<T: Stat + DeserializeOwned> {
	#[serde(flatten)]
	stats: Season<T>,
	is_win: bool,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct WinLoss<T: Stat + DeserializeOwned> {
	pub win: Season<T>,
	pub loss: Season<T>,
}

#[derive(Debug, Error)]
pub enum WinLossFromSplitWrappedVariantError {
	#[error("Did not find exactly two splits")]
	NotLen2,
	#[error("Found multiple win splits")]
	DuplicateWin,
	#[error("Found multiple loss splits")]
	DuplicateLoss,
}

impl<T: Stat + DeserializeOwned> Stat for WinLoss<T> {
	type SplitWrappedVariant = __WinOrLossStruct<T>;
	type TryFromSplitWrappedVariantError = WinLossFromSplitWrappedVariantError;

	fn from_split_wrapped_variant(split_wrapped: Vec<Self::SplitWrappedVariant>) -> Result<Self, Self::TryFromSplitWrappedVariantError>
	where
		Self: Sized
	{
		use WinLossFromSplitWrappedVariantError as Error;

		let [a, b] = <Vec<Self::SplitWrappedVariant> as TryInto<[Self::SplitWrappedVariant; 2]>>::try_into(split_wrapped).map_err(|_| Error::NotLen2)?;
		if a.is_win == b.is_win {
			return Err(if a.is_win { Error::DuplicateWin } else { Error::DuplicateLoss })
		}

		if a.is_win {
			Ok(Self {
				win: a.stats,
				loss: b.stats,
			})
		} else {
			Ok(Self {
				win: b.stats,
				loss: a.stats,
			})
		}
	}
}

#[serde_as]
#[derive(Debug, Deserialize, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
pub struct ExpectedStats {
	#[serde_as(as = "DisplayFromStr")]
	#[serde(rename = "avg")]
	pub avg: f64,
	#[serde_as(as = "DisplayFromStr")]
	#[serde(rename = "slg")]
	pub slg: f64,
	#[serde(rename = "woba")]
	pub wOBA: f64,
	#[serde(rename = "wobacon")]
	pub wOBACON: f64,
}

impl Eq for ExpectedStats {}

impl BaseStat for ExpectedStats {}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename = "camelCase")]
pub struct SprayChart {
	#[serde(rename = "stat")]
	spray: HitSpray,
	batter: Person,
}

impl BaseStat for SprayChart {}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename = "camelCase")]
pub struct HitSpray {
	left_field: PercentageStat,
	left_center_field: PercentageStat,
	center_field: PercentageStat,
	right_center_field: PercentageStat,
	right_field: PercentageStat,
}

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

impl Eq for HotColdZone {}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
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

#[allow(non_snake_case)]
#[derive(Debug, PartialEq, Eq, Clone)]
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
	type SplitWrappedVariant = __HotColdZonesEntryStruct;
	type TryFromSplitWrappedVariantError = HotColdZonesFromSplitWrappedError;

	fn from_split_wrapped_variant(split_wrapped: Vec<Self::SplitWrappedVariant>) -> Result<Self, Self::TryFromSplitWrappedVariantError>
	where
		Self: Sized
	{
		use HotColdZonesFromSplitWrappedError as Error;

		let mut ops: Option<HotColdZones> = None;
		let mut avg: Option<HotColdZones> = None;
		let mut obp: Option<HotColdZones> = None;
		let mut slg: Option<HotColdZones> = None;
		let mut avg_ev: Option<HotColdZones> = None;

		for entry in split_wrapped {
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

#[allow(non_snake_case)]
#[derive(Debug, PartialEq, Eq, Clone)]
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
	type SplitWrappedVariant = __HotColdZonesEntryStruct;
	type TryFromSplitWrappedVariantError = HotColdZonesFromSplitWrappedError;

	fn from_split_wrapped_variant(split_wrapped: Vec<Self::SplitWrappedVariant>) -> Result<Self, Self::TryFromSplitWrappedVariantError>
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

		for entry in split_wrapped {
			let slot = match &*entry.name {
				"battingAverage" => &mut avg,
				"onBasePercentage" => &mut obp,
				"sluggingPercentage" => &mut slg,
				"numberOfStrikes" => &mut num_strikes,
				"earnedRunAverage" => &mut era,
				"numberOfPitches" => &mut num_pitches,
				"onBasePlusSlugging" => &mut ops,
				_ => return Err(Error::Unknown(entry.name.clone())),
			};

			if slot.is_some() {
				return Err(Error::Duplicate(entry.name))
			}

			*slot = Some(entry.zones);
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

#[derive(Debug, Deserialize, PartialEq, Eq, Clone/*, Deref, DerefMut*/)]
pub struct PlayStat {
	// pub play: Play,
}

// todo: replace with real struct
pub type PitchStat = ();

impl BaseStat for PlayStat {}

impl<T: Stat> Stat for Option<T> {
	type SplitWrappedVariant = T::SplitWrappedVariant;
	type TryFromSplitWrappedVariantError = Infallible;

	fn from_split_wrapped_variant(split_wrapped: Vec<Self::SplitWrappedVariant>) -> Result<Self, Self::TryFromSplitWrappedVariantError>
	where
		Self: Sized
	{
		Ok(T::from_split_wrapped_variant(split_wrapped).ok())
	}
}

stat_type_stats!(ProjectedStats { Season<HittingStats>, Season<PitchingStats>, (), () });
stat_type_stats!(ProjectedRosStats { Season<HittingStats>, Season<PitchingStats>, (), () });
stat_type_stats!(YearByYearStats { MultipleSeasons<HittingStats>, MultipleSeasons<PitchingStats>, MultipleSeasons<FieldingStats>, MultipleSeasons<CatchingStats> });
stat_type_stats!(YearByYearAdvancedStats { MultipleSeasons<AdvancedHittingStats>, MultipleSeasons<AdvancedPitchingStats>, (), () });
stat_type_stats!(YearByYearPlayoffsStats { MultipleSeasons<HittingStats>, MultipleSeasons<PitchingStats>, MultipleSeasons<FieldingStats>, MultipleSeasons<CatchingStats> });
stat_type_stats!(SeasonStats { Season<HittingStats>, Season<PitchingStats>, Season<FieldingStats>, Season<CatchingStats> });
// `standard`?
// `advanced`?
stat_type_stats!(CareerStats { HittingStats, PitchingStats, FieldingStats, CatchingStats });
stat_type_stats!(CareerRegularSeasonStats { MultipleSeasons<HittingStats>, MultipleSeasons<PitchingStats>, MultipleSeasons<FieldingStats>, MultipleSeasons<CatchingStats> });
stat_type_stats!(CareerAdvancedStats { AdvancedHittingStats, AdvancedPitchingStats, (), () });
stat_type_stats!(SeasonAdvancedStats { Season<AdvancedHittingStats>, Season<AdvancedPitchingStats>, (), () });
// 'careerStatSplit'?
stat_type_stats!(CareerPlayoffsStats { MultipleSeasons<HittingStats>, MultipleSeasons<PitchingStats>, MultipleSeasons<FieldingStats>, MultipleSeasons<CatchingStats> });
stat_type_stats!(SimplifiedGameLogStats { Option<SimplifiedGameLogHittingStats>, Option<SimplifiedGameLogPitchingStats>, Option<SimplifiedGameLogFieldingStats>, Option<CatchingStats> });
stat_type_stats!(GameLogStats { Multiple<Game<HittingStats>>, Multiple<Game<PitchingStats>>, Multiple<Game<FieldingStats>>, Multiple<Game<CatchingStats>> });
stat_type_stats!(PlayLogStats { Multiple<SingleMatchup<PlayStat>>, Multiple<SingleMatchup<PlayStat>>, Multiple<SingleMatchup<PlayStat>>, Multiple<SingleMatchup<PlayStat>> });
stat_type_stats!(SimplifiedPlayLogStats { Multiple<PlayStat>, Multiple<PlayStat>, Multiple<PlayStat>, Multiple<SingleMatchup<PlayStat>> });
stat_type_stats!(PitchLogStats { Multiple<SingleMatchup<PitchStat>>, Multiple<SingleMatchup<PitchStat>>, Multiple<SingleMatchup<PitchStat>>, Multiple<SingleMatchup<PitchStat>> });
// 'metricLog'?
// 'metricAverages'?
stat_type_stats!(PitchArsenalStats { Multiple<PitchUsage>, Multiple<PitchUsage>, (), () });
// 'outsAboveAverage'?
stat_type_stats!(ExpectedStatisticsStats { Player<ExpectedStats>, Player<ExpectedStats>, (), () });
stat_type_stats!(SabermetricsStats { Player<SabermetricsHittingStats>, Player<SabermetricsPitchingStats>, (), () });
stat_type_stats!(SprayChartStats { SprayChart, SprayChart, (), () });
// 'tracking'?
stat_type_stats!(VsPlayerStats { AccumulatedMatchup<VsPlayerHittingStats>, AccumulatedMatchup<VsPlayerPitchingStats>, (), () });
stat_type_stats!(VsPlayerTotalStats { AccumulatedMatchup<VsPlayerHittingStats>, AccumulatedMatchup<VsPlayerPitchingStats>, (), () });
stat_type_stats!(VsPlayer5YStats { AccumulatedMatchup<VsPlayerHittingStats>, AccumulatedMatchup<VsPlayerPitchingStats>, (), () });
stat_type_stats!(VsTeamStats { Multiple<AccumulatedVsTeamSeasonalPitcherSplit<HittingStats>>, (), (), () });
stat_type_stats!(VsTeam5YStats { Multiple<AccumulatedVsTeamSeasonalPitcherSplit<HittingStats>>, (), (), () });
stat_type_stats!(VsTeamTotalStats { AccumulatedVsTeamTotalMatchup<HittingStats>, (), (), () });
stat_type_stats!(LastXGamesStats { HittingStats, PitchingStats, FieldingStats, CatchingStats });
stat_type_stats!(ByDateRangeStats { HittingStats, PitchingStats, FieldingStats, CatchingStats });
// 'byDateRangeAdvanced'?
stat_type_stats!(ByMonthStats { Month<HittingStats>, Month<PitchingStats>, Month<FieldingStats>, Month<CatchingStats> });
stat_type_stats!(ByMonthPlayoffsStats { Month<HittingStats>, Month<PitchingStats>, Month<FieldingStats>, Month<CatchingStats> });
stat_type_stats!(ByDayOfWeekStats { Weekday<HittingStats>, Weekday<PitchingStats>, Weekday<FieldingStats>, Weekday<CatchingStats> });
stat_type_stats!(ByDayOfWeekPlayoffsStats { Weekday<HittingStats>, Weekday<PitchingStats>, Weekday<FieldingStats>, Weekday<CatchingStats> });
stat_type_stats!(HomeAndAwayStats { HomeAndAway<HittingStats>, HomeAndAway<PitchingStats>, HomeAndAway<FieldingStats>, HomeAndAway<CatchingStats> });
stat_type_stats!(HomeAndAwayPlayoffsStats { HomeAndAway<HittingStats>, HomeAndAway<PitchingStats>, HomeAndAway<FieldingStats>, HomeAndAway<CatchingStats> });
stat_type_stats!(WinLossStats { WinLoss<HittingStats>, WinLoss<PitchingStats>, WinLoss<FieldingStats>, WinLoss<CatchingStats> });
stat_type_stats!(WinLossPlayoffsStats { WinLoss<HittingStats>, WinLoss<PitchingStats>, WinLoss<FieldingStats>, WinLoss<CatchingStats> });
// 'rankings'?
// 'rankingsByYear'?
// 'statsSingleSeason'?
// 'statsSingleSeasonAdvanced'?
stat_type_stats!(HotColdZonesStats { HittingHotColdZones, PitchingHotColdZones, (), () });
// 'availableStats'?
stat_type_stats!(OpponentsFacedStats { Multiple<FieldedMatchup>, Multiple<FieldedMatchup>, Multiple<FieldedMatchup>, Multiple<FieldedMatchup> });
// 'gameTypeStats'?
// 'firstYearStats'?
// 'lastYearStats'?
// 'statSplits'?
// 'statSplitsAdvanced'?
stat_type_stats!(AtGameStartStats { Multiple<Game<HittingStats>>, Multiple<Game<PitchingStats>>, Multiple<Game<FieldingStats>>, Multiple<Game<CatchingStats>> });
// 'vsOpponents'?

