use derive_more::Display;
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Eq, Copy, Clone, Display, Hash)]
#[serde(rename_all = "snake_case")]
pub enum EventType {
    // ---------- Misc ----------
    #[display("Batter Timeout")]
    BatterTimeout,

    #[display("Mound Visit")]
    MoundVisit,
    
    /// Game Status Changes
    #[display("Game Advisory")]
    GameAdvisory,

    #[display("Pitching Substitution")]
    PitchingSubstitution,

    #[display("Defensive Switch")]
    DefensiveSwitch,

    #[display("Defensive Substitution")]
    DefensiveSubstitution,

    #[display("Offensive Substitution")]
    OffensiveSubstitution,

    #[display("Umpire Substitution")]
    UmpireSubstitution,
    
    #[display("Pickoff (Error) (1B)")]
    #[serde(rename = "pickoff_error_1b")]
    PickoffError1B,
    
    #[display("Pickoff (Error) (2B)")]
    #[serde(rename = "pickoff_error_2b")]
    PickoffError2B,
    
    #[display("Pickoff (Error) (3B)")]
    #[serde(rename = "pickoff_error_3b")]
    PickoffError3B,

    #[display("Pitcher Step Off")]
    PitcherStepOff,

    #[display("Fan Interference")]
    FanInterference,

    /// Switches sides of the plate
    #[display("Batter Handedness Switch")]
    #[serde(rename = "batter_turn")]
    BatterHandednessSwitch,

    #[display("Pitcher Handedness Switch")]
    #[serde(rename = "pitcher_switch")]
    PitcherHandednessSwitch,

    #[display("Ejection")]
    Ejection,

    #[display("No Pitch")]
    NoPitch,

    #[serde(rename = "forced_balk")]
    #[display("Disengagement Violation")]
    DisengagementViolation,
    // ---------- ---- ----------


    // ---------- Outs ----------
    #[display("Field Out")]
    FieldOut,

    #[display("Force Out")]
    ForceOut,

    #[display("Fielder's Choice")]
    FieldersChoice,

    #[display("Fielder's Choice (Field Out)")]
    #[serde(rename = "fielders_choice_out")]
    FieldersChoiceFieldOut,
    
    /// Not necessarily an out.
    #[display("Strikeout")]
    Strikeout,

    #[display("Strikeout Double Play")]
    StrikeoutDoublePlay,
    
    #[display("Strikeout Triple Play")]
    StrikeoutTriplePlay,

    #[display("Sacrifice Bunt")]
    #[serde(rename = "sac_bunt")]
    SacrificeBunt,

    #[display("Sacrifice Fly")]
    #[serde(rename = "sac_fly")]
    SacrificeFly,

    #[display("Grounded Into Double Play")]
    GroundedIntoDoublePlay,

    #[display("Grounded Into Triple Play")]
    GroundedIntoTriplePlay,

    /// Unique double plays that aren't groundout + groundout
    #[display("Double Play")]
    DoublePlay,

    /// Unique triple plays that aren't groundout + groundout + groundout
    #[display("Triple Play")]
    TriplePlay,

    #[display("Other Out")]
    OtherOut,

    #[display("Field Error")]
    FieldError,

    /// Misc Error
    #[display("Error")]
    Error,

    #[display("Caught Stealing")]
    #[serde(rename = "caught_stealing")]
    CaughtStealing,

    #[display("Caught Stealing (2B)")]
    #[serde(rename = "caught_stealing_2b")]
    CaughtStealing2B,

    #[display("Caught Stealing (3B)")]
    #[serde(rename = "caught_stealing_3b")]
    CaughtStealing3B,

    #[display("Caught Stealing (HP)")]
    #[serde(rename = "caught_stealing_home")]
    CaughtStealingHome,

    #[display("Caught Stealing Double Play")]
    #[serde(rename = "cs_double_play")]
    CaughtStealingDoublePlay,
    
    #[display("Sacrifice Fly Double Play")]
    #[serde(rename = "sac_fly_double_play")]
    SacrificeFlyDoublePlay,
    
    #[display("Sacrifice Bunt Double Play")]
    #[serde(rename = "sac_bunt_double_play")]
    SacrificeBuntDoublePlay,
    
    #[display("Injury")]
    #[serde(rename = "injury")]
    Injury,
    
    #[display("Official Scorer Ruling Pending (Prior Play)")]
    #[serde(rename = "os_ruling_pending_prior")]
    PriorRulingPending,
    
