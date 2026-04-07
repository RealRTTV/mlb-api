use std::{fmt::Display, num::NonZeroUsize, ops::{Deref, DerefMut}};

use bon::Builder;
use chrono::{DateTime, Utc};
use derive_more::{Deref, DerefMut, Display};
use serde::{Deserialize, Deserializer, de::IgnoredAny};
use serde_with::{serde_as, DefaultOnNull, DefaultOnError};
use uuid::Uuid;

use crate::{Copyright, Handedness, HomeAway, game::{AtBatCount, Base, BattingOrderIndex, ContactHardness, GameId, Inning, InningHalf}, meta::{EventType, HitTrajectory, NamedPosition, PitchCodeId, PitchType, ReviewReasonId}, person::{NamedPerson, PersonId}, request::RequestURL, stats::raw::{HittingHotColdZones, PitchingHotColdZones, StrikeZoneSection}, team::TeamId};

/// A collection of plays, often a whole game's worth.
#[allow(clippy::struct_field_names, clippy::unsafe_derive_deserialize, reason = "not relevant here")]
#[derive(Debug, Deserialize, PartialEq, Clone, Deref)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "_debug", serde(deny_unknown_fields))]
pub struct Plays {
    #[serde(default)]
    pub copyright: Copyright,
    
    #[deref]
    #[serde(rename = "allPlays")]
    plays: Vec<Play>,

    /// Unlinked from the `plays` list
    pub current_play: Option<Play>,

    #[serde(rename = "scoringPlays")]
    pub(super) scoring_play_indices: Vec<usize>,

    #[serde(rename = "playsByInning")]
    pub(super) play_indices_by_inning: Vec<InningPlaysIndices>,
}

impl Plays {
    /// Gives a mutable refernece to the underlying plays.
    ///
    /// # Safety
    /// [`Self::scroing_plays`], [`Self::by_inning`] and [`Self::by_inning_halves`] use caches for these plays, if mutated, these caches will be outdated.
    #[must_use]
    pub const unsafe fn plays_mut(&mut self) -> &mut Vec<Play> {
        &mut self.plays
    }

    /// Reduces this type into the underlying [`Vec<Play>`]
    #[must_use]
    pub fn into_plays(self) -> Vec<Play> {
        self.plays
    }

    /// Iterator over a list of scoring plays.
    ///
    /// ## Examples
    /// ```no_run
    /// let plays: Plays = ...;
    ///
    /// for play in plays.scoring_plays() {
    ///     dbg!(play);
    /// }
    /// ```
    pub fn scoring_plays(&self) -> impl Iterator<Item=&Play> {
        self.scoring_play_indices.iter()
            .filter_map(|&idx| self.plays.get(idx))
    }

    /// Iterator of plays by inning
    ///
    /// ## Examples
    /// ```no_run
    /// let plays: Plays = ...;
    ///
    /// for plays in plays.by_inning() {
    ///     for play in plays {
    ///         dbg!(play);
    ///     }
    /// }
    /// ```
    pub fn by_inning(&self) -> impl Iterator<Item=impl Iterator<Item=&Play>> {
        self.play_indices_by_inning.iter()
            .map(|inning| (inning.start..=inning.end)
                .filter_map(|idx| self.plays.get(idx)))
    }

    /// Iterator of plays by inning halves. (top then bottom)
    ///
    /// ## Examples
    /// ```no_run
    /// let plays: Plays = ...;
    ///
    /// for (top, bottom) in plays.by_inning_halves() {
    ///     for play in top {
    ///         dbg!(play);
    ///     }
    ///     for play in bottom {
    ///         dbg!(play);
    ///     }
    /// }
    /// ```
    pub fn by_inning_halves(&self) -> impl Iterator<Item=(impl Iterator<Item=&Play>, impl Iterator<Item=&Play>)> {
        self.play_indices_by_inning.iter()
            .map(|inning| (
                inning.top_indices.iter().filter_map(|&idx| self.plays.get(idx)),
                inning.bottom_indices.iter().filter_map(|&idx| self.plays.get(idx))
            ))
    }
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "_debug", serde(deny_unknown_fields))]
pub(super) struct InningPlaysIndices {
    #[serde(rename = "startIndex")]
    pub(super) start: usize,
    #[serde(rename = "endIndex")]
    pub(super) end: usize,
    #[serde(rename = "top")]
    pub(super) top_indices: Vec<usize>,
    #[serde(rename = "bottom")]
    pub(super) bottom_indices: Vec<usize>,
    #[doc(hidden)]
    #[serde(rename = "hits", default)]
    pub(super) __balls_in_play: IgnoredAny,
}

/// The play(s) within an "At-Bat"
///
/// For individual "plays" and actions, look to [`PlayEvent`]s
#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "_debug", serde(deny_unknown_fields))]
pub struct Play {
    /// See [`PlayDetails`].
    pub result: PlayDetails,
    /// See [`PlayAbout`].
    pub about: PlayAbout,
    /// Active count in the at-bat.
    pub count: AtBatCount,
    /// See [`PlayMatchup`].
    pub matchup: PlayMatchup,
    /// See [`PlayEvent`].
    pub play_events: Vec<PlayEvent>,
    pub runners: Vec<RunnerData>,
    #[serde(rename = "reviewDetails", default, deserialize_with = "deserialize_review_data")]
    pub reviews: Vec<ReviewData>,
    
    /// Timestamp at which the [`Play`] is called complete.
    #[serde(rename = "playEndTime", deserialize_with = "crate::deserialize_datetime")]
    pub play_end_timestamp: DateTime<Utc>,

