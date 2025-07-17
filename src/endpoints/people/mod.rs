use serde::Deserialize;
use crate::endpoints::person::Person;
use crate::types::Copyright;

pub mod changes;
pub mod free_agents;

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PeopleResponse {
    pub copyright: Copyright,
    #[serde(default)]
    pub people: Vec<Person>,
}
