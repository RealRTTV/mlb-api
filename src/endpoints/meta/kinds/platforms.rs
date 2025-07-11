use std::ops::{Deref, DerefMut};
use derive_more::{Deref, DerefMut, From};
use serde::Deserialize;
use crate::endpoints::meta::MetaKind;

#[derive(Debug, Deserialize, Deref, DerefMut, PartialEq, Eq, Clone)]
pub struct HydratedePlatform {
    #[serde(rename = "platformDescription")] pub name: String,
    
    #[deref]
    #[deref_mut]
    #[serde(flatten)]
    inner: IdentifiablePlatform,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
pub struct IdentifiablePlatform {
    #[serde(rename =  "platformCode")] pub code: String,
}

#[derive(Debug, Deserialize, Eq, Clone, From)]
#[serde(untagged)]
pub enum Platform {
    Hydrated(HydratedePlatform),
    Identifiable(IdentifiablePlatform),
}

impl PartialEq for Platform {
    fn eq(&self, other: &Self) -> bool {
        self.code == other.code
    }
}

impl Deref for Platform {
    type Target = IdentifiablePlatform;

    fn deref(&self) -> &Self::Target {
        match self {
            Self::Hydrated(inner) => inner,
            Self::Identifiable(inner) => inner,
        }
    }
}

impl DerefMut for Platform {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            Self::Hydrated(inner) => inner,
            Self::Identifiable(inner) => inner,
        }
    }
}

impl MetaKind for Platform {
    const ENDPOINT_NAME: &'static str = "platforms";
}
