//! # The Stats API
//!
//! The second most likely reason you're here.
//!
//! [`mlb_api`](crate)'s stats system designed to be simple to use.
//! Create a stats type, then create the hydrations which have stats in it, then request it.
//!
//! Almost all the types here are for private use and occasionally might be
//!
//! ## Examples
//! See [`stats_type!`](crate::stats_type) for examples on how to use the macro.
//!
//! ## Notes
//! 1. The stat type registry is admittedly incomplete, only some stat types are implemented (see [`stat_types`]), more will come in the future.
//! 2. Some stats are only implemented for specific [`StatGroup`](crate::meta::StatGroup)s, if you have a complicated request such as:
//! ```
//! stats_type! {
//!     pub struct TechnicalStats {
//!         [Sabermetrics, Career] = [Hitting, Pitching, Fielding, Catching]
//!     }
//! }
//! ```
//! `stats.sabermetrics.fielding` and `.catching` will be of type `()`.
//! 3. It is an intentional decision that [`SituationCode`](crate::meta::SituationCode)s
//! are not registered in an enum as if new cutting-edge situation-codes come out,
//! this API being outdated shouldn't limit in that factor.

#![allow(clippy::trait_duplication_in_bounds, reason = "serde")]

use crate::hydrations::Hydrations;
use serde::de::DeserializeOwned;
use serde::Deserialize;
use std::convert::Infallible;
use std::fmt::Debug;

#[doc(hidden)]
pub mod macros;
#[doc(hidden)]
pub mod raw;
#[doc(hidden)]
pub mod wrappers;
pub mod leaders;
#[doc(hidden)]
mod units;
#[doc(hidden)]
pub mod parse;
pub mod derived;

pub use units::*;

#[cfg(test)]
mod tests;

// pub use derived::*;

#[doc(hidden)]
pub trait Stats: 'static + Debug + PartialEq + Eq + Clone + Hydrations {}

impl Stats for () {}

pub trait Stat: Debug + Clone + PartialEq + Eq + Default {
	type Split: DeserializeOwned;

	type TryFromSplitError;

	/// # Errors
	/// See [`Self::TryFromSplitError`]
	fn from_splits(splits: impl Iterator<Item=Self::Split>) -> Result<Self, Self::TryFromSplitError> where Self: Sized;
}

/// Represents the types defined in [`raw`], not the wrapped final types. In the serialized format, this represents the `stat` field.
pub trait RawStat: Debug + DeserializeOwned + Clone + Eq + Default {}

impl RawStat for () {}
impl SingletonSplitStat for () {}

/// Represents types that are made from a single 'split' in the serialized format (able to be deserialized)
pub trait SingletonSplitStat: Debug + DeserializeOwned + Clone + PartialEq + Eq + Default {

}

impl<T: SingletonSplitStat> Stat for T {
	type Split = Self;

	type TryFromSplitError = &'static str;

	fn from_splits(mut splits: impl Iterator<Item=Self::Split>) -> Result<Self, Self::TryFromSplitError>
	where
		Self: Sized
	{
		splits.next().ok_or("length of stat splits is not >= 1, cannot convert to unit type.")
	}
}

#[allow(unused)]
pub(crate) trait StatTypeStats {
	type Hitting: Stat;

	type Pitching: Stat;

	type Fielding: Stat;

	type Catching: Stat;
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone, Default)]
pub struct PlayStat {
	// pub play: Play,
}

// todo: replace with real struct once game stuff is implemented
pub type PitchStat = ();

impl RawStat for PlayStat {}

impl<T: Stat> Stat for Option<T> {
	type Split = T::Split;
	type TryFromSplitError = Infallible;

	fn from_splits(splits: impl Iterator<Item=Self::Split>) -> Result<Self, Self::TryFromSplitError>
	where
		Self: Sized
	{
		Ok(T::from_splits(splits).ok())
	}
}

#[doc(hidden)]
pub mod stat_types {
	use super::{StatTypeStats, PlayStat, PitchStat};
	use crate::stats::raw::{catching, fielding, hitting, pitching, FieldedMatchup};
	use crate::stats::wrappers::{AccumulatedVsPlayerMatchup, ByMonth, ByPosition, BySeason, ByWeekday, Career, Map, Map2D, SingleMatchup, WithGame, WithHomeAndAway, WithMonth, WithPlayer, WithPositionAndSeason, WithSeason, WithTeam, WithWeekday, WithWinLoss};

