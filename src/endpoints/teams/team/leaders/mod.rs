use crate::stats::leaders::StatLeaders;
use crate::teams::team::TeamId;
use crate::{BaseballStatId, GameType, StatsAPIEndpointUrl};
use crate::gen_params;
use crate::types::{Copyright, PlayerPool};
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
pub struct TeamStatLeadersEndpoint {
	pub team_id: TeamId,
	pub stats: Vec<BaseballStatId>,
	pub season: Option<u16>,
	pub pool: PlayerPool,

	/// [`None`] represents matching for all [`GameType`]s.
	pub game_types: Option<Vec<GameType>>,
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
	use crate::sports::SportId;
	use crate::teams::team::leaders::TeamStatLeadersEndpoint;
	use crate::teams::TeamsEndpoint;
	use crate::{BaseballStat, StatsAPIEndpointUrl};

	#[tokio::test]
	async fn test_all_mlb_teams_all_stats() {
		let all_stats = MetaEndpoint::<BaseballStat>::new().get().await.unwrap().entries.into_iter().map(|stat| stat.id.clone()).collect::<Vec<_>>();

		for team in (TeamsEndpoint { sport_id: Some(SportId::MLB), season: None }).get().await.unwrap().teams {
			let _all_stats = TeamStatLeadersEndpoint {
				team_id: team.id,
				stats: all_stats.clone(),
				season: None,
				pool: Default::default(),
				game_types: None,
			}
			.get()
			.await
			.expect(&format!("expected team #{} to be valid", team.id));
		}
	}
}