    #[doc(hidden)]
    #[serde(rename = "pitchIndex", default)]
    pub __pitch_indices: IgnoredAny,
    #[doc(hidden)]
    #[serde(rename = "actionIndex", default)]
    pub __action_indices: IgnoredAny,
    #[doc(hidden)]
    #[serde(rename = "runnerIndex", default)]
    pub __runner_indices: IgnoredAny,
    #[doc(hidden)]
    #[serde(rename = "atBatIndex", default)]
    pub __at_bat_index: IgnoredAny,
}

/// The result of a play, such as a Strikeout, Home Run, etc.
#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "_debug", serde(deny_unknown_fields))]
pub struct PlayDetails {
    #[serde(flatten, default)]
    pub completed_play_details: Option<CompletedPlayDetails>,
    
    /// Score as of the end of the play
    pub away_score: usize,
    /// Score as of the end of the play
    pub home_score: usize,

    #[doc(hidden)]
    #[serde(rename = "event", default)]
    pub __event: IgnoredAny,

    #[doc(hidden)]
    #[serde(rename = "type", default)]
    pub __type: IgnoredAny,
}

/// Information supplied to [`PlayDetails`] when the play is complete
#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "_debug", serde(deny_unknown_fields))]
pub struct CompletedPlayDetails {
    #[serde(rename = "eventType")]
    pub event: EventType,
    /// Shohei Ohtani strikes out swinging.
    pub description: String,
    /// Runs batted in
    pub rbi: usize,
    /// Whether the batter in the play is out
    pub is_out: bool,
}

/// Miscallaneous data regarding a play
#[allow(clippy::struct_excessive_bools, reason = "inapplicable")]
#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "_debug", serde(deny_unknown_fields))]
pub struct PlayAbout {
    /// Ordinal at bat of the game (starts at 0)
    #[serde(rename = "atBatIndex")]
    pub at_bat_idx: usize,

    /// The inning half this play is in
    #[serde(rename = "halfInning")]
    pub inning_half: InningHalf,

    /// The inning this play is in
    pub inning: Inning,

    /// The timestamp that this play begins; includes milliseconds
    #[serde(rename = "startTime", deserialize_with = "crate::deserialize_datetime")]
    pub start_timestamp: DateTime<Utc>,

    /// The timestamp that this play ends at; includes milliseconds
    #[serde(rename = "endTime", deserialize_with = "crate::deserialize_datetime")]
    pub end_timestamp: DateTime<Utc>,

    /// Whether the play is "complete" or not, i.e. opposite of ongoing.
    ///
    /// Once a play is complete it cannot be edited
    pub is_complete: bool,

    /// Whether the play itself is scoring, such as a Home Run.
    ///
    /// Note that [`Play`]s that include [`PlayEvent`]s that score runs that are not part of the [`Play`] (such as stealing home) do not indicate this as true.
    ///
    /// This is the predicate for [`Plays::scoring_plays`]
    pub is_scoring_play: Option<bool>,

    /// Whether the play has a replay review occur. Note that At-Bats can have multiple challenges occur.
    #[serde(default)]
    pub has_review: bool,

    /// Whether the play has counted towards an out so far.
    ///
    /// todo: check if includes play events like pickoffs.
    #[serde(default)]
    pub has_out: bool,

    /// Ordinal ranking for +/- WPA effect.
    ///
    /// `1` means largest effect on WPA,
    /// `2` means second most, etc.
    #[serde(default)]
    pub captivating_index: usize,

    #[doc(hidden)]
    #[serde(rename = "isTopInning")]
    pub __is_top_inning: IgnoredAny,
}

/// Hitter & Pitcher matchup information
#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "_debug", serde(deny_unknown_fields))]
pub struct PlayMatchup {
    pub batter: NamedPerson,
    pub pitcher: NamedPerson,
    /// Cannot change during the play
    pub bat_side: Handedness,
    /// Cannot change during the play
    pub pitch_hand: Handedness,
    pub post_on_first: Option<NamedPerson>,
    pub post_on_second: Option<NamedPerson>,
    pub post_on_third: Option<NamedPerson>,

    #[doc(hidden)]
    #[serde(rename = "batterHotColdZones", default)]
    pub __batter_hot_cold_zones: IgnoredAny,
    #[doc(hidden)]
    #[serde(rename = "pitcherHotColdZones", default)]
    pub __pitcher_hot_cold_zones: IgnoredAny,
    #[doc(hidden)]
    #[serde(rename = "batterHotColdZoneStats", default)]
    pub __batter_hot_cold_zone_stats: IgnoredAny,
    #[doc(hidden)]
    #[serde(rename = "pitcherHotColdZoneStats", default)]
    pub __pitcher_hot_cold_zone_stats: IgnoredAny,
    
    // pub batter_hot_cold_zones: HittingHotColdZones,
    // pub pitcher_hot_cold_zones: PitchingHotColdZones,

    pub splits: ApplicablePlayMatchupSplits,
}

/// Batter, Pitcher, and Men-On-Base splits; unknown type.
#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "_debug", serde(deny_unknown_fields))]
pub struct ApplicablePlayMatchupSplits {
    pub batter: String,
    pub pitcher: String,
    pub men_on_base: String,
}

/// Data regarding a baserunner.
#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "_debug", serde(deny_unknown_fields))]
pub struct RunnerData {
    pub movement: RunnerMovement,
    pub details: RunnerDetails,
    #[serde(default)]
    pub credits: Vec<RunnerCredit>,
}

/// Data regarding the basepath of a runner
#[serde_as]
#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "_debug", serde(deny_unknown_fields))]
pub struct RunnerMovement {
    /// The base the runner begins the play at. `None` if they do not start on-base at the beginning of the play.
    pub origin_base: Option<Base>,

