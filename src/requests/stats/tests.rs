#![allow(unused_variables)]

use chrono::NaiveDate;
use crate::person::PersonRequest;
use crate::request::{RequestURL, RequestURLBuilderExt};
use crate::{TEST_YEAR, person_hydrations, single_stat, stats_hydrations};
use crate::meta::{GameType, SituationCodeId};
use crate::stats::units::ThreeDecimalPlaceRateStat;

#[tokio::test]
async fn shohei_ohtani_pitching_2025() {
	stats_hydrations! {
		struct PitchingStats {
			[
                Projected,
                YearByYear,
                YearByYearAdvanced,
                Season,
                Career,
				SeasonAdvanced,
                CareerAdvanced,
				GameLog,
				PlayLog,
				ExpectedStatistics,
				Sabermetrics,
				HomeAndAway,
				WinLoss,
				ByDayOfWeek,
				ByMonth,
				OpponentsFaced,
			] + [Pitching]
		}
	}

	person_hydrations! {
		struct PitchingStatsHydrations {
			stats: PitchingStats,
		}
	}

	let request = PersonRequest::<PitchingStatsHydrations>::builder()
		.id(660_271)
		.hydrations(PitchingStatsHydrations::builder()
			.stats(PitchingStats::builder().season(TEST_YEAR).build())
			.build())
		.build();

	dbg!(request.to_string());

	let response = request
		.get()
		.await
		.unwrap()
		.people;

	// dbg!(&response[0].extras.stats);
}

#[tokio::test]
async fn shohei_ohtani_hitting_2025() {
	stats_hydrations! {
		struct HittingStats {
			[
				Projected,
                YearByYear,
                YearByYearAdvanced,
                Season,
                Career,
				SeasonAdvanced,
                CareerAdvanced,
				GameLog,
				PlayLog,
				ExpectedStatistics,
				Sabermetrics,
				HomeAndAway,
				WinLoss,
				ByDayOfWeek,
				ByMonth,
				OpponentsFaced,
			] + [Hitting]
		}
	}

	person_hydrations! {
		struct HittingStatsHydrations {
			stats: HittingStats,
		}
	}

	let request = PersonRequest::<HittingStatsHydrations>::builder()
		.id(660_271)
		.hydrations(HittingStatsHydrations::builder()
			.stats(HittingStats::builder().season(TEST_YEAR)))
		.build();

	dbg!(request.to_string());

	let response = request
		.get()
		.await
		.unwrap()
		.people;

	// dbg!(&response[0].extras.stats);

	let stats = &response[0].extras.stats;

	dbg!(ThreeDecimalPlaceRateStat::new(stats.season.hitting.hits.unwrap_or_default() as f64 / stats.season.hitting.at_bats.unwrap_or_default() as f64));
	dbg!(stats.sabermetrics.hitting.wOBA.unwrap_or_default());
}

#[tokio::test]
async fn shohei_ohtani_hitting_2025_custom() {
	stats_hydrations! {
		struct HittingStats {
			[
				VsPlayer5Y,
				LastXGames,
				ByDateRange,
				ByDateRangeAdvanced,
				StatSplits,
				StatSplitsAdvanced,
			] + [Hitting]
		}
	}

	person_hydrations! {
		struct HittingStatsHydrations {
			stats: HittingStats,
		}
	}

	let request = PersonRequest::<HittingStatsHydrations>::builder()
		.id(660_271)
		.hydrations(HittingStatsHydrations::builder()
			.stats(HittingStats::builder()
				.season(TEST_YEAR)
				.opponent_player(453_286)
				.game_type(GameType::RegularSeason)
				.games_back(5)
				.date_range(NaiveDate::from_ymd_opt(TEST_YEAR as _, 5, 5).unwrap()..=NaiveDate::from_ymd_opt(TEST_YEAR as _, 6, 12).unwrap())
				.situations(vec![SituationCodeId::new("c00")])))
		.build();

	dbg!(request.to_string());

	let response = request
		.get()
		.await
		.unwrap()
		.people;
}

