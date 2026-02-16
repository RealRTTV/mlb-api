#![allow(clippy::trait_duplication_in_bounds, reason = "serde")]

use crate::hydrations::Hydrations;
use serde::de::DeserializeOwned;
use serde::Deserialize;
use std::convert::Infallible;
use std::fmt::Debug;

pub mod macros;
pub mod raw;
pub mod wrappers;
// pub mod pieces;
// pub mod piece_impls;
pub mod leaders;
pub mod units;
pub mod parse;

pub trait Stats: 'static + Debug + PartialEq + Eq + Clone + Hydrations {}

impl Stats for () {}

pub trait Stat: Debug + Clone + PartialEq + Eq + Default {
	type Split: DeserializeOwned;

	type TryFromSplitError;

	/// # Errors
	/// See [`Self::TryFromSplitError`]
	fn from_splits(splits: impl Iterator<Item=Self::Split>) -> Result<Self, Self::TryFromSplitError> where Self: Sized;
}

/// Represents the types defined in [`raw`], not the wrapped final types. In the serialized format, this represents the `stat' field.
pub trait RawStat: Debug + DeserializeOwned + Clone + Eq + Default {}

impl RawStat for () {}
impl SingletonSplitStat for () {}

/// Represents types that are made from a single 'split' in the serialized format (able to be deserialized)
pub trait SingletonSplitStat: Debug + DeserializeOwned + Clone + PartialEq + Eq + Default {

}

impl<T: SingletonSplitStat> Stat for T {
	type Split = Self;

	type TryFromSplitError = &'static str;

