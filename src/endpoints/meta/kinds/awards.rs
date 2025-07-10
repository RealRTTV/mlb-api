use crate::endpoints::meta::MetaKind;
use crate::endpoints::sports::Sport;
use serde::Deserialize;
use crate::endpoints::league::League;

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
pub struct HydratedAward {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub sport: Option<Sport>,
    pub league: Option<League>,
    pub notes: Option<String>,
}

impl From<HydratedAward> for Award {
    fn from(value: HydratedAward) -> Self {
        Award::Hydrated(value)
    }
}

#[cfg(feature = "static_awards")]
pub use r#static::*;

#[cfg(feature = "static_awards")]
mod r#static {
    use serde::Deserialize;
    use mlb_api_proc::HttpCache;
    use crate::endpoints::league::{League, StaticLeague};
    use crate::endpoints::meta::awards::Award;
    use crate::endpoints::sports::{Sport, StaticSport};
    use crate::endpoints::StaticParseError;

    macro_rules! generate {
    ({
        "id": $id:literal,
        "name": $name:literal $(,)?
        $("description": $description:literal $(,)?)?
        $("sortOrder": $_3:literal $(,)?)?
        $("sport": {
            "id": $sport_id:literal,
            "link": $_1:literal
        } $(,)?)?
        $("league": {
            "id": $league_id:literal,
            "link": $_2:literal
        } $(,)?)?
        $("notes": $notes:literal $(,)?)?
    }) => {
        StaticAward {
            id: $id,
            name: $name,
            description: generate!(@ if $({ Some($description) })? else { None }),
            league: generate!(@ if $({ Some(&League::Static(StaticLeague::__internal_const_try_from($crate::endpoints::league::LeagueId::new($league_id)).unwrap())) })? else { None }),
            sport: generate!(@ if $({ Some(&Sport::Static(StaticSport::__internal_const_try_from($crate::endpoints::sports::SportId::new($sport_id)).unwrap())) })? else { None }),
            notes: generate!(@ if $({ Some($notes) })? else { None }),
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
    #[serde(try_from = "IdentifiableAward")]
    #[request(type = "GET", url = "http://statsapi.mlb.com/api/v1/awards")]
    #[parse(macro = generate, variant_name = "id", type = StaticAward)]
    #[try_from(lifetimes = ['a], type = &'a str, field = "id")]
    pub struct StaticAward {
        pub id: &'static str,
        pub name: &'static str,
        pub description: Option<&'static str>,
        pub sport: Option<&'static Sport>,
        pub league: Option<&'static League>,
        pub notes: Option<&'static str>,
    }

    #[derive(Deserialize)]
    struct IdentifiableAward {
        id: String,
    }

    impl TryFrom<IdentifiableAward> for StaticAward {
        type Error = StaticParseError<String>;

        fn try_from(value: IdentifiableAward) -> Result<Self, Self::Error> {
            Self::try_from(&*value.id).map_err(|_| StaticParseError::InvalidId(value.id))
        }
    }
    
    impl From<StaticAward> for Award {
        fn from(value: StaticAward) -> Self {
            Award::Static(value)
        }
    }
}

#[derive(Debug, Deserialize, Eq, Clone)]
#[serde(untagged)]
pub enum Award {
    #[cfg(feature = "static_awards")]
    Static(StaticAward),
    Hydrated(HydratedAward),
}

impl Award {
    #[must_use]
    pub fn id(&self) -> &str {
        match self {
            #[cfg(feature = "static_awards")]
            Self::Static(inner) => &inner.id,
            Self::Hydrated(inner) => &inner.id,
        }
    }

    #[must_use]
    pub fn name(&self) -> &str {
        match self {
            #[cfg(feature = "static_awards")]
            Self::Static(inner) => &inner.name,
            Self::Hydrated(inner) => &inner.name,
        }
    }

    #[must_use]
    pub fn description(&self) -> Option<&str> {
        match self {
            #[cfg(feature = "static_awards")]
            Self::Static(inner) => inner.description.as_deref(),
            Self::Hydrated(inner) => inner.description.as_deref(),
        }
    }

    #[must_use]
    pub fn sport(&self) -> Option<&Sport> {
        match self {
            #[cfg(feature = "static_awards")]
            Self::Static(inner) => inner.sport,
            Self::Hydrated(inner) => inner.sport.as_ref(),
        }
    }

    #[must_use]
    pub fn league(&self) -> Option<&League> {
        match self {
            #[cfg(feature = "static_awards")]
            Self::Static(inner) => inner.league,
            Self::Hydrated(inner) => inner.league.as_ref(),
        }
    }

    #[must_use]
    pub fn notes(&self) -> Option<&str> {
        match self {
            #[cfg(feature = "static_awards")]
            Self::Static(inner) => inner.notes.as_deref(),
            Self::Hydrated(inner) => inner.notes.as_deref(),
        }
    }
}

impl PartialEq for Award {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}

impl MetaKind for Award {
    const ENDPOINT_NAME: &'static str = "awards";
}