    #[display("Official Scorer Ruling Pending (Current Play)")]
    #[serde(rename = "os_ruling_pending_primary")]
    RulingPending,

    /// Seemingly unused
    #[display("At Bat Start")]
    #[serde(rename = "at_bat_start")]
    AtBatStart,

    /// Successful pickoff
    #[display("Pickoff (1B)")]
    #[serde(rename = "pickoff_1b")]
    Pickoff1B,

    /// Successful pickoff
    #[display("Pickoff (2B)")]
    #[serde(rename = "pickoff_2b")]
    Pickoff2B,

    /// Successful pickoff
    #[display("Pickoff (3B)")]
    #[serde(rename = "pickoff_3b")]
    Pickoff3B,    
    
    #[display("Pickoff (Caught Stealing) (2B)")]
    #[serde(rename = "pickoff_caught_stealing_2b")]
    PickoffCaughtStealing2B,
    
    #[display("Pickoff (Caught Stealing) (3B)")]
    #[serde(rename = "pickoff_caught_stealing_3b")]
    PickoffCaughtStealing3B,
    
    #[display("Pickoff (Caught Stealing) (HP)")]
    #[serde(rename = "pickoff_caught_stealing_home")]
    PickoffCaughtStealingHome,

    #[serde(rename = "batter_interference")]
    #[display("Batter's Interference")]
    BattersInterference,

    #[serde(rename = "runner_interference")]
    #[display("Runner's Interference")]
    RunnersInterference,
    
    #[serde(rename = "runner_double_play")]
    #[display("Runner's Interference Double Play")]
    RunnersInterferenceDoublePlay,

    #[display("Runner Placed On Base")]
    RunnerPlaced,
    // ---------- ---- ----------


    // --------- On Base --------
	#[display("Balk")]
	Balk,
    
    #[display("Walk")]
    Walk,

    #[display("Intentional Walk")]
    #[serde(rename = "intent_walk")]
    IntentionalWalk,

    #[display("Hit By Pitch")]
    HitByPitch,
     
    #[display("Single")]
    Single,

    #[display("Double")]
    Double,

    #[display("Triple")]
    Triple,

    #[display("Home Run")]
    HomeRun,
    
	#[display("Stolen Base")]
	#[serde(rename = "stolen_base")]
    StolenBase,

    #[display("Stolen Base (2B)")]
    #[serde(rename = "stolen_base_2b")]
    StolenBase2B,

    #[display("Stolen Base (3B)")]
    #[serde(rename = "stolen_base_3b")]
    StolenBase3B,

    #[display("Stolen Base (HP)")]
    #[serde(rename = "stolen_base_home")]
    StolenBaseHome,

    /// A stolen base but unchallenged by the defense; not counted as a SB.
    #[display("Defensive Indifference")]
    #[serde(rename = "defensive_indiff")]
    DefensiveIndifference,

    #[display("Passed Ball")]
    PassedBall,

    #[display("Wild Pitch")]
    WildPitch,

    #[serde(rename = "catcher_interf")]
    #[display("Catcher's Interference")]
    CatchersInterference,
    
    #[serde(rename = "fielder_interference")]
    #[display("Fielder's Interference")]
    FieldersInterference,

    #[serde(rename = "other_advance")]
    #[display("Base Advancement (Other)")]
    OtherAdvancement,
    // --------- -- ---- ---------
}

