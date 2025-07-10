pub mod players;

use derive_more::{Deref, DerefMut};
use serde::Deserialize;
use crate::types::Copyright;

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SportsResponse {
    pub copyright: Copyright,
    pub sports: Vec<Sport>,
}

pub use id::*;

mod id {
    use std::fmt::{Display, Formatter};
    use derive_more::{Deref, Display};
    use serde::Deserialize;
    use crate::endpoints::sports::SportsResponse;
    use crate::endpoints::Url;
    use crate::gen_params;

    #[repr(transparent)]
    #[derive(Debug, Deserialize, Deref, Display, PartialEq, Eq, Copy, Clone)]
    pub struct SportId(pub(super) u32);

    impl SportId {
        #[must_use]
        pub const fn new(id: u32) -> Self {
            Self(id)
        }
    }

    impl Default for SportId {
        fn default() -> Self {
            Self(1)
        }
    }

    pub struct SportsEndpointUrl {
        pub id: Option<SportId>,
    }

    impl Display for SportsEndpointUrl {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "http://statsapi.mlb.com/api/v1/sports{params}",
                params = gen_params! { "sportId"?: self.id }
            )
        }
    }

    impl Url<SportsResponse> for SportsEndpointUrl {}
}

#[derive(Debug, Deserialize, PartialEq, Eq, Copy, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IdentifiableSport {
    pub id: SportId,
}

impl From<IdentifiableSport> for Sport {
    fn from(value: IdentifiableSport) -> Self {
        #[cfg(feature = "static_sport")] {
            if let Ok(sport) = StaticSport::try_from(value.id) {
                return Sport::Static(sport)
            }
        }
        
        Sport::Identifiable(value)
    }
}

#[derive(Debug, Deserialize, Deref, DerefMut, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NamedSport {
    pub name: String,

    #[deref]
    #[deref_mut]
    #[serde(flatten)]
    pub(super) inner: IdentifiableSport,
}

impl From<NamedSport> for Sport {
    fn from(value: NamedSport) -> Self {
        #[cfg(feature = "static_sport")] {
            if let Ok(sport) = StaticSport::try_from(value.id) {
                return Sport::Static(sport)
            }
        }
        
        Sport::Named(value)
    }
}

pub use hydrated::*;

mod hydrated {
    use derive_more::{Deref, DerefMut};
    use serde::Deserialize;
    use crate::endpoints::sports::{NamedSport, Sport};

    #[derive(Debug, Deserialize, Deref, DerefMut, PartialEq, Eq, Clone)]
    #[serde(rename_all = "camelCase")]
    pub struct HydratedSport {
        pub code: String,
        pub abbreviation: String,
        #[serde(rename = "activeStatus")] pub active: bool,

        #[deref]
        #[deref_mut]
        #[serde(flatten)]
        pub(super) inner: NamedSport,
    }
    
    impl From<HydratedSport> for Sport {
        fn from(value: HydratedSport) -> Self {
            Sport::Hydrated(value)
        }
    }
}

#[cfg(feature = "static_sport")]
pub use r#static::*;

#[cfg(feature = "static_sport")]
mod r#static {
    use serde::Deserialize;
    use mlb_api_proc::HttpCache;
    use crate::endpoints::sports::{IdentifiableSport, Sport, SportId};
    use crate::endpoints::StaticParseError;

    macro_rules! generate {
    ({
        "id": $id:literal,
        "code": $code:literal,
        "link": $_0:literal,
        "name": $name:literal,
        "abbreviation": $abbreviation:literal,
        "sortOrder": $_2:literal,
        "activeStatus": $activeStatus:literal
    }) => {
        StaticSport {
            id: SportId($id),
            code: $code,
            name: $name,
            abbreviation: $abbreviation,
            active: $activeStatus,
        }
    };
}

    #[derive(Debug, Deserialize, PartialEq, Eq, Copy, Clone, HttpCache)]
    #[serde(try_from = "IdentifiableSport")]
    #[request(type = "GET", url = "http://statsapi.mlb.com/api/v1/sports")]
    #[parse(macro = generate, variant_name = "abbreviation", type = StaticSport)]
    #[try_from(type = SportId, field = "id", destruct = true)]
    pub struct StaticSport {
        pub id: SportId,
        pub name: &'static str,
        pub code: &'static str,
        pub abbreviation: &'static str,
        pub active: bool,
    }
    
    impl TryFrom<IdentifiableSport> for StaticSport {
        type Error = StaticParseError<SportId>;

        fn try_from(value: IdentifiableSport) -> Result<Self, Self::Error> {
            Self::try_from(value.id).map_err(|_| StaticParseError::InvalidId(value.id))
        }
    }
    
    impl From<StaticSport> for Sport {
        fn from(value: StaticSport) -> Self {
            Sport::Static(value)
        }
    }
}

#[derive(Debug, Deserialize, Eq, Clone)]
#[serde(untagged)]
pub enum Sport {
    #[cfg(feature = "static_sport")]
    Static(StaticSport),
    Hydrated(HydratedSport),
    Named(NamedSport),
    Identifiable(IdentifiableSport),
}

impl PartialEq for Sport {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}

impl Sport {
    #[must_use]
    pub fn id(&self) -> SportId {
        match self {
            #[cfg(feature = "static_sport")]
            Self::Static(inner) => inner.id,
            Self::Hydrated(inner) => inner.id,
            Self::Named(inner) => inner.id,
            Self::Identifiable(inner) => inner.id,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::endpoints::Url;

    #[tokio::test]
    async fn check_updated() {
        let _result = SportsEndpointUrl { id: None }.get().await.unwrap();
    }
}
