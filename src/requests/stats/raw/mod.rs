use std::ops::Add;
use serde::{Deserialize, Deserializer};
use serde::de::DeserializeOwned;
use thiserror::Error;

macro_rules! register_fields {
    ($d:tt $($name:ident: $ty:ty),+ $(,)?) => {
        macro_rules! api_name_to_type {
            $(
            ($name) => { $ty };
            )+
            ($d ($d t:tt)*) => { compile_error!("Invalid field name") };
        }
    };
}

register_fields![ $
    games_played: crate::stats::units::CountingStat,
    games_pitched: crate::stats::units::CountingStat,
    games_started: crate::stats::units::CountingStat,
    games_finished: crate::stats::units::CountingStat,
    groundouts: crate::stats::units::CountingStat,
    airouts: crate::stats::units::CountingStat,
    runs: crate::stats::units::CountingStat,
    doubles: crate::stats::units::CountingStat,
    triples: crate::stats::units::CountingStat,
    home_runs: crate::stats::units::CountingStat,
    strikeouts: crate::stats::units::CountingStat,
    base_on_balls: crate::stats::units::CountingStat,
    intentional_walks: crate::stats::units::CountingStat,
    hits: crate::stats::units::CountingStat,
    hit_by_pitch: crate::stats::units::CountingStat,
    at_bats: crate::stats::units::CountingStat,
    caught_stealing: crate::stats::units::CountingStat,
    stolen_bases: crate::stats::units::CountingStat,
    grounded_into_double_play: crate::stats::units::CountingStat,
    grounded_into_triple_play: crate::stats::units::CountingStat,
    grounded_into_double_play_opponent: crate::stats::units::CountingStat,
    number_of_pitches: crate::stats::units::CountingStat,
    innings_pitched: crate::stats::units::InningsPitched,
    plate_appearances: crate::stats::units::CountingStat,
    total_bases: crate::stats::units::CountingStat,
    rbi: crate::stats::units::CountingStat,
    left_on_base: crate::stats::units::CountingStat,
    sac_bunts: crate::stats::units::CountingStat,
    sac_flies: crate::stats::units::CountingStat,
    bunts_failed: crate::stats::units::CountingStat,
    bunts_missed_tipped: crate::stats::units::CountingStat,
    catchers_interference: crate::stats::units::CountingStat,
    wOBA: crate::stats::units::ThreeDecimalPlaceRateStat,
    wRAA: crate::stats::units::FloatCountingStat<2>,
    wRC: crate::stats::units::FloatCountingStat<2>,
    wRCp: crate::stats::units::FloatCountingStat<0>,
    RAR: crate::stats::units::FloatCountingStat<1>,
    bWAR: crate::stats::units::FloatCountingStat<1>,
    fWAR: crate::stats::units::FloatCountingStat<1>,
    FIP: crate::stats::units::TwoDecimalPlaceRateStat,
    xFIP: crate::stats::units::TwoDecimalPlaceRateStat,
    xAVG: crate::stats::units::ThreeDecimalPlaceRateStat,
    xSLG: crate::stats::units::ThreeDecimalPlaceRateStat,
    wOBA: crate::stats::units::ThreeDecimalPlaceRateStat,
    xwOBA: crate::stats::units::ThreeDecimalPlaceRateStat,
    wOBACON: crate::stats::units::ThreeDecimalPlaceRateStat,
    xwOBACON: crate::stats::units::ThreeDecimalPlaceRateStat,
    wLeague: crate::stats::units::FloatCountingStat<1>,
    FIPm: crate::stats::units::TwoDecimalPlaceRateStat,
    shutdowns: crate::stats::units::CountingStat,
    meltdowns: crate::stats::units::CountingStat,
    leverage_index: crate::stats::units::TwoDecimalPlaceRateStat,
    inning_start_leverage_index: crate::stats::units::TwoDecimalPlaceRateStat,
    game_leverage_index: crate::stats::units::TwoDecimalPlaceRateStat,
    exiting_leverage_index: crate::stats::units::TwoDecimalPlaceRateStat,
    batting_run_value: crate::stats::units::FloatCountingStat<0>,
    fielding_run_value: crate::stats::units::FloatCountingStat<0>,
    baserunning_run_value: crate::stats::units::FloatCountingStat<0>,
    positional_run_adjustment: crate::stats::units::FloatCountingStat<1>,
    replacement_run_value: crate::stats::units::FloatCountingStat<0>,
    ERAm: crate::stats::units::FloatCountingStat<0>,
    SPD: crate::stats::units::TwoDecimalPlaceRateStat,
    UBR: crate::stats::units::TwoDecimalPlaceRateStat,
    wSB: crate::stats::units::TwoDecimalPlaceRateStat,
    wGDP: crate::stats::units::TwoDecimalPlaceRateStat,
    age: crate::stats::units::CountingStat,
    extra_base_hits: crate::stats::units::CountingStat,
    reached_on_error: crate::stats::units::CountingStat,
    walkoffs: crate::stats::units::CountingStat,
    flyouts: crate::stats::units::CountingStat,
    total_swings: crate::stats::units::CountingStat,
    whiffs: crate::stats::units::CountingStat,
    balls_in_play: crate::stats::units::CountingStat,
    popouts: crate::stats::units::CountingStat,
    lineouts: crate::stats::units::CountingStat,
    flyball_hits: crate::stats::units::CountingStat,
    popfly_hits: crate::stats::units::CountingStat,
    line_drive_hits: crate::stats::units::CountingStat,
    groundball_hits: crate::stats::units::CountingStat,
    wins: crate::stats::units::CountingStat,
    losses: crate::stats::units::CountingStat,
    saves: crate::stats::units::CountingStat,
    save_opportunities: crate::stats::units::CountingStat,
    holds: crate::stats::units::CountingStat,
    blown_saves: crate::stats::units::CountingStat,
    earned_runs: crate::stats::units::CountingStat,
    batters_faced: crate::stats::units::CountingStat,
    outs: crate::stats::units::CountingStat,
    outs_pitched: crate::stats::units::CountingStat,
    lob_wins: crate::stats::units::TwoDecimalPlaceRateStat,
    bip_wins: crate::stats::units::TwoDecimalPlaceRateStat,
    fdp_wins: crate::stats::units::TwoDecimalPlaceRateStat,
    quality_starts: crate::stats::units::CountingStat,
    complete_games: crate::stats::units::CountingStat,
    shutouts: crate::stats::units::CountingStat,
    strikes: crate::stats::units::CountingStat,
    balls: crate::stats::units::CountingStat,
    balks: crate::stats::units::CountingStat,
    wild_pitches: crate::stats::units::CountingStat,
    passed_balls: crate::stats::units::CountingStat,
    pickoffs: crate::stats::units::CountingStat,
    pickoff_attempts: crate::stats::units::CountingStat,
    inherited_runners: crate::stats::units::CountingStat,
    inherited_runners_scored: crate::stats::units::CountingStat,
    bequeathed_runners: crate::stats::units::CountingStat,
    bequeathed_runners_scored: crate::stats::units::CountingStat,
    run_support: crate::stats::units::CountingStat,
    assists: crate::stats::units::CountingStat,
    putouts: crate::stats::units::CountingStat,
    errors: crate::stats::units::CountingStat,
    chances: crate::stats::units::CountingStat,
    position: crate::positions::NamedPosition,
    innings: crate::stats::units::InningsPitched,
    games: crate::stats::units::CountingStat,
    double_plays: crate::stats::units::CountingStat,
    triple_plays: crate::stats::units::CountingStat,
    throwing_errors: crate::stats::units::CountingStat,
];

