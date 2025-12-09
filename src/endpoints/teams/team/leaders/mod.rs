use crate::gen_params;
use crate::seasons::season::SeasonId;
use crate::stats::leaders::StatLeaders;
use crate::teams::team::TeamId;
use crate::types::{Copyright, PlayerPool};
use crate::{BaseballStatId, GameType, StatsAPIEndpointUrl};
use bon::Builder;
use itertools::Itertools;
use serde::Deserialize;
use std::fmt::{Display, Formatter};

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TeamStatLeadersResponse {
	pub copyright: Copyright,
	pub team_leaders: Vec<StatLeaders>,
}

/// Stat leaders per team
#[derive(Builder)]
#[builder(derive(Into))]
pub struct TeamStatLeadersEndpoint {
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

impl<S: team_stat_leaders_endpoint_builder::State> crate::endpoints::links::StatsAPIEndpointUrlBuilderExt for TeamStatLeadersEndpointBuilder<S> where S: team_stat_leaders_endpoint_builder::IsComplete {
	type Built = TeamStatLeadersEndpoint;
}

impl Display for TeamStatLeadersEndpoint {
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

impl StatsAPIEndpointUrl for TeamStatLeadersEndpoint {
	type Response = TeamStatLeadersResponse;
}

#[cfg(test)]
mod tests {
	use crate::meta::MetaEndpoint;
	use crate::teams::team::leaders::TeamStatLeadersEndpoint;
	use crate::teams::TeamsEndpoint;
	use crate::{BaseballStat, StatsAPIEndpointUrl, StatsAPIEndpointUrlBuilderExt};

	#[tokio::test]
	async fn test_all_mlb_teams_all_stats() {
		let all_stats = MetaEndpoint::<BaseballStat>::new().get().await.unwrap().entries.into_iter().map(|stat| stat.id.clone()).collect::<Vec<_>>();

		for team in TeamsEndpoint::builder().build_and_get().await.unwrap().teams {
			let _all_stats = TeamStatLeadersEndpoint::builder().team_id(team.id).stats(all_stats.clone())
			.build_and_get()
			.await
			.expect(&format!("expected team #{} to be valid", team.id));
		}
	}
}
