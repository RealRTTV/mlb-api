pub mod all_star_ballot;
pub mod all_star_write_ins;
pub mod all_star_final_vote;

use crate::endpoints::seasons::season::{Season, SeasonState};
use crate::endpoints::sports::NamedSport;
use derive_more::{Deref, DerefMut, Display};
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LeagueResponse {
    pub copyright: String,
    pub leagues: Vec<League>,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Copy, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IdentifiableLeague {
    pub id: LeagueId,
}

impl From<IdentifiableLeague> for League {
    fn from(value: IdentifiableLeague) -> Self {
        #[cfg(feature = "static_league")] {
            if let Ok(league) = StaticLeague::try_from(value.id) {
                return League::Static(league);
            }
        }
        
        League::Identifiable(value)
    }
}

impl Default for IdentifiableLeague {
    fn default() -> Self {
        Self {
            id: LeagueId(0)
        }
    }
}

#[derive(Debug, Deserialize, Deref, DerefMut, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NamedLeague {
    pub name: String,

    #[deref]
    #[deref_mut]
    #[serde(flatten)]
    inner: IdentifiableLeague,
}

impl From<NamedLeague> for League {
    fn from(value: NamedLeague) -> Self {
        #[cfg(feature = "static_league")] {
            if let Ok(league) = StaticLeague::try_from(value.id) {
                return League::Static(league);
            }
        }
        
        League::Named(value)
    }
}

impl Default for NamedLeague {
    fn default() -> Self {
        Self {
            name: "null".to_owned(),
            inner: IdentifiableLeague::default(),
        }
    }
}

#[derive(Debug, Deserialize, Deref, DerefMut, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HydratedLeague {
    pub abbreviation: String,
    #[serde(rename = "nameShort")] pub short_name: Option<String>,
    #[serde(rename = "orgCode")] pub code: String,
    pub season_state: SeasonState,
    #[serde(flatten, deserialize_with = "bad_league_season_schema_deserializer")]
    #[serde(rename = "seasonDateInfo")] pub season: Season,
    #[serde(default)]
    pub has_split_season: bool,
    pub num_games: u8,
    pub has_playoff_points: Option<bool>,
    pub num_teams: u8,
    pub num_wildcard_teams: Option<u8>,
    #[serde(rename = "conferencesInUse")] pub has_conferences: bool,
    #[serde(rename = "divisionsInUse")] pub has_divisions: bool,
    pub sport: Option<NamedSport>,
    pub active: bool,

    #[deref]
    #[deref_mut]
    #[serde(flatten)]
    inner: NamedLeague,
}

impl From<HydratedLeague> for League {
    fn from(value: HydratedLeague) -> Self {
        League::Hydrated(value)
    }
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

#[cfg(feature = "static_league")]
pub use r#static::*;

#[cfg(feature = "static_league")]
mod r#static {
    use crate::endpoints::league::{IdentifiableLeague, League, LeagueId};
    use crate::endpoints::sports::StaticSport;
    use crate::endpoints::StaticParseError;
    use mlb_api_proc::HttpCache;
    use serde::Deserialize;