    /// Unsure how it is different from ``origin_base``
    #[serde(rename = "start")]
    pub start_base: Option<Base>,

    /// The latest base the runner is called "safe" at. `None` if the runner was never safe at any base.
    #[serde(rename = "end")]
    pub end_base: Option<Base>,

    /// The base the runner was called out at. `None` if the runner was never called out.
    pub out_base: Option<Base>,

    /// Identical to `out_base.is_some()`
    #[serde_as(deserialize_as = "DefaultOnNull")]
    #[serde(default)]
    pub is_out: bool,
    
    /// Ordinal of out in the game. `None` if the runner was not called out. Otherwise 1, 2, or 3.
    pub out_number: Option<usize>,
}

/// Details about the runner's movement
#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "_debug", serde(deny_unknown_fields))]
pub struct RunnerDetails {
    /// `None` represents completely unforced movement, such as hitting a single with no-one on.
    pub movement_reason: Option<MovementReason>,
    pub runner: NamedPerson,
    pub is_scoring_event: bool,
    #[serde(rename = "rbi")]
    pub is_rbi: bool,
    #[serde(rename = "earned")]
    pub is_earned: bool,

    // Same as [`PlayDetails`].event
    #[doc(hidden)]
    #[serde(rename = "eventType", default)]
    pub __event_tyoe: IgnoredAny,

    #[doc(hidden)]
    #[serde(rename = "event", default)]
    pub __event_type: IgnoredAny,

    #[doc(hidden)]
    #[serde(rename = "responsiblePitcher", default)]
    pub __responsible_pitcher: IgnoredAny,

    #[doc(hidden)]
    #[serde(rename = "teamUnearned", default)]
    pub __team_unearned: IgnoredAny,
    
    #[doc(hidden)]
    #[serde(rename = "playIndex", default)]
    pub __play_index: IgnoredAny,
}

/// Reasons for baserunner movement
#[derive(Debug, Deserialize, PartialEq, Eq, Copy, Clone, Display)]
pub enum MovementReason {
    //// Unforced base advancement, such as going first to third on a single.
    #[display("Unforced Base Advancement")]
    #[serde(rename = "r_adv_play")]
    AdvancementUnforced,
    
    /// Forced base advancement, such as moving up one base on a single.
    #[display("Forced Base Advancement")]
    #[serde(rename = "r_adv_force")]
    AdancementForced,

    /// Advancement from a choice in throwing, such as throwing home and allowing this runner to move up a base instead.
    #[display("Advancement from Throw")]
    #[serde(rename = "r_adv_throw")]
    AdvancementThrow,

    /// Runner fails to tag up and is forced out.
    #[display("Doubled Off")]
    #[serde(rename = "r_doubled_off")]
    DoubledOff,

    #[display("Thrown Out")]
    #[serde(rename = "r_thrown_out")]
    ThrownOut,

    #[display("Called Out Returning")]
    #[serde(rename = "r_out_returning")]
    CalledOutReturning,
        
    /// Standard force-out.
    #[display("Forced Out")]
    #[serde(rename = "r_force_out")]
    ForceOut,

    /// Deviation from the basepath, etc.
    #[display("Runner Called Out")]
    #[serde(rename = "r_runner_out")]
    RunnerCalledOut,

    /// A Stolen Base with no throw.
    #[display("Defensive Indifference")]
    #[serde(rename = "r_defensive_indiff")]
    DefensiveIndifference,

    #[display("Rundown")]
    #[serde(rename = "r_rundown")]
    Rundown,

    /// Stretching a single into a double.
    #[display("Out Stretching")]
    #[serde(rename = "r_out_stretching")]
    OutStretching,

    #[display("Stolen Base (2B)")]
    #[serde(rename = "r_stolen_base_2b")]
    StolenBase2B,

    #[display("Stolen Base (3B)")]
    #[serde(rename = "r_stolen_base_3b")]
    StolenBase3B,

    #[display("Stolen Base (HP)")]
    #[serde(rename = "r_stolen_base_home")]
    StolenBaseHome,

    #[display("Caught Stealing (2B)")]
    #[serde(rename = "r_caught_stealing_2b")]
    CaughtStealing2B,

    #[display("Caught Stealing (3B)")]
    #[serde(rename = "r_caught_stealing_3b")]
    CaughtStealing3B,

    #[display("Caught Stealing (HP)")]
    #[serde(rename = "r_caught_stealing_home")]
    CaughtStealingHome,
    
    /// Successful pickoff
    #[display("Pickoff (1B)")]
    #[serde(rename = "r_pickoff_1b")]
    Pickoff1B,
    
    /// Successful pickoff
    #[display("Pickoff (2B)")]
    #[serde(rename = "r_pickoff_2b")]
    Pickoff2B,
    
    /// Successful pickoff
    #[display("Pickoff (3B)")]
    #[serde(rename = "r_pickoff_3b")]
    Pickoff3B,
    
    #[display("Pickoff (Error) (1B)")]
    #[serde(rename = "r_pickoff_error_1b")]
    PickoffError1B,
    
    #[display("Pickoff (Error) (2B)")]
    #[serde(rename = "r_pickoff_error_2b")]
    PickoffError2B,
    
    #[display("Pickoff (Error) (3B)")]
    #[serde(rename = "r_pickoff_error_3b")]
    PickoffError3B,

    #[display("Pickoff (Caught Stealing) (2B)")]
    #[serde(rename = "r_pickoff_caught_stealing_2b")]
    PickoffCaughtStealing2B,
    
    #[display("Pickoff (Caught Stealing) (3B)")]
    #[serde(rename = "r_pickoff_caught_stealing_3b")]
    PickoffCaughtStealing3B,
    