	macro_rules! stat_type_stats {
		($name:ident {
			$hitting:ty,
			$pitching:ty,
			$catching:ty,
			$fielding:ty $(,)?
		}) => {
			::pastey::paste! {
				#[doc(hidden)]
				pub struct [<__ $name StatTypeStats>];

				impl StatTypeStats for [<__ $name StatTypeStats>] {
					type Hitting = $hitting;
					type Pitching = $pitching;
					type Fielding = $fielding;
					type Catching = $catching;
				}
			}
		};
	}

	// NOTES
	// 1. Make sure all modules are correct, `hitting`, `pitching`, `catching`, then `fielding`.

	stat_type_stats!(Projected { WithPlayer<hitting::__ProjectedStatsData>, WithPlayer<pitching::__ProjectedStatsData>, (), () });
	stat_type_stats!(YearByYear { Map<WithSeason<hitting::__YearByYearStatsData>, BySeason>, Map<WithSeason<pitching::__YearByYearStatsData>, BySeason>, Map<WithSeason<catching::__YearByYearStatsData>, BySeason>, Map2D<WithPositionAndSeason<fielding::__YearByYearStatsData>, BySeason, ByPosition> });
	stat_type_stats!(YearByYearAdvanced { Map<WithSeason<hitting::__YearByYearAdvancedStatsData>, BySeason>, Map<WithSeason<pitching::__YearByYearAdvancedStatsData>, BySeason>, (), () });
	stat_type_stats!(Season { WithSeason<hitting::__SeasonStatsData>, WithSeason<pitching::__SeasonStatsData>, WithSeason<catching::__SeasonStatsData>, Map2D<WithPositionAndSeason<fielding::__SeasonStatsData>, BySeason, ByPosition> });
	stat_type_stats!(Career { Career<hitting::__CareerStatsData>, Career<pitching::__CareerStatsData>, Career<catching::__CareerStatsData>, Career<fielding::__CareerStatsData> });
	stat_type_stats!(SeasonAdvanced { WithSeason<hitting::__SeasonAdvancedStatsData>, WithSeason<pitching::__SeasonAdvancedStatsData>, (), () });
	stat_type_stats!(CareerAdvanced { Career<hitting::__CareerAdvancedStatsData>, Career<pitching::__CareerAdvancedStatsData>, (), () });
	stat_type_stats!(GameLog { Vec<WithGame<hitting::__GameLogStatsData>>, Vec<WithGame<pitching::__GameLogStatsData>>, Vec<WithGame<catching::__GameLogStatsData>>, Vec<WithGame<fielding::__GameLogStatsData>> });
	stat_type_stats!(PlayLog { Vec<SingleMatchup<PlayStat>>, Vec<SingleMatchup<PlayStat>>, Vec<SingleMatchup<PlayStat>>, Vec<SingleMatchup<PlayStat>> });
	stat_type_stats!(PitchLog { Vec<SingleMatchup<PitchStat>>, Vec<SingleMatchup<PitchStat>>, Vec<SingleMatchup<PitchStat>>, Vec<SingleMatchup<PitchStat>> });
	// 'metricLog'?
	// 'metricAverages'?
	// stat_type_stats!(PitchArsenal { Vec<PitchUsage>, Vec<PitchUsage>, (), () }); // has no stat group
	// 'outsAboveAverage'?
	stat_type_stats!(ExpectedStatistics { WithPlayer<hitting::__ExpectedStatisticsStatsData>, WithPlayer<pitching::__ExpectedStatisticsStatsData>, (), () });
	stat_type_stats!(Sabermetrics { WithPlayer<hitting::__SabermetricsStatsData>, WithPlayer<pitching::__SabermetricsStatsData>, (), () });
	// stat_type_stats!(SprayChart { SprayChart, SprayChart, (), () }); // does not have statGroup on the response
	// 'tracking'?
	// stat_type_stats!(VsPlayer { AccumulatedMatchup<VsPlayerHittingStats>, AccumulatedMatchup<VsPlayerPitchingStats>, (), () });
	// stat_type_stats!(VsPlayerTotal { AccumulatedMatchup<VsPlayerHittingStats>, AccumulatedMatchup<VsPlayerPitchingStats>, (), () });
	stat_type_stats!(VsPlayer5Y { AccumulatedVsPlayerMatchup<hitting::__VsPlayerStatsData>, AccumulatedVsPlayerMatchup<pitching::__VsPlayerStatsData>, (), () });
	// stat_type_stats!(VsTeam { Multiple<AccumulatedVsTeamSeasonalPitcherSplit<HittingStats>>, (), (), () });
	// stat_type_stats!(VsTeam5Y { Multiple<AccumulatedVsTeamSeasonalPitcherSplit<HittingStats>>, (), (), () });
	// stat_type_stats!(VsTeamTotal { AccumulatedVsTeamTotalMatchup<HittingStats>, (), (), () });
	stat_type_stats!(LastXGames { WithTeam<hitting::__LastXGamesStatsData>, WithTeam<pitching::__LastXGamesStatsData>, WithTeam<catching::__LastXGamesStatsData>, WithTeam<fielding::__LastXGamesStatsData> });
	stat_type_stats!(ByDateRange { WithTeam<hitting::__ByDateRangeStatsData>, WithTeam<pitching::__ByDateRangeStatsData>, WithTeam<catching::__ByDateRangeStatsData>, WithTeam<fielding::__ByDateRangeStatsData> });
	stat_type_stats!(ByDateRangeAdvanced { WithTeam<hitting::__ByDateRangeAdvancedStatsData>, WithTeam<pitching::__ByDateRangeAdvancedStatsData>, WithTeam<catching::__ByDateRangeAdvancedStatsData>, WithTeam<fielding::__ByDateRangeAdvancedStatsData> });
	stat_type_stats!(ByMonth { Map<WithMonth<hitting::__ByMonthStatsData>, ByMonth>, Map<WithMonth<pitching::__ByMonthStatsData>, ByMonth>, Map<WithMonth<catching::__ByMonthStatsData>, ByMonth>, Map<WithMonth<fielding::__ByMonthStatsData>, ByMonth> });
	stat_type_stats!(ByDayOfWeek { Map<WithWeekday<hitting::__ByDayOfWeekStatsData>, ByWeekday>, Map<WithWeekday<pitching::__ByDayOfWeekStatsData>, ByWeekday>, Map<WithWeekday<catching::__ByDayOfWeekStatsData>, ByWeekday>, Map<WithWeekday<fielding::__ByDayOfWeekStatsData>, ByWeekday> });
	stat_type_stats!(HomeAndAway { WithHomeAndAway<hitting::__HomeAndAwayStatsData>, WithHomeAndAway<pitching::__HomeAndAwayStatsData>, WithHomeAndAway<catching::__HomeAndAwayStatsData>, WithHomeAndAway<fielding::__HomeAndAwayStatsData> });
	stat_type_stats!(WinLoss { WithWinLoss<hitting::__WinLossStatsData>, WithWinLoss<pitching::__WinLossStatsData>, WithWinLoss<catching::__WinLossStatsData>, WithWinLoss<fielding::__WinLossStatsData> });
	// stat_type_stats!(Rankings { WithPlayerAndTeam<hitting::__RankingsStatsData>, WithPlayerAndTeam<pitching::__RankingsStatsData>, (), () });
	// stat_type_stats!(RankingsByYear { Map<WithPlayerAndTeam<hitting::__RankingsByYearStatsData>, BySeason>, Map<WithPlayerAndTeam<pitching::__RankingsByYearStatsData>, BySeason>, (), () });
	// stat_type_stats!(HotColdZones { HittingHotColdZones, PitchingHotColdZones, (), () }); // has no stat group
	stat_type_stats!(OpponentsFaced { Vec<FieldedMatchup>, Vec<FieldedMatchup>, Vec<FieldedMatchup>, Vec<FieldedMatchup> });
	stat_type_stats!(StatSplits { WithSeason<hitting::__StatSplitsStatsData>, WithSeason<pitching::__StatSplitsStatsData>, (), () });
	stat_type_stats!(StatSplitsAdvanced { WithSeason<hitting::__StatSplitsAdvancedStatsData>, WithSeason<pitching::__StatSplitsAdvancedStatsData>, (), () });
	// stat_type_stats!(AtGameStart { Multiple<WithGame<hitting::AtGameStart>>, Multiple<WithGame<pitching::AtGameStart>>, Multiple<WithGame<catching::AtGameStart>>, Multiple<WithGame<fielding::AtGameStart>> });
	// 'vsOpponents'?
}
