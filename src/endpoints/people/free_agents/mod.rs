use serde_with::DefaultOnError;
use crate::endpoints::person::Person;
use crate::endpoints::teams::team::Team;
use crate::endpoints::{Position, StatsAPIEndpointUrl};
use crate::gen_params;
use crate::types::Copyright;
use chrono::NaiveDate;
use serde::Deserialize;
use std::fmt::{Display, Formatter};
use serde_with::serde_as;

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

pub struct FreeAgentsEndpoint {
	pub season: u16,
}

impl Display for FreeAgentsEndpoint {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "http://statsapi.mlb.com/api/v1/people/freeAgents{}", gen_params! { "season": self.season })
	}
}

impl StatsAPIEndpointUrl for FreeAgentsEndpoint {
	type Response = FreeAgentsResponse;
}

#[cfg(test)]
mod tests {
	use crate::endpoints::people::free_agents::FreeAgentsEndpoint;
	use crate::endpoints::StatsAPIEndpointUrl;
	use chrono::{Datelike, Local};

	#[tokio::test]
	async fn test_2025() {
		let _response = FreeAgentsEndpoint { season: 2025 }.get().await.unwrap();
	}

	#[tokio::test]
	async fn test_all_seasons() {
		for season in 2001..=Local::now().year() as _ {
			let _response = FreeAgentsEndpoint { season }.get().await.unwrap();
		}
	}
}
