#![allow(clippy::trait_duplication_in_bounds, reason = "serde")]

use crate::league::NamedLeague;
use crate::stats::units::PercentageStat;
use crate::sport::SportId;
use crate::types::{RGBAColor, SimpleTemperature};
use chrono::{NaiveDate, Utc};
use derive_more::{Deref, DerefMut, TryFrom};
use fxhash::FxHashMap;
use serde::de::{DeserializeOwned, Error, Visitor};
use serde::{Deserialize, Deserializer};
use serde_json::Value;
use serde_with::serde_as;
use serde_with::DisplayFromStr;
use std::collections::hash_map::Entry;
use std::convert::Infallible;
use std::fmt::{Debug, Display, Formatter};
use std::num::ParseIntError;
use std::str::FromStr;
use smallvec::SmallVec;
use thiserror::Error;
use crate::game::GameId;
use crate::game_types::GameType;
use crate::hydrations::Hydrations;
use crate::person::NamedPerson;
use crate::season::SeasonId;
use crate::stat_groups::StatGroup;
use crate::stat_types::StatType;
use crate::stats::groups::pitching::PitchUsage;
use crate::team::NamedTeam;

pub mod macros;
pub mod groups;
pub mod pieces;
pub mod piece_impls;
pub mod leaders;
pub mod units;

pub trait Stats: 'static + Debug + PartialEq + Eq + Clone + Hydrations {}

impl Stats for () {}

pub trait Stat: Debug + Clone + PartialEq + Eq + Default {
	type SplitWrappedVariant: DeserializeOwned;

	type TryFromSplitWrappedError;

	/// # Errors
	/// See [`Self::TryFromSplitWrappedError`]
	fn from_split_wrapped(split_wrapped: Vec<Self::SplitWrappedVariant>) -> Result<Self, Self::TryFromSplitWrappedError> where Self: Sized;
}

pub trait SingletonWrappedEntryStat: Debug + DeserializeOwned + Clone + PartialEq + Eq + Default {

}

impl<T: SingletonWrappedEntryStat> Stat for T {
	type SplitWrappedVariant = Self;

	type TryFromSplitWrappedError = &'static str;

	fn from_split_wrapped(split_wrapped: Vec<Self::SplitWrappedVariant>) -> Result<Self, Self::TryFromSplitWrappedError>
	where
		Self: Sized
	{
		<Vec<Self> as TryInto<[Self; 1]>>::try_into(split_wrapped)
			.map_err(|_| "length of stat splits is is not 1, cannot convert to unit type.")
			.map(|[x]| x)
	}
}

pub trait StatTypeStats {
	type Hitting: Stat;

	type Pitching: Stat;

	type Fielding: Stat;

	type Catching: Stat;
}

#[derive(Deserialize)]
#[doc(hidden)]
struct __RawStats {
    #[serde(alias = "stat")]
	stats: Vec<__RawStatEntry>,
}

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
			splits: SmallVec::from_buf::<1>([value.stat]),
		}
	}
}

impl From<__Depth1StatEntry> for Vec<__Depth0StatEntry> {
	fn from(value: __Depth1StatEntry) -> Self {
		value.splits.into_iter().map(Into::into).collect()
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
		let mut entries = Vec::with_capacity(value.stats.len());
		for entry in value.stats {
			match entry {
				__RawStatEntry::Depth0(entry) => entries.push(entry),
				__RawStatEntry::Depth1(entry) => {
					entries.reserve(entry.splits.len());
					for entry in entry.splits {
						entries.push(__ParsedStatEntry::from(entry));
					}
				},
			}
		}
		Self {
			entries
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
	splits: SmallVec<Value, 1>,
}

#[doc(hidden)]
#[derive(Debug, Error)]
pub enum MakeStatSplitsError<S: Stat> {
	#[error("Failed to deserialize json into split type ({name}): {0}", name = core::any::type_name::<S>())]
	FailedPartialDeserialize(serde_json::Error),
	// FailedPartialDeserialize(serde_path_to_error::Error<serde_json::Error>),
	#[error("Failed to deserialize splits into greater split type ({name}): {0}", name = core::any::type_name::<S>())]
	FailedFullDeserialize(S::TryFromSplitWrappedError),
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
		let partially_deserialized_is_empty = partially_deserialized.is_empty();
		match <S as Stat>::from_split_wrapped(partially_deserialized) {
			Ok(s) => Ok(S::default()),
			Err(_) if partially_deserialized_is_empty => Ok(S::default()),
			Err(e) => Err(MakeStatSplitsError::FailedFullDeserialize(e)),
		}
	} else {
		Ok(S::default())
	}
}

impl SingletonWrappedEntryStat for () {}

#[derive(Debug, PartialEq, Eq, Clone, Default, Deref, DerefMut)]
pub struct Multiple<T: SingletonWrappedEntryStat> {
	pub entries: Vec<T>,
}

impl<T: SingletonWrappedEntryStat> Stat for Multiple<T> {
	type SplitWrappedVariant = T;
	type TryFromSplitWrappedError = Infallible;