    #[display("Pickoff (Caught Stealing) (HP)")]
    #[serde(rename = "r_pickoff_caught_stealing_home")]
    PickoffCaughtStealingHome,

    /// General interference
    #[display("Interference")]
    #[serde(rename = "r_interference")]
    Interference,

    #[display("Hit By Ball")]
    #[serde(rename = "r_hbr")]
    HitByBall,
}

impl MovementReason {
    /// If the movement reason is a pickoff
    #[must_use]
    pub const fn is_pickoff(self) -> bool {
        matches!(self, Self::Pickoff1B | Self::Pickoff2B | Self::Pickoff3B | Self::PickoffError1B | Self::PickoffError2B | Self::PickoffError3B | Self::PickoffCaughtStealing2B | Self::PickoffCaughtStealing3B | Self::PickoffCaughtStealingHome)
    }

    /// If the movement reason is a stolen base attempt
    #[must_use]
    pub const fn is_stolen_base_attempt(self) -> bool {
        matches!(self, Self::StolenBase2B | Self::StolenBase3B | Self::StolenBaseHome | Self::CaughtStealing2B | Self::CaughtStealing3B | Self::CaughtStealingHome | Self::PickoffCaughtStealing2B | Self::PickoffCaughtStealing3B | Self::PickoffCaughtStealingHome)
    }

    /// If the movement reason is a stolen base
    #[must_use]
    pub const fn is_stolen_base(self) -> bool {
        matches!(self, Self::StolenBase2B | Self::StolenBase3B | Self::StolenBaseHome)
    }
}

/// Fielder credits to outs
#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "_debug", serde(deny_unknown_fields))]
pub struct RunnerCredit {
    pub player: PersonId,
    pub position: NamedPosition,
    pub credit: CreditKind,
}

/// Statistical credits to fielders; putouts, assists, etc.
#[derive(Debug, Deserialize, PartialEq, Eq, Copy, Clone, Display)]
pub enum CreditKind {
    #[display("Putout")]
    #[serde(rename = "f_putout")]
    Putout,
    
    #[display("Assist")]
    #[serde(rename = "f_assist")]
    Assist,
    
    #[display("Outfield Assist")]
    #[serde(rename = "f_assist_of")]
    OutfieldAssist,
    
    /// The fielder just, fielded the ball, no outs or anything.
    #[display("Fielded Ball")]
    #[serde(rename = "f_fielded_ball")]
    FieldedBall,
    
    #[display("Fielding Error")]
    #[serde(rename = "f_fielding_error")]
    FieldingError,
    
    #[display("Throwing Error")]
    #[serde(rename = "f_throwing_error")]
    ThrowingError,
    
    #[display("Deflection")]
    #[serde(rename = "f_deflection")]
    Deflection,

    /// They literally touched it, no deflection, just a tap.
    #[display("Touch")]
    #[serde(rename = "f_touch")]
    Touch,

    #[display("Dropped Ball Error")]
    #[serde(rename = "f_error_dropped_ball")]
    DroppedBallError,

    #[display("Defensive Shift Violation")]
    #[serde(rename = "f_defensive_shift_violation_error")]
    DefensiveShiftViolation,

    #[display("Interference")]
    #[serde(rename = "f_interference")]
    Interference,

    #[display("Catcher's Interference")]
    #[serde(rename = "c_catcher_interf")]
    CatchersInterference,
}

/// # Errors
/// See `D::Error`, likely [`serde_json::Error`]
pub fn deserialize_review_data<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Vec<ReviewData>, D::Error> {
    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[cfg_attr(feature = "_debug", serde(deny_unknown_fields))]
    struct RawReviewData {
        #[serde(flatten)]
        base: ReviewData,
        #[serde(default)]
        additional_reviews: Vec<ReviewData>,
    }

    let RawReviewData { base, mut additional_reviews } = RawReviewData::deserialize(deserializer)?;
    additional_reviews.insert(0, base);
    Ok(additional_reviews)
}

/// Data regarding replay reviews; present on [`Play`], not [`PlayEvent`].
#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "_debug", serde(deny_unknown_fields))]
pub struct ReviewData {
    pub is_overturned: bool,
    #[serde(rename = "inProgress")]
    pub is_in_progress: bool,
    pub review_type: ReviewReasonId,
    /// If `None`, then a crew-chief review.
    #[serde(alias = "challengeTeamId")]
    pub challenging_team: Option<TeamId>,
    /// For ABS challenges
    pub player: Option<NamedPerson>,
}

/// An "indivisible" play, such as pickoff, pitch, stolen base, etc.
#[allow(clippy::large_enum_variant, reason = "not a problemo dw")]
#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(rename_all_fields = "camelCase", tag = "type")]
#[cfg_attr(feature = "_debug", serde(deny_unknown_fields))]
pub enum PlayEvent {
    #[serde(rename = "action")]
    Action {
        details: ActionPlayDetails,
        #[serde(rename = "actionPlayId")]
        play_id: Option<Uuid>,
        
        #[serde(flatten)]
        common: PlayEventCommon,
    },
    #[serde(rename = "pitch")]
    Pitch {
        details: PitchPlayDetails,
        pitch_data: Option<PitchData>,
        hit_data: Option<HitData>,
        /// Starts at 1
        #[serde(rename = "pitchNumber")]
        pitch_ordinal: usize,
        play_id: Uuid,

        #[serde(flatten)]
        common: PlayEventCommon,
    },
    #[serde(rename = "stepoff")]
    Stepoff {
        details: StepoffPlayDetails,
        play_id: Option<Uuid>,
        
        #[serde(flatten)]
        common: PlayEventCommon,
    },
    #[serde(rename = "no_pitch")]
    NoPitch {
        details: NoPitchPlayDetails,
        play_id: Option<Uuid>,
        /// Starts at 1
        #[serde(rename = "pitchNumber", default)]
        pitch_ordinal: usize,

        #[serde(flatten)]
        common: PlayEventCommon,
    },
    #[serde(rename = "pickoff")]
    Pickoff {
        details: PickoffPlayDetails,
        #[serde(alias = "actionPlayId", default)]
        play_id: Option<Uuid>,

        #[serde(flatten)]
        common: PlayEventCommon,
    }
}

