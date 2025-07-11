use std::ops::{Deref, DerefMut};
use derive_more::{Deref, DerefMut, From};
use serde::Deserialize;
use crate::endpoints::meta::MetaKind;

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
pub struct IdentifiableRosterType {
    #[serde(rename = "lookupName")] pub name: String,
}

#[derive(Debug, Deserialize, Deref, DerefMut, PartialEq, Eq, Clone)]
pub struct HydratedRosterType {
    pub parameter: String,
    pub description: String,
    
    #[deref]
    #[deref_mut]
    #[serde(flatten)]
    inner: IdentifiableRosterType,
}

#[derive(Debug, Deserialize, Eq, Clone, From)]
#[serde(untagged)]
pub enum RosterType {
    Hydrated(HydratedRosterType),
    Identifiable(IdentifiableRosterType),
}

impl PartialEq for RosterType {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Deref for RosterType {
    type Target = IdentifiableRosterType;

    fn deref(&self) -> &Self::Target {
        match self {
            Self::Hydrated(inner) => inner,
            Self::Identifiable(inner) => inner,
        }
    }
}

impl DerefMut for RosterType {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            Self::Hydrated(inner) => inner,
            Self::Identifiable(inner) => inner,
        }
    }
}

impl MetaKind for RosterType {
    const ENDPOINT_NAME: &'static str = "rosterTypes";
}
