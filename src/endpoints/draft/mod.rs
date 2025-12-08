use crate::person::{Person, PersonId};
use crate::teams::team::{Team, TeamId};
use crate::{Position, StatsAPIEndpointUrl};
use crate::gen_params;
use crate::types::{Copyright, Location};
use derive_more::{Deref, Display, From};
use serde::Deserialize;
use std::fmt::{Display, Formatter};
use thiserror::Error;
use crate::seasons::season::SeasonId;

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DraftResponse {
	pub copyright: Copyright,
	pub drafts: DraftYear,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DraftYear {
	#[serde(rename = "draftYear")]
	pub year: u32,
	pub rounds: Vec<DraftRound>,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DraftRound {
	pub round: String,
	pub picks: Vec<DraftPick>,
}

#[repr(transparent)]
#[derive(Debug, Deserialize, Deref, Display, PartialEq, Eq, Copy, Clone, Hash, From)]
pub struct EBISPersonId(u32);

impl EBISPersonId {
	#[must_use]
	pub const fn new(id: u32) -> Self {
		Self(id)
	}
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DraftProspectsResponse {
	pub copyright: Copyright,
	#[serde(rename = "totalSize")]
	pub total_prospects: usize,
	#[serde(rename = "returnedSize")]
	pub returned_prospects: usize,
	pub offset: usize,
	pub prospects: Vec<DraftPick>,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DraftPick {
	/// PlayerId on the EBIS System
	#[serde(rename = "bisPlayerId")]
	pub ebis_player_id: Option<EBISPersonId>,
	#[serde(default, rename = "pickRound")]
	pub round: String,
	#[serde(default)]
	pub pick_number: u32,
	#[serde(rename = "displayPickNumber")]
	pub displayed_pick_number: Option<u32>,
	pub rank: Option<u32>,
	#[serde(default, deserialize_with = "crate::types::try_from_str")]
	pub signing_bonus: Option<u32>,
	pub home: Location,
	pub scouting_report_url: Option<String>,
	pub school: School,
	pub blurb: Option<String>,
	#[serde(rename = "headshotLink", default = "get_default_headshot")]
	pub headshot_url: String,
	#[serde(default = "Person::unknown_person")]
	pub person: Person,
	#[serde(default = "Team::unknown_team")]
	pub team: Team,
	pub draft_type: DraftType,
	pub is_drafted: bool,
	pub is_pass: bool,
	pub year: SeasonId,
}

#[must_use]
pub fn get_default_headshot() -> String {
	"https://img.mlbstatic.com/mlb-photos/image/upload/d_people:generic:headshot:silo:current.png/w_120,q_auto:best/v1/people/0/headshot/draft/current".to_owned()
}

impl DraftPick {
	#[must_use]
	pub fn displayed_pick_number(&self) -> u32 {
		self.displayed_pick_number.unwrap_or(self.pick_number)
	}
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct School {
	pub name: Option<String>,
	pub city: Option<String>,
	pub class: Option<String>,
	pub country: Option<String>,
	pub state: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Copy, Clone, Display)]
#[serde(try_from = "__DraftTypeStruct")]
pub enum DraftType {
	#[display("June Amateur Draft")]
	JR,
	/// Never appears
	JS,
	/// Never appears
	NS,
	/// Never appears
	NR,
	/// Never appears
	AL,
	/// Never appears
	RA,
	/// Never appears
	RT,
	/// Never appears
	JD,
	/// Never appears
	AD,
}

#[derive(Deserialize)]
#[doc(hidden)]
struct __DraftTypeStruct {
	code: String,
}

#[derive(Debug, Error)]
enum DraftTypeParseError {
	#[error("Invalid draft type code {0}")]
	InvalidCode(String),
}

impl TryFrom<__DraftTypeStruct> for DraftType {
	type Error = DraftTypeParseError;

	fn try_from(value: __DraftTypeStruct) -> Result<Self, Self::Error> {
		Ok(match &*value.code {
			"JR" => DraftType::JR,
			_ => return Err(DraftTypeParseError::InvalidCode(value.code)),
		})
	}
}

/// This endpoint sorts into rounds
#[derive(Clone)]
pub struct DraftEndpoint {
	/// Year of the draft.
	pub year: Option<u32>,
	/// Kind of request to make.
	pub kind: DraftEndpointKind,
}

#[derive(Clone)]
pub enum DraftEndpointKind {
	/// Gets the latest draft pick.\
	/// During the draft, this is the most recent draft pick, however when the draft has ended, this is the last draft pick.
	Latest,
	/// A regular draft pick endpoint request.
	Regular(DraftEndpointData),
}

#[derive(Clone)]
pub struct DraftEndpointData {
	/// Number of results to return.
	pub limit: Option<u32>,
	/// Offset in the results (used for pagination).
	pub offset: Option<u32>,
	/// Draft round.
	pub round: Option<u32>,

	/// Include only successfully drafted players
	pub drafted_only: Option<bool>,
	/// Filter players by the first character of their last name.
	pub last_name: Option<char>,
	/// Filter players by the first character of their school they were drafted from.
	pub school: Option<char>,
	/// Filter players by their position.
	pub position: Option<Position>,
	/// Filter players by the team they were drafted by.
	pub team_id: Option<TeamId>,
	/// Filter players by their home country.
	pub home_country: Option<String>,
	/// Filter for a specific player id.
	pub player_id: Option<PersonId>,
}

impl Display for DraftEndpoint {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self.kind.clone() {
			DraftEndpointKind::Latest => write!(f, "http://statsapi.mlb.com/api/v1/draft/{year}/latest", year = self.year.map_or(String::new(), |x| x.to_string())),
			DraftEndpointKind::Regular(DraftEndpointData {
				                              limit,
				                              offset,
				                              round,
				                              drafted_only,
				                              last_name,
				                              school,
				                              position,
				                              team_id,
				                              home_country,
				                              player_id,
			                              }) => write!(
				f,
				"http://statsapi.mlb.com/api/v1/draft/{year}{params}",
				year = self.year.map_or(String::new(), |x| x.to_string()),
				params = gen_params! {
					"limit"?: limit,
					"offset"?: offset,
					"round"?: round,
					"drafted"?: drafted_only,
					"name"?: last_name,
					"school"?: school,
					"position"?: position.as_ref().map(|pos| &pos.code),
					"teamId"?: team_id,
					"homeCountry"?: home_country,
					"playerId"?: player_id,
				}
			),
		}
	}
}

impl StatsAPIEndpointUrl for DraftEndpoint {
	type Response = DraftResponse;
}

// todo: make type system allow for only the `Regular` variant here
/// This endpoint gives a list of prospects.
pub struct DraftProspectsEndpoint {
	/// Year of the draft.
	pub year: Option<u32>,
	/// Kind of request to make.
	pub kind: DraftEndpointData,
}

impl Display for DraftProspectsEndpoint {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		let DraftEndpointData {
			limit,
			offset,
			round,
			drafted_only,
			last_name,
			school,
			position,
			team_id,
			home_country,
			player_id,
		} = &self.kind;
		write!(
			f,
			"http://statsapi.mlb.com/api/v1/draft/prospects/{year}{params}",
			year = self.year.map_or(String::new(), |x| x.to_string()),
			params = gen_params! {
						"limit"?: limit,
						"offset"?: offset,
						"round"?: round,
						"drafted"?: drafted_only,
						"name"?: last_name,
						"school"?: school,
						"position"?: position.as_ref().map(|pos| &pos.code),
						"teamId"?: team_id,
						"homeCountry"?: home_country,
						"playerId"?: player_id,
				}
		)
	}
}

impl StatsAPIEndpointUrl for DraftProspectsEndpoint {
	type Response = DraftProspectsResponse;
}

#[cfg(test)]
mod tests {
	use crate::StatsAPIEndpointUrl;
	use crate::draft::{DraftEndpoint, DraftEndpointKind, DraftProspectsEndpoint, DraftEndpointData};
	use chrono::{Datelike, Local};

	#[tokio::test]
	async fn draft_2025() {
		let _ = DraftEndpoint {
			year: Some(2025),
			kind: DraftEndpointKind::Regular(DraftEndpointData {
				limit: None,
				offset: None,
				round: None,
				drafted_only: None,
				last_name: None,
				school: None,
				position: None,
				team_id: None,
				home_country: None,
				player_id: None,
			}),
		}
			.get()
			.await
			.unwrap();

		let _ = DraftProspectsEndpoint {
			year: Some(2025),
			kind: DraftEndpointData {
				limit: None,
				offset: None,
				round: None,
				drafted_only: None,
				last_name: None,
				school: None,
				position: None,
				team_id: None,
				home_country: None,
				player_id: None,
			},
		}
			.get()
			.await
			.unwrap();
	}

	#[tokio::test]
	#[cfg_attr(not(feature = "_heavy_tests"), ignore)]
	async fn draft_all_years() {
		for year in 1965..=Local::now().year() as _ {
			let _ = crate::serde_path_to_error_parse(DraftEndpoint {
				year: Some(year),
				kind: DraftEndpointKind::Regular(DraftEndpointData {
					limit: None,
					offset: None,
					round: None,
					drafted_only: None,
					last_name: None,
					school: None,
					position: None,
					team_id: None,
					home_country: None,
					player_id: None,
				}),
			}).await;

			let _ = crate::serde_path_to_error_parse(DraftProspectsEndpoint {
				year: Some(year),
				kind: DraftEndpointData {
					limit: None,
					offset: None,
					round: None,
					drafted_only: None,
					last_name: None,
					school: None,
					position: None,
					team_id: None,
					home_country: None,
					player_id: None,
				},
			}).await;
		}
	}
}