    macro_rules! generate {
        ({
            "id": $id:literal,
            "name": $name:literal,
            "link": $_0:literal,
            "abbreviation": $abbreviation:literal,
            $("nameShort": $nameShort:literal,)?
            "seasonState": $_1:literal,
            "hasWildCard": $hasWildCard:ident,
            $("hasSplitSeason": $hasSplitSeason:ident,)?
            $("numGames": $numGames:literal,)?
            $("hasPlayoffPoints": $hasPlayoffPoints:ident,)?
            $("numTeams": $numTeams:literal,)?
            $("numWildcardTeams": $numWildcardTeams:literal,)?
            "seasonDateInfo": { $($_2:tt)* },
            "season": $_3:literal,
            "orgCode": $orgCode:literal,
            "conferencesInUse": $conferencesInUse:ident,
            "divisionsInUse": $divisionsInUse:ident,
            $("sport": {
                "id": $sport_id:literal,
                "link": $_4:literal
            },)?
            "sortOrder": $_5:literal,
            "active": $active:ident
        }) => {
            StaticLeague {
                id: LeagueId::new($id),
                name: $name,
                abbreviation: $abbreviation,
                short_name: generate!(@ if $({ Some($nameShort) })? else { None }),
                has_wildcard: $hasWildCard,
                has_split_season: generate!(@ if $({ $hasSplitSeason })? else { false }),
                num_games: generate!(@ if $({ $numGames })? else { 0 }),
                has_playoff_points: generate!(@ if $({ Some($hasPlayoffPoints) })? else { None }),
                num_teams: generate!(@ if $({ $numTeams })? else { 0 }),
                num_wildcard_teams: generate!(@ if $({ Some($numWildcardTeams) })? else { None }),
                code: $orgCode,
                has_conferences: $conferencesInUse,
                has_divisions: $divisionsInUse,
                sport: generate!(@ if $({ Some($crate::endpoints::sports::StaticSport::__internal_const_try_from($crate::endpoints::sports::SportId::new($sport_id)).unwrap()) })? else { None }),
                active: $active
            }
        };
        (@ if  else $default:block) => {
            $default
        };
        (@ if $value:block else $default:block) => {
            $value
        };
    }

    #[derive(Debug, Deserialize, PartialEq, Eq, Copy, Clone, HttpCache)]
    #[serde(try_from = "IdentifiableLeague")]
    #[request(type = "GET", url = "http://statsapi.mlb.com/api/v1/league")]
    #[parse(macro = generate, variant_name = "name", type = StaticLeague)]
    #[try_from(type = LeagueId, field = "id", destruct = true)]
    pub struct StaticLeague {
        pub id: LeagueId,
        pub name: &'static str,
        pub abbreviation: &'static str,
        pub short_name: Option<&'static str>,
        pub code: &'static str,
        pub has_wildcard: bool,
        // pub season_state: SeasonState, // this is something that changes every year, we shouldn't store it
        // pub season: Season, // this holds the year so we are going to be just... not store it
        pub has_split_season: bool,
        pub num_games: u8,
        pub has_playoff_points: Option<bool>,
        pub num_teams: u8,
        pub num_wildcard_teams: Option<u8>,
        pub has_conferences: bool,
        pub has_divisions: bool,
        pub sport: Option<StaticSport>,
        pub active: bool,
    }

    impl TryFrom<IdentifiableLeague> for StaticLeague {
        type Error = StaticParseError<LeagueId>;

        fn try_from(value: IdentifiableLeague) -> Result<Self, Self::Error> {
            Self::try_from(value.id).map_err(|_| StaticParseError::InvalidId(value.id))
        }
    }

    impl From<StaticLeague> for League {
        fn from(value: StaticLeague) -> Self {
            League::Static(value)
        }
    }
}

#[repr(transparent)]
#[derive(Debug, Deserialize, Deref, Display, PartialEq, Eq, Copy, Clone)]
pub struct LeagueId(u32);

impl LeagueId {
    #[must_use]
    pub const fn new(id: u32) -> Self {
        Self(id)
    }
}

#[derive(Debug, Deserialize, Eq, Clone)]
#[serde(untagged)]
pub enum League {
    #[cfg(feature = "static_league")]
    Static(StaticLeague),
    Hydrated(HydratedLeague),
    Named(NamedLeague),
    Identifiable(IdentifiableLeague),
}

impl PartialEq for League {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}

impl League {
    #[must_use]
    pub fn id(&self) -> LeagueId {
        match self {
            #[cfg(feature = "static_league")]
            Self::Static(inner) => inner.id,
            Self::Hydrated(inner) => inner.id,
            Self::Named(inner) => inner.id,
            Self::Identifiable(inner) => inner.id,
        }
    }
}
