use std::ops::{Deref, DerefMut};
use derive_more::{Deref, DerefMut, Display};
use serde::Deserialize;
use crate::endpoints::seasons::season::{Season, SeasonState};
use crate::endpoints::sports::Sport;

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LeagueResponse {
    pub copyright: String,
    pub leagues: Vec<HydratedLeague>,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UnhydratedLeague {
    pub id: LeagueId,
    pub name: String,
}

#[derive(Debug, Deserialize, Deref, DerefMut, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HydratedLeague {
    #[deref]
    #[deref_mut]
    inner: UnhydratedLeague,
    pub abbreviation: String,
    #[serde(rename = "nameShort")] pub short_name: String,
    #[serde(rename = "orgCode")] pub code: String,
    pub season_state: SeasonState,
    #[serde(flatten, deserialize_with = "bad_league_season_schema_deserializer")]
    #[serde(rename = "seasonDateInfo")] pub season: Season,
    pub has_split_season: bool,
    pub num_games: u8,
    pub has_playoff_points: bool,
    pub num_teams: u8,
    pub num_wildcard_teams: Option<u8>,
    #[serde(rename = "conferencesInUse")] pub has_conferences: bool,
    #[serde(rename = "divisionsInUse")] pub has_divisions: bool,
    pub sport: Option<Sport>,
    pub sort_order: u32,
    pub active: bool,
}

// this is annoying me that it exists
#[derive(Deserialize)]
struct BadLeagueSeasonSchema {
    #[serde(rename = "hasWildCard")] has_wildcard: bool,
    #[serde(rename = "seasonDateInfo")] rest: Season,
}

fn bad_league_season_schema_deserializer<'de, D: serde::Deserializer<'de>>(deserializer: D) -> Result<Season, D::Error> {
    let BadLeagueSeasonSchema { has_wildcard, mut rest } = BadLeagueSeasonSchema::deserialize(deserializer)?;
    rest.has_wildcard = has_wildcard;
    Ok(rest)
}

#[repr(transparent)]
#[derive(Debug, Deserialize, Deref, Display, PartialEq, Eq, Copy, Clone)]
pub struct LeagueId(u32);

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(untagged)]
pub enum League {
    Hydrated(HydratedLeague),
    Unhydrated(UnhydratedLeague),
}

impl Deref for League {
    type Target = UnhydratedLeague;

    fn deref(&self) -> &Self::Target {
        match self {
            Self::Hydrated(inner) => inner,
            Self::Unhydrated(inner) => inner,
        }
    }
}

impl DerefMut for League {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            Self::Hydrated(inner) => inner,
            Self::Unhydrated(inner) => inner,
        }
    }
}
