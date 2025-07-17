
use crate::endpoints::{Position, RosterTypeId, StatsAPIUrl};
use crate::endpoints::teams::team::TeamId;
use crate::gen_params;
use std::fmt::{Display, Formatter};
use chrono::NaiveDate;
use serde::Deserialize;
use crate::endpoints::person::Person;
use crate::types::{Copyright, MLB_API_DATE_FORMAT};

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RosterResponse {
    pub copyright: Copyright,
    #[serde(default)]
    pub roster: Vec<RosterPlayer>,
    pub team_id: TeamId,
    pub roster_type: RosterTypeId,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RosterPlayer {
    pub person: Person,
    #[serde(deserialize_with = "crate::types::try_from_str")]
    pub jersey_number: Option<u8>,
    pub position: Position,
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
}

#[derive(Deserialize)]
struct __RosterStatusStruct {
    code: String,
    description: String,
}

impl TryFrom<__RosterStatusStruct> for RosterStatus {
    type Error = String;

    fn try_from(value: __RosterStatusStruct) -> Result<Self, Self::Error> {
        Ok(match &*value.code {
            "A" => RosterStatus::Active,
            "CL" => RosterStatus::Claimed,
            "RM" => RosterStatus::ReassignedToMinors,
            "RL" => RosterStatus::Released,
            "MIN" => RosterStatus::MinorLeagueContract,
            "D7" => RosterStatus::InjuryLeave10Day,
            "D10" => RosterStatus::InjuryLeave10Day,
            "D15" => RosterStatus::InjuryLeave15Day,
            "D60" => RosterStatus::InjuryLeave60Day,
            "TR" => RosterStatus::Traded,
            "DES" => RosterStatus::DesignatedForAssignment,
            "FA" => RosterStatus::FreeAgent,
            "RST" => RosterStatus::RestrictedList,
            code => return Err(format!("Invalid code '{code}' (desc: {})", value.description)),
        })
    }
}

pub struct RosterEndpointUrl {
    team_id: TeamId,
    season: Option<u16>,
    date: Option<NaiveDate>,
    roster_type: RosterTypeId,
}

impl Display for RosterEndpointUrl {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "http://statsapi.mlb.com/api/v1/teams/{}/roster{}", self.team_id, gen_params! { "season"?: self.season, "date"?: self.date.as_ref().map(|date| date.format(MLB_API_DATE_FORMAT)), "rosterType": &self.roster_type })
    }
}

impl StatsAPIUrl for RosterEndpointUrl {
    type Response = RosterResponse;
}

#[cfg(test)]
mod tests {
    use crate::endpoints::{RosterType, StatsAPIUrl};
    use crate::endpoints::teams::TeamsEndpointUrl;
    use crate::endpoints::teams::team::roster::RosterEndpointUrl;
    use chrono::{Datelike, Local};
    use crate::endpoints::meta::MetaEndpointUrl;
    use crate::endpoints::sports::SportId;

    #[tokio::test]
    #[cfg_attr(not(feature = "_heavy_tests"), ignore)]
    async fn test_this_year_all_mlb_teams_all_roster_types() {
        let season = Local::now().year() as _;
        let teams = TeamsEndpointUrl { sport_id: Some(SportId::MLB), season: Some(season) }.get().await.unwrap().teams;
        let roster_types = MetaEndpointUrl::<RosterType>::new().get().await.unwrap().entries;
        for team in teams {
            for roster_type in &roster_types {
                let json = reqwest::get(RosterEndpointUrl { team_id: team.id, season: Some(season), date: None, roster_type: roster_type.id.clone() }.to_string()).await.unwrap().bytes().await.unwrap();
                let mut de = serde_json::Deserializer::from_slice(&json);
                let result: Result<<RosterEndpointUrl as StatsAPIUrl>::Response, serde_path_to_error::Error<serde_json::Error>> = serde_path_to_error::deserialize(&mut de);
                match result {
                    Ok(_) => {}
                    Err(e) if format!("{:?}", e.inner()).contains("missing field `copyright`") => {}
                    Err(e) => panic!("Err: {:?}", e),
                }
            }
        }
    }
}

