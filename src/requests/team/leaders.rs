use crate::season::SeasonId;
use crate::stats::leaders::StatLeaders;
use crate::team::TeamId;
use crate::types::{Copyright, PlayerPool};
use bon::Builder;
use itertools::Itertools;
use serde::Deserialize;
use std::fmt::{Display, Formatter};
use crate::baseball_stats::BaseballStatId;
use crate::game_types::GameType;
use crate::request::RequestURL;

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TeamStatLeadersResponse {
	pub copyright: Copyright,
	pub team_leaders: Vec<StatLeaders>,
}

/// Stat leaders per team
#[derive(Builder)]
#[builder(derive(Into))]
pub struct TeamStatLeadersRequest {
	#[builder(into)]
	pub team_id: TeamId,
	pub stats: Vec<BaseballStatId>,
	#[builder(into)]
	pub season: Option<SeasonId>,
	#[builder(default)]
	pub pool: PlayerPool,

	/// [`None`] represents matching for all [`GameType`]s.
	pub game_types: Option<Vec<GameType>>,
}

impl<S: team_stat_leaders_request_builder::State + team_stat_leaders_request_builder::IsComplete> crate::request::RequestURLBuilderExt for TeamStatLeadersRequestBuilder<S> {
	type Built = TeamStatLeadersRequest;
}

impl Display for TeamStatLeadersRequest {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"http://statsapi.mlb.com/api/v1/teams/{}/leaders{params}",
			self.team_id,
			params = gen_params! {
				"leaderCategories": self.stats.iter().join(","),
				"season"?: self.season,
				"pool": self.pool,
				"game_types"?: self.game_types.as_ref().map(|x| x.iter().join(",")),
			}
		)
	}
}

impl RequestURL for TeamStatLeadersRequest {
	type Response = TeamStatLeadersResponse;
}

#[cfg(test)]
mod tests {
	use crate::baseball_stats::BaseballStat;
	use crate::meta::MetaRequest;
	use crate::request::{RequestURL, RequestURLBuilderExt};
	use crate::team::leaders::TeamStatLeadersRequest;
	use crate::team::teams::TeamsRequest;

	#[tokio::test]
	async fn test_all_mlb_teams_all_stats() {
		let all_stats = MetaRequest::<BaseballStat>::new().get().await.unwrap().entries.into_iter().map(|stat| stat.id.clone()).collect::<Vec<_>>();

		for team in TeamsRequest::mlb_teams().build_and_get().await.unwrap().teams {
			let request = TeamStatLeadersRequest::builder().team_id(team.id).stats(all_stats.clone()).build();
			let _all_stats = request.get()
			.await
			.unwrap_or_else(|_| panic!("expected team #{} to be valid; {request}", team.id));
		}
	}
}