	fn from_split_wrapped(split_wrapped: Vec<Self::SplitWrappedVariant>) -> Result<Self, Self::TryFromSplitWrappedError>
	where
		Self: Sized
	{
		Ok(Self { entries: split_wrapped })
	}
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone, Deref, DerefMut)]
#[serde(bound = "T: SingletonWrappedEntryStat")]
#[serde(rename_all = "camelCase")]
pub struct Career<T: SingletonWrappedEntryStat> {
	pub team: Option<NamedTeam>,
	pub player: NamedPerson,
	pub league: Option<NamedLeague>,
	pub sport: Option<SportId>,
	pub game_type: GameType,
	#[deref]
	#[deref_mut]
	#[serde(rename = "stat")]
	pub stats: T,
}

impl<T: SingletonWrappedEntryStat> Default for Career<T> {
	fn default() -> Self {
		Self {
			team: None,
			player: NamedPerson::unknown_person(),
			league: None,
			sport: None,
			game_type: GameType::default(),
			stats: T::default(),
		}
	}
}

impl<T: SingletonWrappedEntryStat> SingletonWrappedEntryStat for Career<T> {

}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone, Deref, DerefMut)]
#[serde(bound = "T: SingletonWrappedEntryStat")]
pub struct Season<T: SingletonWrappedEntryStat> {
	pub season: SeasonId,
	#[deref]
	#[deref_mut]
	#[serde(rename = "stat")]
	pub stats: T,
}

impl<T: SingletonWrappedEntryStat> Default for Season<T> {
	fn default() -> Self {
		Self {
			season: SeasonId::current_season(),
			stats: T::default(),
		}
	}
}

impl<T: SingletonWrappedEntryStat> SingletonWrappedEntryStat for Season<T> {

}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone, Deref, DerefMut)]
#[serde(bound = "T: SingletonWrappedEntryStat")]
pub struct Team<T: SingletonWrappedEntryStat> {
	#[deref]
	#[deref_mut]
	#[serde(rename = "stat")]
	stats: T,
	pub team: NamedTeam,
}

impl<T: SingletonWrappedEntryStat> Default for Team<T> {
	fn default() -> Self {
		Self {
			stats: T::default(),
			team: NamedTeam::unknown_team(),
		}
	}
}

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct MultipleSeasons<T: SingletonWrappedEntryStat> {
	pub seasons: FxHashMap<SeasonId, Season<T>>,
}

#[derive(Debug, Error)]
pub enum MultipleSeasonsFromSplitWrappedVariantError {
	#[error("Duplicate entry for season {season} found")]
	DuplicateEntry { season: SeasonId },
}

impl<T: SingletonWrappedEntryStat> Stat for MultipleSeasons<T> {
	type SplitWrappedVariant = Season<T>;
	type TryFromSplitWrappedError = MultipleSeasonsFromSplitWrappedVariantError;

	fn from_split_wrapped(split_wrapped: Vec<Self::SplitWrappedVariant>) -> Result<Self, Self::TryFromSplitWrappedError>
	where
		Self: Sized
	{
		let mut this = Self { seasons: FxHashMap::default() };
		for season in split_wrapped {
			match this.seasons.entry(season.season) {
				Entry::Occupied(_) => return Err (Self::TryFromSplitWrappedError::DuplicateEntry { season: season.season }),
				Entry::Vacant(slot) => slot.insert(season),
			};
		}
		Ok(this)
	}
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone, Deref, DerefMut)]
#[serde(rename_all = "camelCase")]
#[serde(bound = "T: SingletonWrappedEntryStat")]
pub struct Game<T: SingletonWrappedEntryStat> {
	#[deref]
	#[deref_mut]
	#[serde(flatten)]
	stats: Season<T>,
	pub opponent: NamedTeam,
	pub date: NaiveDate,
	pub is_home: bool,
	pub game: GameId,
}

