mod types;

pub use types::*;

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
        let result = SportsResponseUrl::new(None).get().await.unwrap();
        assert_eq!(result, target);
    }
}