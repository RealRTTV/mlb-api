use derive_more::{Deref, DerefMut};
use serde::Deserialize;
use crate::endpoints::meta::MetaKind;

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IdentifiableGameStatus {
    pub detailed_state: String,
}

impl From<IdentifiableGameStatus> for GameStatus {
    fn from(value: IdentifiableGameStatus) -> Self {
        #[cfg(feature = "static_game_status")] {
            if let Ok(status) = StaticGameStatus::try_from(&*value.detailed_state) {
                return GameStatus::Static(status);
            }
        }
        
        GameStatus::Identifiable(value)
    }
}

#[derive(Debug, Deserialize, Deref, DerefMut, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HydratedGameStatus {
    pub abstract_game_state: String,
    pub coded_game_state: String,
    pub status_code: String,
    pub reason: Option<String>,
    pub abstract_game_code: String,
    
    #[deref]
    #[deref_mut]
    #[serde(flatten)]
    inner: IdentifiableGameStatus,
}

#[cfg(feature = "static_game_status")]
pub use r#static::*;

#[cfg(feature = "static_game_status")]
mod r#static {
    use serde::Deserialize;
    use mlb_api_proc::HttpCache;
    use crate::endpoints::meta::game_status::{GameStatus, IdentifiableGameStatus};
    use crate::endpoints::StaticParseError;

    macro_rules! generate {
    ({
        "abstractGameState": $abstract_game_state:literal,
        "codedGameState": $coded_game_state:literal,
        "detailedState": $detailed_state:literal,
        "statusCode": $status_code:literal,
        $("reason": $reason:literal,)?
        "abstractGameCode": $abstract_game_code:literal
    }) => {
        StaticGameStatus {
            abstract_game_state: $abstract_game_state,
            coded_game_state: $coded_game_state,
            detailed_state: $detailed_state,
            status_code: $status_code,
            reason: generate!(@ if $({ Some($reason) })? else { None }),
            abstract_game_code: $abstract_game_code,
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
    #[serde(try_from = "IdentifiableGameStatus")]
    #[request(type = "GET", url = "http://statsapi.mlb.com/api/v1/gameStatus")]
    #[parse(macro = generate, variant_name = "detailedState", type = StaticGameStatus)]
    #[try_from(lifetimes = ['a], type = &'a str, field = "detailedState")]
    pub struct StaticGameStatus {
        pub abstract_game_state: &'static str,
        pub coded_game_state: &'static str,
        pub detailed_state: &'static str,
        pub status_code: &'static str,
        pub reason: Option<&'static str>,
        pub abstract_game_code: &'static str,
    }
    
    impl TryFrom<IdentifiableGameStatus> for StaticGameStatus {
        type Error = StaticParseError<String>;

        fn try_from(value: IdentifiableGameStatus) -> Result<Self, Self::Error> {
            Self::try_from(&*value.detailed_state).map_err(|_| StaticParseError::InvalidId(value.detailed_state))
        }
    }
    
    impl From<StaticGameStatus> for GameStatus {
        fn from(value: StaticGameStatus) -> Self {
            GameStatus::Static(value)
        }
    }
}

#[derive(Debug, Deserialize, Eq, Clone)]
pub enum GameStatus {
    #[cfg(feature = "static_game_status")]
    Static(StaticGameStatus),
    Hydrated(HydratedGameStatus),
    Identifiable(IdentifiableGameStatus),
}

impl PartialEq for GameStatus {
    fn eq(&self, other: &Self) -> bool {
        self.detailed_state() == other.detailed_state()
    }
}

impl GameStatus {
    #[must_use]
    pub fn detailed_state(&self) -> &str {
        match self {
            #[cfg(feature = "static_game_status")]
            Self::Static(inner) => inner.detailed_state,
            Self::Hydrated(inner) => &inner.detailed_state,
            Self::Identifiable(inner) => &inner.detailed_state,
        }
    }
}

impl MetaKind for GameStatus {
    const ENDPOINT_NAME: &'static str = "gameStatus";
}
