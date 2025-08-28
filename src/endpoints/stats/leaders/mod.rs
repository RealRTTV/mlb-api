use crate::endpoints::league::League;
use crate::endpoints::person::Person;
use crate::endpoints::sports::{Sport, SportId};
use crate::endpoints::teams::team::Team;
use crate::endpoints::{BaseballStat, BaseballStatId, GameType, IdentifiableBaseballStat, StatGroup, StatType, StatsAPIUrl};
use crate::gen_params;
use crate::types::{Copyright, IntegerOrFloatStat, PlayerPool};
use chrono::NaiveDate;
use itertools::Itertools;
use serde::Deserialize;
use serde_with::serde_as;
use serde_with::DisplayFromStr;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StatLeadersResponse {
	pub copyright: Copyright,
	pub league_leaders: Vec<StatLeaders>,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(try_from = "__StatLeadersStruct")]
pub struct StatLeaders {
	pub category: BaseballStat,
	pub game_type: GameType,
	pub leaders: Vec<StatLeader>,
	pub stat_group: StatGroup,
	pub total_splits: u32,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct __StatLeadersStruct {
	leader_category: String,
	game_type: GameType,
	#[serde(default)]
	leaders: Vec<StatLeader>,
	stat_group: String,
	total_splits: u32,
}

impl TryFrom<__StatLeadersStruct> for StatLeaders {
	type Error = <StatGroup as FromStr>::Err;

	fn try_from(value: __StatLeadersStruct) -> Result<Self, Self::Error> {
		Ok(StatLeaders {
			category: BaseballStat::Identifiable(IdentifiableBaseballStat {
				id: BaseballStatId::new(value.leader_category),
			}),
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
pub struct StatLeader {
	pub rank: u32,
	pub value: IntegerOrFloatStat,
	#[serde(default = "Team::unknown_team")]
	pub team: Team,
	pub num_teams: u32,
	#[serde(default = "League::unknown_league")]
	pub league: League,
	pub person: Person,
	pub sport: Sport,
	#[serde_as(as = "DisplayFromStr")]
	pub season: u16,
}

pub struct StatLeadersEndpointUrl {
	pub stats: Vec<BaseballStat>,
	pub stat_groups: Vec<StatGroup>,
	pub season: Option<u16>,
	pub sport_id: SportId,
	pub stat_types: Vec<StatType>,
	pub start_date: Option<NaiveDate>,
	pub end_date: Option<NaiveDate>,
	pub pool: PlayerPool,

	/// Number of days to go back for data (starting from yesterday)
	pub days_back: Option<u32>,

	/// Limit on how many leaders to show per stat.
	/// Default is 5.
	pub limit: Option<u16>,
	/// Offset into results.
	pub offset: Option<u16>,

	/// [`None`] represents all game types.
	pub game_types: Option<Vec<GameType>>,
}

impl Display for StatLeadersEndpointUrl {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "http://statsapi.mlb.com/api/v1/stats/leaders{params}", params = gen_params! {
			"leaderCategories": self.stats.iter().map(|stat| &stat.id).join(","),
			"statGroup": self.stat_groups.iter().join(","),
			"season"?: self.season,
			"sportId": self.sport_id,
			"stats": self.stat_types.iter().join(","),
			"startDate"?: self.start_date,
			"endDate"?: self.end_date,
			"playerPool": self.pool,
			"daysBack"?: self.days_back,
			"limit"?: self.limit,
			"offset"?: self.offset,
			"gameTypes"?: self.game_types.as_ref().map(|x| x.iter().join(",")),
		})
	}
}

impl StatsAPIUrl for StatLeadersEndpointUrl {
	type Response = StatLeadersResponse;
}

#[cfg(test)]
mod tests {
	use crate::endpoints::meta::MetaEndpointUrl;
	use crate::endpoints::sports::SportId;
	use crate::endpoints::stats::leaders::StatLeadersEndpointUrl;
	use crate::endpoints::{BaseballStat, GameType, StatsAPIUrl};
	use crate::types::PlayerPool;

	#[tokio::test]
	async fn test_stat_leaders() {
		let all_stats = MetaEndpointUrl::<BaseballStat>::new().get().await.unwrap().entries;
		let all_game_types = MetaEndpointUrl::<GameType>::new().get().await.unwrap().entries;

		let request = StatLeadersEndpointUrl {
			stats: all_stats,
			stat_groups: vec![],
			season: None,
			sport_id: SportId::MLB,
			stat_types: vec![],
			start_date: None,
			end_date: None,
			pool: PlayerPool::All,
			days_back: None,
			limit: Some(100),
			offset: None,
			game_types: Some(all_game_types),
		};
		let response_str = reqwest::get(request.to_string()).await.unwrap().text().await.unwrap();
		let mut de = serde_json::Deserializer::from_str(&response_str);
		let result: Result<<StatLeadersEndpointUrl as StatsAPIUrl>::Response, serde_path_to_error::Error<_>> = serde_path_to_error::deserialize(&mut de);
		match result {
			Ok(_) => {}
			Err(e) if format!("{:?}", e.inner()).contains("missing field `copyright`") => {}
			Err(e) => panic!("Err: {:?}", e),
		}
	}
}
