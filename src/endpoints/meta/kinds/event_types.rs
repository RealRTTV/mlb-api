use derive_more::{Deref, DerefMut};
use serde::Deserialize;
use crate::endpoints::meta::MetaKind;

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IdentifiableEventType {
    pub code: String,
}

impl From<IdentifiableEventType> for EventType {
    fn from(value: IdentifiableEventType) -> Self {
        #[cfg(feature = "static_event_types")] {
            if let Ok(event_type) = StaticEventType::try_from(&*value.code) {
                return EventType::Static(event_type);
            }
        }
        
        EventType::Identifiable(value)
    }
}

#[derive(Debug, Deserialize, Deref, DerefMut, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HydratedEventType {
    plate_appearance: bool,
    hit: bool,
    base_running_event: bool,
    description: String,
    
    #[deref]
    #[deref_mut]
    #[serde(flatten)]
    inner: IdentifiableEventType,
}

impl From<HydratedEventType> for EventType {
    fn from(value: HydratedEventType) -> Self {
        EventType::Hydrated(value)
    }
}

#[cfg(feature = "static_event_types")]
pub use r#static::*;

#[cfg(feature = "static_event_types")]
mod r#static {
    use serde::Deserialize;
    use mlb_api_proc::HttpCache;
    use crate::endpoints::meta::event_types::IdentifiableEventType;
    use crate::endpoints::StaticParseError;

    macro_rules! generate {
        ({
            "plateAppearance": $plate_appearance:ident,
            "hit": $hit:ident,
            "code": $code:literal,
            "baseRunningEvent": $base_running_event:ident,
            "description": $description:literal
        }) => {
            StaticEventType {
                plate_appearance: $plate_appearance,
                hit: $hit,
                base_running_event: $base_running_event,
                code: $code,
                description: $description,
            }
        };
    }

    #[derive(Debug, Deserialize, PartialEq, Eq, Copy, Clone, HttpCache)]
    #[serde(try_from = "IdentifiableEventType")]
    #[request(type = "GET", url = "http://statsapi.mlb.com/api/v1/eventTypes")]
    #[parse(macro = generate, variant_name = "code", type = StaticEventType)]
    #[try_from(lifetimes = ['a], type = &'a str, field = "code")]
    pub struct StaticEventType {
        pub plate_appearance: bool,
        pub hit: bool,
        pub base_running_event: bool,
        pub code: &'static str,
        pub description: &'static str,
    }
    
    impl TryFrom<IdentifiableEventType> for StaticEventType {
        type Error = StaticParseError<String>;

        fn try_from(value: IdentifiableEventType) -> Result<Self, Self::Error> {
            Self::try_from(&*value.code).map_err(|_| StaticParseError::InvalidId(value.code))
        }
    }
}

#[derive(Debug, Deserialize, Eq, Clone)]
#[serde(untagged)]
pub enum EventType {
    #[cfg(feature = "static_event_types")]
    Static(StaticEventType),
    Hydrated(HydratedEventType),
    Identifiable(IdentifiableEventType),
}

impl PartialEq for EventType {
    fn eq(&self, other: &Self) -> bool {
        self.code() == other.code()
    }
}

impl EventType {
    #[must_use]
    pub fn code(&self) -> &str {
        match self {
            #[cfg(feature = "static_event_types")]
            Self::Static(inner) => inner.code,
            Self::Hydrated(inner) => &inner.code,
            Self::Identifiable(inner) => &inner.code,
        }
    }
}

impl MetaKind for EventType {
    const ENDPOINT_NAME: &'static str = "eventTypes";
}
