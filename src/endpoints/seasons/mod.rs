pub mod season;

mod types;

use std::fmt::{Display, Formatter};
pub use types::*;
use crate::endpoints::sports::SportId;
use crate::endpoints::Url;

pub struct SeasonsEndpointUrl {
    sport_id: SportId,
}

impl Display for SeasonsEndpointUrl {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "https://statsapi.mlb.com/api/v1/seasons?sportId={sport_id}", sport_id = self.sport_id)
    }
}

impl Url<SeasonsResponse> for SeasonsEndpointUrl {}

#[cfg(test)]
mod tests {
    use crate::endpoints::seasons::SeasonsEndpointUrl;
    use crate::endpoints::sports::SportId;
    use crate::endpoints::Url;

    #[tokio::test]
    async fn parses_all_seasons() {
        for id in SportId::IDS {
            let _response = SeasonsEndpointUrl { sport_id: id }.get().await.unwrap();
        }
    }
}
