use std::fmt::{Display, Formatter};
use crate::endpoints::league::{League, LeagueId};
use crate::endpoints::sports::{Sport, SportId};
use derive_more::{Deref, DerefMut, Display, From};
use serde::Deserialize;
use std::ops::Deref;
use crate::endpoints::StatsAPIUrl;
use crate::gen_params;
use crate::types::Copyright;

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
pub struct AwardsResponse {
    pub copyright: Copyright,
    pub awards: Vec<Award>,
}

#[derive(Debug, Deserialize, Deref, DerefMut, PartialEq, Eq, Clone)]
pub struct HydratedAward {
    pub name: String,
    pub description: Option<String>,
    pub sport: Option<Sport>,
    pub league: Option<League>,
    pub notes: Option<String>,

    #[deref]
    #[deref_mut]
    #[serde(flatten)]
    inner: IdentifiableAward,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
pub struct IdentifiableAward {
    pub id: AwardId,
}

#[repr(transparent)]
#[derive(Debug, Deserialize, Deref, Display, PartialEq, Eq, Clone)]
pub struct AwardId(String);

#[derive(Debug, Deserialize, Eq, Clone, From)]
#[serde(untagged)]
pub enum Award {
    Hydrated(HydratedAward),
    Identifiable(IdentifiableAward),
}

impl PartialEq for Award {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Deref for Award {
    type Target = IdentifiableAward;

    fn deref(&self) -> &Self::Target {
        match self {
            Self::Hydrated(inner) => inner,
            Self::Identifiable(inner) => inner,
        }
    }
}

#[derive(Default)]
pub struct AwardEndpointUrl {
    pub award_id: Option<AwardId>,
    pub sport_id: Option<SportId>,
    pub league_id: Option<LeagueId>,
    pub season: Option<u16>,
}

impl Display for AwardEndpointUrl {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "http://statsapi.mlb.com/api/v1/awards{params}", params = gen_params! { "awardId"?: self.award_id.as_ref(), "sportId"?: self.sport_id, "leagueId"?: self.league_id, "season"?: self.season })
    }
}

impl StatsAPIUrl<AwardsResponse> for AwardEndpointUrl {}

#[cfg(test)]
mod tests {
    use crate::endpoints::awards::AwardEndpointUrl;
    use crate::endpoints::StatsAPIUrl;

    #[tokio::test]
    async fn parse_this_season() {
        let _response = AwardEndpointUrl::default().get().await.unwrap();
    }
}
