//! # Stat Type
//! Describes different "types" (effectively splits) of statistics, seasonal, career, etc.
//!
//! The full list can be found via:
//! ```
//! mlb_api::meta::MetaRequest::<mlb_api::stat_types::StatType>::new()
//!     .get()
//!     .await?
//!     .entries
//! ```
//!
//! The current list of valid [`StatType`]s for statistic-request purposes is:
//! - `Projected` (likely ZIPS projections)
//! - `YearByYear` - `Season`al splits in a [`HashMap<SeasonId, WithSeason<_>>`]
//! - `YearByYearAdvanced` - `Season`al splits in a [`HashMap<SeasonId, WithSeason<_>>`]
//! - `Season` - [`WithSeason<_>`]
//! - `Career` - [`Career<_>`]
//! - `SeasonAdvanced`
//! - `CareerAdvanced`
//! - `GameLog` - [`Vec<WithGame<_>>`]
//! - `PlayLog` - [`Vec<Play_>`]
//! - `PitchLog` - [`Vec<SingleMatchup<PitchStat>>`]
//! - `ExpectedStatistics` (`xAVG`, `xwOBA`, etc.)
//! - `Sabermetrics` (`xFIP`, `fWAR`, etc.)
//! - `VsPlayer5Y` - [`AccumulatedVsPlayerMatchup<_>`]
//! - `LastXGames` - [`WithTeam<_>`]
//! - `ByDateRange` - [`WithTeam<_>`]
//! - `ByDateRangeAdvanced` - [`WithTeam<_>`]
//! - `ByMonth` - [`WithMonth<_>`]
//! - `ByDayOfWeek` - [`WithWeekday<_>`]
//! - `HomeAndAway` - [`WithHomeAndAway<_>`]
//! - `WinLoss` - [`WithWinLoss<_>`]
//! - `OpponentsFaced` - [`FieldedMatchup`]
//! - `StatSplits` - [`WithSeason<_>`]
//! - `StatSplitsAdvanced` - [`WithSeason<_>`]
//!
//! [`HashMap<SeasonId, WithSeason<_>>`]: crate::stats::wrappers::WithSeason
//! [`WithSeason<_>`]: crate::stats::wrappers::WithSeason
//! [`Career<_>`]: crate::stats::wrappers::Career
//! [`Vec<WithGame<_>>`]: crate::stats::wrappers::WithGame
//! [`Vec<SingleMatchup<PitchStat>>`]: crate::stats::wrappers::SingleMatchup
//! [`Vec<PitchUsage>`]: crate::stats::raw::PitchUsage
//! [`AccumulatedVsPlayerMatchup<_>`]: crate::stats::wrappers::AccumulatedVsPlayerMatchup
//! [`WithTeam<_>`]: crate::stats::wrappers::WithTeam
//! [`WithMonth<_>`]: crate::stats::wrappers::WithMonth
//! [`WithWeekday<_>`]: crate::stats::wrappers::WithWeekday
//! [`WithHomeAndAway<_>`]: crate::stats::wrappers::WithHomeAndAway
//! [`WithWinLoss<_>`]: crate::stats::wrappers::WithWinLoss
//! [`HittingHotColdZones`]: crate::stats::raw::HittingHotColdZones
//! [`PitchingHotColdZones`]: crate::stats::raw::PitchingHotColdZones
//! [`FieldedMatchup`]: crate::stats::raw::FieldedMatchup

id!(StatType { displayName: String });

meta_kind_impl!("statTypes" => StatType);
static_request_entry_cache_impl!(StatType);
test_impl!(StatType);
