mod types;
pub use types::*;

#[cfg(test)]
mod tests {
    use crate::endpoints::sports::SportId;
    use crate::endpoints::Url;
    use super::*;

    #[tokio::test]
    async fn parse_all_players() {
        let _response = SportsPlayersResponseUrl { id: SportId::MLB }.get().await.unwrap();
    }
}
