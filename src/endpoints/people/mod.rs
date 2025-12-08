use serde::Deserialize;
use crate::person::{Person, PersonHydrations};
use crate::types::Copyright;

pub mod free_agents;

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(bound = "H: PersonHydrations")]
pub struct PeopleResponse<H: PersonHydrations> {
    pub copyright: Copyright,
    #[serde(default)]
    pub people: Vec<Person<H>>,
}