macro_rules! group_and_type {
    ($name:ident { $($(#[$meta:meta])* $serde:literal => $piece:ident),* $(,)? }) => {
        ::pastey::paste! {
            #[doc(hidden)]
            #[allow(non_snake_case)]
            #[derive(Debug, ::serde::Deserialize, Clone)]
            // #[cfg_attr(test, serde(deny_unknown_fields))] // todo: find out how to fix this to discard some
            pub struct [<__ $name StatsData>] {
                $(
                #[serde(deserialize_with = "crate::stats::raw::deserialize_stat", rename = $serde)]
                #[cfg_attr(not(test), serde(default = "crate::stats::raw::default_stat"))]
                $(#[$meta])*
                pub $piece: Result<api_name_to_type![$piece], crate::stats::raw::OmittedStatError>,
                )*
            }

            impl ::std::cmp::PartialEq for [<__ $name StatsData>] {
                fn eq(&self, rhs: &Self) -> bool {
                    true && $(crate::stats::raw::stat_eq(&self.$piece, &rhs.$piece))&&*
                }
            }

            impl ::std::cmp::Eq for [<__ $name StatsData>] {}

            impl ::std::ops::Add for [<__ $name StatsData>]
            where
                $(
                for<'no_rfc_2056> api_name_to_type![$piece]: ::std::ops::Add
                ),*
            {
                type Output = Self;

                fn add(self, rhs: Self) -> Self::Output {
                    Self {
                        $(
                        $piece: crate::stats::raw::stat_add(self.$piece, rhs.$piece),
                        )*
                    }
                }
            }

            impl ::std::ops::AddAssign for [<__ $name StatsData>] where Self: ::std::ops::Add {
                fn add_assign(&mut self, rhs: Self) {
                    *self = *self + rhs;
                }
            }

            impl Default for [<__ $name StatsData>] {
                fn default() -> Self {
                    Self {
                        $(
                        $piece: Err(crate::stats::raw::OmittedStatError)
                        ),*
                    }
                }
            }

            impl crate::stats::RawStat for [<__ $name StatsData>] {}
        }
    };
}

pub mod pitching;
pub mod hitting;
pub mod fielding;
pub mod catching;

mod fielded_matchup;
mod spray_chart;
mod hit_spray;
mod hot_cold_zones;
mod pitch_usage;

pub use fielded_matchup::*;
pub use spray_chart::*;
pub use hit_spray::*;
pub use hot_cold_zones::*;
pub use pitch_usage::*;

/// For old data, some stats are just omitted, like pitch count and stuff. So every field is now possibly omittable. Gotta `?` everything now. (this api is legendary)
#[derive(Debug, Error, PartialEq, Eq, Copy, Clone)]
#[error("This stat was omitted.")]
pub struct OmittedStatError;

pub(crate) fn deserialize_stat<'de, D: Deserializer<'de>, T: DeserializeOwned>(deserializer: D) -> Result<Result<T, OmittedStatError>, D::Error> {
    Ok(Option::<T>::deserialize(deserializer)?.ok_or(OmittedStatError))
}

pub(crate) fn default_stat<T>() -> Result<T, OmittedStatError> {
    Err(OmittedStatError)
}

pub(crate) fn stat_eq<T: PartialEq>(a: &Result<T, OmittedStatError>, b: &Result<T, OmittedStatError>) -> bool {
    match (a, b) {
        (Ok(a), Ok(b)) => a == b,
        (Err(a), Err(b)) => a == b,
        _ => false,
    }
}

pub(crate) fn stat_add<T: Add>(a: Result<T, OmittedStatError>, b: Result<T, OmittedStatError>) -> Result<T, OmittedStatError> {
    match (a, b) {
        (Ok(a), Ok(b)) => Ok(a + b),
        // if one record does not have the value, the value is not 0 - it is unknown.
        _ => Err(OmittedStatError),
    }
}
