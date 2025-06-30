use std::borrow::Cow;
use std::cmp::Ordering;
use derive_more::{Deref, Display};
use serde::Deserialize;
use crate::endpoints::sports::SportsResponseUrl;
use crate::endpoints::types::Copyright;

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SportsResponse {
    pub copyright: Copyright,
    pub sports: Vec<Sport>,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Sport {
    pub id: SportId,
    pub code: Cow<'static, str>,
    pub name: Cow<'static, str>,
    pub abbreviation: Cow<'static, str>,
    pub sort_order: usize,
    #[serde(rename = "activeStatus")] pub active: bool,
}

/// This endpoint won't change that often, so we'll store the values here if you need them.\
/// This technically means the API is held "statically" (which is the point of an API to not have).\
/// However, if the SportIds are being stored as constants (for obvious reason -- it's the most consistent way to look them up as an ID will never be re-used), we might as well store the sports too.
impl Sport {
    pub const MLB: Self = Self {
        id: SportId::MLB,
        code: Cow::Borrowed("mlb"),
        name: Cow::Borrowed("Major League Baseball"),
        abbreviation: Cow::Borrowed("MLB"),
        sort_order: 11,
        active: true,
    };

    pub const AAA: Self = Self {
        id: SportId::AAA,
        code: Cow::Borrowed("aaa"),
        name: Cow::Borrowed("Triple-A"),
        abbreviation: Cow::Borrowed("AAA"),
        sort_order: 101,
        active: true,
    };

    pub const AA: Self = Self {
        id: SportId::AA,
        code: Cow::Borrowed("aax"),
        name: Cow::Borrowed("Double-A"),
        abbreviation: Cow::Borrowed("AA"),
        sort_order: 201,
        active: true,
    };

    pub const HIGH_A: Self = Self {
        id: SportId::HIGH_A,
        code: Cow::Borrowed("afa"),
        name: Cow::Borrowed("High-A"),
        abbreviation: Cow::Borrowed("A+"),
        sort_order: 301,
        active: true,
    };

    pub const A: Self = Self {
        id: SportId::A,
        code: Cow::Borrowed("afx"),
        name: Cow::Borrowed("Single-A"),
        abbreviation: Cow::Borrowed("A"),
        sort_order: 401,
        active: true,
    };

    pub const ROOKIE: Self = Self {
        id: SportId::ROOKIE,
        code: Cow::Borrowed("rok"),
        name: Cow::Borrowed("Rookie"),
        abbreviation: Cow::Borrowed("ROK"),
        sort_order: 701,
        active: true,
    };

    pub const WINTER: Self = Self {
        id: SportId::WINTER,
        code: Cow::Borrowed("win"),
        name: Cow::Borrowed("Winter Leagues"),
        abbreviation: Cow::Borrowed("WIN"),
        sort_order: 1301,
        active: true,
    };


    pub const MILB: Self = Self {
        id: SportId::MILB,
        code: Cow::Borrowed("min"),
        name: Cow::Borrowed("Minor League Baseball"),
        abbreviation: Cow::Borrowed("Minors"),
        sort_order: 1402,
        active: true,
    };

    pub const INDIE: Self = Self {
        id: SportId::INDIE,
        code: Cow::Borrowed("ind"),
        name: Cow::Borrowed("Independent Leagues"),
        abbreviation: Cow::Borrowed("IND"),
        sort_order: 2101,
        active: true,
    };

    pub const NLB: Self = Self {
        id: SportId::NLB,
        code: Cow::Borrowed("nlb"),
        name: Cow::Borrowed("Negro League Baseball"),
        abbreviation: Cow::Borrowed("NLB"),
        sort_order: 2401,
        active: true, // what??
    };

    pub const KBO: Self = Self {
        id: SportId::KBO,
        code: Cow::Borrowed("kor"),
        name: Cow::Borrowed("Korean Baseball Organization"),
        abbreviation: Cow::Borrowed("KOR"),
        sort_order: 2601,
        active: true,
    };
    pub const NPB: Self = Self {
        id: SportId::NPB,
        code: Cow::Borrowed("jml"),
        name: Cow::Borrowed("Nippon Professional Baseball"),
        abbreviation: Cow::Borrowed("NPB"),
        sort_order: 2701,
        active: true,
    };

    pub const INTERNATIONAL: Self = Self {
        id: SportId::INTERNATIONAL,
        code: Cow::Borrowed("int"),
        name: Cow::Borrowed("International Baseball"),
        abbreviation: Cow::Borrowed("INT"),
        sort_order: 3501,
        active: true,
    };
    pub const INTERNATIONAL_18U: Self = Self {
        id: SportId::INTERNATIONAL_18U,
        code: Cow::Borrowed("nae"),
        name: Cow::Borrowed("International Baseball (18U)"),
        abbreviation: Cow::Borrowed("18U"),
        sort_order: 3503,
        active: true,
    };
    pub const INTERNATIONAL_16U: Self = Self {
        id: SportId::INTERNATIONAL_16U,
        code: Cow::Borrowed("nas"),
        name: Cow::Borrowed("International Baseball (16 and under)"),
        abbreviation: Cow::Borrowed("16U"),
        sort_order: 3505,
        active: true,
    };
    pub const INTERNATIONAL_AMATEUR: Self = Self {
        id: SportId::INTERNATIONAL_AMATEUR,
        code: Cow::Borrowed("ame"),
        name: Cow::Borrowed("International Baseball (amateur)"),
        abbreviation: Cow::Borrowed("AME"),
        sort_order: 3509,
        active: true,
    };

    pub const COLLEGE: Self = Self {
        id: SportId::COLLEGE,
        code: Cow::Borrowed("bbc"),
        name: Cow::Borrowed("College Baseball"),
        abbreviation: Cow::Borrowed("College"),
        sort_order: 5101,
        active: true,
    };
    pub const HIGH_SCHOOL: Self = Self {
        id: SportId::HIGH_SCHOOL,
        code: Cow::Borrowed("hsb"),
        name: Cow::Borrowed("High School Baseball"),
        abbreviation: Cow::Borrowed("H.S."),
        sort_order: 6201,
        active: true,
    };

    pub const WPF: Self = Self {
        id: SportId::WPF,
        code: Cow::Borrowed("wps"),
        name: Cow::Borrowed("Women's Professional Softball"),
        abbreviation: Cow::Borrowed("WPS"),
        sort_order: 7001,
        active: true,
    };
}

impl Sport {
    #[must_use]
    pub fn as_link(&self) -> SportsResponseUrl {
        SportsResponseUrl {
            id: Some(self.id)
        }
    }
}

impl Default for Sport {
    fn default() -> Self {
        Self::MLB
    }
}

impl Ord for Sport {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.sort_order.cmp(&other.sort_order)
    }
}

impl PartialOrd for Sport {
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