impl<T: SingletonWrappedEntryStat> Default for Game<T> {
	fn default() -> Self {
		Self {
			stats: Season::default(),
			opponent: NamedTeam::unknown_team(),
			date: Utc::now().date_naive(),
			is_home: true,
			game: GameId::new(0),
		}
	}
}

impl<T: SingletonWrappedEntryStat> SingletonWrappedEntryStat for Game<T> {}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone, Deref, DerefMut)]
#[serde(rename_all = "camelCase")]
#[serde(bound = "T: SingletonWrappedEntryStat")]
pub struct Player<T: SingletonWrappedEntryStat> {
	#[deref]
	#[deref_mut]
	#[serde(flatten)]
	stats: Season<T>,
	pub player: NamedPerson,
	pub game_type: GameType,
}

impl<T: SingletonWrappedEntryStat> Default for Player<T> {
	fn default() -> Self {
		Self {
			stats: Season::default(),
			player: NamedPerson::unknown_person(),
			game_type: GameType::default(),
		}
	}
}

impl<T: SingletonWrappedEntryStat> SingletonWrappedEntryStat for Player<T> {}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone, Deref, DerefMut)]
#[serde(rename_all = "camelCase")]
#[serde(bound = "T: SingletonWrappedEntryStat")]
pub struct SingleMatchup<T: SingletonWrappedEntryStat> {
	#[deref]
	#[deref_mut]
	#[serde(flatten)]
	stats: Game<T>,
	pub pitcher: NamedPerson,
	pub batter: NamedPerson,
}

impl<T: SingletonWrappedEntryStat> Default for SingleMatchup<T> {
	fn default() -> Self {
		Self {
			stats: Game::default(),
			pitcher: NamedPerson::unknown_person(),
			batter: NamedPerson::unknown_person(),
		}
	}
}

impl<T: SingletonWrappedEntryStat> SingletonWrappedEntryStat for SingleMatchup<T> {}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone, Deref, DerefMut)]
#[serde(rename_all = "camelCase")]
#[serde(bound = "T: SingletonWrappedEntryStat")]
pub struct AccumulatedMatchup<T: SingletonWrappedEntryStat> {
	#[deref]
	#[deref_mut]
	#[serde(flatten)]
	pub stats: Team<T>,
	#[serde(rename = "opponent")]
	pub opposing_team: NamedTeam,
	pub game_type: GameType,
}

impl<T: SingletonWrappedEntryStat> Default for AccumulatedMatchup<T> {
	fn default() -> Self {
		Self {
			stats: T::default(),
			opposing_team: NamedTeam::unknown_team(),
			game_type: GameType::default(),
		}
	}
}

impl<T: SingletonWrappedEntryStat> SingletonWrappedEntryStat for AccumulatedMatchup<T> {}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone, Deref, DerefMut)]
#[serde(rename_all = "camelCase")]
#[serde(bound = "T: SingletonWrappedEntryStat")]
pub struct AccumulatedVsPlayerMatchup<T: SingletonWrappedEntryStat> {
	#[deref]
	#[deref_mut]
	#[serde(flatten)]
	stats: AccumulatedMatchup<T>,
	pub pitcher: NamedPerson,
	pub batter: NamedPerson,
}

impl<T: SingletonWrappedEntryStat> Default for AccumulatedVsPlayerMatchup<T> {
	fn default() -> Self {
		Self {
			stats: AccumulatedMatchup::<T>::default(),
			pitcher: NamedPerson::unknown_person(),
			batter: NamedPerson::unknown_person(),
		}
	}
}

impl<T: SingletonWrappedEntryStat> SingletonWrappedEntryStat for AccumulatedVsPlayerMatchup<T> {}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone, Deref, DerefMut)]
#[serde(rename_all = "camelCase")]
#[serde(bound = "T: SingletonWrappedEntryStat")]
pub struct AccumulatedVsTeamTotalMatchup<T: SingletonWrappedEntryStat> {
	#[deref]
	#[deref_mut]
	#[serde(flatten)]
	stats: AccumulatedMatchup<T>,
	pub rank: usize,
	pub batter: NamedPerson,
}

impl<T: SingletonWrappedEntryStat> Default for AccumulatedVsTeamTotalMatchup<T> {
	fn default() -> Self {
		Self {
			stats: AccumulatedMatchup::<T>::default(),
			rank: 0,
			batter: NamedPerson::unknown_person(),
		}
	}
}

