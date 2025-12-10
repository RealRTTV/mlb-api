use derive_more::{Add, AddAssign, AsRef};
use serde::Deserialize;
use crate::requests::PitchType;
use crate::requests::stats::units::PercentageStat;
use crate::requests::stats::BaseStat;
use crate::requests::stats::pieces::{bWARData, fWARData, xFIPData, AtBatData, BalksData, BaseOnBallsData, BattersFacedData, CatchersInterferenceData, CompleteGamesData, DecisionsData, ERAMinusData, EarnedRunsData, ExtraBaseHitsData, FIPData, FIPMinusData, FlyoutsData, GIDPData, GamesFinishedData, GamesPitchedData, GamesPlayedData, GamesStartedData, HitByPitchData, InheritedRunnersData, IntentionalWalksData, InningsPitchedData, PassedBallData, PickoffsData, PitchQuantityData, RARData, RBIData, RunsData, SacrificeHitsData, ShutdownsAndMeltdownsData, FieldOutsData, HitsData, StealingData, StrikeoutsData, StrikesData, WildPitchData};

#[derive(Debug, Deserialize, PartialEq, Eq, Clone, Add, AddAssign, Default, AsRef)]
#[serde(rename_all = "camelCase")]
pub struct PitchingStats {
	#[serde(flatten)] field_outs: FieldOutsData,
	#[serde(flatten)] at_bats: AtBatData,
	#[serde(flatten)] batters_faced: BattersFacedData,
	#[serde(flatten)] base_on_balls: BaseOnBallsData,
	#[serde(flatten)] stealing: StealingData,
	#[serde(flatten)] extra_base_hits: ExtraBaseHitsData,
	#[serde(flatten)] games_played: GamesPlayedData,
	#[serde(flatten)] catchers_interference: CatchersInterferenceData,
	#[serde(flatten)] hit_by_pitch_data: HitByPitchData,
	#[serde(flatten)] hits: HitsData,
	#[serde(flatten)] gidp: GIDPData,
	#[serde(flatten)] intentional_walks: IntentionalWalksData,
	#[serde(flatten)] pitch_quantity: PitchQuantityData,
	#[serde(flatten)] strikes: StrikesData,
	#[serde(flatten)] pickoffs: PickoffsData,
	#[serde(flatten)] sacrifice_hits: SacrificeHitsData,
	#[serde(flatten)] strikeouts: StrikeoutsData,
	#[serde(flatten)] runs: RunsData,
	#[serde(flatten)] earned_runs: EarnedRunsData,
	#[serde(flatten)] decisions: DecisionsData,
	#[serde(flatten)] games_pitched: GamesPitchedData,
	#[serde(flatten)] games_started: GamesStartedData,
	#[serde(flatten)] games_finished: GamesFinishedData,
	#[serde(flatten)] innings_pitched: InningsPitchedData,
	#[serde(flatten)] balks: BalksData,
	#[serde(flatten)] wild_pitch: WildPitchData,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone, Add, AddAssign, Default)]
#[serde(rename_all = "camelCase")]
pub struct SimplifiedGameLogPitchingStats {
	#[serde(flatten)] games_played: GamesPlayedData,
	#[serde(flatten)] games_started: GamesStartedData,
	#[serde(flatten)] flyouts: FlyoutsData,
	#[serde(flatten)] field_outs: FieldOutsData,
	#[serde(flatten)] runs: RunsData,
	#[serde(flatten)] extra_base_hits: ExtraBaseHitsData,
	#[serde(flatten)] strikeouts: StrikeoutsData,
	#[serde(flatten)] base_on_balls: BaseOnBallsData,
	#[serde(flatten)] intentional_walks: IntentionalWalksData,
	#[serde(flatten)] hits: HitsData,
	#[serde(flatten)] hit_by_pitch: HitByPitchData,
	#[serde(flatten)] at_bat: AtBatData,
	#[serde(flatten)] stealing: StealingData,
	#[serde(flatten)] pitch_quantity: PitchQuantityData,
	#[serde(flatten)] decisions: DecisionsData,
	#[serde(flatten)] earned_runs: EarnedRunsData,
	#[serde(flatten)] batters_faced: BattersFacedData,
	#[serde(flatten)] innings_pitched: InningsPitchedData,
	#[serde(flatten)] games_pitched: GamesPitchedData,
	#[serde(flatten)] complete_games: CompleteGamesData,
	#[serde(flatten)] strikes: StrikesData,
	#[serde(flatten)] balks: BalksData,
	#[serde(flatten)] wild_pitch: WildPitchData,
	#[serde(flatten)] pickoffs: PickoffsData,
	#[serde(flatten)] rbi: RBIData,
	#[serde(flatten)] games_finished: GamesFinishedData,
	#[serde(flatten)] catchers_interference: CatchersInterferenceData,
	#[serde(flatten)] sacrifice_hits: SacrificeHitsData,
	#[serde(flatten)] passed_ball: PassedBallData,
	#[serde(flatten)] inherited_runners: InheritedRunnersData,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone, Add, AddAssign, Default)]
#[serde(rename_all = "camelCase")]
pub struct VsPlayerPitchingStats {
	#[serde(flatten)] games_played_data: GamesPlayedData,
	#[serde(flatten)] field_outs_data: FieldOutsData,
	#[serde(flatten)] extra_base_hits_data: ExtraBaseHitsData,
	#[serde(flatten)] strikeouts_data: StrikeoutsData,
	#[serde(flatten)] base_on_balls_data: BaseOnBallsData,
	#[serde(flatten)] intentional_walks_data: IntentionalWalksData,
	#[serde(flatten)] hits_data: HitsData,
	#[serde(flatten)] hit_by_pitch_data: HitByPitchData,
	#[serde(flatten)] at_bat_data: AtBatData,
	#[serde(flatten)] gidp_data: GIDPData,
	#[serde(flatten)] pitch_quantity: PitchQuantityData,
	#[serde(flatten)] innings_pitched_data: InningsPitchedData,
	#[serde(flatten)] batters_faced_data: BattersFacedData,
	#[serde(flatten)] games_pitched_data: GamesPitchedData,
	#[serde(flatten)] strikes_data: StrikesData,
	#[serde(flatten)] balks_data: BalksData,
	#[serde(flatten)] wild_pitch_data: WildPitchData,
	#[serde(flatten)] rbi_data: RBIData,
	#[serde(flatten)] catchers_interference_data: CatchersInterferenceData,
	#[serde(flatten)] sacrifice_hits_data: SacrificeHitsData,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(from = "__PitchUsageStruct")]
pub struct PitchUsage {
	pub count: u32,
	pub total_pitches: u32,
	pub average_speed: uom::si::f64::Velocity,
	pub pitch_type: PitchType,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct __PitchUsageStruct {
	count: u32,
	total_pitches: u32,
	average_speed: f64,
	pitch_type: PitchType,
}

impl From<__PitchUsageStruct> for PitchUsage {
	fn from(value: __PitchUsageStruct) -> Self {
		Self {
			count: value.count,
			total_pitches: value.total_pitches,
			average_speed: uom::si::f64::Velocity::new::<uom::si::velocity::mile_per_hour>(value.average_speed),
			pitch_type: value.pitch_type,
		}
	}
}

impl Eq for PitchUsage {}

impl BaseStat for PitchUsage {}

impl PitchUsage {
	/// Percentage of total pitches that are this pitch.
	#[must_use]
	pub fn pct(&self) -> PercentageStat {
		(self.count as f64 / self.total_pitches as f64).into()
	}
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone, Add, AddAssign, Default)]
#[serde(rename_all = "camelCase")]
pub struct AdvancedPitchingStats {
	pub batters_faced: u32,
	pub inherited_runners: u32,
	pub inherited_runners_scored: u32,
	pub bequeathed_runners: u32,
	pub bequeathed_runners_scored: u32,
	pub stolen_bases: u32,
	pub caught_stealing: u32,
	pub quality_starts: u32,
	pub games_finished: u32,
	pub doubles: u32,
	pub triples: u32,
	#[serde(rename = "gidp")]
	pub ground_into_double_play: u32,
	#[serde(rename = "gidpOpp")]
	pub ground_into_double_play_opponent: u32,
	pub wild_pitches: u32,
	pub balks: u32,
	pub pickoffs: u32,
	pub total_swings: u32,
	pub whiffs: u32,
	pub bunts_failed: u32,
	pub bunts_missed_tipped: u32,
	pub balls_in_play: u32,
	pub run_support: u32,
	#[serde(rename = "flyOuts")]
	pub flyouts: u32,
	#[serde(rename = "popOuts")]
	pub popouts: u32,
	#[serde(rename = "lineOuts")]
	pub lineouts: u32,
	#[serde(rename = "groundOuts")]
	pub groundouts: u32,
	#[serde(rename = "flyHits")]
	pub flyball_hits: u32,
	#[serde(rename = "popHits")]
	pub popfly_hits: u32,
	#[serde(rename = "lineHits")]
	pub line_drive_hits: u32,
	#[serde(rename = "groundHits")]
	pub groundball_hits: u32,
}

#[derive(Debug, Deserialize, PartialEq, Clone, Default, AsRef)]
#[serde(rename_all = "camelCase")]
#[allow(non_snake_case)]
pub struct SabermetricsPitchingStats {
	#[serde(flatten)] FIP: FIPData,
	#[serde(flatten)] xFIP: xFIPData,
	#[serde(flatten)] FIP_minus: FIPMinusData,
	#[serde(flatten)] bWAR: bWARData,
	#[serde(flatten)] RAR: RARData,
	#[serde(flatten)] fWAR: fWARData,
	#[serde(flatten)] shutdowns_and_meltdowns: ShutdownsAndMeltdownsData,
	#[serde(flatten)] era_minus: ERAMinusData,
}

impl Eq for SabermetricsPitchingStats {}
