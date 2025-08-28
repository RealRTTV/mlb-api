use crate::endpoints::league::League;
use crate::endpoints::person::Person;
use crate::endpoints::sports::Sport;
use crate::endpoints::teams::team::{Team, TeamId};
use crate::endpoints::{BaseballStat, BaseballStatId, GameType, IdentifiableBaseballStat, StatGroup, StatsAPIUrl};
use crate::gen_params;
use crate::types::{Copyright, IntegerOrFloat, PlayerPool};
use itertools::Itertools;
use serde::Deserialize;
use serde_with::DisplayFromStr;
use serde_with::serde_as;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TeamStatLeadersResponse {
	pub copyright: Copyright,
	pub team_leaders: Vec<TeamStatLeaders>,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(try_from = "__TeamStatLeadersStruct")]
pub struct TeamStatLeaders {
	pub category: BaseballStat,
	pub season: u16,
	pub game_type: GameType,
	pub leaders: Vec<TeamStatLeader>,
	pub stat_group: StatGroup,
	pub total_splits: u32,
}

#[serde_as]
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct __TeamStatLeadersStruct {
	leader_category: String,
	#[serde_as(as = "DisplayFromStr")]
	season: u16,
	game_type: GameType,
	leaders: Vec<TeamStatLeader>,
	stat_group: String,
	total_splits: u32,
}

impl TryFrom<__TeamStatLeadersStruct> for TeamStatLeaders {
	type Error = <StatGroup as FromStr>::Err;

	fn try_from(value: __TeamStatLeadersStruct) -> Result<Self, Self::Error> {
		Ok(TeamStatLeaders {
			category: BaseballStat::Identifiable(IdentifiableBaseballStat {
				id: BaseballStatId::new(value.leader_category),
			}),
			season: value.season,
			game_type: value.game_type,
			leaders: value.leaders,
			stat_group: StatGroup::from_str(&value.stat_group)?,
			total_splits: value.total_splits,
		})
	}
}

#[serde_as]
#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TeamStatLeader {
	pub rank: u32,
	pub value: IntegerOrFloat,
	pub team: Team,
	pub league: League,
	pub person: Person,
	pub sport: Sport,
	#[serde_as(as = "DisplayFromStr")]
	pub season: u16,
}

/// Represents the stat leaders per team
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
	use crate::endpoints::{BaseballStat, StatsAPIUrl};
	use crate::endpoints::meta::MetaEndpointUrl;
	use crate::endpoints::sports::SportId;
	use crate::endpoints::teams::TeamsEndpointUrl;
	use crate::endpoints::teams::team::leaders::TeamStatLeadersEndpointUrl;

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
