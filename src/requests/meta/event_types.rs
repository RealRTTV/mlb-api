use derive_more::Display;
use serde::Deserialize;
use serde::de::Error;
use thiserror::Error;
use std::str::FromStr;

// todo: replace with macro and use attributes for fns

#[derive(Debug, PartialEq, Eq, Copy, Clone, Display, Hash)]
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
    PickoffError1B,
    
    #[display("Pickoff (Error) (2B)")]
    PickoffError2B,
    
    #[display("Pickoff (Error) (3B)")]
    PickoffError3B,

    #[display("Pitcher Step Off")]
    PitcherStepOff,

    #[display("Fan Interference")]
    FanInterference,

    /// Switches sides of the plate
    #[display("Batter Handedness Switch")]
    BatterHandednessSwitch,

    #[display("Pitcher Handedness Switch")]
    PitcherHandednessSwitch,

    #[display("Ejection")]
    Ejection,

    #[display("No Pitch")]
    NoPitch,

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
    FieldersChoiceFieldOut,
    
    /// Not necessarily an out.
    #[display("Strikeout")]
    Strikeout,

    #[display("Strikeout Double Play")]
    StrikeoutDoublePlay,
    
    #[display("Strikeout Triple Play")]
    StrikeoutTriplePlay,

    #[display("Sacrifice Bunt")]
    SacrificeBunt,

    #[display("Sacrifice Fly")]
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
    CaughtStealing,

    #[display("Caught Stealing (2B)")]
    CaughtStealing2B,

    #[display("Caught Stealing (3B)")]
    CaughtStealing3B,

    #[display("Caught Stealing (HP)")]
    CaughtStealingHome,

    #[display("Caught Stealing Double Play")]
    CaughtStealingDoublePlay,
    
    #[display("Sacrifice Fly Double Play")]
    SacrificeFlyDoublePlay,
    
    #[display("Sacrifice Bunt Double Play")]
    SacrificeBuntDoublePlay,
    
    #[display("Injury")]
    Injury,
    
    #[display("Official Scorer Ruling Pending (Prior Play)")]
    PriorRulingPending,
    
    #[display("Official Scorer Ruling Pending (Current Play)")]
    RulingPending,

    /// Seemingly unused
    #[display("At Bat Start")]
    AtBatStart,

    /// Successful pickoff
    #[display("Pickoff (1B)")]
    Pickoff1B,

    /// Successful pickoff
    #[display("Pickoff (2B)")]
    Pickoff2B,

    /// Successful pickoff
    #[display("Pickoff (3B)")]
    Pickoff3B,    
    
    #[display("Pickoff (Caught Stealing) (2B)")]
    PickoffCaughtStealing2B,
    
    #[display("Pickoff (Caught Stealing) (3B)")]
    PickoffCaughtStealing3B,
    
    #[display("Pickoff (Caught Stealing) (HP)")]
    PickoffCaughtStealingHome,

    #[display("Batter's Interference")]
    BattersInterference,

    #[display("Runner's Interference")]
    RunnersInterference,
    
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
    StolenBase,

    #[display("Stolen Base (2B)")]
    StolenBase2B,

    #[display("Stolen Base (3B)")]
    StolenBase3B,

    #[display("Stolen Base (HP)")]
    StolenBaseHome,

    /// A stolen base but unchallenged by the defense; not counted as a SB.
    #[display("Defensive Indifference")]
    DefensiveIndifference,

    #[display("Passed Ball")]
    PassedBall,

    #[display("Wild Pitch")]
    WildPitch,

    #[display("Catcher's Interference")]
    CatchersInterference,
    
    #[display("Fielder's Interference")]
    FieldersInterference,

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
//          Self::StrikeoutDoublePlay => false,
//          Self::StrikeoutTriplePlay => false,
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
//          Self::StrikeoutDoublePlay => false,
//          Self::StrikeoutTriplePlay => false,
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
//          Self::StrikeoutDoublePlay => false,
//          Self::StrikeoutTriplePlay => false,
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

#[derive(Debug, Error)]
pub enum EventTypeFromStrError {
    #[error("Invalid event type: {0}")]
    Invalid(String),
}

