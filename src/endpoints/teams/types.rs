use serde::Deserialize;
use crate::endpoints::teams::team::RegularTeam;
use crate::types::Copyright;

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TeamsResponse {
    pub copyright: Copyright,
    pub teams: Vec<RegularTeam>,
}