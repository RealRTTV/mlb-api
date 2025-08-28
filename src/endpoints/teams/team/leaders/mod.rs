use crate::endpoints::stats::leaders::StatLeaders;
use crate::endpoints::teams::team::TeamId;
use crate::endpoints::{BaseballStat, GameType, StatsAPIUrl};
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
pub struct TeamStatLeadersEndpointUrl {
	pub team_id: TeamId,
	pub stats: Vec<BaseballStat>,
	pub season: Option<u16>,
	pub pool: PlayerPool,

	/// [`None`] represents matching for all [`GameType`]s.
	pub game_types: Option<Vec<GameType>>,
}

impl Display for TeamStatLeadersEndpointUrl {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"http://statsapi.mlb.com/api/v1/teams/{}/leaders{params}",
			self.team_id,
			params = gen_params! {
				"leaderCategories": self.stats.iter().map(|stat| &stat.id).join(","),
				"season"?: self.season,
				"pool": self.pool,
				"game_types"?: self.game_types.as_ref().map(|x| x.iter().join(",")),
			}
		)
	}
}

impl StatsAPIUrl for TeamStatLeadersEndpointUrl {
	type Response = TeamStatLeadersResponse;
}

#[cfg(test)]
mod tests {
	use crate::endpoints::meta::MetaEndpointUrl;
	use crate::endpoints::sports::SportId;
	use crate::endpoints::teams::team::leaders::TeamStatLeadersEndpointUrl;
	use crate::endpoints::teams::TeamsEndpointUrl;
	use crate::endpoints::{BaseballStat, StatsAPIUrl};

	#[tokio::test]
	async fn test_all_mlb_teams_all_stats() {
		let all_categories = MetaEndpointUrl::<BaseballStat>::new().get().await.unwrap().entries;

		for team in (TeamsEndpointUrl { sport_id: Some(SportId::MLB), season: None }).get().await.unwrap().teams {
			let _all_stats = TeamStatLeadersEndpointUrl {
				team_id: team.id,
				stats: all_categories.clone(),
				season: None,
				pool: Default::default(),
				game_types: None,
			}
			.get()
			.await
			.unwrap();
		}
	}
}
