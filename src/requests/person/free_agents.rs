use crate::season::SeasonId;
use crate::Copyright;
use bon::Builder;
use chrono::NaiveDate;
use serde::Deserialize;
use serde_with::serde_as;
use serde_with::DefaultOnError;
use std::fmt::{Display, Formatter};
use crate::person::NamedPerson;
use crate::meta::NamedPosition;
use crate::request::RequestURL;
use crate::team::NamedTeam;

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FreeAgentsResponse {
	pub copyright: Copyright,
	pub free_agents: Vec<FreeAgent>,
}

#[serde_as]
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
#[doc(hidden)]
struct __FreeAgentStruct {
	player: NamedPerson,
	#[serde_as(deserialize_as = "DefaultOnError")]
	original_team: Option<NamedTeam>,
	#[serde_as(deserialize_as = "DefaultOnError")]
	new_team: Option<NamedTeam>,
	notes: Option<String>,
	date_signed: Option<NaiveDate>,
	date_declared: Option<NaiveDate>,
	position: NamedPosition,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(from = "__FreeAgentStruct")]
pub struct FreeAgent {
	pub player: NamedPerson,
	pub original_team: NamedTeam,
	pub new_team: NamedTeam,
	pub notes: Option<String>,
	pub date_signed: NaiveDate,
	pub date_declared: NaiveDate,
	pub position: NamedPosition,
}

impl From<__FreeAgentStruct> for FreeAgent {
	fn from(value: __FreeAgentStruct) -> Self {
		Self {
			player: value.player,
			original_team: value.original_team.unwrap_or_else(NamedTeam::unknown_team),
			new_team: value.new_team.unwrap_or_else(NamedTeam::unknown_team),
			notes: value.notes,
			date_signed: value.date_signed.or(value.date_declared).unwrap_or_default(),
			date_declared: value.date_declared.or(value.date_signed).unwrap_or_default(),
			position: value.position,
		}
	}
}

#[derive(Builder)]
#[builder(derive(Into))]
pub struct FreeAgentsRequest {
	#[builder(into)]
	season: SeasonId,
}

impl<S: free_agents_request_builder::State + free_agents_request_builder::IsComplete> crate::request::RequestURLBuilderExt for FreeAgentsRequestBuilder<S> {
	type Built = FreeAgentsRequest;
}

impl Display for FreeAgentsRequest {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "http://statsapi.mlb.com/api/v1/people/freeAgents{}", gen_params! { "season": self.season })
	}
}

impl RequestURL for FreeAgentsRequest {
	type Response = FreeAgentsResponse;
}

#[cfg(test)]
mod tests {
	use crate::request::RequestURLBuilderExt;
	use crate::person::free_agents::FreeAgentsRequest;
	use crate::TEST_YEAR;

	#[tokio::test]
	async fn test_one_year() {
		let _response = FreeAgentsRequest::builder().season(TEST_YEAR).build_and_get().await.unwrap();
	}

	#[tokio::test]
	async fn test_all_seasons() {
		for season in 2001..=TEST_YEAR {
			let _response = FreeAgentsRequest::builder().season(season).build_and_get().await.unwrap();
		}
	}
}
