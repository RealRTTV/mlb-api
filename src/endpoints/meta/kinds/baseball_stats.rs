use std::ops::{Deref, DerefMut};
use crate::endpoints::meta::stat_groups::StatGroup;
use crate::endpoints::meta::MetaKind;
use derive_more::{Deref, DerefMut, From};
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
pub struct IdentifiableBaseballStat {
    #[serde(rename = "name")] pub id: String,
}

#[derive(Debug, Deserialize, Deref, DerefMut, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HydratedBaseballStat {
    lookup_param: Option<String>,
    is_counting: bool,
    label: Option<String>,
    stat_groups: Vec<StatGroup>,

    #[deref]
    #[deref_mut]
    #[serde(flatten)]
    inner: IdentifiableBaseballStat,
}

#[derive(Debug, Deserialize, Eq, Clone, From)]
#[serde(untagged)]
pub enum BaseballStat {
    Hydrated(HydratedBaseballStat),
    Identifiable(IdentifiableBaseballStat),
}

impl PartialEq for BaseballStat {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Deref for BaseballStat {
    type Target = IdentifiableBaseballStat;

    fn deref(&self) -> &Self::Target {
        match self {
            Self::Hydrated(inner) => inner,
            Self::Identifiable(inner) => inner,
        }
    }
}

impl DerefMut for BaseballStat {
    fn deref_mut(&mut  self) -> &mut Self::Target {
        match self {
            Self::Hydrated(inner) => inner,
            Self::Identifiable(inner) => inner,
        }
    }
}

impl MetaKind for BaseballStat {
    const ENDPOINT_NAME: &'static str = "baseballStats";
}
