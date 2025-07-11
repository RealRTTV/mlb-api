use std::ops::{Deref, DerefMut};
use derive_more::{Deref, DerefMut};
use serde::Deserialize;
use crate::endpoints::meta::MetaKind;

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
pub struct IdentifiableStandingsType {
    #[serde(rename = "name")] pub code: String,
}

#[derive(Debug, Deserialize, Deref, DerefMut, PartialEq, Eq, Clone)]
pub struct HydratedStandingsType {
    pub description: String,
    
    #[deref]
    #[deref_mut]
    #[serde(flatten)]
    inner: IdentifiableStandingsType,
}

#[derive(Debug, Deserialize, Eq, Clone)]
pub enum StandingsType {
    Hydrated(HydratedStandingsType),
    Identifiable(IdentifiableStandingsType),
}

impl PartialEq for StandingsType {
    fn eq(&self, other: &Self) -> bool {
        self.code == other.code
    }
}

impl Deref for StandingsType {
    type Target = IdentifiableStandingsType;

    fn deref(&self) -> &Self::Target {
        match self {
            Self::Hydrated(inner) => inner,
            Self::Identifiable(inner) => inner,
        }
    }
}

impl DerefMut for StandingsType {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            Self::Hydrated(inner) => inner,
            Self::Identifiable(inner) => inner,
        }
    }
}

impl MetaKind for StandingsType {
    const ENDPOINT_NAME: &'static str = "standingsTypes";
}
