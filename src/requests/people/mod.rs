#![allow(clippy::trait_duplication_in_bounds, reason = "serde duplicates it")]

use crate::person::{Person, PersonHydrations};
use crate::types::Copyright;
use serde::Deserialize;

pub mod free_agents;

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(bound = "H: PersonHydrations")]
pub struct PeopleResponse<H: PersonHydrations> {
    pub copyright: Copyright,
    #[serde(default)]
    pub people: Vec<Person<H>>,
}
