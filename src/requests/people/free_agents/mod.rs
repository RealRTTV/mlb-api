use crate::gen_params;
use crate::person::Person;
use crate::seasons::season::SeasonId;
use crate::teams::team::Team;
use crate::types::Copyright;
use crate::{Position, StatsAPIRequestUrl};
use bon::Builder;
use chrono::NaiveDate;
use serde::Deserialize;
use serde_with::serde_as;
use serde_with::DefaultOnError;
use std::fmt::{Display, Formatter};

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
	player: Person,
	#[serde_as(deserialize_as = "DefaultOnError")]
	original_team: Option<Team>,
	#[serde_as(deserialize_as = "DefaultOnError")]
	new_team: Option<Team>,
	notes: Option<String>,
	date_signed: Option<NaiveDate>,
	date_declared: Option<NaiveDate>,
	position: Position,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(from = "__FreeAgentStruct")]
pub struct FreeAgent {
	pub player: Person,
	pub original_team: Team,
	pub new_team: Team,
	pub notes: Option<String>,
	pub date_signed: NaiveDate,
	pub date_declared: NaiveDate,
	pub position: Position,
}

impl From<__FreeAgentStruct> for FreeAgent {
	fn from(value: __FreeAgentStruct) -> Self {
		FreeAgent {
			player: value.player,
			original_team: value.original_team.unwrap_or_else(Team::unknown_team),
			new_team: value.new_team.unwrap_or_else(Team::unknown_team),
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

impl<S: free_agents_request_builder::State> crate::requests::links::StatsAPIRequestUrlBuilderExt for FreeAgentsRequestBuilder<S> where S: free_agents_request_builder::IsComplete {
	type Built = FreeAgentsRequest;
}

impl Display for FreeAgentsRequest {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "http://statsapi.mlb.com/api/v1/people/freeAgents{}", gen_params! { "season": self.season })
	}
}

impl StatsAPIRequestUrl for FreeAgentsRequest {
	type Response = FreeAgentsResponse;
}

#[cfg(test)]
mod tests {
	use crate::people::free_agents::FreeAgentsRequest;
	use crate::StatsAPIRequestUrlBuilderExt;
	use chrono::{Datelike, Local};

	#[tokio::test]
	async fn test_2025() {
		let _response = FreeAgentsRequest::builder().season(2025).build_and_get().await.unwrap();
	}

	#[tokio::test]
	async fn test_all_seasons() {
		for season in 2001..=Local::now().year() as _ {
			let _response = FreeAgentsRequest::builder().season(season).build_and_get().await.unwrap();
		}
	}
}
