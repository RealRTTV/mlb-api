use std::fmt::{Display, Formatter};
use std::ops::{Deref, DerefMut};
use derive_more::{Deref, DerefMut, Display, From};
use serde::Deserialize;
use crate::endpoints::StatsAPIUrl;
use crate::endpoints::league::League;
use crate::endpoints::sports::Sport;
use crate::gen_params;
use crate::types::Copyright;

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ConferencesResponse {
    pub copyright: Copyright,
    pub conferences: Vec<Conference>,
}

#[derive(Debug, Deserialize, Deref, DerefMut, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HydratedConference {
    pub abbreviation: String,
    #[serde(rename = "nameShort")] pub short_name: String,
    pub has_wildcard: bool,
    pub league: League,
    pub sport: Sport,

    #[deref]
    #[deref_mut]
    #[serde(flatten)]
    inner: IdentifiableConference,
}

#[derive(Debug, Deserialize, Deref, DerefMut, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NamedConference {
    pub name: String,

    #[deref]
    #[deref_mut]
    #[serde(flatten)]
    inner: IdentifiableConference,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
pub struct IdentifiableConference {
    pub id: ConferenceId,
}

#[repr(transparent)]
#[derive(Debug, Deserialize, Deref, Display, PartialEq, Eq, Copy, Clone)]
pub struct ConferenceId(u32);

#[derive(Debug, Deserialize, Eq, Clone, From)]
#[serde(untagged)]
pub enum Conference {
    Hydrated(HydratedConference),
    Named(NamedConference),
    Identifiable(IdentifiableConference),
}

impl Deref for Conference {
    type Target = IdentifiableConference;

    fn deref(&self) -> &Self::Target {
        match self {
            Self::Hydrated(inner) => inner,
            Self::Named(inner) => inner,
            Self::Identifiable(inner) => inner,
        }
    }
}

impl DerefMut for Conference {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            Self::Hydrated(inner) => inner,
            Self::Named(inner) => inner,
            Self::Identifiable(inner) => inner,
        }
    }
}

impl PartialEq for Conference {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

#[derive(Default)]
pub struct ConferencesEndpointUrl {
    pub conference_id: Option<ConferenceId>,
    pub season: Option<u16>,
}

impl Display for ConferencesEndpointUrl {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "http://statsapi.mlb.com/api/v1/conferences{params}", params = gen_params! { "conferenceId"?: self.conference_id, "season"?: self.season })
    }
}

impl StatsAPIUrl<ConferencesResponse> for ConferencesEndpointUrl {}

#[cfg(test)]
mod tests {
    use crate::endpoints::conferences::ConferencesEndpointUrl;
    use crate::endpoints::StatsAPIUrl;

    #[tokio::test]
    async fn parse_all_conferences() {
        let _response = ConferencesEndpointUrl { ..ConferencesEndpointUrl::default() }.get().await.unwrap();
    }
}
