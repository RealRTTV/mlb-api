use serde::Deserialize;
use crate::endpoints::person::DetailedNamedPerson;
use crate::endpoints::types::Copyright;

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SportsPlayersResponse {
    pub copyright: Copyright,
    pub people: Vec<DetailedNamedPerson>,
}
