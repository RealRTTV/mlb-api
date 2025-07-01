mod types;
pub use types::*;
pub mod players;

use crate::endpoints::Url;
use crate::gen_params;
use std::fmt::{Display, Formatter};

pub struct SportsEndpointUrl {
    pub id: Option<SportId>,
}

impl Display for SportsEndpointUrl {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "https://statsapi.mlb.com/api/v1/sports{params}",
            params = gen_params! { "sportId"?: self.id }
        )
    }
}

impl Url<SportsResponse> for SportsEndpointUrl {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::endpoints::Url;
    use std::borrow::Cow;

    #[tokio::test]
    async fn check_updated() {
        let target = SportsResponse {
            copyright: Default::default(),
            sports: vec![
                HydratedSport {
                    inner: UnhydratedSport {
                        id: SportId::MLB,
                        name: Cow::Borrowed("Major League Baseball"),
                    },
                    code: Cow::Borrowed("mlb"),
                    abbreviation: Cow::Borrowed("MLB"),
                    sort_order: 11,
                    active: true,
                },
                HydratedSport {
                    inner: UnhydratedSport {
                        id: SportId::AAA,
                        name: Cow::Borrowed("Triple-A"),
                    },
                    code: Cow::Borrowed("aaa"),
                    abbreviation: Cow::Borrowed("AAA"),
                    sort_order: 101,
                    active: true,
                },
                HydratedSport {
                    inner: UnhydratedSport {
                        id: SportId::AA,
                        name: Cow::Borrowed("Double-A"),
                    },
                    code: Cow::Borrowed("aax"),
                    abbreviation: Cow::Borrowed("AA"),
                    sort_order: 201,
                    active: true,
                },
                HydratedSport {
                    inner: UnhydratedSport {
                        id: SportId::HIGH_A,
                        name: Cow::Borrowed("High-A"),
                    },
                    code: Cow::Borrowed("afa"),
                    abbreviation: Cow::Borrowed("A+"),
                    sort_order: 301,
                    active: true,
                },
                HydratedSport {
                    inner: UnhydratedSport {
                        id: SportId::A,
                        name: Cow::Borrowed("Single-A"),
                    },
                    code: Cow::Borrowed("afx"),
                    abbreviation: Cow::Borrowed("A"),
                    sort_order: 401,
                    active: true,
                },
                HydratedSport {
                    inner: UnhydratedSport {
                        id: SportId::ROOKIE,
                        name: Cow::Borrowed("Rookie"),
                    },
                    code: Cow::Borrowed("rok"),
                    abbreviation: Cow::Borrowed("ROK"),
                    sort_order: 701,
                    active: true,
                },
                HydratedSport {
                    inner: UnhydratedSport {
                        id: SportId::WINTER,
                        name: Cow::Borrowed("Winter Leagues"),
                    },
                    code: Cow::Borrowed("win"),
                    abbreviation: Cow::Borrowed("WIN"),
                    sort_order: 1301,
                    active: true,
                },
                HydratedSport {
                    inner: UnhydratedSport {
                        id: SportId::MILB,
                        name: Cow::Borrowed("Minor League Baseball"),
                    },
                    code: Cow::Borrowed("min"),
                    abbreviation: Cow::Borrowed("Minors"),
                    sort_order: 1402,
                    active: true,
                },
                HydratedSport {
                    inner: UnhydratedSport {
                        id: SportId::INDIE,
                        name: Cow::Borrowed("Independent Leagues"),
                    },
                    code: Cow::Borrowed("ind"),
                    abbreviation: Cow::Borrowed("IND"),
                    sort_order: 2101,
                    active: true,
                },
                HydratedSport {
                    inner: UnhydratedSport {
                        id: SportId::NLB,
                        name: Cow::Borrowed("Negro League Baseball"),
                    },
                    code: Cow::Borrowed("nlb"),
                    abbreviation: Cow::Borrowed("NLB"),
                    sort_order: 2401,
                    active: true, // what??
                },
                HydratedSport {
                    inner: UnhydratedSport {
                        id: SportId::KBO,
                        name: Cow::Borrowed("Korean Baseball Organization"),
                    },
                    code: Cow::Borrowed("kor"),
                    abbreviation: Cow::Borrowed("KOR"),
                    sort_order: 2601,
                    active: true,
                },
                HydratedSport {
                    inner: UnhydratedSport {
                        id: SportId::NPB,
                        name: Cow::Borrowed("Nippon Professional Baseball"),
                    },
                    code: Cow::Borrowed("jml"),
                    abbreviation: Cow::Borrowed("NPB"),
                    sort_order: 2701,
                    active: true,
                },
                HydratedSport {
                    inner: UnhydratedSport {
                        id: SportId::INTERNATIONAL,
                        name: Cow::Borrowed("International Baseball"),
                    },
                    code: Cow::Borrowed("int"),
                    abbreviation: Cow::Borrowed("INT"),
                    sort_order: 3501,
                    active: true,
                },
                HydratedSport {
                    inner: UnhydratedSport {
                        id: SportId::INTERNATIONAL_18U,
                        name: Cow::Borrowed("International Baseball (18U)"),
                    },
                    code: Cow::Borrowed("nae"),
                    abbreviation: Cow::Borrowed("18U"),
                    sort_order: 3503,
                    active: true,
                },
                HydratedSport {
                    inner: UnhydratedSport {
                        id: SportId::INTERNATIONAL_16U,
                        name: Cow::Borrowed("International Baseball (16 and under)"),
                    },
                    code: Cow::Borrowed("nas"),
                    abbreviation: Cow::Borrowed("16U"),
                    sort_order: 3505,
                    active: true,
                },
                HydratedSport {
                    inner: UnhydratedSport {
                        id: SportId::INTERNATIONAL_AMATEUR,
                        name: Cow::Borrowed("International Baseball (amateur)"),
                    },
                    code: Cow::Borrowed("ame"),
                    abbreviation: Cow::Borrowed("AME"),
                    sort_order: 3509,
                    active: true,
                },
                HydratedSport {
                    inner: UnhydratedSport {
                        id: SportId::COLLEGE,
                        name: Cow::Borrowed("College Baseball"),
                    },
                    code: Cow::Borrowed("bbc"),
                    abbreviation: Cow::Borrowed("College"),
                    sort_order: 5101,
                    active: true,
                },
                HydratedSport {
                    inner: UnhydratedSport {
                        id: SportId::HIGH_SCHOOL,
                        name: Cow::Borrowed("High School Baseball"),
                    },
                    code: Cow::Borrowed("hsb"),
                    abbreviation: Cow::Borrowed("H.S."),
                    sort_order: 6201,
                    active: true,
                },
                HydratedSport {
                    inner: UnhydratedSport {
                        id: SportId::WPF,
                        name: Cow::Borrowed("Women's Professional Softball"),
                    },
                    code: Cow::Borrowed("wps"),
                    abbreviation: Cow::Borrowed("WPS"),
                    sort_order: 7001,
                    active: true,
                },
            ],
        };
        let result = SportsEndpointUrl { id: None }.get().await.unwrap();
        assert_eq!(result, target);
    }
}
