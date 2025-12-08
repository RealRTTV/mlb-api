#![allow(non_snake_case, non_camel_case_types)]

use derive_more::{Add, AddAssign};
use serde::Deserialize;
use serde_with::serde_as;
use crate::endpoints::stats::units::InningsPitched;

macro_rules! piece {
    (
	    $(#[$struct_meta:meta])*
	    struct $struct_name:ident : $(#[$trait_meta:meta])* $trait_name:ident {
	    $(
	    $(#[$field_meta:meta])*
	    $field_vis:vis $field_name:ident : $field_type:ty
	    ),* $(,)?
    }) => {
	    $(#[$struct_meta])*
		#[derive(Debug, ::serde::Deserialize, PartialEq, Clone, Default)]
		#[serde(rename_all = "camelCase")]
	    pub(crate) struct $struct_name {
			$(
			$(#[$field_meta])*
			$field_vis $field_name : $field_type,
			)*
		}

	    impl Eq for $struct_name {}

	    $(#[$trait_meta])*
	    pub trait $trait_name {
			$(
			#[must_use]
			fn $field_name(&self) -> $field_type;
			)*
		}

	    impl<T: AsRef<$struct_name>> $trait_name for T {
			$(
			fn $field_name(&self) -> $field_type {
				self.as_ref().$field_name
			}
			)*
		}
    };
}

piece! {
	#[derive(Add, AddAssign)]
	struct GamesPlayedData: GamesPlayedPiece {
		pub games_played: u32,
	}
}

piece! {
	#[derive(Add, AddAssign)]
	struct GamesPitchedData: GamesPitchedPiece {
		pub games_pitched: u32,
	}
}

piece! {
	#[derive(Add, AddAssign)]
	struct GamesFinishedData: GamesFinishedPiece {
		pub games_finished: u32,
	}
}

piece! {
	#[derive(Add, AddAssign)]
	struct GamesStartedData: GamesStartedPiece {
		pub games_started: u32,
	}
}

piece! {
	#[derive(Add, AddAssign)]
	struct FieldOutsData: FieldOutsPiece {
		#[serde(rename = "groundOuts")]
		pub groundouts: u32,
		#[serde(rename = "airOuts")]
		pub airouts: u32,
	}
}

piece! {
	#[derive(Add, AddAssign)]
	struct FlyoutsData: FlyoutsPiece {
		#[serde(rename = "flyOuts")]
		pub flyouts: u32,
	}
}

piece! {
	#[derive(Add, AddAssign)]
	struct StrikeoutsData: StrikeoutsPiece {
		#[serde(rename = "strikeOuts")]
		pub strikeouts: u32,
	}
}

piece! {
	#[derive(Add, AddAssign)]
	struct HitsData: HitsPiece {
		pub hits: u32,
	}
}

piece! {
	#[derive(Add, AddAssign)]
	struct BaseOnBallsData: BaseOnBallsPiece {
		pub base_on_balls: u32,
	}
}

piece! {
	#[derive(Add, AddAssign)]
	struct IntentionalWalksData: IntentionalWalksPiece {
		pub intentional_walks: u32,
	}
}

piece! {
	#[derive(Add, AddAssign)]
	struct HitByPitchData: HitByPitchPiece {
		pub hit_by_pitch: u32,
	}
}

piece! {
	#[derive(Add, AddAssign)]
	struct ExtraBaseHitsData: ExtraBaseHitsPiece {
		pub doubles: u32,
		pub triples: u32,
		pub home_runs: u32,
	}
}

piece! {
	#[derive(Add, AddAssign)]
	struct AdvancedFieldOutsData: AdvancedFieldOutsPiece {
		#[serde(rename = "popOuts")]
		pub popouts: u32,
		#[serde(rename = "lineOuts")]
		pub lineouts: u32,
		#[serde(rename = "groundOuts")]
		pub groundouts: u32,
	}
}

piece! {
	#[derive(Add, AddAssign)]
	struct AdvancedHitsData: AdvancedHitsPiece {
		#[serde(rename = "flyHits")]
		pub flyball_hits: u32,
		#[serde(rename = "popHits")]
		pub popfly_hits: u32,
		#[serde(rename = "lineHits")]
		pub line_drive_hits: u32,
		#[serde(rename = "groundHits")]
		pub groundball_hits: u32,
	}
}

piece! {
	#[derive(Add, AddAssign)]
	struct StealingData: StealingPiece {
		pub caught_stealing: u32,
		pub stolen_bases: u32,
	}
}

piece! {
	#[derive(Add, AddAssign)]
	struct SacrificeHitsData: SacrificeHitsPiece {
		pub sac_bunts: u32,
		pub sac_flies: u32,
	}
}

piece! {
	#[derive(Add, AddAssign)]
	struct RunsData: RunsPiece {
		pub runs: u32,
	}
}

piece! {
	#[derive(Add, AddAssign)]
	struct EarnedRunsData: EarnedRunsPiece {
		pub earned_runs: u32,
	}
}

piece! {
	#[derive(Add, AddAssign)]
	struct InheritedRunnersData: InheritedRunnersPiece {
		pub inherited_runners: u32,
		pub inherited_runners_scored: u32,
	}
}

piece! {
	#[derive(Add, AddAssign)]
	struct PitchQuantityData: PitchQuantityPiece {
		#[serde(rename = "numberOfPitches")]
		pub num_pitches: u32,
	}
}

piece! {
	#[derive(Add, AddAssign)]
	struct StrikesData: StrikesPiece {
		pub strikes: u32,
	}
}

piece! {
	#[derive(Add, AddAssign)]
	struct SwingDataData: SwingDataPiece {
		#[serde(rename = "swingAndMisses")]
		pub whiffs: u32,
		pub total_swings: u32,
	}
}

piece! {
	#[derive(Add, AddAssign)]
	struct TotalBallsInPlayData: TotalBallsInPlayPiece {
		pub balls_in_play: u32,
	}
}

piece! {
	#[derive(Add, AddAssign)]
	struct ReachedOnErrorData: ReachedOnErrorPiece {
		pub reached_on_error: u32,
	}
}

piece! {
	#[derive(Add, AddAssign)]
	struct WalkoffsData: WalkoffsPiece {
		pub walkoffs: u32,
	}
}

piece! {
	#[derive(Add, AddAssign)]
	struct RBIData: RFieldiece {
		#[serde(rename = "rbi")]
		pub runs_batted_in: u32,
	}
}

piece! {
	#[derive(Add, AddAssign)]
	struct PickoffsData: PickoffsPiece {
		pub pickoffs: u32,
	}
}

piece! {
	#[derive(Add, AddAssign)]
	struct CatchersInterferenceData: CatchersInterferencePiece {
		pub catchers_interference: u32,
	}
}

piece! {
	#[derive(Add, AddAssign)]
	struct WildPitchData: WildPitchPiece {
		pub wild_pitches: u32,
	}
}

piece! {
	#[derive(Add, AddAssign)]
	struct PassedBallData: PassedBallPiece {
		#[serde(rename = "passedBall")]
		pub passed_balls: u32,
	}
}

piece! {
	#[derive(Add, AddAssign)]
	struct GIDPData: GIDPPiece {
		#[serde(rename = "groundedIntoDoublePlay", alias = "gidp")]
		pub ground_into_double_play: u32,
	}
}

piece! {
	#[derive(Add, AddAssign)]
	struct OpponentGIDPData: OpponentGIDPPiece {
		#[serde(rename = "gidpOpp")]
		pub opponent_ground_into_double_play: u32,
	}
}

piece! {
	#[derive(Add, AddAssign)]
	struct GITPData: GITPPiece {
		#[serde(rename = "groundedIntoTriplePlay")]
		pub ground_into_triple_play: u32,
	}
}

piece! {
	#[derive(Add, AddAssign)]
	struct AtBatData: AtBatPiece {
		pub at_bats: u32,
	}
}

piece! {
	#[derive(Add, AddAssign)]
	struct PlateAppearanceData: PlateAppearancePiece {
		pub plate_appearances: u32,
	}
}

piece! {
	#[derive(Add, AddAssign)]
	struct BattersFacedData: BattersFacedPiece {
		pub batters_faced: u32,
	}
}

piece! {
	#[derive(Add, AddAssign)]
	struct LOBData: LOBPiece {
		pub left_on_base: u32,
	}
}

piece! {
	#[derive(Add, AddAssign)]
	struct TotalBasesData: TotalBasesPiece {
		pub total_bases: u32,
	}
}

piece! {
	#[derive(Add, AddAssign)]
	struct ExtraBasesHitsQuantityData: ExtraBasesHitsQuantityPiece {
		pub extra_base_hits: u32,
	}
}

piece! {
	#[derive(Add, AddAssign)]
	struct wOBAAccumulatedData: wOBAAccumulatedPiece {
		#[serde(rename = "wRaa")]
		pub wRaa: f64,
		#[serde(rename = "wRc")]
		pub wRC: f64,
	}
}

piece! {
	struct wOBARateData: wOBARatePiece {
		#[serde(rename = "woba")]
		pub wOBA: f64,
		#[serde(rename = "wRcPlus")]
		pub wRC_plus: f64,
	}
}

piece! {
	#[derive(Add, AddAssign)]
	#[doc = "[FIP-Based WAR](https://library.fangraphs.com/war/differences-fwar-rwar/)"]
	struct fWARData: fWARPiece {
		#[serde(rename = "war")]
		pub fWAR: f64,
	}
}

piece! {
	#[derive(Add, AddAssign)]
	#[doc = "[RA/9-Based WAR](https://library.fangraphs.com/war/differences-fwar-rwar/)"]
	struct bWARData: bWARPiece {
		#[serde(rename = "ra9War")]
		pub bWAR: f64,
	}
}

piece! {
	#[derive(Add, AddAssign)]
	#[doc = "Runs above replacement (FIP-Based)."]
	#[doc = "Equivalent to fWAR * R/W (runs per win -- a guts constant)"]
	struct RARData: RARPiece {
		#[serde(rename = "rar")]
		pub RAR: f64,
	}
}

piece! {
	#[derive(Add, AddAssign)]
	struct BattingRunValueData: BattingRunValuePiece {
		#[serde(rename = "batting")]
		pub batting_run_value: f64,
	}
}

piece! {
	#[derive(Add, AddAssign)]
	struct FieldingRunValueData: FieldingRunValuePiece {
		#[serde(rename = "fielding")]
		pub fielding_run_value: f64,
	}
}

piece! {
	#[derive(Add, AddAssign)]
	struct BaserunningRunValueData: BaserunningRunValuePiece {
		#[serde(rename = "baseRunning")]
		pub baserunning_run_value: f64,
	}
}

piece! {
	#[derive(Add, AddAssign)]
	struct PositionalRunValueOffsetData: PositionalRunValueOffsetPiece {
		#[serde(rename = "positional")]
		pub positional_run_value_offset: f64,
	}
}

piece! {
	#[derive(Add, AddAssign)]
	
	struct SpeedData: SpeedPiece {
		#[serde(rename = "spd")]
		pub SPD: f64,
	}
}

piece! {
	#[derive(Add, AddAssign)]
	struct UltimateBaserunningData: UltimateBaserunningPiece {
		#[serde(rename = "ubr")]
		pub UBR: f64,
	}
}

piece! {
	#[derive(Add, AddAssign)]
	struct WeightedGroundIntoDoublePlayData: WeightedGroundIntoDoublePlayPiece {
		#[serde(rename = "wgdp")]
		pub wGDP: f64,
	}
}

piece! {
	#[derive(Add, AddAssign)]
	struct WeightedStolenBaseValueData: WeightedStolenBaseValuePiece {
		#[serde(rename = "wSb")]
		pub wSB: f64,
	}
}

piece! {
	struct FIPData: FIPPiece {
		#[serde(rename = "fip")]
		pub FIP: f64,
	}
}

piece! {
	struct xFIPData: xFIPPiece {
		#[serde(rename = "xfip")]
		pub xFIP: f64,
	}
}

piece! {
	struct FIPMinusData: FIPMinusPiece {
		#[serde(rename = "fipMinus")]
		pub FIP_minus: f64,
	}
}

piece! {
	struct LeverageIndicesData: LeverageIndicesPiece {
		/// [Average Pitcher Leverage Index](https://library.fangraphs.com/misc/li/)
		#[serde(rename = "pli")]
		pub pLI: f64,

		/// [Average Pitcher Leverage Index per Inning](https://library.fangraphs.com/misc/li/)
		#[serde(rename = "inli")]
		pub inLI: f64,

		/// [Average Pitcher Leverage Index at start of Appearance](https://library.fangraphs.com/misc/li/)
		#[serde(rename = "gmli")]
		pub gmLI: f64,

		/// [Average Pitcher Leverage Index at end of Appearance](https://library.fangraphs.com/misc/li/)
		#[serde(rename = "exli")]
		pub exLI: f64,
	}
}

piece! {
	struct ERAMinusData: ERAMinusPiece {
		#[serde(rename = "eraMinus")]
		pub ERA_minus: f64,
	}
}

piece! {
	#[derive(Add, AddAssign)]
	struct ShutdownsAndMeltdownsData: ShutdownsAndMeltdownsPiece {
		#[doc = "[Relief Appearance of >= 0.06 WPA](https://blogs.fangraphs.com/shutdowns-meltdowns/)"]
		pub shutdowns: u32,
		#[doc = "[Relief Appearance of <= -0.06 WPA](https://blogs.fangraphs.com/shutdowns-meltdowns/)"]
		pub meltdowns: u32,
	}
}

piece! {
	#[derive(Add, AddAssign)]
	struct BalksData: BalksPiece {
		pub balks: u32
	}
}

piece! {
	#[derive(Add, AddAssign)]
	struct CompleteGamesData: CompleteGamesPiece {
		pub complete_games: u32,
		pub shutouts: u32,
	}
}

#[derive(Debug, Deserialize, Add, AddAssign, PartialEq, Eq, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub(crate) struct DecisionsData {
	pub wins: u32,
	pub losses: u32,
	pub holds: u32,
	pub saves: u32,
	pub save_opportunities: u32,
	pub blown_saves: u32,
}
pub trait DecisionsPiece {
	#[must_use]
	fn wins(&self) -> u32;
	
	#[must_use]
	fn losses(&self) -> u32;
	
	#[must_use]
	fn holds(&self) -> u32;
	
	#[must_use]
	fn saves(&self) -> u32;
	
	#[must_use]
	fn save_opportunities(&self) -> u32;
	
	#[must_use]
	fn blown_saves(&self) -> u32;
	
	#[must_use]
	fn decisions(&self) -> u32 {
		self.wins() + self.losses() + self.holds() + self.saves()
	}
}
impl<T: AsRef<DecisionsData>> DecisionsPiece for T {
	fn wins(&self) -> u32 {
		self.as_ref().wins
	}
	
	fn losses(&self) -> u32 {
		self.as_ref().losses
	}
	
	fn holds(&self) -> u32 {
		self.as_ref().holds
	}
	
	fn saves(&self) -> u32 {
		self.as_ref().saves
	}
	
	fn save_opportunities(&self) -> u32 {
		self.as_ref().save_opportunities
	}
	
	fn blown_saves(&self) -> u32 {
		self.as_ref().blown_saves
	}
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone, Default, Add, AddAssign)]
#[serde(try_from = "__InningsPitchedDataStruct")]
pub(crate) struct InningsPitchedData {
	pub innings_pitched: InningsPitched,
}

#[serde_as]
#[derive(Deserialize)]
struct __InningsPitchedDataStruct {
	innings_pitched: Option<InningsPitched>,
	outs: Option<u32>,
}

impl TryFrom<__InningsPitchedDataStruct> for InningsPitchedData {
	type Error = &'static str;

	fn try_from(value: __InningsPitchedDataStruct) -> Result<Self, Self::Error> {
		value.innings_pitched.or_else(|| value.outs.map(InningsPitched::from_outs)).map(|innings_pitched| InningsPitchedData { innings_pitched }).ok_or("could not get either outs-based field (innings_pitched`")
	}
}

pub trait InningsPitchedPiece {
	#[must_use]
	fn outs(&self) -> u32 {
		self.innings_pitched().as_outs()
	}

	#[must_use]
	fn innings_pitched(&self) -> InningsPitched;
}

impl<T: AsRef<InningsPitchedData>> InningsPitchedPiece for T {
	fn innings_pitched(&self) -> InningsPitched {
		self.as_ref().innings_pitched
	}
}