impl Deref for PlayEvent {
    type Target = PlayEventCommon;

    fn deref(&self) -> &Self::Target {
        let (Self::Action { common, .. } | Self::Pitch { common, .. } | Self::Stepoff { common, .. } | Self::NoPitch { common, .. } | Self::Pickoff { common, .. }) = self;
        common
    }
}

impl DerefMut for PlayEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        let (Self::Action { common, .. } | Self::Pitch { common, .. } | Self::Stepoff { common, .. } | Self::NoPitch { common, .. } | Self::Pickoff { common, .. }) = self;
        common
    }
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PlayEventCommon {
    /// At the end of the play event
    pub count: AtBatCount,
    #[serde(rename = "startTime", deserialize_with = "crate::deserialize_datetime")]
    pub start_timestamp: DateTime<Utc>,
    #[serde(rename = "endTime", deserialize_with = "crate::deserialize_datetime")]
    pub end_timestamp: DateTime<Utc>,
    pub is_pitch: bool,
    #[serde(rename = "isBaseRunningPlay", default)]
    pub is_baserunning_play: bool,
    /// Pitching Subsitution, Defensive Switches, Pinch Hitting, etc.
    #[serde(default)]
    pub is_substitution: bool,

    /// A player involved in the play.
    pub player: Option<PersonId>,
    /// An umpire involved in the play, ex: Ejection
    pub umpire: Option<PersonId>,
    /// Position (typically a complement of ``player``)
    pub position: Option<NamedPosition>,
    /// Also not always present, check by the [`EventType`]; [`PitchingSubsitution`](EventType::PitchingSubstitution)s don't have it.
    pub replaced_player: Option<PersonId>,
    /// Batting Order Index, typically supplied with a [`DefensiveSwitch`](EventType::DefensiveSwitch) or [`OffensiveSubstitution`](EventType::OffensiveSubstitution)
    #[serde(rename = "battingOrder")]
    pub batting_order_index: Option<BattingOrderIndex>,
    /// Base correlated with play, such as a stolen base
    pub base: Option<Base>,
    #[serde(rename = "reviewDetails", default, deserialize_with = "deserialize_review_data")]
    pub reviews: Vec<ReviewData>,
    pub injury_type: Option<String>,
    
    #[doc(hidden)]
    #[serde(rename = "index", default)]
    pub __index: IgnoredAny,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "_debug", serde(deny_unknown_fields))]
pub struct ActionPlayDetails {
    #[serde(rename = "eventType")]
    pub event: EventType,
    /// Shohei Ohtani strikes out swinging.
    pub description: String,

    /// Score as of the end of the play
    pub away_score: usize,
    /// Score as of the end of the play
    pub home_score: usize,

    /// Whether the batter in the play is out
    pub is_out: bool,
    pub is_scoring_play: bool,
    #[serde(default)]
    pub has_review: bool,

    #[serde(rename = "disengagementNum", default)]
    pub disengagements: Option<NonZeroUsize>,
    
    #[doc(hidden)]
    #[serde(rename = "event", default)]
    pub __event: IgnoredAny,

    #[doc(hidden)]
    #[serde(rename = "type", default)]
    pub __type: IgnoredAny,

    // redundant
    #[doc(hidden)]
    #[serde(rename = "violation", default)]
    pub __violation: IgnoredAny,
}

#[allow(clippy::struct_excessive_bools, reason = "inapplicable")]
#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "_debug", serde(deny_unknown_fields))]
pub struct PitchPlayDetails {
    pub is_in_play: bool,
    pub is_strike: bool,
    pub is_ball: bool,
    pub is_out: bool,
    #[serde(default)]
    pub has_review: bool,
    #[serde(default)]
    pub runner_going: bool,
    #[serde(rename = "disengagementNum", default)]
    pub disengagements: Option<NonZeroUsize>,
    
    #[serde(rename = "type", deserialize_with = "crate::meta::fallback_pitch_type_deserializer", default = "crate::meta::unknown_pitch_type")]
    pub pitch_type: PitchType,

    pub call: PitchCodeId,

    #[doc(hidden)]
    #[serde(rename = "ballColor", default)]
    pub __ball_color: IgnoredAny,

    #[doc(hidden)]
    #[serde(rename = "trailColor", default)]
    pub __trail_color: IgnoredAny,

    #[doc(hidden)]
    #[serde(rename = "description", default)]
    pub __description: IgnoredAny,

    #[doc(hidden)]
    #[serde(rename = "code", default)]
    pub __code: IgnoredAny,

    // redundant
    #[doc(hidden)]
    #[serde(rename = "violation", default)]
    pub __violation: IgnoredAny,
}

#[allow(clippy::struct_excessive_bools, reason = "inapplicable")]
#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "_debug", serde(deny_unknown_fields))]
pub struct StepoffPlayDetails {
    pub description: String,
    /// Typically "PSO" - "Pitcher Step Off"
    pub code: PitchCodeId,
    pub is_out: bool,
    #[serde(default)]
    pub has_review: bool,
    /// Catcher-called mound disengagement.
    pub from_catcher: bool,
    #[serde(rename = "disengagementNum", default)]
    pub disengagements: Option<NonZeroUsize>,

