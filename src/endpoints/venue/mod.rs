mod types;
pub use types::*;

use std::fmt::{Display, Formatter};
use crate::endpoints::sports::SportId;
use crate::endpoints::Url;
use crate::gen_params;

#[derive(Default)]
pub struct VenuesEndpointUrl {
    pub id: Option<SportId>,
    pub season: Option<u16>,
}

impl Display for VenuesEndpointUrl {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "https://statsapi.mlb.com/api/v1/venues{id}{params}", id = self.id.map_or(String::new(), |id| format!("/{id}")), params = gen_params! { "season"?: self.season })
    }
}

impl Url<VenuesResponse> for VenuesEndpointUrl {}

#[cfg(test)]
mod tests {
    use chrono::{Datelike, Local};
    use crate::endpoints::Url;
    use crate::endpoints::venue::VenuesEndpointUrl;

    #[tokio::test]
    async fn parse_all_venues() {
        for season in 1876..=Local::now().year() as _ {
            let _response = VenuesEndpointUrl { id: None, season: Some(season) }.get().await.unwrap();
        }
    }
}