// impl EventType {
// 	#[must_use]
// 	pub const fn is_plate_appearance(self) -> bool {
// 		match self {
// 			Self::Pickoff1B => false,
// 			Self::Pickoff2B => false,
// 			Self::Pickoff3B => false,
// 			Self::PitcherStepOff => false,
// 			Self::PickoffError1B => false,
// 			Self::PickoffError2B => false,
// 			Self::PickoffError3B => false,
// 			Self::BatterTimeout => false,
// 			Self::MoundVisit => false,
// 			Self::NoPitch => false,
// 			Self::Single => false,
// 			Self::Double => false,
// 			Self::Triple => false,
// 			Self::HomeRun => false,
// 			Self::DoublePlay => false,
// 			Self::FieldError => false,
// 			Self::Error => false,
// 			Self::FieldOut => false,
// 			Self::FieldersChoice => false,
// 			Self::FieldersChoiceFieldOut => false,
// 			Self::ForceOut => false,
// 			Self::GroundedIntoDoublePlay => false,
// 			Self::GroundedIntoTriplePlay => false,
// 			Self::Strikeout => false,
// 			Self::TriplePlay => false,
// 			Self::SacrificeFly => false,
// 			Self::CatchersInterference => false,
// 			Self::BattersInterference => false,
// 			Self::FieldersInterference => false,
// 			Self::RunnersInterference => false,
// 			Self::FanInterference => false,
// 			Self::BatterHandednessSwitch => false,
// 			Self::Ejection => false,
// 			Self::CaughtStealingDoublePlay => false,
// 			Self::DefensiveIndifference => false,
// 			Self::SacrificeFlyDoublePlay => false,
// 			Self::SacrificeBunt => false,
// 			Self::SacrificeBuntDoublePlay => false,
// 			Self::Walk => false,
// 			Self::IntentionalWalk => false,
// 			Self::HitByPitch => false,
// 			Self::Injury => false,
// 			Self::PriorRulingPending => false,
// 			Self::RulingPending => false,
// 			Self::AtBatStart => false,
// 			Self::PassedBall => false,
// 			Self::OtherAdvancement => false,
// 			Self::RunnersInterferenceDoublePlay => false,
// 			Self::RunnerPlaced => false,
// 			Self::PitchingSubstitution => false,
// 			Self::OffensiveSubstitution => false,
// 			Self::DefensiveSwitch => false,
// 			Self::UmpireSubstitution => false,
// 			Self::PitcherHandednessSwitch => false,
// 			Self::GameAdvisory => false,
// 			Self::StolenBase => false,
// 			Self::StolenBase2B => false,
// 			Self::StolenBase3B => false,
// 			Self::StolenBaseHome => false,
// 			Self::CaughtStealing => false,
// 			Self::CaughtStealing2B => false,
// 			Self::CaughtStealing3B => false,
// 			Self::CaughtStealingHome => false,
// 			Self::DefensiveSubstitution => false,
// 			Self::PickoffCaughtStealing2B => false,
// 			Self::PickoffCaughtStealing3B => false,
// 			Self::PickoffCaughtStealingHome => false,
// 			Self::Balk => false,
// 			Self::DisengagementViolation => false,
// 			Self::WildPitch => false,
// 			Self::OtherOut => false,
// 		}
// 	}

// 	#[must_use]
// 	pub const fn is_hit(self) -> bool {
// 		match self {
// 			Self::Pickoff1B => false,
// 			Self::Pickoff2B => false,
// 			Self::Pickoff3B => false,
// 			Self::PitcherStepOff => false,
// 			Self::PickoffError1B => false,
// 			Self::PickoffError2B => false,
// 			Self::PickoffError3B => false,
// 			Self::BatterTimeout => false,
// 			Self::MoundVisit => false,
// 			Self::NoPitch => false,
// 			Self::Single => false,
// 			Self::Double => false,
// 			Self::Triple => false,
// 			Self::HomeRun => false,
// 			Self::DoublePlay => false,
// 			Self::FieldError => false,
// 			Self::Error => false,
// 			Self::FieldOut => false,
// 			Self::FieldersChoice => false,
// 			Self::FieldersChoiceFieldOut => false,
// 			Self::ForceOut => false,
// 			Self::GroundedIntoDoublePlay => false,
// 			Self::GroundedIntoTriplePlay => false,
// 			Self::Strikeout => false,
// 			Self::TriplePlay => false,
// 			Self::SacrificeFly => false,
// 			Self::CatchersInterference => false,
// 			Self::BattersInterference => false,
// 			Self::FieldersInterference => false,
// 			Self::RunnersInterference => false,
// 			Self::FanInterference => false,
// 			Self::BatterHandednessSwitch => false,
// 			Self::Ejection => false,
// 			Self::CaughtStealingDoublePlay => false,
// 			Self::DefensiveIndifference => false,
// 			Self::SacrificeFlyDoublePlay => false,
// 			Self::SacrificeBunt => false,
// 			Self::SacrificeBuntDoublePlay => false,
// 			Self::Walk => false,
// 			Self::IntentionalWalk => false,
// 			Self::HitByPitch => false,
// 			Self::Injury => false,
// 			Self::PriorRulingPending => false,
// 			Self::RulingPending => false,
// 			Self::AtBatStart => false,
// 			Self::PassedBall => false,
// 			Self::OtherAdvancement => false,
// 			Self::RunnersInterferenceDoublePlay => false,
// 			Self::RunnerPlaced => false,
// 			Self::PitchingSubstitution => false,
// 			Self::OffensiveSubstitution => false,
// 			Self::DefensiveSwitch => false,
// 			Self::UmpireSubstitution => false,
// 			Self::PitcherHandednessSwitch => false,
// 			Self::GameAdvisory => false,
// 			Self::StolenBase => false,
// 			Self::StolenBase2B => false,
// 			Self::StolenBase3B => false,
// 			Self::StolenBaseHome => false,
// 			Self::CaughtStealing => false,
// 			Self::CaughtStealing2B => false,
// 			Self::CaughtStealing3B => false,
// 			Self::CaughtStealingHome => false,
// 			Self::DefensiveSubstitution => false,
// 			Self::PickoffCaughtStealing2B => false,
// 			Self::PickoffCaughtStealing3B => false,
// 			Self::PickoffCaughtStealingHome => false,
// 			Self::Balk => false,
// 			Self::DisengagementViolation => false,
// 			Self::WildPitch => false,
// 			Self::OtherOut => false,
// 		}
// 	}