    // redundant
    #[doc(hidden)]
    #[serde(rename = "violation", default)]
    pub __violation: IgnoredAny,
}

#[allow(clippy::struct_excessive_bools, reason = "inapplicable")]
#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "_debug", serde(deny_unknown_fields))]
pub struct NoPitchPlayDetails {
    #[serde(default)]
    pub is_in_play: bool,
    #[serde(default)]
    pub is_strike: bool,
    #[serde(default)]
    pub is_ball: bool,
    pub is_out: bool,
    #[serde(default)]
    pub has_review: bool,
    #[serde(default)]
    pub runner_going: bool,

    #[serde(default = "crate::meta::unknown_pitch_code")]
    pub call: PitchCodeId,

    #[serde(rename = "disengagementNum", default)]
    pub disengagements: Option<NonZeroUsize>,
    
    #[doc(hidden)]
    #[serde(rename = "description", default)]
    pub __description: IgnoredAny,

    #[doc(hidden)]
    #[serde(rename = "code", default)]
    pub __code: IgnoredAny,

    // redundant
    #[doc(hidden)]
    #[serde(rename = "violation", default)]
    pub __violation: IgnoredAny,
}

#[allow(clippy::struct_excessive_bools, reason = "inapplicable")]
#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "_debug", serde(deny_unknown_fields))]
pub struct PickoffPlayDetails {
    pub description: String,
    /// Typically "1" - "Pickoff Attempt 1B", "2" - "Pickoff Attempt 2B", "3" - "Pickoff Attempt 3B"
    pub code: PitchCodeId,
    pub is_out: bool,
    #[serde(default)]
    pub has_review: bool,
    /// Catcher-called pickoff.
    pub from_catcher: bool,

    #[serde(rename = "disengagementNum", default)]
    pub disengagements: Option<NonZeroUsize>,

    // redundant
    #[doc(hidden)]
    #[serde(rename = "violation", default)]
    pub __violation: IgnoredAny,
}

/// Statistical data regarding a pitch.
///
/// Some acronyms are an existing spec, best to keep with that.
#[allow(non_snake_case, reason = "spec")]
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct PitchData {
    /// Velocity measured at release, measured in mph
    pub release_speed: f64,
    /// Velocity measured crossing home plate, measured in mph
    pub plate_speed: f64,
    
    /// Height above home plate for the bottom of the hitter's strike zone
    ///
    /// Measured in feet.
    pub sz_bot: f64,
    /// Height above home plate for the top of the hitter's strike zone
    ///
    /// Measured in feet.
    pub sz_top: f64,
    /// Width of the strike zone
    ///
    /// Measured in inches.
    pub sz_wid: f64,
    /// Depth of the strike zone
    ///
    /// Measured in inches.
    pub sz_dep: f64,
    
    /// Acceleration of the pitch near release, horizontal movement axis,
    /// catchers perspective (positive means RHP sweep)
    /// 
    /// Measured in feet/s^2.
    pub aX: f64,
    /// Acceleration of the pitch near release, depth axis,
    /// catchers perspective (positive means deceleration)
    /// 
    /// Measured in feet/s^2.
    pub aY: f64,
    /// Acceleration of the pitch near release, vertical movement axis,
    /// catchers perspective (positive means literal carry)
    ///
    /// Measured in feet/s^2.
    pub aZ: f64,
    
    /// Might be broken, use ``horizontal_movement`` instead
    ///
    /// Horizontal movement of the pitch between the release point and home plate,
    /// catchers perspective (positive means RHP sweep)
    /// as compared to a theoretical pitch thrown at with the same velocity vector
    /// and no spin-induced movement.
    /// This parameter is measured at y=40 feet regardless of the y0 value.
    /// 
    /// Measured in inches.
    ///
    /// Does not account for seam-shifted wake!
    pub pfxX: f64,
    /// Might be broken, use ``vertical_drop`` instead
    ///
    /// Vertical movement of the pitch between the release point and home plate,
    /// catchers perspective (positive means literal rise),
    /// as compared to a theoretical pitch thrown at with the same velocity vector
    /// and no spin-induced movement.
    /// This parameter is measured at y=40 feet regardless of the y0 value.
    /// 
    /// Measured in inches.
    ///
    /// Does not account for seam-shifted wake!
    pub pfxZ: f64,

    /// Horizontal coordinate of the pitch as it crosses home plate, 0 is the middle of the plate.
    /// Catchers perspective, positive means arm-side for a RHP, negative means glove-side for a RHP.
    /// 
    /// Measured in feet.
    pub pX: f64,
    /// Vertical coordinate of the pitch as it crosses home plate, 0 is the plate itself
    /// 
    /// Measured in feet.
    pub pZ: f64,

    /// Horizontal component of velocity out of the hand, catchers perspective, positive means RHP glove-side.
    ///
    /// Measured in feet per second.
    pub vX0: f64,
    /// Depth component of velocity out of the hand, catchers perspective, positive means the ball isn't going into centerfield.
    ///
    /// Measured in feet per second.
    pub vY0: f64,
    /// Vertical component of velocity out of the hand, measured in feet per second.
    ///
    /// Measured in feet per second.
    pub vZ0: f64,

    /// X coordinate of pitch at release
    ///
    /// Measured in feet
    pub x0: f64,
    /// Y coordinate of pitch at release, typically as close to 50 as possible.
    ///
    /// Measured in feet
    pub y0: f64,
    /// Z coordinate of pitch at release
    ///
    /// Measured in feet
    pub z0: f64,

    /// No clue.
    pub x: f64,
    /// No clue.
    pub y: f64,

    /// No clue. Does not match theta angle of induced break vector. Consistently 36.0. Strange.
    pub break_angle: f64,
    /// No clue. Does not match length of induced break vector.
    pub break_length: f64,

    /// Standard metric, amount of vertical movement induced.
    ///
    /// Measured in inches
    pub induced_vertical_movement: f64,
    /// Standard metric, amount of vertical movement the pitch has (including gravity).
    ///
    /// Measured in inches
    pub vertical_drop: f64,
    /// Standard metric, amount of horizontal movement the pitch has.
    ///
    /// Measured in inches
    pub horizontal_movement: f64,
    /// No clue. Thought to be the amount of depth-based movement (acceleration), but it's consistently 24.0. Strange.
    pub depth_break: f64,
    
    /// RPMs out of the hand
    pub spin_rate: f64,

    /// Measured in degrees.
    ///
    /// 0 means complete topspin.
    /// 180 means complete backspin.
    /// 90 means complete sidespin (RHP sweeper).
    /// 270 means complete sidespin (elite RHP changeup).
    /// 
    /// ~225 is your average RHP fastball.
    pub spin_axis: f64,

    pub zone: StrikeZoneSection,

    /// AI model confidence about pitch type designation.
    ///
    /// Sometimes greater than 1.0
    pub type_confidence: f64,

    /// Time from out of the hand to crossing the plate.
    ///
    /// Measured in seconds
    pub time_to_plate: f64,

    /// Extension
    ///
    /// Measured in feet
    pub extension: f64,
}

