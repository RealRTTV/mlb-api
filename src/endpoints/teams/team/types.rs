use std::ops::{Deref, DerefMut};
use derive_more::{Deref, DerefMut, Display};
use serde::Deserialize;
use crate::endpoints::league::League;
use crate::endpoints::sports::Sport;
use crate::endpoints::venue::Venue;

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RegularTeam {
    #[serde(deserialize_with = "crate::types::from_yes_no")]
    pub all_star_status: bool,
    pub active: bool,
    pub id: TeamId,
    pub season: u16,
    pub venue: Venue,
    #[serde(flatten)]
    pub name: TeamName,
    pub location_name: String,
    #[serde(deserialize_with = "crate::types::from_str")]
    pub first_year_of_play: u16,
    pub league: League,
    // pub division: Option<Division>,
    pub sport: Sport,
    #[serde(flatten)]
    pub parent_organization: Option<Organization>,
}

#[derive(Debug, Deserialize, Deref, DerefMut, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MLBTeam {
    #[deref]
    #[deref_mut]
    #[serde(flatten)]
    pub inner: RegularTeam,
    pub spring_venue: Venue,
    pub spring_league: League,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(untagged)]
pub enum Team {
    MLB(MLBTeam),
    Regular(RegularTeam),
}

impl Deref for Team {
    type Target = RegularTeam;

    fn deref(&self) -> &Self::Target {
        match self {
            Self::MLB(inner) => inner,
            Self::Regular(inner) => inner,
        }
    }
}

impl DerefMut for Team {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            Self::MLB(inner) => inner,
            Self::Regular(inner) => inner,
        }
    }
}

#[derive(Debug, Deserialize, Deref, DerefMut, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TeamName {
    #[deref]
    #[deref_mut]
    pub name: String,
    pub team_code: String,
    pub file_code: String,
    pub abbreviation: String,
    pub team_name: String,
    pub short_name: String,
    pub franchise_name: String,
    pub club_name: String,
}

#[repr(transparent)]
#[derive(Debug, Deserialize, Deref, Display, PartialEq, Eq, Copy, Clone)]
pub struct TeamId(u32);

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UnhydratedOrganization {
    #[serde(rename = "parentOrgName")] pub name: String,
    #[serde(rename = "parentOrgId")] pub id: OrganizationId,
}

#[repr(transparent)]
#[derive(Debug, Deserialize, Deref, Display, PartialEq, Eq, Copy, Clone)]
pub struct OrganizationId(u16);

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(untagged)]
pub enum Organization {
    Unhydrated(UnhydratedOrganization),
}

impl Deref for Organization {
    type Target = UnhydratedOrganization;

    fn deref(&self) -> &Self::Target {
        match self {
            Self::Unhydrated(inner) => inner,
        }
    }
}

impl DerefMut for Organization {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            Organization::Unhydrated(inner) => inner,
        }
    }
}