// 	#[must_use]
// 	pub const fn is_base_running_event(self) -> bool {
// 		match self {
// 			Self::Pickoff1B => false,
// 			Self::Pickoff2B => false,
// 			Self::Pickoff3B => false,
// 			Self::PitcherStepOff => false,
// 			Self::PickoffError1B => false,
// 			Self::PickoffError2B => false,
// 			Self::PickoffError3B => false,
// 			Self::BatterTimeout => false,
// 			Self::MoundVisit => false,
// 			Self::NoPitch => false,
// 			Self::Single => false,
// 			Self::Double => false,
// 			Self::Triple => false,
// 			Self::HomeRun => false,
// 			Self::DoublePlay => false,
// 			Self::FieldError => false,
// 			Self::Error => false,
// 			Self::FieldOut => false,
// 			Self::FieldersChoice => false,
// 			Self::FieldersChoiceFieldOut => false,
// 			Self::ForceOut => false,
// 			Self::GroundedIntoDoublePlay => false,
// 			Self::GroundedIntoTriplePlay => false,
// 			Self::Strikeout => false,
// 			Self::TriplePlay => false,
// 			Self::SacrificeFly => false,
// 			Self::CatchersInterference => false,
// 			Self::BattersInterference => false,
// 			Self::FieldersInterference => false,
// 			Self::RunnersInterference => false,
// 			Self::FanInterference => false,
// 			Self::BatterHandednessSwitch => false,
// 			Self::Ejection => false,
// 			Self::CaughtStealingDoublePlay => false,
// 			Self::DefensiveIndifference => false,
// 			Self::SacrificeFlyDoublePlay => false,
// 			Self::SacrificeBunt => false,
// 			Self::SacrificeBuntDoublePlay => false,
// 			Self::Walk => false,
// 			Self::IntentionalWalk => false,
// 			Self::HitByPitch => false,
// 			Self::Injury => false,
// 			Self::PriorRulingPending => false,
// 			Self::RulingPending => false,
// 			Self::AtBatStart => false,
// 			Self::PassedBall => false,
// 			Self::OtherAdvancement => false,
// 			Self::RunnersInterferenceDoublePlay => false,
// 			Self::RunnerPlaced => false,
// 			Self::PitchingSubstitution => false,
// 			Self::OffensiveSubstitution => false,
// 			Self::DefensiveSwitch => false,
// 			Self::UmpireSubstitution => false,
// 			Self::PitcherHandednessSwitch => false,
// 			Self::GameAdvisory => false,
// 			Self::StolenBase => false,
// 			Self::StolenBase2B => false,
// 			Self::StolenBase3B => false,
// 			Self::StolenBaseHome => false,
// 			Self::CaughtStealing => false,
// 			Self::CaughtStealing2B => false,
// 			Self::CaughtStealing3B => false,
// 			Self::CaughtStealingHome => false,
// 			Self::DefensiveSubstitution => false,
// 			Self::PickoffCaughtStealing2B => false,
// 			Self::PickoffCaughtStealing3B => false,
// 			Self::PickoffCaughtStealingHome => false,
// 			Self::Balk => false,
// 			Self::DisengagementViolation => false,
// 			Self::WildPitch => false,
// 			Self::OtherOut => false,
// 		}
// 	}
//}

meta_kind_impl!("eventTypes" => EventType);
static_request_entry_cache_impl!(EventType);
test_impl!(EventType);