impl<'de> Deserialize<'de> for PitchData {
    #[allow(clippy::too_many_lines, reason = "deserialization is hard")]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>
    {
        #[must_use]
        const fn default_strike_zone_width() -> f64 {
            17.0
        }

        #[must_use]
        const fn default_strike_zone_depth() -> f64 {
            17.0
        }

        #[must_use]
        const fn default_nan() -> f64 {
            f64::NAN
        }

        #[must_use]
        const fn default_strike_zone_section() -> StrikeZoneSection {
            StrikeZoneSection::MiddleMiddle
        }
        
        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        #[cfg_attr(feature = "_debug", serde(deny_unknown_fields))]
        struct Raw {
            #[serde(default = "default_nan")]
            start_speed: f64,
            #[serde(default = "default_nan")]
            end_speed: f64,
            #[serde(default = "default_nan")]
            strike_zone_top: f64,
            #[serde(default = "default_nan")]
            strike_zone_bottom: f64,
            #[serde(default = "default_strike_zone_width")]
            strike_zone_width: f64,
            #[serde(default = "default_strike_zone_depth")]
            strike_zone_depth: f64,
            coordinates: RawCoordinates,
            breaks: RawBreaks,
            #[serde(default = "default_strike_zone_section")]
            zone: StrikeZoneSection,
            #[serde(default = "default_nan")]
            type_confidence: f64,
            #[serde(default = "default_nan")]
            plate_time: f64,
            #[serde(default = "default_nan")]
            extension: f64,
        }

        #[allow(non_snake_case, reason = "spec")]
        #[derive(Deserialize)]
        #[cfg_attr(feature = "_debug", serde(deny_unknown_fields))]
        struct RawCoordinates {
            #[serde(default = "default_nan")]
            aX: f64,
            #[serde(default = "default_nan")]
            aY: f64,
            #[serde(default = "default_nan")]
            aZ: f64,
            #[serde(default = "default_nan")]
            pfxX: f64,
            #[serde(default = "default_nan")]
            pfxZ: f64,
            #[serde(default = "default_nan")]
            pX: f64,
            #[serde(default = "default_nan")]
            pZ: f64,
            #[serde(default = "default_nan")]
            vX0: f64,
            #[serde(default = "default_nan")]
            vY0: f64,
            #[serde(default = "default_nan")]
            vZ0: f64,
            #[serde(default = "default_nan")]
            x: f64,
            #[serde(default = "default_nan")]
            y: f64,
            #[serde(default = "default_nan")]
            x0: f64,
            #[serde(default = "default_nan")]
            y0: f64,
            #[serde(default = "default_nan")]
            z0: f64,
        }

        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        #[cfg_attr(feature = "_debug", serde(deny_unknown_fields))]
        struct RawBreaks {
            #[serde(default = "default_nan")]
            break_angle: f64,
            #[serde(default = "default_nan")]
            break_length: f64,
            #[serde(default = "default_nan")]
            break_y: f64,
            #[serde(default = "default_nan")]
            break_vertical: f64,
            #[serde(default = "default_nan")]
            break_vertical_induced: f64,
            #[serde(default = "default_nan")]
            break_horizontal: f64,
            #[serde(default = "default_nan")]
            spin_rate: f64,
            #[serde(default = "default_nan")]
            spin_direction: f64,
        }

        let Raw {
            start_speed,
            end_speed,
            strike_zone_top,
            strike_zone_bottom,
            strike_zone_width,
            strike_zone_depth,
            coordinates: RawCoordinates {
                pfxX,
                pfxZ,
                aX,
                aY,
                aZ,
                pX,
                pZ,
                vX0,
                vY0,
                vZ0,
                x,
                y,
                x0,
                y0,
                z0,
            },
            breaks: RawBreaks {
                break_angle,
                break_length,
                break_y,
                break_vertical,
                break_vertical_induced,
                break_horizontal,
                spin_rate,
                spin_direction,
            },
            zone,
            type_confidence,
            plate_time,
            extension,
        } =  Raw::deserialize(deserializer)?;

        Ok(Self {
            release_speed: start_speed,
            plate_speed: end_speed,
            sz_bot: strike_zone_bottom,
            sz_top: strike_zone_top,
            sz_wid: strike_zone_width,
            sz_dep: strike_zone_depth,
            aX,
            aY,
            aZ,
            pfxX,
            pfxZ,
            pX,
            pZ,
            vX0,
            vY0,
            vZ0,
            x0,
            y0,
            z0,
            horizontal_movement: break_horizontal,
            x,
            y,
            break_angle,
            break_length,
            induced_vertical_movement: break_vertical_induced,
            vertical_drop: break_vertical,
            depth_break: break_y,
            spin_rate,
            spin_axis: spin_direction,
            zone,
            type_confidence,
            time_to_plate: plate_time,
            extension,
        })
    }
}

