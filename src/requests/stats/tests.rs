#![allow(unused_variables)]

use chrono::NaiveDate;
use crate::person::PersonRequest;
use crate::request::RequestURL;
use crate::{person_hydrations, stats};
use crate::game_types::GameType;
use crate::situations::SituationCodeId;
use crate::stats::units::ThreeDecimalPlaceRateStat;

#[tokio::test]
async fn shohei_ohtani_pitching_2025() {
	stats! {
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
	stats! {
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
	stats! {
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
				.game_type(GameType::RegularSeason)
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
	stats! {
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
				.game_type(GameType::RegularSeason)
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
