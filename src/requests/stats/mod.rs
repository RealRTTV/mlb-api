#![allow(clippy::trait_duplication_in_bounds, reason = "serde")]

use crate::hydrations::Hydrations;
use crate::positions::NamedPosition;
use crate::season::SeasonId;
use serde::de::{DeserializeOwned, Error, Visitor};
use serde::{Deserialize, Deserializer};
use std::convert::Infallible;
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::ops::{Deref, DerefMut};
use std::str::FromStr;

pub mod macros;
pub mod raw;
pub mod wrappers;
pub mod pieces;
pub mod piece_impls;
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

pub trait RawStat: Debug + DeserializeOwned + Clone + Eq + Default {}

impl RawStat for () {}

impl<T: RawStat> SingletonSplitStat for T {}

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

pub mod stat_types {
	use super::*;
	use crate::stats::pieces::PitchUsage;
	use crate::stats::raw::{catching, fielding, hitting, pitching, FieldedMatchup, HittingHotColdZones, PitchingHotColdZones};
	use crate::stats::wrappers::{Career, Map, Map2D, SingleMatchup, WithGame, WithHomeAndAway, WithMonth, WithPlayer, WithPositionAndSeason, WithSeason, WithTeam, WithWeekday, WithWinLoss};

	macro_rules! stat_type_stats {
		($name:ident {
			$hitting:ty,
			$pitching:ty,
			$fielding:ty,
			$catching:ty $(,)?
		}) => {
			::pastey::paste! {
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
	
	stat_type_stats!(Projected { WithPlayer<hitting::__ProjectedStatsMarker>, WithPlayer<pitching::__ProjectedStatsMarker>, (), () });
	stat_type_stats!(YearByYear { Map<WithSeason<hitting::__YearByYearStatsMarker>, SeasonId>, Map<WithSeason<pitching::__YearByYearStatsMarker>, SeasonId>, Map<WithSeason<catching::__YearByYearStatsMarker>, SeasonId>, Map2D<WithPositionAndSeason<fielding::__YearByYearStatsMarker>, SeasonId, NamedPosition> });
	stat_type_stats!(YearByYearAdvanced { Map<WithSeason<hitting::__YearByYearAdvancedStatsMarker>, SeasonId>, Map<WithSeason<pitching::__YearByYearAdvancedStatsMarker>, SeasonId>, (), () });
	stat_type_stats!(Season { WithSeason<hitting::__SeasonStatsMarker>, WithSeason<pitching::__SeasonStatsMarker>, WithSeason<catching::__SeasonStatsMarker>, Map2D<WithPositionAndSeason<fielding::__SeasonStatsMarker>, SeasonId, NamedPosition> });
	stat_type_stats!(Career { Career<hitting::__CareerStatsMarker>, Career<pitching::__CareerStatsMarker>, Career<catching::__CareerStatsMarker>, Career<fielding::__CareerStatsMarker> });
	stat_type_stats!(SeasonAdvanced { WithSeason<hitting::__SeasonAdvancedStatsMarker>, WithSeason<pitching::__SeasonAdvancedStatsMarker>, (), () });
	stat_type_stats!(CareerAdvanced { Career<hitting::__CareerAdvancedStatsMarker>, Career<pitching::__CareerAdvancedStatsMarker>, (), () });
	stat_type_stats!(GameLog { Vec<WithGame<hitting::__GameLogStatsMarker>>, Vec<WithGame<pitching::__GameLogStatsMarker>>, Vec<WithGame<catching::__GameLogStatsMarker>>, Vec<WithGame<fielding::__GameLogStatsMarker>> });
	stat_type_stats!(PlayLog { Vec<SingleMatchup<PlayStat>>, Vec<SingleMatchup<PlayStat>>, Vec<SingleMatchup<PlayStat>>, Vec<SingleMatchup<PlayStat>> });
	stat_type_stats!(PitchLog { Vec<SingleMatchup<PitchStat>>, Vec<SingleMatchup<PitchStat>>, Vec<SingleMatchup<PitchStat>>, Vec<SingleMatchup<PitchStat>> });
	// 'metricLog'?
	// 'metricAverages'?
	stat_type_stats!(PitchArsenal { Vec<PitchUsage>, Vec<PitchUsage>, (), () });
	// 'outsAboveAverage'?
	stat_type_stats!(ExpectedStatistics { WithPlayer<hitting::__ExpectedStatisticsStatsMarker>, WithPlayer<pitching::__ExpectedStatisticsStatsMarker>, (), () });
	stat_type_stats!(Sabermetrics { WithPlayer<hitting::__SabermetricsStatsMarker>, WithPlayer<pitching::__SabermetricsStatsMarker>, (), () });
	// stat_type_stats!(raw SprayChart { SprayChart, SprayChart, (), () }); // does not have statGroup on the response
	// 'tracking'?
	// stat_type_stats!(VsPlayerStats { AccumulatedMatchup<VsPlayerHittingStats>, AccumulatedMatchup<VsPlayerPitchingStats>, (), () });
	// stat_type_stats!(VsPlayerTotalStats { AccumulatedMatchup<VsPlayerHittingStats>, AccumulatedMatchup<VsPlayerPitchingStats>, (), () });
	// stat_type_stats!(VsPlayer5Y { AccumulatedMatchup, AccumulatedMatchup, (), () });
	// stat_type_stats!(VsTeamStats { Multiple<AccumulatedVsTeamSeasonalPitcherSplit<HittingStats>>, (), (), () });
	// stat_type_stats!(VsTeam5YStats { Multiple<AccumulatedVsTeamSeasonalPitcherSplit<HittingStats>>, (), (), () });
	// stat_type_stats!(VsTeamTotalStats { AccumulatedVsTeamTotalMatchup<HittingStats>, (), (), () });
	stat_type_stats!(LastXGames { WithTeam<hitting::__LastXGamesStatsMarker>, WithTeam<pitching::__LastXGamesStatsMarker>, WithTeam<catching::__LastXGamesStatsMarker>, WithTeam<fielding::__LastXGamesStatsMarker> });
	stat_type_stats!(ByDateRange { WithTeam<hitting::__ByDateRangeStatsMarker>, WithTeam<pitching::__ByDateRangeStatsMarker>, WithTeam<catching::__ByDateRangeStatsMarker>, WithTeam<fielding::__ByDateRangeStatsMarker> });
	stat_type_stats!(ByDateRangeAdvanced { WithTeam<hitting::__ByDateRangeAdvancedStatsMarker>, WithTeam<pitching::__ByDateRangeAdvancedStatsMarker>, WithTeam<catching::__ByDateRangeAdvancedStatsMarker>, WithTeam<fielding::__ByDateRangeAdvancedStatsMarker> });
	stat_type_stats!(ByMonth { WithMonth<hitting::__ByMonthStatsMarker>, WithMonth<pitching::__ByMonthStatsMarker>, WithMonth<catching::__ByMonthStatsMarker>, WithMonth<fielding::__ByMonthStatsMarker> });
	stat_type_stats!(ByDayOfWeek { WithWeekday<hitting::__ByDayOfWeekStatsMarker>, WithWeekday<pitching::__ByDayOfWeekStatsMarker>, WithWeekday<catching::__ByDayOfWeekStatsMarker>, WithWeekday<fielding::__ByDayOfWeekStatsMarker> });
	stat_type_stats!(HomeAndAway { WithHomeAndAway<hitting::__HomeAndAwayStatsMarker>, WithHomeAndAway<pitching::__HomeAndAwayStatsMarker>, WithHomeAndAway<catching::__HomeAndAwayStatsMarker>, WithHomeAndAway<fielding::__HomeAndAwayStatsMarker> });
	stat_type_stats!(WinLoss { WithWinLoss<hitting::__WinLossStatsMarker>, WithWinLoss<pitching::__WinLossStatsMarker>, WithWinLoss<catching::__WinLossStatsMarker>, WithWinLoss<fielding::__WinLossStatsMarker> });
	stat_type_stats!(Rankings { WithSeason<WithPlayer<WithTeam<hitting::__RankingsStatsMarker>>>, WithSeason<WithPlayer<WithTeam<pitching::__RankingsStatsMarker>>>, (), () });
	stat_type_stats!(RankingsByYear { Map<WithSeason<WithPlayer<WithTeam<hitting::__RankingsByYearStatsMarker>>>, SeasonId>, Map<WithSeason<WithPlayer<WithTeam<pitching::__RankingsByYearStatsMarker>>>, SeasonId>, (), () });
	stat_type_stats!(HotColdZones { HittingHotColdZones, PitchingHotColdZones, (), () });
	stat_type_stats!(OpponentsFaced { Vec<FieldedMatchup>, Vec<FieldedMatchup>, Vec<FieldedMatchup>, Vec<FieldedMatchup> });
	stat_type_stats!(StatSplits { WithSeason<hitting::__StatSplitsStatsMarker>, WithSeason<pitching::__StatSplitsStatsMarker>, (), () });
	stat_type_stats!(StatSplitsAdvanced { WithSeason<hitting::__StatSplitsAdvancedStatsMarker>, WithSeason<pitching::__StatSplitsAdvancedStatsMarker>, (), () });
	// stat_type_stats!(AtGameStart { Multiple<WithGame<hitting::AtGameStart>>, Multiple<WithGame<pitching::AtGameStart>>, Multiple<WithGame<catching::AtGameStart>>, Multiple<WithGame<fielding::AtGameStart>> });
	// 'vsOpponents'?
}
