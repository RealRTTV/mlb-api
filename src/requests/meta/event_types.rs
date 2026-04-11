use derive_more::Display;
use serde::Deserialize;
use serde::de::Error;
use thiserror::Error;
use std::str::FromStr;

#[derive(Debug, Error)]
pub enum EventTypeFromStrError {
    #[error("Invalid event type: {0}")]
    Invalid(String),
}

macro_rules! unwrap_or_false {
    ($expr:expr) => { $expr };
    () => { false };
}

macro_rules! event_type {
    ($(
        #[doc = $doc:literal]
        #[display($display:literal)]
        #[from_str($from_str:pat)]
        $(#[hit = $is_hit:expr])?
        $(#[plate_appearance = $is_plate_appearance:expr])?
        $(#[base_running_event = $is_base_running_event:expr])?
        $variant_name:ident
    ),* $(,)?) => {
        #[derive(Debug, PartialEq, Eq, Copy, Clone, Display, Hash)]
        pub enum EventType {
            $(
                #[doc = $doc]
                #[display($display)]
                $variant_name
            ),*
        }

        impl EventType {
            #[must_use]
            pub const fn is_hit(self) -> bool {
                match self {
                    $(
                        Self::$variant_name => unwrap_or_false! { $($is_hit)? },
                    )*
                }
            }

            #[must_use]
            pub const fn is_plate_appearance(self) -> bool {
                match self {
                    $(
                        Self::$variant_name => unwrap_or_false! { $($is_plate_appearance)? },
                    )*
                }
            }

            #[must_use]
            pub const fn is_base_running_event(self) -> bool {
                match self {
                    $(
                        Self::$variant_name => unwrap_or_false! { $($is_base_running_event)? },
                    )*
                }
            }

            #[must_use]
            pub const fn is_out(self) -> bool {
                self.is_plate_appearance() && !self.is_hit()
            }
        }
        
        impl FromStr for EventType {
            type Err = EventTypeFromStrError;
        
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                Ok(match s {
                    $(
                        $from_str => Self::$variant_name
                    ),*,
                    _ => return Err(EventTypeFromStrError::Invalid(s.to_owned()))
                })
            }
        }
    };
}

event_type! {
    // ---------- Misc ----------

    /// Batter requested timeout
    #[display("Batter Timeout")]
    #[from_str("batter_timeout")]
    BatterTimeout,

    /// Any kind of mound visit, decrements MVR
    #[display("Mound Visit")]
    #[from_str("mound_visit")]
    MoundVisit,
    
    /// Game Status Changes
    #[display("Game Advisory")]
    #[from_str("game_advisory")]
    GameAdvisory,

    /// Pitching substitution, only can occur during the inning half where the team is pitching
    #[display("Pitching Substitution")]
    #[from_str("pitching_substitution")]
    PitchingSubstitution,

    /// Switching existing fielder positions
    #[display("Defensive Switch")]
    #[from_str("defensive_switch")]
    DefensiveSwitch,

    /// Swawp out fielder for new fielder off the bench
    #[display("Defensive Substitution")]
    #[from_str("defensive_substitution")]
    DefensiveSubstitution,

    /// Pinch-hitting
    #[display("Offensive Substitution")]
    #[from_str("offensive_substitution")]
    OffensiveSubstitution,

    /// Swap out umpire
    #[display("Umpire Substitution")]
    #[from_str("umpire_substitution")]
    UmpireSubstitution,

    /// Pickoff that leads to an error at 1B
    #[display("Pickoff (Error) (1B)")]
    #[from_str("pickoff_error_1b")]
    PickoffError1B,
    
    /// Pickoff that leads to an error at 2B
    #[display("Pickoff (Error) (2B)")]
    #[from_str("pickoff_error_2b")]
    PickoffError2B,
    
    /// Pickoff that leads to an error at 3B
    #[display("Pickoff (Error) (3B)")]
    #[from_str("pickoff_error_3b")]
    PickoffError3B,

    /// Pitcher steps off the mound for a break
    #[display("Pitcher Step Off")]
    #[from_str("pitcher_step_off")]
    PitcherStepOff,

    /// Fan interference in the play
    #[display("Fan Interference")]
    #[from_str("fan_interference")]
    #[plate_appearance = true]
    FanInterference,

    /// Batter switches sides of the plate
    #[display("Batter Handedness Switch")]
    #[from_str("batter_turn")]
    BatterHandednessSwitch,

    /// Pitcher switches handedness
    #[display("Pitcher Handedness Switch")]
    #[from_str("pitcher_switch")]
    PitcherHandednessSwitch,

    /// Person gets ejected
    #[display("Ejection")]
    #[from_str("ejection")]
    Ejection,

    /// Official No Pitch
    #[display("No Pitch")]
    #[from_str("no_pitch")]
    NoPitch,

    /// Batter does not step in the box in time, or pitcher does not step in the box in time
    #[display("Disengagement Violation")]
    #[from_str("forced_balk")]
    #[base_running_event = true]
    DisengagementViolation,
    // ---------- ---- ----------


    // ---------- Outs ----------
    /// Field out, such as a tag play
    #[display("Field Out")]
    #[from_str("field_out")]
    #[plate_appearance = true]
    FieldOut,

    /// Force out, such as a simple groundout
    #[display("Force Out")]
    #[from_str("force_out")]
    #[plate_appearance = true]
    ForceOut,

    /// Fielder's Choice
    #[display("Fielder's Choice")]
    #[from_str("fielders_choice")]
    #[plate_appearance = true]
    FieldersChoice,

    /// Fielder's Choice but a tag play
    #[display("Fielder's Choice (Field Out)")]
    #[from_str("fielders_choice_out")]
    #[plate_appearance = true]
    FieldersChoiceFieldOut,
    
    /// Not necessarily an out; check for `is_out` field on a runner to be false.
    #[display("Strikeout")]
    #[from_str("strikeout" | "strike_out")]
    #[plate_appearance = true]
    Strikeout,

    /// Strike-em-out-throw-em-out, etc.
    #[display("Strikeout Double Play")]
    #[from_str("strikeout_double_play")]
    #[plate_appearance = true]
    StrikeoutDoublePlay,

    /// Similar to [`Self::StrikeoutDoublePlay`]
    #[display("Strikeout Triple Play")]
    #[from_str("strikeout_triple_play")]
    #[plate_appearance = true]
    StrikeoutTriplePlay,

    /// Sacrifice Bunt
    #[display("Sacrifice Bunt")]
    #[from_str("sac_bunt")]
    #[plate_appearance = true]
    SacrificeBunt,

    /// Sacrifice Fly
    #[display("Sacrifice Fly")]
    #[from_str("sac_fly")]
    #[plate_appearance = true]
    SacrificeFly,

    /// Textbook
    #[display("Grounded Into Double Play")]
    #[from_str("grounded_into_double_play")]
    #[plate_appearance = true]
    GroundedIntoDoublePlay,

    /// 5-4-3, etc.
    #[display("Grounded Into Triple Play")]
    #[from_str("grounded_into_triple_play")]
    GroundedIntoTriplePlay,

    /// Unique double plays that aren't groundout + groundout
    #[display("Double Play")]
    #[from_str("double_play")]
    #[plate_appearance = true]
    DoublePlay,

    /// Unique triple plays that aren't groundout + groundout + groundout
    #[display("Triple Play")]
    #[from_str("triple_play")]
    #[plate_appearance = true]
    TriplePlay,

    /// Out so confusing the official scorer had no clue what to put
    #[display("Other Out")]
    #[from_str("other_out")]
    #[plate_appearance = true]
    #[base_running_event = true]
    OtherOut,

    /// Error in fielding
    #[display("Field Error")]
    #[from_str("field_error")]
    #[plate_appearance = true]
    FieldError,

    /// Misc Error
    #[display("Error")]
    #[from_str("error")]
    Error,

    /// Runner Caught Stealing; often seen in old data, base is included in the [`PlayEvent`] now.
    #[display("Caught Stealing")]
    #[from_str("caught_stealing")]
    #[base_running_event = true]
    CaughtStealing,

    /// Caught stealing second base
    #[display("Caught Stealing (2B)")]
    #[from_str("caught_stealing_2b")]
    #[base_running_event = true]
    CaughtStealing2B,

    /// Caught stealing third base
    #[display("Caught Stealing (3B)")]
    #[from_str("caught_stealing_3b")]
    #[base_running_event = true]
    CaughtStealing3B,

    /// Caught stealing home
    #[display("Caught Stealing (HP)")]
    #[from_str("caught_stealing_home")]
    #[base_running_event = true]
    CaughtStealingHome,

    /// Caught Stealing Double Play -- not a strikeout double play. Such as a caught stealing at second, and a throw home to get a runner on third trying to score from the steal attempt.
    #[display("Caught Stealing Double Play")]
    #[from_str("cs_double_play")]
    #[base_running_event = true]
    CaughtStealingDoublePlay,

    /// Sacrifice Fly that ends in a double play, but is also a successful sacrifice fly. Such as a runner on second and third and the runner on second trying to go to third but getting thrown out. 
    #[display("Sacrifice Fly Double Play")]
    #[from_str("sac_fly_double_play")]
    #[plate_appearance = true]
    SacrificeFlyDoublePlay,

    /// Sacrifice bunt that leds to a double play
    #[display("Sacrifice Bunt Double Play")]
    #[from_str("sac_bunt_double_play")]
    #[plate_appearance = true]
    SacrificeBuntDoublePlay,

    /// Player gets injury, delay of game
    #[display("Injury")]
    #[from_str("injury")]
    Injury,

    /// Ruling is currently pending on the prior play, please wait.
    #[display("Official Scorer Ruling Pending (Prior Play)")]
    #[from_str("os_ruling_pending_prior")]
    #[base_running_event = true]
    PriorRulingPending,

    /// Ruling is currently pending on the active play, please wait.
    #[display("Official Scorer Ruling Pending (Current Play)")]
    #[from_str("os_ruling_pending_primary")]
    #[plate_appearance = true]
    RulingPending,

    /// Seemingly unused
    #[display("At Bat Start")]
    #[from_str("at_bat_start")]
    AtBatStart,

    /// Successful pickoff
    #[display("Pickoff (1B)")]
    #[from_str("pickoff_1b")]
    #[base_running_event = true]
    Pickoff1B,

    /// Successful pickoff
    #[display("Pickoff (2B)")]
    #[from_str("pickoff_2b")]
    #[base_running_event = true]
    Pickoff2B,

    /// Successful pickoff
    #[display("Pickoff (3B)")]
    #[from_str("pickoff_3b")]
    #[base_running_event = true]
    Pickoff3B,

    /// Pickoff on a stolen base attempt, such as going too early and getting thrown out.
    #[display("Pickoff (Caught Stealing) (2B)")]
    #[from_str("pickoff_caught_stealing_2b")]
    #[base_running_event = true]
    PickoffCaughtStealing2B,
    
    /// Pickoff on a stolen base attempt, such as going too early and getting thrown out.
    #[display("Pickoff (Caught Stealing) (3B)")]
    #[from_str("pickoff_caught_stealing_3b")]
    #[base_running_event = true]
    PickoffCaughtStealing3B,
    
    /// Pickoff on a stolen base attempt, such as going too early and getting thrown out.
    #[display("Pickoff (Caught Stealing) (HP)")]
    #[from_str("pickoff_caught_stealing_home")]
    #[base_running_event = true]
    PickoffCaughtStealingHome,

    /// Batter interferes with the play
    #[display("Batter's Interference")]
    #[from_str("batter_interference")]
    #[plate_appearance = true]
    BattersInterference,

    /// Runner interferes with the play, such as kicking the ball.
    #[display("Runner's Interference")]
    #[from_str("runner_interference")]
    RunnersInterference,

    /// Ex: Runner intentionally interferes with the play to not get someone on a later base out, but is so obvious that the umpires and official scorer rule both players out.
    #[display("Runner's Interference Double Play")]
    #[from_str("runner_double_play")]
    #[base_running_event = true]
    RunnersInterferenceDoublePlay,

    /// Runner placed on a base, such as extra innings
    #[display("Runner Placed On Base")]
    #[from_str("runner_placed")]
    RunnerPlaced,
    // ---------- ---- ----------


    // --------- On Base --------
    /// Balk
	#[display("Balk")]
	#[from_str("balk")]
    #[base_running_event = true]
	Balk,

	/// Base on Balls
    #[display("Walk")]
    #[from_str("walk")]
    #[plate_appearance = true]
    Walk,

    /// Intentional Walk
    #[display("Intentional Walk")]
    #[from_str("intent_walk")]
    #[plate_appearance = true]
    IntentionalWalk,

    /// Hit by Pitch
    #[display("Hit By Pitch")]
    #[from_str("hit_by_pitch")]
    #[plate_appearance = true]
    HitByPitch,

    /// Single
    #[display("Single")]
    #[from_str("single")]
    #[hit = true]
    #[plate_appearance = true]
    Single,

    /// Double, could be ground-rule
    #[display("Double")]
    #[from_str("double")]
    #[hit = true]
    #[plate_appearance = true]
    Double,

    /// Triple
    #[display("Triple")]
    #[from_str("triple")]
    #[hit = true]
    #[plate_appearance = true]
    Triple,

    /// Home Run, could be inside the park.
    #[display("Home Run")]
    #[from_str("home_run")]
    #[hit = true]
    #[plate_appearance = true]
    HomeRun,

    /// Stolen base, modern games include the base in the [`PlayEvent`].
	#[display("Stolen Base")]
	#[from_str("stolen_base")]
    StolenBase,

    /// Stolen second base
    #[display("Stolen Base (2B)")]
    #[from_str("stolen_base_2b")]
    StolenBase2B,

    /// Stolen third base
    #[display("Stolen Base (3B)")]
    #[from_str("stolen_base_3b")]
    StolenBase3B,

    /// Stolen home
    #[display("Stolen Base (HP)")]
    #[from_str("stolen_base_home")]
    StolenBaseHome,

    /// A stolen base but unchallenged by the defense; not counted as a SB.
    #[display("Defensive Indifference")]
    #[from_str("defensive_indiff")]
    #[base_running_event = true]
    DefensiveIndifference,

    /// Catcher fails at his job
    #[display("Passed Ball")]
    #[from_str("passed_ball")]
    #[base_running_event = true]
    PassedBall,

    /// Pitcher throws a ball so badly it gets away from the catcher
    #[display("Wild Pitch")]
    #[from_str("wild_pitch")]
    #[base_running_event = true]
    WildPitch,

    /// Catcher interferes with the play
    #[display("Catcher's Interference")]
    #[from_str("catcher_interf")]
    #[plate_appearance = true]
    CatchersInterference,

    /// Fielder interferes with the play
    #[display("Fielder's Interference")]
    #[from_str("fielder_interference")]
    FieldersInterference,

    /// Something so confusing that the Official Scorer has no clue what to call it.
    #[display("Base Advancement (Other)")]
    #[from_str("other_advance")]
    #[base_running_event = true]
    OtherAdvancement,
    // --------- -- ---- ---------
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
