use std::ops::{Deref, DerefMut};
use derive_more::{Deref, DerefMut, From};
use serde::Deserialize;
use crate::endpoints::meta::MetaKind;

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
pub struct IdentifiableSkyDescription {
    pub code: String,
}

#[derive(Debug, Deserialize, Deref, DerefMut, PartialEq, Eq, Clone)]
pub struct HydratedSkyDescription {
    pub description: String,
    
    #[deref]
    #[deref_mut]
    #[serde(flatten)]
    inner: IdentifiableSkyDescription,
}

#[derive(Debug, Deserialize, Eq, Clone, From)]
pub enum SkyDescription {
    Hydrated(HydratedSkyDescription),
    Identifiable(IdentifiableSkyDescription),
}

impl PartialEq for SkyDescription {
    fn eq(&self, other: &Self) -> bool {
        self.code == other.code
    }
}

impl Deref for SkyDescription {
    type Target = IdentifiableSkyDescription;

    fn deref(&self) -> &Self::Target {
        match self {
            Self::Hydrated(inner) => inner,
            Self::Identifiable(inner) => inner,
        }
    }
}

impl DerefMut for SkyDescription {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            Self::Hydrated(inner) => inner,
            Self::Identifiable(inner) => inner,
        }
    }
}

impl MetaKind for SkyDescription {
    const ENDPOINT_NAME: &'static str = "sky";
}