#[tokio::test]
async fn shohei_ohtani_pitching_2025_custom() {
	stats_hydrations! {
		struct PitchingStats {
			[
				VsPlayer5Y,
				LastXGames,
				ByDateRange,
				ByDateRangeAdvanced,
				StatSplits,
				StatSplitsAdvanced,
			] + [Pitching]
		}
	}

	person_hydrations! {
		struct PitchingStatsHydrations {
			stats: PitchingStats,
		}
	}

	let request = PersonRequest::<PitchingStatsHydrations>::builder()
		.id(660_271)
		.hydrations(PitchingStatsHydrations::builder()
			.stats(PitchingStats::builder()
				.season(TEST_YEAR)
				.opponent_player(672386)
				.game_type(GameType::RegularSeason)
				.games_back(5)
				.date_range(NaiveDate::from_ymd_opt(TEST_YEAR as _, 5, 5).unwrap()..=NaiveDate::from_ymd_opt(TEST_YEAR as _, 6, 12).unwrap())
				.situations(vec![SituationCodeId::new("c00")])))
		.build();

	dbg!(request.to_string());

	let response = request
		.get()
		.await
		.unwrap()
		.people;
}

#[tokio::test]
async fn daulton_varsho_fielding_2025() {
	stats_hydrations! {
		struct FieldingStats {
			[
				YearByYear,
				Season,
				Career,
				GameLog,
				PlayLog,
				PitchLog,
				ByMonth,
				ByDayOfWeek,
				HomeAndAway,
				WinLoss,
				OpponentsFaced
			] + [Fielding]
		}
	}

	person_hydrations! {
		struct FieldingStatsHydrations {
			stats: FieldingStats,
		}
	}

	let response = PersonRequest::<FieldingStatsHydrations>::builder()
		.id(662_139)
		.hydrations(FieldingStatsHydrations::builder().stats(FieldingStats::builder().season(TEST_YEAR)))
		.build_and_get()
		.await
		.unwrap();
}

#[tokio::test]
async fn daulton_varsho_fielding_2025_custom() {
	stats_hydrations! {
		struct FieldingStats {
			[
				LastXGames,
				ByDateRange,
				ByDateRangeAdvanced,
			] + [Fielding]
		}
	}

	person_hydrations! {
		struct FieldingStatsHydrations {
			stats: FieldingStats,
		}
	}

	let response = PersonRequest::<FieldingStatsHydrations>::builder()
		.id(662_139)
		.hydrations(FieldingStatsHydrations::builder().stats(FieldingStats::builder()
			.season(TEST_YEAR)
			.games_back(10)
			.date_range(NaiveDate::from_ymd_opt(TEST_YEAR as _, 6, 1).unwrap()..=NaiveDate::from_ymd_opt(TEST_YEAR as _, 8, 1).unwrap())
		))
		.build_and_get()
		.await
		.unwrap();
}

#[tokio::test]
async fn alejandro_kirk_catching_2025() {
	stats_hydrations! {
		struct CatchingStats {
			[
				YearByYear,
				Season,
				Career,
				GameLog,
				PlayLog,
				PitchLog,
				ByMonth,
				ByDayOfWeek,
				HomeAndAway,
				WinLoss,
				OpponentsFaced
			] + [Catching]
		}
	}

	person_hydrations! {
		struct CatchingStatsHydrations {
			stats: CatchingStats,
		}
	}

	let response = PersonRequest::<CatchingStatsHydrations>::builder()
		.id(672_386)
		.hydrations(CatchingStatsHydrations::builder().stats(CatchingStats::builder().season(TEST_YEAR)))
		.build_and_get()
		.await
		.unwrap();
}

#[tokio::test]
async fn alejandro_kirk_catching_2025_custom() {
	stats_hydrations! {
		struct CatchingStats {
			[
				LastXGames,
				ByDateRange,
				ByDateRangeAdvanced,
			] + [Catching]
		}
	}

	person_hydrations! {
		struct CatchingStatsHydrations {
			stats: CatchingStats,
		}
	}

	let response = PersonRequest::<CatchingStatsHydrations>::builder()
		.id(672_386)
		.hydrations(CatchingStatsHydrations::builder().stats(CatchingStats::builder().season(TEST_YEAR).date_range(NaiveDate::from_ymd_opt(TEST_YEAR as _, 6, 1).unwrap()..=NaiveDate::from_ymd_opt(TEST_YEAR as _, 8, 1).unwrap()).games_back(10)))
		.build_and_get()
		.await
		.unwrap();
}

#[tokio::test]
async fn single_stat_tests() {
	let _stats = single_stat! { Career + Catching for 672_386 }.await.unwrap();
}
