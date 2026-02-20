#![allow(unused_variables)]

use chrono::NaiveDate;
use crate::person::PersonRequest;
use crate::request::{RequestURL, RequestURLBuilderExt};
use crate::{person_hydrations, stats_type};
use crate::meta::{StandingsType, SituationCodeId};
use crate::stats::units::ThreeDecimalPlaceRateStat;

#[tokio::test]
async fn shohei_ohtani_pitching_2025() {
	stats_type! {
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
			] = [Pitching]
		}
	}

	person_hydrations! {
		struct PitchingStatsHydrations {
			stats: PitchingStats,
		}
	}

	let request = PersonRequest::<PitchingStatsHydrations>::builder()
		.id(660271)
		.hydrations(PitchingStatsHydrations::builder()
			.stats(PitchingStats::builder().season(2025).build())
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
	stats_type! {
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
			] = [Hitting]
		}
	}

	person_hydrations! {
		struct HittingStatsHydrations {
			stats: HittingStats,
		}
	}

	let request = PersonRequest::<HittingStatsHydrations>::builder()
		.id(660271)
		.hydrations(HittingStatsHydrations::builder()
			.stats(HittingStats::builder().season(2025)))
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
	stats_type! {
		struct HittingStats {
			[
				VsPlayer5Y,
				LastXGames,
				ByDateRange,
				ByDateRangeAdvanced,
				StatSplits,
				StatSplitsAdvanced,
			] = [Hitting]
		}
	}

	person_hydrations! {
		struct HittingStatsHydrations {
			stats: HittingStats,
		}
	}

	let request = PersonRequest::<HittingStatsHydrations>::builder()
		.id(660271)
		.hydrations(HittingStatsHydrations::builder()
			.stats(HittingStats::builder()
				.season(2025)
				.opponent_player(453286)
				.game_type(StandingsType::RegularSeason)
				.games_back(5)
				.date_range(NaiveDate::from_ymd_opt(2025, 5, 5).unwrap()..=NaiveDate::from_ymd_opt(2025, 6, 12).unwrap())
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
	stats_type! {
		struct PitchingStats {
			[
				VsPlayer5Y,
				LastXGames,
				ByDateRange,
				ByDateRangeAdvanced,
				StatSplits,
				StatSplitsAdvanced,
			] = [Pitching]
		}
	}

	person_hydrations! {
		struct PitchingStatsHydrations {
			stats: PitchingStats,
		}
	}

	let request = PersonRequest::<PitchingStatsHydrations>::builder()
		.id(660271)
		.hydrations(PitchingStatsHydrations::builder()
			.stats(PitchingStats::builder()
				.season(2025)
				.opponent_player(672386)
				.game_type(StandingsType::RegularSeason)
				.games_back(5)
				.date_range(NaiveDate::from_ymd_opt(2025, 5, 5).unwrap()..=NaiveDate::from_ymd_opt(2025, 6, 12).unwrap())
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
	stats_type! {
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
			] = [Fielding]
		}
	}

	person_hydrations! {
		struct FieldingStatsHydrations {
			stats: FieldingStats,
		}
	}

	let response = PersonRequest::<FieldingStatsHydrations>::builder()
		.id(662139)
		.hydrations(FieldingStatsHydrations::builder().stats(FieldingStats::builder().season(2025)))
		.build_and_get()
		.await
		.unwrap();
}

#[tokio::test]
async fn daulton_varsho_fielding_2025_custom() {
	stats_type! {
		struct FieldingStats {
			[
				LastXGames,
				ByDateRange,
				ByDateRangeAdvanced,
			] = [Fielding]
		}
	}

	person_hydrations! {
		struct FieldingStatsHydrations {
			stats: FieldingStats,
		}
	}

	let response = PersonRequest::<FieldingStatsHydrations>::builder()
		.id(662139)
		.hydrations(FieldingStatsHydrations::builder().stats(FieldingStats::builder()
			.season(2025)
			.games_back(10)
			.date_range(NaiveDate::from_ymd_opt(2025, 6, 1).unwrap()..=NaiveDate::from_ymd_opt(2025, 8, 1).unwrap())
		))
		.build_and_get()
		.await
		.unwrap();
}

#[tokio::test]
async fn alejandro_kirk_catching_2025() {
	stats_type! {
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
			] = [Catching]
		}
	}

	person_hydrations! {
		struct CatchingStatsHydrations {
			stats: CatchingStats,
		}
	}

	let response = PersonRequest::<CatchingStatsHydrations>::builder()
		.id(672386)
		.hydrations(CatchingStatsHydrations::builder().stats(CatchingStats::builder().season(2025)))
		.build_and_get()
		.await
		.unwrap();
}

#[tokio::test]
async fn alejandro_kirk_catching_2025_custom() {
	stats_type! {
		struct CatchingStats {
			[
				LastXGames,
				ByDateRange,
				ByDateRangeAdvanced,
			] = [Catching]
		}
	}

	person_hydrations! {
		struct CatchingStatsHydrations {
			stats: CatchingStats,
		}
	}

	let response = PersonRequest::<CatchingStatsHydrations>::builder()
		.id(672386)
		.hydrations(CatchingStatsHydrations::builder().stats(CatchingStats::builder().season(2025).date_range(NaiveDate::from_ymd_opt(2025, 6, 1).unwrap()..=NaiveDate::from_ymd_opt(2025, 8, 1).unwrap()).games_back(10)))
		.build_and_get()
		.await
		.unwrap();
}
