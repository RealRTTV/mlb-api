use std::borrow::Cow;
use std::cmp::Ordering;
use std::ops::{Deref, DerefMut};
use derive_more::{Deref, DerefMut, Display};
use serde::Deserialize;
use crate::endpoints::sports::SportsEndpointUrl;
use crate::types::Copyright;

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SportsResponse {
    pub copyright: Copyright,
    pub sports: Vec<HydratedSport>,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UnhydratedSport {
    pub id: SportId,
    pub name: Cow<'static, str>,
}

#[derive(Debug, Deserialize, Deref, DerefMut, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HydratedSport {
    #[deref]
    #[deref_mut]
    #[serde(flatten)]
    pub(super) inner: UnhydratedSport,
    pub code: Cow<'static, str>,
    pub abbreviation: Cow<'static, str>,
    pub sort_order: usize,
    #[serde(rename = "activeStatus")] pub active: bool,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(untagged)]
pub enum Sport {
    Hydrated(HydratedSport),
    Unhydrated(UnhydratedSport),
}

impl Deref for Sport {
    type Target = UnhydratedSport;

    fn deref(&self) -> &Self::Target {
        match self {
            Self::Unhydrated(inner) => inner,
            Self::Hydrated(inner) => inner,
        }
    }
}

impl DerefMut for Sport {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            Self::Unhydrated(inner) => inner,
            Self::Hydrated(inner) => inner,
        }
    }
}



impl HydratedSport {
    #[must_use]
    pub fn as_link(&self) -> SportsEndpointUrl {
        SportsEndpointUrl {
            id: Some(self.id)
        }
    }
}

impl Ord for HydratedSport {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.sort_order.cmp(&other.sort_order)
    }
}

impl PartialOrd for HydratedSport {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[repr(transparent)]
#[derive(Debug, Deserialize, Deref, Display, PartialEq, Eq, Copy, Clone)]
pub struct SportId(u32);

impl Default for SportId {
    fn default() -> Self {
        Self::MLB
    }
}

impl SportId {
    pub const IDS: [Self; 19] = [
        Self::MLB,
        Self::AAA,
        Self::AA,
        Self::HIGH_A,
        Self::A,
        Self::ROOKIE,
        Self::WINTER,
        Self::MILB,
        Self::INDIE,
        Self::NLB,
        Self::KBO,
        Self::NPB,
        Self::INTERNATIONAL,
        Self::INTERNATIONAL_18U,
        Self::INTERNATIONAL_16U,
        Self::INTERNATIONAL_AMATEUR,
        Self::COLLEGE,
        Self::HIGH_SCHOOL,
        Self::WPF,
    ];
    
    pub const MLB: Self = Self(1);

    pub const AAA: Self = Self(11);
    pub const AA: Self = Self(12);
    pub const HIGH_A: Self = Self(13);
    pub const A: Self = Self(14);
    pub const ROOKIE: Self = Self(16);
    pub const WINTER: Self = Self(17);

    pub const MILB: Self = Self(21);
    pub const INDIE: Self = Self(23);

    pub const NLB: Self = Self(61);

    pub const KBO: Self = Self(32);
    pub const NPB: Self = Self(31);

    pub const INTERNATIONAL: Self = Self(51);
    pub const INTERNATIONAL_18U: Self = Self(509);
    pub const INTERNATIONAL_16U: Self = Self(510);
    pub const INTERNATIONAL_AMATEUR: Self = Self(6005);

    pub const COLLEGE: Self = Self(22);
    pub const HIGH_SCHOOL: Self = Self(586);

    pub const WPF: Self = Self(576);
}
