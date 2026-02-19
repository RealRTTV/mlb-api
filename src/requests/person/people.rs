#![allow(clippy::trait_duplication_in_bounds, reason = "serde duplicates it")]

use crate::person::{Ballplayer, PersonHydrations};
use crate::Copyright;
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(bound = "H: PersonHydrations")]
pub struct PeopleResponse<H: PersonHydrations> {
    pub copyright: Copyright,
    #[serde(default)]
    pub people: Vec<Ballplayer<H>>,
}
