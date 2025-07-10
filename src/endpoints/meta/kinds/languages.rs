use derive_more::{Deref, DerefMut};
use serde::Deserialize;
use crate::endpoints::meta::kinds::MetaKind;

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IdentifiableLanguage {
    id: LanguageId,
}

impl From<IdentifiableLanguage> for Language {
    fn from(value: IdentifiableLanguage) -> Self {
        #[cfg(feature = "static_languages")] {
            if let Ok(inner) = StaticLanguage::try_from(value.id) {
                return Language::Static(inner)
            }
        }
        
        Language::Identifiable(value)
    }
}

#[derive(Debug, Deserialize, Deref, DerefMut, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HydratedLanguage {
    #[serde(rename = "languageCode")] pub code: String,
    pub name: String,
    pub locale: String,
    
    #[deref]
    #[deref_mut]
    #[serde(flatten)]
    inner: IdentifiableLanguage,
}

impl From<HydratedLanguage> for Language {
    fn from(value: HydratedLanguage) -> Self {
        Language::Hydrated(value)
    }
}

#[cfg(feature = "static_languages")]
pub use r#static::*;

#[cfg(feature = "static_languages")]
mod r#static {
    use serde::Deserialize;
    use mlb_api_proc::HttpCache;
    use crate::endpoints::meta::languages::{IdentifiableLanguage, Language, LanguageId};
    use crate::endpoints::StaticParseError;
    
    macro_rules! generate {
        ({
            "languageId": $languageId:literal,
            "languageCode": $languageCode:literal,
            "name": $name:literal,
            "locale": $locale:literal
        }) => {
            StaticLanguage {
                id: LanguageId($languageId),
                code: $languageCode,
                name: $name,
                locale: $locale,
            }
        };
    }

    #[derive(Debug, Deserialize, PartialEq, Eq, Copy, Clone, HttpCache)]
    #[serde(try_from = "IdentifiableLanguage")]
    #[request(type = "GET", url = "http://statsapi.mlb.com/api/v1/languages")]
    #[parse(macro = generate, variant_name = "languageCode", type = StaticLanguage)]
    #[try_from(type = LanguageId, field = "languageId", destruct = true)]
    pub struct StaticLanguage {
        pub id: LanguageId,
        pub code: &'static str,
        pub name: &'static str,
        pub locale: &'static str,
    }
    
    impl TryFrom<IdentifiableLanguage> for StaticLanguage {
        type Error = StaticParseError<LanguageId>;

        fn try_from(value: IdentifiableLanguage) -> Result<Self, Self::Error> {
            StaticLanguage::try_from(value.id).map_err(|_| StaticParseError::InvalidId(value.id))
        }
    }
    
    impl From<StaticLanguage> for Language {
        fn from(value: StaticLanguage) -> Self {
            Language::Static(value)
        }
    }
}

pub use id::*;

mod id {
    use derive_more::{Deref, Display};
    use serde::Deserialize;

    #[repr(transparent)]
    #[derive(Debug, Deserialize, Deref, Display, PartialEq, Eq, Copy, Clone)]
    pub struct LanguageId(pub(super) u32);
    
    impl LanguageId {
        #[must_use]
        pub const fn new(id: u32) -> Self {
            Self(id)
        }
    }
}

#[derive(Debug, Deserialize, Eq, Clone)]
#[serde(untagged)]
pub enum Language {
    #[cfg(feature = "static_languages")]
    Static(StaticLanguage),
    Hydrated(HydratedLanguage),
    Identifiable(IdentifiableLanguage),
}

impl Language {
    #[must_use]
    pub fn id(&self) -> LanguageId {
        match self {
            #[cfg(feature = "static_languages")]
            Self::Static(inner) => inner.id,
            Self::Hydrated(inner) => inner.id,
            Self::Identifiable(inner) => inner.id,
        }
    }
}

impl PartialEq for Language {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}

impl MetaKind for Language {
    const ENDPOINT_NAME: &'static str = "languages";
}