impl FromStr for EventType {
    type Err = EventTypeFromStrError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
			"pickoff_1b" => Self::Pickoff1B,
			"pickoff_2b" => Self::Pickoff2B,
			"pickoff_3b" => Self::Pickoff3B,
			"pitcher_step_off" => Self::PitcherStepOff,
			"pickoff_error_1b" => Self::PickoffError1B,
			"pickoff_error_2b" => Self::PickoffError2B,
			"pickoff_error_3b" => Self::PickoffError3B,
			"batter_timeout" => Self::BatterTimeout,
			"mound_visit" => Self::MoundVisit,
			"no_pitch" => Self::NoPitch,
			"single" => Self::Single,
			"double" => Self::Double,
			"triple" => Self::Triple,
			"home_run" => Self::HomeRun,
			"double_play" => Self::DoublePlay,
			"field_error" => Self::FieldError,
			"error" => Self::Error,
			"field_out" => Self::FieldOut,
			"fielders_choice" => Self::FieldersChoice,
			"fielders_choice_out" => Self::FieldersChoiceFieldOut,
			"force_out" => Self::ForceOut,
			"grounded_into_double_play" => Self::GroundedIntoDoublePlay,
			"grounded_into_triple_play" => Self::GroundedIntoTriplePlay,
			"strikeout" | "strike_out" => Self::Strikeout,
            "strikeout_double_play" => Self::StrikeoutDoublePlay,
            "strikeout_triple_play" => Self::StrikeoutTriplePlay,
			"triple_play" => Self::TriplePlay,
			"sac_fly" => Self::SacrificeFly,
			"catcher_interf" => Self::CatchersInterference,
			"batter_interference" => Self::BattersInterference,
			"fielder_interference" => Self::FieldersInterference,
			"runner_interference" => Self::RunnersInterference,
			"fan_interference" => Self::FanInterference,
			"batter_turn" => Self::BatterHandednessSwitch,
			"ejection" => Self::Ejection,
			"cs_double_play" => Self::CaughtStealingDoublePlay,
			"defensive_indiff" => Self::DefensiveIndifference,
			"sac_fly_double_play" => Self::SacrificeFlyDoublePlay,
			"sac_bunt" => Self::SacrificeBunt,
			"sac_bunt_double_play" => Self::SacrificeBuntDoublePlay,
			"walk" => Self::Walk,
			"intent_walk" => Self::IntentionalWalk,
			"hit_by_pitch" => Self::HitByPitch,
			"injury" => Self::Injury,
			"os_ruling_pending_prior" => Self::PriorRulingPending,
			"os_ruling_pending_primary" => Self::RulingPending,
			"at_bat_start" => Self::AtBatStart,
			"passed_ball" => Self::PassedBall,
			"other_advance" => Self::OtherAdvancement,
			"runner_double_play" => Self::RunnersInterferenceDoublePlay,
			"runner_placed" => Self::RunnerPlaced,
			"pitching_substitution" => Self::PitchingSubstitution,
			"offensive_substitution" => Self::OffensiveSubstitution,
			"defensive_switch" => Self::DefensiveSwitch,
			"umpire_substitution" => Self::UmpireSubstitution,
			"pitcher_switch" => Self::PitcherHandednessSwitch,
 			"game_advisory" => Self::GameAdvisory,
			"stolen_base" => Self::StolenBase,
			"stolen_base_2b" => Self::StolenBase2B,
			"stolen_base_3b" => Self::StolenBase3B,
			"stolen_base_home" => Self::StolenBaseHome,
			"caught_stealing" => Self::CaughtStealing,
			"caught_stealing_2b" => Self::CaughtStealing2B,
			"caught_stealing_3b" => Self::CaughtStealing3B,
			"caught_stealing_home" => Self::CaughtStealingHome,
			"defensive_substitution" => Self::DefensiveSubstitution,
			"pickoff_caught_stealing_2b" => Self::PickoffCaughtStealing2B,
			"pickoff_caught_stealing_3b" => Self::PickoffCaughtStealing3B,
			"pickoff_caught_stealing_home" => Self::PickoffCaughtStealingHome,
			"balk" => Self::Balk,
			"forced_balk" => Self::DisengagementViolation,
			"wild_pitch" => Self::WildPitch,
			"other_out" => Self::OtherOut,
			_ => return Err(EventTypeFromStrError::Invalid(s.to_owned()))
        })
    }
}

impl<'de> Deserialize<'de> for EventType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum Repr {
            Inline(String),
            Wrapped {
                code: String,
            }
        }

        let (Repr::Inline(code) | Repr::Wrapped { code }) = Repr::deserialize(deserializer)?;
        Self::from_str(&code).map_err(D::Error::custom)
    }
}

meta_kind_impl!("eventTypes" => EventType);
static_request_entry_cache_impl!(EventType);
test_impl!(EventType);
