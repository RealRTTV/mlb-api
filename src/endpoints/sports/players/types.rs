use std::fmt::{Display, Formatter};
use serde::Deserialize;
use crate::endpoints::person::DetailedNamedPerson;
use crate::endpoints::sports::SportId;
use crate::endpoints::types::Copyright;
use crate::endpoints::Url;

pub struct SportsPlayersResponseUrl {
    pub id: SportId,
}

impl Display for SportsPlayersResponseUrl {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "https://statsapi.mlb.com/api/v1/sports/{id}/players", id = self.id)
    }
}

impl Url<SportsPlayersResponse> for SportsPlayersResponseUrl {}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SportsPlayersResponse {
    pub copyright: Copyright,
    pub people: Vec<DetailedNamedPerson>,
}
