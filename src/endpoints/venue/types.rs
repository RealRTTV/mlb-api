use derive_more::{Deref, DerefMut, Display};
use serde::Deserialize;
use crate::types::Copyright;

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VenuesResponse {
    pub copyright: Copyright,
    pub venues: Vec<HydratedVenue>,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UnhydratedVenue {
    id: VenueId,
    name: String,
}

#[derive(Debug, Deserialize, Deref, DerefMut, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HydratedVenue {
    #[deref]
    #[deref_mut]
    inner: UnhydratedVenue,
    active: bool,
    #[serde(deserialize_with = "crate::types::from_str")]
    season: u16,
}

#[repr(transparent)]
#[derive(Debug, Deserialize, Deref, Display, PartialEq, Eq, Copy, Clone)]
pub struct VenueId(u32);

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(untagged)]
pub enum Venue {
    Hydrated(HydratedVenue),
    Unhydrated(UnhydratedVenue),
}
