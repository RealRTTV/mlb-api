mod types;
pub use types::*;
pub mod players;

use std::fmt::{Display, Formatter};
use crate::endpoints::Url;
use crate::gen_params;

pub struct SportsResponseUrl {
    pub id: Option<SportId>,
}

impl Display for SportsResponseUrl {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "https://statsapi.mlb.com/api/v1/sports{params}", params = gen_params! { "sportId"?: self.id })
    }
}

impl Url<SportsResponse> for SportsResponseUrl {}

#[cfg(test)]
mod tests {
    use crate::endpoints::Url;
    use super::*;

    #[tokio::test]
    async fn check_updated() {
        let target = SportsResponse {
            copyright: Default::default(),
            sports: vec![
                Sport::MLB,
                Sport::AAA,
                Sport::AA,
                Sport::HIGH_A,
                Sport::A,
                Sport::ROOKIE,
                Sport::WINTER,
                Sport::MILB,
                Sport::INDIE,
                Sport::NLB,
                Sport::KBO,
                Sport::NPB,
                Sport::INTERNATIONAL,
                Sport::INTERNATIONAL_18U,
                Sport::INTERNATIONAL_16U,
                Sport::INTERNATIONAL_AMATEUR,
                Sport::COLLEGE,
                Sport::HIGH_SCHOOL,
                Sport::WPF,
            ],
        };
        let result = SportsResponseUrl { id: None }.get().await.unwrap();
        assert_eq!(result, target);
    }
}