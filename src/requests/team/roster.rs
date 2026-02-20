use crate::person::NamedPerson;
use crate::season::SeasonId;
use crate::team::TeamId;
use crate::{Copyright, MLB_API_DATE_FORMAT};
use bon::Builder;
use chrono::NaiveDate;
use serde::Deserialize;
use std::fmt::{Display, Formatter};
use crate::meta::NamedPosition;
use crate::request::RequestURL;
use crate::meta::RosterType;
use crate::team::NamedTeam;

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RosterResponse {
    pub copyright: Copyright,
    #[serde(default)]
    pub roster: Vec<RosterPlayer>,
    pub team_id: TeamId,
    pub roster_type: RosterType,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RosterPlayer {
    pub person: NamedPerson,
    #[serde(deserialize_with = "crate::try_from_str")]
    pub jersey_number: Option<u8>,
    pub position: NamedPosition,
    pub status: RosterStatus,
    pub parent_team_id: Option<TeamId>,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Copy, Clone)]
#[serde(try_from = "__RosterStatusStruct")]
pub enum RosterStatus {
    Active,
    Claimed,
    ReassignedToMinors,
    Released,
    MinorLeagueContract,
    InjuryLeave7Day,
    InjuryLeave10Day,
    InjuryLeave15Day,
    InjuryLeave60Day,
    Traded,
    DesignatedForAssignment,
    FreeAgent,
    RestrictedList,
    AssignedToNewTeam,
    RehabAssignment,
    NonRosterInvitee,
    Waived,
    Deceased,
    VoluntarilyRetired,
}

#[derive(Deserialize)]
#[doc(hidden)]
struct __RosterStatusStruct {
    code: String,
    description: String,
}

impl TryFrom<__RosterStatusStruct> for RosterStatus {
    type Error = String;

    fn try_from(value: __RosterStatusStruct) -> Result<Self, Self::Error> {
        Ok(match &*value.code {
            "A" => Self::Active,
            "CL" => Self::Claimed,
            "RM" => Self::ReassignedToMinors,
            "RL" => Self::Released,
            "MIN" => Self::MinorLeagueContract,
            "D7" => Self::InjuryLeave7Day,
            "D10" => Self::InjuryLeave10Day,
            "D15" => Self::InjuryLeave15Day,
            "D60" => Self::InjuryLeave60Day,
            "TR" => Self::Traded,
            "DES" => Self::DesignatedForAssignment,
            "FA" => Self::FreeAgent,
            "RST" => Self::RestrictedList,
            "ASG" => Self::AssignedToNewTeam,
            "RA" => Self::RehabAssignment,
            "NRI" => Self::NonRosterInvitee,
            "WA" => Self::Waived,
            "DEC" => Self::Deceased,
            "RET" => Self::VoluntarilyRetired,
            code => return Err(format!("Invalid code '{code}' (desc: {})", value.description)),
        })
    }
}

#[derive(Builder)]
#[builder(derive(Into))]
pub struct RosterRequest {
    #[builder(into)]
    team_id: TeamId,
    #[builder(into)]
    season: Option<SeasonId>,
    date: Option<NaiveDate>,
    #[builder(into)]
    roster_type: RosterType,
}

impl<S: roster_request_builder::State + roster_request_builder::IsComplete> crate::request::RequestURLBuilderExt for RosterRequestBuilder<S> {
    type Built = RosterRequest;
}

impl Display for RosterRequest {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "http://statsapi.mlb.com/api/v1/teams/{}/roster{}", self.team_id, gen_params! { "season"?: self.season, "date"?: self.date.as_ref().map(|date| date.format(MLB_API_DATE_FORMAT)), "rosterType": &self.roster_type })
    }
}

impl RequestURL for RosterRequest {
    type Response = RosterResponse;
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RosterEntry {
    pub position: NamedPosition,
    pub status: RosterStatus,
    pub team: NamedTeam,
    pub is_active: bool,
    pub is_active_forty_man: bool,
    pub start_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
    pub status_date: NaiveDate,
}

#[cfg(test)]
mod tests {
	use crate::meta::MetaRequest;
    use crate::request::{RequestURL, RequestURLBuilderExt};
    use crate::meta::RosterType;
    use crate::team::roster::RosterRequest;
	use crate::team::teams::TeamsRequest;
    use crate::TEST_YEAR;

    #[tokio::test]
    #[cfg_attr(not(feature = "_heavy_tests"), ignore)]
    async fn test_this_year_all_mlb_teams_all_roster_types() {
        let season = TEST_YEAR;
        let teams = TeamsRequest::mlb_teams().season(season).build_and_get().await.unwrap().teams;
        let roster_types = MetaRequest::<RosterType>::new().get().await.unwrap().entries;
        for team in teams {
            for roster_type in &roster_types {
                let _ = RosterRequest::builder().team_id(team.id).season(season).roster_type(*roster_type).build_and_get().await.unwrap();
            }
        }
    }
}