impl<T: SingletonWrappedEntryStat> SingletonWrappedEntryStat for AccumulatedVsTeamTotalMatchup<T> {}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone, Deref, DerefMut)]
#[serde(rename_all = "camelCase")]
#[serde(bound = "T: SingletonWrappedEntryStat")]
pub struct AccumulatedVsTeamSeasonalPitcherSplit<T: SingletonWrappedEntryStat> {
	#[deref]
	#[deref_mut]
	#[serde(flatten)]
	stats: AccumulatedMatchup<Season<T>>,
	pub rank: usize,
	pub pitcher: NamedPerson,
	pub batter: NamedPerson,
}

impl<T: SingletonWrappedEntryStat> Default for AccumulatedVsTeamSeasonalPitcherSplit<T> {
	fn default() -> Self {
		Self {
			stats: AccumulatedMatchup::<Season<T>>::default(),
			rank: 0,
			pitcher: NamedPerson::unknown_person(),
			batter: NamedPerson::unknown_person(),
		}
	}
}

impl<T: SingletonWrappedEntryStat> SingletonWrappedEntryStat for AccumulatedVsTeamSeasonalPitcherSplit<T> {}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FieldedMatchup {
	pub pitcher: NamedPerson,
	pub batter: NamedPerson,
	pub fielding_team: NamedTeam,
}

impl Default for FieldedMatchup {
	fn default() -> Self {
		Self {
			pitcher: NamedPerson::unknown_person(),
			batter: NamedPerson::unknown_person(),
			fielding_team: NamedTeam::unknown_team(),
		}
	}
}

impl SingletonWrappedEntryStat for FieldedMatchup {}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone, Deref, DerefMut)]
#[serde(rename_all = "camelCase")]
#[serde(bound = "T: SingletonWrappedEntryStat")]
pub struct Month<T: SingletonWrappedEntryStat> {
	#[deref]
	#[deref_mut]
	#[serde(flatten)]
	stats: Season<T>,
	pub month: chrono::Month,
}

impl<T: SingletonWrappedEntryStat> Default for Month<T> {
	fn default() -> Self {
		Self {
			stats: Season::default(),
			month: chrono::Month::January,
		}
	}
}

impl<T: SingletonWrappedEntryStat> SingletonWrappedEntryStat for Month<T> {}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone, Deref, DerefMut)]
#[serde(rename_all = "camelCase")]
#[serde(bound = "T: SingletonWrappedEntryStat")]
pub struct Weekday<T: SingletonWrappedEntryStat> {
	#[deref]
	#[deref_mut]
	#[serde(flatten)]
	stats: Season<T>,
	#[serde(deserialize_with = "deserialize_day_of_week")]
	pub weekday: chrono::Weekday,
}

impl<T: SingletonWrappedEntryStat> Default for Weekday<T> {
	fn default() -> Self {
		Self {
			stats: Season::default(),
			weekday: chrono::Weekday::Mon,
		}
	}
}

impl<T: SingletonWrappedEntryStat> SingletonWrappedEntryStat for Weekday<T> {}

