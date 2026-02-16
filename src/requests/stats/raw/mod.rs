use serde::{Deserialize, Deserializer};
use serde::de::DeserializeOwned;
use thiserror::Error;

macro_rules! api_names_to_types {
    ($d:tt $($name:ident: $ty:ty),+ $(,)?) => {
        macro_rules! api_name_to_type {
            $(
            ($d $name:ident) => { $ty };
            )+
        }
    };
}

api_names_to_types![ $
    gamesPlayed: crate::stats::units::CountingStat,
    groundOuts: crate::stats::units::CountingStat,
    airOuts: crate::stats::units::CountingStat,
    runs: crate::stats::units::CountingStat,
    doubles: crate::stats::units::CountingStat,
    triples: crate::stats::units::CountingStat,
    homeRuns: crate::stats::units::CountingStat,
    strikeOuts: crate::stats::units::CountingStat,
    baseOnBalls: crate::stats::units::CountingStat,
    intentionalWalks: crate::stats::units::CountingStat,
    hits: crate::stats::units::CountingStat,
    hitByPitch: crate::stats::units::CountingStat,
    atBats: crate::stats::units::CountingStat,
    caughtStealing: crate::stats::units::CountingStat,
    stolenBases: crate::stats::units::CountingStat,
    groundIntoDoublePlay: crate::stats::units::CountingStat,
    numberOfPitches: crate::stats::units::CountingStat,
    plateAppearances: crate::stats::units::CountingStat,
    totalBases: crate::stats::units::CountingStat,
    rbi: crate::stats::units::CountingStat,
    leftOnBase: crate::stats::units::CountingStat,
    sacBunts: crate::stats::units::CountingStat,
    sacFlies: crate::stats::units::CountingStat,
    catchersInterference: crate::stats::units::CountingStat,
    woba: crate::stats::units::ThreeDecimalPlacRateStat,
    wRaa: crate::stats::units::FloatCountingStat<2>,
    wRc: crate::stats::units::FloatCountingStat<2>,
    wRcPlus: crate::stats::units::FloatCountingStat<0>,
    rar: crate::stats::units::FloatCountingStat<1>,
    war: crate::stats::units::FloatCountingStat<1>,
    batting: crate::stats::units::FloatCountingStat<0>,
    fielding: crate::stats::units::FloatCountingStat<0>,
    baseRunning: crate::stats::units::FloatCountingStat<0>,
    positional: crate::stats::units::FloatCountingStat<1>,
    replacement: crate::stats::units::FloatCountingStat<0>,
    spd: crate::stats::units::TwoDecimalPlaceStat,
    ubr: crate::stats::units::TwoDecimalPlaceStat,
    wSb: crate::stats::units::TwoDecimalPlaceStat,
    age: crate::stats::units::CountingStat,
    extraBaseHits: crate::stats::units::CountingStat,
    gidp: crate::stats::units::CountingStat,
    gidpOpp: crate::stats::units::CountingStat,
    numberOfPitches: crate::stats::units::CountingStat,
    reachedOnError: crate::stats::units::CountingStat,
    walkOffs: crate::stats::units::CountingStat,
    flyOuts: crate::stats::units::CountingStat,
    totalSwings: crate::stats::units::CountingStat,
    swingAndMisses: crate::stats::units::CountingStat,
    ballsInPlay: crate::stats::units::CountingStat,
    popOuts: crate::stats::units::CountingStat,
    lineOuts: crate::stats::units::CountingStat,
    groundOuts: crate::stats::units::CountingStat,
    flyHits: crate::stats::units::CountingStat,
    popHits: crate::stats::units::CountingStat,
    lineHits: crate::stats::units::CountingStat,
    groundHits: crate::stats::units::CountingStat,
];

macro_rules! group_and_type {
    ($name:ident { $($piece:ident)* }) => {
        ::pastey::paste! {
            #[doc(hidden)]
            #[derive(Debug, ::serde::Deserialize, Clone, PartialEq, Eq)]
            #[serde(rename_all = "camelCase")]
            pub struct [<__ $name StatsMarker>] {
                $(
                #[serde(deserialize_with = "crate::stats::raw::deserialize_stat")]
                [<$piece:snake>]: Result<api_name_to_type![$piece], crate::stats::raw::OmittedStatError>,
                )*
            }

            impl Default for [<__ $name StatsMarker>] {
                fn default() -> Self {
                    Self {
                        $(
                        [<$piece:snake>]: Err(crate::stats::raw::OmittedStatError)
                        ),*
                    }
                }
            }

            impl crate::stats::RawStat for [<__ $name StatsMarker>] {}
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

pub use fielded_matchup::*;
pub use spray_chart::*;
pub use hit_spray::*;
pub use hot_cold_zones::*;

/// For old data, some stats are just omitted, like pitch count and stuff. So every field is now possibly omittable. Gotta `?` everything now. (this api is legendary)
#[derive(Debug, Error, PartialEq, Eq, Copy, Clone)]
#[error("This stat was omitted.")]
pub struct OmittedStatError;

pub fn deserialize_stat<'de, D: Deserializer<'de>, T: DeserializeOwned>(deserializer: D) -> Result<Result<T, OmittedStatError>, D::Error> {
    Option::<T>::deserialize(deserializer)?.map_or(Err(OmittedStatError), Ok)
}
