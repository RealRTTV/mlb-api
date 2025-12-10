use crate::requests::stats::pieces::{fWARData, wOBAAccumulatedData, wOBARateData, AdvancedFieldOutsData, AdvancedHitsData, AtBatData, BaseOnBallsData, BaserunningRunValueData, BattingRunValueData, CatchersInterferenceData, ExtraBaseHitsData, FieldingRunValueData, FlyoutsData, GIDPData, GITPData, GamesPlayedData, HitByPitchData, IntentionalWalksData, LOBData, OpponentGIDPData, PickoffsData, PitchQuantityData, PlateAppearanceData, PositionalRunValueOffsetData, RARData, RBIData, ReachedOnErrorData, RunsData, SacrificeHitsData, FieldOutsData, HitsData, SpeedData, StealingData, StrikeoutsData, SwingDataData, TotalBallsInPlayData, TotalBasesData, UltimateBaserunningData, WeightedGroundIntoDoublePlayData, WeightedStolenBaseValueData};
use derive_more::{Add, AddAssign, AsRef};
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Eq, Clone, Add, AddAssign, Default, AsRef)]
#[serde(rename_all = "camelCase")]
pub struct HittingStats {
	#[serde(flatten)] games_played: GamesPlayedData,
	#[serde(flatten)] field_outs: FieldOutsData,
	#[serde(flatten)] runs: RunsData,
	#[serde(flatten)] extra_base_hits: ExtraBaseHitsData,
	#[serde(flatten)] strikeouts: StrikeoutsData,
	#[serde(flatten)] walks: BaseOnBallsData,
	#[serde(flatten)] intentional_walks: IntentionalWalksData,
	#[serde(flatten)] hits: HitsData,
	#[serde(flatten)] hit_by_pitch: HitByPitchData,
	#[serde(flatten)] at_bat: AtBatData,
	#[serde(flatten)] stealing: StealingData,
	#[serde(flatten)] gidp: GIDPData,
	#[serde(flatten)] pitch_quantity: PitchQuantityData,
	#[serde(flatten)] plate_appearance: PlateAppearanceData,
	#[serde(flatten)] rbi: RBIData,
	#[serde(flatten)] lob: LOBData,
	#[serde(flatten)] sacrifice_hits: SacrificeHitsData,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone, Add, AddAssign, Default, AsRef)]
pub struct SimplifiedGameLogHittingStats {
	#[serde(flatten)] games_played: GamesPlayedData,
	#[serde(flatten)] field_outs: FieldOutsData,
	#[serde(flatten)] flyouts: FlyoutsData,
	#[serde(flatten)] runs: RunsData,
	#[serde(flatten)] extra_base_hits: ExtraBaseHitsData,
	#[serde(flatten)] strikeouts: StrikeoutsData,
	#[serde(flatten)] base_on_balls: BaseOnBallsData,
	#[serde(flatten)] intentional_walks: IntentionalWalksData,
	#[serde(flatten)] hits: HitsData,
	#[serde(flatten)] hit_by_pitch: HitByPitchData,
	#[serde(flatten)] at_bat: AtBatData,
	#[serde(flatten)] stealing: StealingData,
	#[serde(flatten)] gidp: GIDPData,
	#[serde(flatten)] gitp: GITPData,
	#[serde(flatten)] plate_appearance: PlateAppearanceData,
	#[serde(flatten)] rbi: RBIData,
	#[serde(flatten)] lob: LOBData,
	#[serde(flatten)] sacrifice_hits: SacrificeHitsData,
	#[serde(flatten)] catchers_interference: CatchersInterferenceData,
	#[serde(flatten)] pickoffs: PickoffsData,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone, Add, AddAssign, Default, AsRef)]
pub struct VsPlayerHittingStats {
	#[serde(flatten)] games_played: GamesPlayedData,
	#[serde(flatten)] field_outs: FieldOutsData,
	#[serde(flatten)] extra_base_hits: ExtraBaseHitsData,
	#[serde(flatten)] strikeouts: StrikeoutsData,
	#[serde(flatten)] base_on_balls: BaseOnBallsData,
	#[serde(flatten)] intentional_walks: IntentionalWalksData,
	#[serde(flatten)] hits: HitsData,
	#[serde(flatten)] hit_by_pitch: HitByPitchData,
	#[serde(flatten)] at_bat: AtBatData,
	#[serde(flatten)] gidp: GIDPData,
	#[serde(flatten)] plate_appearance: PlateAppearanceData,
	#[serde(flatten)] rbi: RBIData,
	#[serde(flatten)] lob: LOBData,
	#[serde(flatten)] sacrifice_hits: SacrificeHitsData,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone, Add, AddAssign, Default, AsRef)]
#[serde(rename_all = "camelCase")]
pub struct AdvancedHittingStats {
	#[serde(flatten)] plate_appearance: PlateAppearanceData,
	#[serde(flatten)] total_bases: TotalBasesData,
	#[serde(flatten)] lob: LOBData,
	#[serde(flatten)] sacrifice_hits: SacrificeHitsData,
	#[serde(flatten)] extra_base_hits: ExtraBaseHitsData,
	#[serde(flatten)] hit_by_pitch: HitByPitchData,
	#[serde(flatten)] gidp: GIDPData,
	#[serde(flatten)] opponent_gidp: OpponentGIDPData,
	#[serde(flatten)] pitch_quantity: PitchQuantityData,
	#[serde(flatten)] reached_on_error: ReachedOnErrorData,
	#[serde(flatten)] swing_data: SwingDataData,
	#[serde(flatten)] total_balls_in_play: TotalBallsInPlayData,
	#[serde(flatten)] flyouts: FlyoutsData,
	#[serde(flatten)] advanced_field_outs: AdvancedFieldOutsData,
	#[serde(flatten)] advanced_hits: AdvancedHitsData,
}

#[derive(Debug, Deserialize, PartialEq, Clone, Default, AsRef)]
#[allow(non_snake_case)]
pub struct SabermetricsHittingStats {
	#[serde(flatten)] wOBARate: wOBARateData,
	#[serde(flatten)] wOBAAccumulated: wOBAAccumulatedData,
	#[serde(flatten)] RAR: RARData,
	#[serde(flatten)] fWAR: fWARData,
	#[serde(flatten)] batting_run_value: BattingRunValueData,
	#[serde(flatten)] fielding_run_value: FieldingRunValueData,
	#[serde(flatten)] baserunning_run_value: BaserunningRunValueData,
	#[serde(flatten)] positional_run_value_offset: PositionalRunValueOffsetData,
	#[serde(flatten)] SPD: SpeedData,
	#[serde(flatten)] UBR: UltimateBaserunningData,
	#[serde(flatten)] wGDP: WeightedGroundIntoDoublePlayData,
	#[serde(flatten)] wSB: WeightedStolenBaseValueData,
}

impl Eq for SabermetricsHittingStats {}