fn deserialize_day_of_week<'de, D: Deserializer<'de>>(deserializer: D) -> Result<chrono::Weekday, D::Error> {
	struct WeekdayVisitor;

	impl Visitor<'_> for WeekdayVisitor {
		type Value = chrono::Weekday;

		fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
			formatter.write_str("an integer between 0 and 6 representing the day of the week")
		}

		#[allow(clippy::cast_sign_loss, reason = "needlessly pedantic")]
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
#[serde(bound = "T: SingletonWrappedEntryStat")]
#[doc(hidden)]
pub struct __HomeOrAwayStruct<T: SingletonWrappedEntryStat> {
	#[serde(flatten)]
	stats: Season<T>,
	is_home: bool,
}

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct HomeAndAway<T: SingletonWrappedEntryStat> {
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

impl<T: SingletonWrappedEntryStat> Stat for HomeAndAway<T> {
	type SplitWrappedVariant = __HomeOrAwayStruct<T>;
	type TryFromSplitWrappedError = HomeAndAwayFromSplitWrappedVariantError;

	fn from_split_wrapped(split_wrapped: Vec<Self::SplitWrappedVariant>) -> Result<Self, Self::TryFromSplitWrappedError>
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
#[serde(bound = "T: SingletonWrappedEntryStat")]
#[doc(hidden)]
pub struct __WinOrLossStruct<T: SingletonWrappedEntryStat> {
	#[serde(flatten)]
	stats: Season<T>,
	is_win: bool,
}

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct WinLoss<T: SingletonWrappedEntryStat> {
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

impl<T: SingletonWrappedEntryStat> Stat for WinLoss<T> {
	type SplitWrappedVariant = __WinOrLossStruct<T>;
	type TryFromSplitWrappedError = WinLossFromSplitWrappedVariantError;

	fn from_split_wrapped(split_wrapped: Vec<Self::SplitWrappedVariant>) -> Result<Self, Self::TryFromSplitWrappedError>
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

impl SingletonWrappedEntryStat for ExpectedStats {}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename = "camelCase")]
pub struct SprayChart {
	#[serde(rename = "stat")]
	spray: HitSpray,
	batter: NamedPerson,
}

impl Default for SprayChart {
	fn default() -> Self {
		Self {
			spray: HitSpray::default(),
			batter: NamedPerson::unknown_person(),
		}
	}
}

impl SingletonWrappedEntryStat for SprayChart {}

#[allow(clippy::struct_field_names, reason = "is a piece")]
#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename = "camelCase")]
pub struct HitSpray {
	left_field: PercentageStat,
	left_center_field: PercentageStat,
	center_field: PercentageStat,
	right_center_field: PercentageStat,
	right_field: PercentageStat,
}

impl Default for HitSpray {
	fn default() -> Self {
		Self {
			left_field: PercentageStat::new(0.0),
			left_center_field: PercentageStat::new(0.0),
			center_field: PercentageStat::new(0.0),
			right_center_field: PercentageStat::new(0.0),
			right_field: PercentageStat::new(0.0),
		}
	}
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

#[allow(non_snake_case)]
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
	type SplitWrappedVariant = __HotColdZonesEntryStruct;
	type TryFromSplitWrappedError = HotColdZonesFromSplitWrappedError;

	fn from_split_wrapped(split_wrapped: Vec<Self::SplitWrappedVariant>) -> Result<Self, Self::TryFromSplitWrappedError>
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
	type SplitWrappedVariant = __HotColdZonesEntryStruct;
	type TryFromSplitWrappedError = HotColdZonesFromSplitWrappedError;

	fn from_split_wrapped(split_wrapped: Vec<Self::SplitWrappedVariant>) -> Result<Self, Self::TryFromSplitWrappedError>
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

#[derive(Debug, Deserialize, PartialEq, Eq, Clone, Default)]
pub struct PlayStat {
	// pub play: Play,
}

// todo: replace with real struct once game stuff is implemented
pub type PitchStat = ();

impl SingletonWrappedEntryStat for PlayStat {}

impl<T: Stat> Stat for Option<T> {
	type SplitWrappedVariant = T::SplitWrappedVariant;
	type TryFromSplitWrappedError = Infallible;

	fn from_split_wrapped(split_wrapped: Vec<Self::SplitWrappedVariant>) -> Result<Self, Self::TryFromSplitWrappedError>
	where
		Self: Sized
	{
		Ok(T::from_split_wrapped(split_wrapped).ok())
	}
}

pub mod stat_types {
	use super::*;

	macro_rules! stat_type_stats {
		($name:ident {
			$hitting:ty,
			$pitching:ty,
			$fielding:ty,
			$catching:ty $(,)?
		}) => {
			pub struct $name;

			impl StatTypeStats for $name {
				type Hitting = stat_type_stats!(@ $name :: Hitting = $hitting);
				type Pitching = stat_type_stats!(@ $name :: Pitching = $pitching);
				type Fielding = stat_type_stats!(@ $name :: Fielding = $fielding);
				type Catching = stat_type_stats!(@ $name :: Catching = $catching);
			}
		};
		(raw $name:ident {
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
		(@ $name:ident :: $group:ident = !) => { () };
		(@ $name:ident :: $group:ident = self) => { ::pastey::paste!($crate::stats::groups::[<$group:snake>]::$name) };
		(@ $name:ident :: $group:ident = $($wrapper:ident <)* $last_wrapper:ident $($t:tt)*) => { ::pastey::paste!($($wrapper <)* $last_wrapper <$crate::stats::groups::[<$group:snake>]::$name> $($t)*) };
	}

	stat_type_stats!(Projected { Season<Player>, Season<Player>, !, ! });
	stat_type_stats!(YearByYear { MultipleSeasons, MultipleSeasons, MultipleSeasons, MultipleSeasons });
	stat_type_stats!(YearByYearAdvanced { MultipleSeasons, MultipleSeasons, !, ! });
	stat_type_stats!(Season { Season, Season, Season, Season });
	stat_type_stats!(Career { Career, Career, Career, Career });
	stat_type_stats!(SeasonAdvanced { Season, Season, Season, Season });
	stat_type_stats!(CareerAdvanced { Career, Career, Career, Career });
	stat_type_stats!(GameLog { Multiple<Game>, Multiple<Game>, Multiple<Game>, Multiple<Game> });
	stat_type_stats!(raw PlayLog { Multiple<SingleMatchup<PlayStat>>, Multiple<SingleMatchup<PlayStat>>, Multiple<SingleMatchup<PlayStat>>, Multiple<SingleMatchup<PlayStat>> });
	stat_type_stats!(raw PitchLog { Multiple<SingleMatchup<PitchStat>>, Multiple<SingleMatchup<PitchStat>>, Multiple<SingleMatchup<PitchStat>>, Multiple<SingleMatchup<PitchStat>> });
	// 'metricLog'?
	// 'metricAverages'?
	stat_type_stats!(raw PitchArsenal { Multiple<PitchUsage>, Multiple<PitchUsage>, (), () });
	// 'outsAboveAverage'?
	stat_type_stats!(ExpectedStatistics { Player, Player, !, ! });
	stat_type_stats!(Sabermetrics { Player, Player, !, ! });
	// stat_type_stats!(raw SprayChart { SprayChart, SprayChart, (), () }); // todo: does not have statGroup on the response
	// 'tracking'?
	// stat_type_stats!(VsPlayerStats { AccumulatedMatchup<VsPlayerHittingStats>, AccumulatedMatchup<VsPlayerPitchingStats>, (), () });
	// stat_type_stats!(VsPlayerTotalStats { AccumulatedMatchup<VsPlayerHittingStats>, AccumulatedMatchup<VsPlayerPitchingStats>, (), () });
	// stat_type_stats!(VsPlayer5Y { AccumulatedMatchup, AccumulatedMatchup, (), () });
	// stat_type_stats!(VsTeamStats { Multiple<AccumulatedVsTeamSeasonalPitcherSplit<HittingStats>>, (), (), () });
	// stat_type_stats!(VsTeam5YStats { Multiple<AccumulatedVsTeamSeasonalPitcherSplit<HittingStats>>, (), (), () });
	// stat_type_stats!(VsTeamTotalStats { AccumulatedVsTeamTotalMatchup<HittingStats>, (), (), () });
	stat_type_stats!(LastXGames { Team, Team, Team, Team });
	stat_type_stats!(ByDateRange { Team, Team, Team, Team });
	stat_type_stats!(ByDateRangeAdvanced { Team, Team, Team, Team });
	stat_type_stats!(ByMonth { Month, Month, Month, Month });
	stat_type_stats!(ByDayOfWeek { Weekday, Weekday, Weekday, Weekday });
	stat_type_stats!(HomeAndAway { HomeAndAway, HomeAndAway, HomeAndAway, HomeAndAway });
	stat_type_stats!(WinLoss { WinLoss, WinLoss, WinLoss, WinLoss });
	stat_type_stats!(Rankings { Season<Player<Team>>, Season<Player<Team>>, Season<Player<Team>>, Season<Player<Team>> });
	stat_type_stats!(RankingsByYear { MultipleSeasons<Player<Team>>, MultipleSeasons<Player<Team>>, MultipleSeasons<Player<Team>>, MultipleSeasons<Player<Team>> });
	stat_type_stats!(raw HotColdZones { HittingHotColdZones, PitchingHotColdZones, (), () });
	stat_type_stats!(OpponentsFaced { Multiple<FieldedMatchup>, Multiple<FieldedMatchup>, Multiple<FieldedMatchup>, Multiple<FieldedMatchup> });
	stat_type_stats!(StatSplits { Season, Season, Season, Season });
	stat_type_stats!(StatSplitsAdvanced { Season, Season, Season, Season });
	stat_type_stats!(AtGameStart { Multiple<Game>, Multiple<Game>, Multiple<Game>, Multiple<Game> });
	// 'vsOpponents'?
}
