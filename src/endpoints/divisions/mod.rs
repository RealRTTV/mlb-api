use serde_with::DisplayFromStr;
use std::ops::{Deref, DerefMut};
use derive_more::{Deref, DerefMut, Display, From};
use serde::Deserialize;
use serde_with::serde_as;
use crate::endpoints::league::IdentifiableLeague;
use crate::endpoints::sports::IdentifiableSport;
use crate::types::Copyright;

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DivisionsResponse {
    pub copyright: Copyright,
    pub divisions: Vec<HydratedDivision>,
}
#[derive(Debug, Deserialize, PartialEq, Eq, Copy, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IdentifiableDivision {
    pub id: DivisionId,
}

#[derive(Debug, Deserialize, Deref, DerefMut, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NamedDivision {
    pub name: String,

    #[deref]
    #[deref_mut]
    #[serde(flatten)]
    inner: IdentifiableDivision,
}

#[serde_as]
#[derive(Debug, Deserialize, Deref, DerefMut, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HydratedDivision {
    #[serde(rename = "nameShort")] pub short_name: String,
    #[serde_as(as = "DisplayFromStr")]
    pub season: u16,
    pub abbreviation: String,
    pub league: IdentifiableLeague,
    pub sport: IdentifiableSport,
    pub has_wildcard: bool,
    pub num_playoff_teams: u8,
    pub active: bool,

    #[deref]
    #[deref_mut]
    #[serde(flatten)]
    inner: NamedDivision,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone, From)]
#[serde(untagged)]
pub enum Division {
    Hydrated(HydratedDivision),
    Named(NamedDivision),
    Identifiable(IdentifiableDivision),
}

impl Deref for Division {
    type Target = IdentifiableDivision;

    fn deref(&self) -> &Self::Target {
        match self {
            Self::Hydrated(inner) => inner,
            Self::Named(inner) => inner,
            Self::Identifiable(inner) => inner,
        }
    }
}

impl DerefMut for Division {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            Self::Hydrated(inner) => inner,
            Self::Named(inner) => inner,
            Self::Identifiable(inner) => inner,
        }
    }
}

#[repr(transparent)]
#[derive(Debug, Deserialize, Deref, Display, PartialEq, Eq, Copy, Clone)]
pub struct DivisionId(u32);
