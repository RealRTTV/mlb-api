use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
pub struct HydratedLeagueLeaderType {
    #[serde(rename = "displayName")] pub name: String,
}

impl From<HydratedLeagueLeaderType> for LeagueLeaderType {
    fn from(value: HydratedLeagueLeaderType) -> Self {
        LeagueLeaderType::Hydrated(value)
    }
}

pub type IdentifiableLeagueLeaderType = HydratedLeagueLeaderType;

#[cfg(feature = "static_league_leader_types")]
pub use r#static::*;

#[cfg(feature = "static_league_leader_types")]
mod r#static {
    use serde::Deserialize;
    use mlb_api_proc::HttpCache;
    use crate::endpoints::meta::league_leader_types::{IdentifiableLeagueLeaderType, LeagueLeaderType};
    use crate::endpoints::StaticParseError;

    macro_rules! generate {
        ({
            "displayName": $displayName:literal
        }) => {
            StaticLeagueLeaderType {
                name: $displayName,
            }
        };
    }

    #[derive(Debug, Deserialize, PartialEq, Eq, Copy, Clone, HttpCache)]
    #[serde(try_from = "IdentifiableLeagueLeaderType")]
    #[request(type = "GET", url = "http://statsapi.mlb.com/api/v1/leagueLeaderTypes")]
    #[parse(macro = generate, variant_name = "displayName", type = StaticLeagueLeaderType)]
    #[try_from(lifetimes = ['a], type = &'a str, field = "displayName")]
    pub struct StaticLeagueLeaderType {
        pub name: &'static str,
    }
    
    impl TryFrom<IdentifiableLeagueLeaderType> for StaticLeagueLeaderType {
        type Error = StaticParseError<String>;

        fn try_from(value: IdentifiableLeagueLeaderType) -> Result<Self, Self::Error> {
            Self::try_from(&*value.name).map_err(|_| StaticParseError::InvalidId(value.name))
        }
    }
    
    impl From<StaticLeagueLeaderType> for LeagueLeaderType {
        fn from(value: StaticLeagueLeaderType) -> Self {
            LeagueLeaderType::Static(value)
        }
    }
}

#[derive(Debug, Deserialize, Eq, Clone)]
pub enum LeagueLeaderType {
    #[cfg(feature = "static_league_leader_types")]
    Static(StaticLeagueLeaderType),
    Hydrated(HydratedLeagueLeaderType),
}

impl LeagueLeaderType {
    #[must_use]
    pub fn name(&self) -> &str {
        match self {
            #[cfg(feature = "static_league_leader_types")]
            Self::Static(inner) => inner.name,
            Self::Hydrated(inner) => &inner.name,
        }
    }
}

impl PartialEq for LeagueLeaderType {
    fn eq(&self, other: &Self) -> bool {
        self.name() == other.name()
    }
}
