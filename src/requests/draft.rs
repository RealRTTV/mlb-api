use crate::person::{Person, PersonId};
use crate::season::SeasonId;
use crate::team::TeamId;
use crate::types::{Copyright, Location};
use crate::positions::PositionCode;
use crate::request::RequestURL;
use bon::Builder;
use derive_more::Display;
use serde::Deserialize;
use std::fmt::{Display, Formatter};
use thiserror::Error;
use crate::team::NamedTeam;

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

id!(EBISPersonId { id: u32 });

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
	/// a `PlayerId` on the EBIS System
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
	pub person: Option<Person>,
	#[serde(default = "NamedTeam::unknown_team")]
	pub team: NamedTeam,
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
			"JR" => Self::JR,
			_ => return Err(DraftTypeParseError::InvalidCode(value.code)),
		})
	}
}

#[derive(Builder)]
#[builder(start_fn = __latest)]
pub struct DraftRequestLatest {
	/// Year of the draft.
	#[builder(into)]
	year: Option<SeasonId>,
}

impl Display for DraftRequestLatest {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "http://statsapi.mlb.com/api/v1/draft/{year}/latest", year = self.year.map_or(String::new(), |x| x.to_string()))
	}
}

impl RequestURL for DraftRequestLatest {
	type Response = DraftResponse;
}

/// This request sorts into rounds
#[derive(Builder)]
#[builder(start_fn = regular)]
#[builder(derive(Into))]
pub struct DraftRequest {
	/// Year of the draft.
	#[builder(into)]
	year: Option<SeasonId>,
	/// Number of results to return.
	#[builder(into)]
	limit: Option<u32>,
	/// Offset in the results (used for pagination).
	#[builder(into)]
	offset: Option<u32>,
	/// Draft round.
	#[builder(into)]
	round: Option<u32>,

	/// Include only successfully drafted players
	#[builder(into)]
	drafted_only: Option<bool>,
	/// Filter players by the first character of their last name.
	#[builder(into)]
	last_name: Option<char>,
	/// Filter players by the first character of their school they were drafted from.
	#[builder(into)]
	school: Option<char>,
	/// Filter players by their position.
	#[builder(into)]
	position: Option<PositionCode>,
	/// Filter players by the team they were drafted by.
	#[builder(into)]
	team_id: Option<TeamId>,
	/// Filter players by their home country.
	#[builder(into)]
	home_country: Option<String>,
	/// Filter for a specific player id.
	#[builder(into)]
	player_id: Option<PersonId>,
}

impl<S: draft_request_builder::State + draft_request_builder::IsComplete> crate::request::RequestURLBuilderExt for DraftRequestBuilder<S> {
	type Built = DraftRequest;
}

impl DraftRequest {
	pub fn latest() -> DraftRequestLatestBuilder {
		DraftRequestLatest::__latest()
	}
}

impl Display for DraftRequest {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		let Self {
			year,
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
		} = self;
		write!(
			f,
			"http://statsapi.mlb.com/api/v1/draft/{year}{params}",
			year = year.map_or(String::new(), |x| x.to_string()),
			params = gen_params! {
					"limit"?: limit,
					"offset"?: offset,
					"round"?: round,
					"drafted"?: drafted_only,
					"name"?: last_name,
					"school"?: school,
					"position"?: position,
					"teamId"?: team_id,
					"homeCountry"?: home_country,
					"playerId"?: player_id,
				}
		)
	}
}

impl RequestURL for DraftRequest {
	type Response = DraftResponse;
}

/// This request gives a list of prospects.
#[derive(Builder)]
#[builder(start_fn = regular)]
#[builder(derive(Into))]
pub struct DraftProspectsRequest {
	/// Year of the draft.
	#[builder(into)]
	year: Option<SeasonId>,
	/// Number of results to return.
	#[builder(into)]
	limit: Option<u32>,
	/// Offset in the results (used for pagination).
	#[builder(into)]
	offset: Option<u32>,
	/// Draft round.
	#[builder(into)]
	round: Option<u32>,

	/// Include only successfully drafted players
	#[builder(into)]
	drafted_only: Option<bool>,
	/// Filter players by the first character of their last name.
	#[builder(into)]
	last_name: Option<char>,
	/// Filter players by the first character of their school they were drafted from.
	#[builder(into)]
	school: Option<char>,
	/// Filter players by their position.
	#[builder(into)]
	position: Option<PositionCode>,
	/// Filter players by the team they were drafted by.
	#[builder(into)]
	team_id: Option<TeamId>,
	/// Filter players by their home country.
	#[builder(into)]
	home_country: Option<String>,
	/// Filter for a specific player id.
	#[builder(into)]
	player_id: Option<PersonId>,
}

impl<S: draft_prospects_request_builder::State + draft_prospects_request_builder::IsComplete> crate::request::RequestURLBuilderExt for DraftProspectsRequestBuilder<S> {
    type Built = DraftProspectsRequest;
}

impl Display for DraftProspectsRequest {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		let Self {
			year,
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
		} = self;
		write!(
			f,
			"http://statsapi.mlb.com/api/v1/draft/prospects/{year}{params}",
			year = year.map_or(String::new(), |x| x.to_string()),
			params = gen_params! {
						"limit"?: limit,
						"offset"?: offset,
						"round"?: round,
						"drafted"?: drafted_only,
						"name"?: last_name,
						"school"?: school,
						"position"?: position,
						"teamId"?: team_id,
						"homeCountry"?: home_country,
						"playerId"?: player_id,
				}
		)
	}
}

impl RequestURL for DraftProspectsRequest {
	type Response = DraftProspectsResponse;
}

#[cfg(test)]
mod tests {
	use crate::draft::{DraftProspectsRequest, DraftRequest};
	use crate::request::RequestURLBuilderExt;
	use crate::TEST_YEAR;

	#[tokio::test]
	async fn draft_test_year() {
		let _ = DraftRequest::regular().year(TEST_YEAR).build_and_get().await.unwrap();
		let _ = DraftProspectsRequest::regular().year(TEST_YEAR).build_and_get().await.unwrap();
	}

	#[tokio::test]
	#[cfg_attr(not(feature = "_heavy_tests"), ignore)]
	async fn draft_all_years() {
		for year in 1965..=TEST_YEAR {
			let _ = DraftRequest::regular().year(year).build_and_get().await.unwrap();
			let _ = DraftProspectsRequest::regular().year(year).build_and_get().await.unwrap();
		}
	}
}