	fn from_splits(splits: impl Iterator<Item=Self::Split>) -> Result<Self, Self::TryFromSplitError>
	where
		Self: Sized
	{
		<Vec<Self> as TryInto<[Self; 1]>>::try_into(splits.collect())
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
	use super::*;
	use crate::stats::raw::{catching, fielding, hitting, pitching, FieldedMatchup, HittingHotColdZones, PitchingHotColdZones, PitchUsage};
	use crate::stats::wrappers::{AccumulatedVsPlayerMatchup, ByPosition, BySeason, Career, Map, Map2D, SingleMatchup, WithGame, WithHomeAndAway, WithMonth, WithPlayer, WithPositionAndSeason, WithSeason, WithTeam, WithWeekday, WithWinLoss};

	macro_rules! stat_type_stats {
		($name:ident {
			$hitting:ty,
			$pitching:ty,
			$fielding:ty,
			$catching:ty $(,)?
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
	stat_type_stats!(PitchArsenal { Vec<PitchUsage>, Vec<PitchUsage>, (), () });
	// 'outsAboveAverage'?
	stat_type_stats!(ExpectedStatistics { WithPlayer<hitting::__ExpectedStatisticsStatsData>, WithPlayer<pitching::__ExpectedStatisticsStatsData>, (), () });
	stat_type_stats!(Sabermetrics { WithPlayer<hitting::__SabermetricsStatsData>, WithPlayer<pitching::__SabermetricsStatsData>, (), () });
	// stat_type_stats!(SprayChart { SprayChart, SprayChart, (), () }); // does not have statGroup on the response
	// 'tracking'?
	// stat_type_stats!(VsPlayerStats { AccumulatedMatchup<VsPlayerHittingStats>, AccumulatedMatchup<VsPlayerPitchingStats>, (), () });
	// stat_type_stats!(VsPlayerTotalStats { AccumulatedMatchup<VsPlayerHittingStats>, AccumulatedMatchup<VsPlayerPitchingStats>, (), () });
	stat_type_stats!(VsPlayer5Y { AccumulatedVsPlayerMatchup<hitting::__VsPlayerStatsData>, AccumulatedVsPlayerMatchup<pitching::__VsPlayerStatsData>, (), () });
	// stat_type_stats!(VsTeamStats { Multiple<AccumulatedVsTeamSeasonalPitcherSplit<HittingStats>>, (), (), () });
	// stat_type_stats!(VsTeam5YStats { Multiple<AccumulatedVsTeamSeasonalPitcherSplit<HittingStats>>, (), (), () });
	// stat_type_stats!(VsTeamTotalStats { AccumulatedVsTeamTotalMatchup<HittingStats>, (), (), () });
	stat_type_stats!(LastXGames { WithTeam<hitting::__LastXGamesStatsData>, WithTeam<pitching::__LastXGamesStatsData>, WithTeam<catching::__LastXGamesStatsData>, WithTeam<fielding::__LastXGamesStatsData> });
	stat_type_stats!(ByDateRange { WithTeam<hitting::__ByDateRangeStatsData>, WithTeam<pitching::__ByDateRangeStatsData>, WithTeam<catching::__ByDateRangeStatsData>, WithTeam<fielding::__ByDateRangeStatsData> });
	stat_type_stats!(ByDateRangeAdvanced { WithTeam<hitting::__ByDateRangeAdvancedStatsData>, WithTeam<pitching::__ByDateRangeAdvancedStatsData>, WithTeam<catching::__ByDateRangeAdvancedStatsData>, WithTeam<fielding::__ByDateRangeAdvancedStatsData> });
	stat_type_stats!(ByMonth { WithMonth<hitting::__ByMonthStatsData>, WithMonth<pitching::__ByMonthStatsData>, WithMonth<catching::__ByMonthStatsData>, WithMonth<fielding::__ByMonthStatsData> });
	stat_type_stats!(ByDayOfWeek { WithWeekday<hitting::__ByDayOfWeekStatsData>, WithWeekday<pitching::__ByDayOfWeekStatsData>, WithWeekday<catching::__ByDayOfWeekStatsData>, WithWeekday<fielding::__ByDayOfWeekStatsData> });
	stat_type_stats!(HomeAndAway { WithHomeAndAway<hitting::__HomeAndAwayStatsData>, WithHomeAndAway<pitching::__HomeAndAwayStatsData>, WithHomeAndAway<catching::__HomeAndAwayStatsData>, WithHomeAndAway<fielding::__HomeAndAwayStatsData> });
	stat_type_stats!(WinLoss { WithWinLoss<hitting::__WinLossStatsData>, WithWinLoss<pitching::__WinLossStatsData>, WithWinLoss<catching::__WinLossStatsData>, WithWinLoss<fielding::__WinLossStatsData> });
	// stat_type_stats!(Rankings { WithPlayerAndTeam<hitting::__RankingsStatsData>, WithPlayerAndTeam<pitching::__RankingsStatsData>, (), () });
	// stat_type_stats!(RankingsByYear { Map<WithPlayerAndTeam<hitting::__RankingsByYearStatsData>, BySeason>, Map<WithPlayerAndTeam<pitching::__RankingsByYearStatsData>, BySeason>, (), () });
	stat_type_stats!(HotColdZones { HittingHotColdZones, PitchingHotColdZones, (), () });
	stat_type_stats!(OpponentsFaced { Vec<FieldedMatchup>, Vec<FieldedMatchup>, Vec<FieldedMatchup>, Vec<FieldedMatchup> });
	stat_type_stats!(StatSplits { WithSeason<hitting::__StatSplitsStatsData>, WithSeason<pitching::__StatSplitsStatsData>, (), () });
	stat_type_stats!(StatSplitsAdvanced { WithSeason<hitting::__StatSplitsAdvancedStatsData>, WithSeason<pitching::__StatSplitsAdvancedStatsData>, (), () });
	// stat_type_stats!(AtGameStart { Multiple<WithGame<hitting::AtGameStart>>, Multiple<WithGame<pitching::AtGameStart>>, Multiple<WithGame<catching::AtGameStart>>, Multiple<WithGame<fielding::AtGameStart>> });
	// 'vsOpponents'?
}