/// Data regarding batted-balls
#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(from = "__HitDataStruct")]
pub struct HitData {
    /// sometimes just takes a second to be present
    pub hit_trajectory: Option<HitTrajectory>,
    /// sometimes just takes a second to be present
    pub contact_hardness: Option<ContactHardness>,
    pub statcast: Option<StatcastHitData>,
}

#[serde_as]
#[doc(hidden)]
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "_debug", serde(deny_unknown_fields))]
struct __HitDataStruct {
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[serde(rename = "trajectory", default)]
    hit_trajectory: Option<HitTrajectory>,
    #[serde(rename = "hardness", default)]
    contact_hardness: Option<ContactHardness>,

    #[serde(flatten, default)]
    statcast: Option<StatcastHitData>,

    #[doc(hidden)]
    #[serde(rename = "location", default)]
    __location: IgnoredAny,

    #[doc(hidden)]
    #[serde(rename = "coordinates")]
    __coordinates: IgnoredAny,
}

impl From<__HitDataStruct> for HitData {
    fn from(__HitDataStruct { hit_trajectory, contact_hardness, statcast, .. }: __HitDataStruct) -> Self {
        Self {
            hit_trajectory: hit_trajectory.or_else(|| statcast.as_ref().map(|statcast| statcast.launch_angle).map(HitTrajectory::from_launch_angle)),
            contact_hardness,
            statcast,
        }
    }
}

/// Statcast data regarding batted balls, only sometimes present.
#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "_debug", serde(deny_unknown_fields))]
pub struct StatcastHitData {
    /// Speed of the ball as it leaves the bat
    ///
    /// Measured in mph
    #[serde(rename = "launchSpeed")]
    pub exit_velocity: f64,
    /// Vertical Angle in degrees at which the ball leaves the bat.
    ///
    /// Measured in degrees
    pub launch_angle: f64,

    /// Distance the ball travels before being caught or rolling.
    ///
    /// Measured in feet
    #[serde(rename = "totalDistance")]
    pub distance: f64,
}

#[derive(Builder)]
#[builder(derive(Into))]
pub struct PlayByPlayRequest {
    #[builder(into)]
    id: GameId,
}

impl<S: play_by_play_request_builder::State + play_by_play_request_builder::IsComplete> crate::request::RequestURLBuilderExt for PlayByPlayRequestBuilder<S> {
    type Built = PlayByPlayRequest;
}

impl Display for PlayByPlayRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "http://statsapi.mlb.com/api/v1/game/{}/playByPlay", self.id)
    }
}

impl RequestURL for PlayByPlayRequest {
    type Response = Plays;
}

#[cfg(test)]
mod tests {
    use crate::TEST_YEAR;
    use crate::game::PlayByPlayRequest;
    use crate::meta::GameType;
    use crate::request::RequestURLBuilderExt;
    use crate::schedule::ScheduleRequest;
    use crate::season::{Season, SeasonsRequest};
    use crate::sport::SportId;

    #[tokio::test]
    async fn ws_gm7_2025_pbp() {
        let _ = PlayByPlayRequest::builder().id(813_024).build_and_get().await.unwrap();
    }

    #[tokio::test]
    async fn postseason_pbp() {
        let [season]: [Season; 1] = SeasonsRequest::builder().season(TEST_YEAR).sport_id(SportId::MLB).build_and_get().await.unwrap().seasons.try_into().unwrap();
		let postseason = season.postseason.expect("Expected the MLB to have a postseason");
		let games = ScheduleRequest::<()>::builder().date_range(postseason).sport_id(SportId::MLB).build_and_get().await.unwrap();
		let games = games.dates.into_iter().flat_map(|date| date.games).filter(|game| game.game_type.is_postseason()).map(|game| game.game_id).collect::<Vec<_>>();
		let mut has_errors = false;
		for game in games {
			if let Err(e) = PlayByPlayRequest::builder().id(game).build_and_get().await {
			    dbg!(e);
			    has_errors = true;
			}
		}
		assert!(!has_errors, "Has errors.");
    }

    #[cfg_attr(not(feature = "_heavy_tests"), ignore)]
    #[tokio::test]
    async fn regular_season_pbp() {
        let [season]: [Season; 1] = SeasonsRequest::builder().season(TEST_YEAR).sport_id(SportId::MLB).build_and_get().await.unwrap().seasons.try_into().unwrap();
        let regular_season = season.regular_season;
        let games = ScheduleRequest::<()>::builder().date_range(regular_season).sport_id(SportId::MLB).build_and_get().await.unwrap();
        let games = games.dates.into_iter().flat_map(|date| date.games).filter(|game| game.game_type == GameType::RegularSeason).collect::<Vec<_>>();
        let mut has_errors = false;
        for game in games {
            if let Err(e) = PlayByPlayRequest::builder().id(game.game_id).build_and_get().await {
                dbg!(e);
                has_errors = true;
            }
        }
        assert!(!has_errors, "Has errors.");
    }
}
