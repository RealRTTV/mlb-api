use crate::gen_params;
use crate::league::League;
use crate::person::Person;
use crate::seasons::season::SeasonId;
use crate::sports::{Sport, SportId};
use crate::teams::team::Team;
use crate::types::{Copyright, IntegerOrFloatStat, PlayerPool, MLB_API_DATE_FORMAT};
use crate::{BaseballStat, BaseballStatId, GameType, IdentifiableBaseballStat, StatGroup, StatType, StatsAPIEndpointUrl};
use bon::Builder;
use chrono::NaiveDate;
use itertools::Itertools;
use serde::Deserialize;
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
#[doc(hidden)]
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

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StatLeader {
	pub rank: u32,
	pub value: IntegerOrFloatStat,
	#[serde(default = "Team::unknown_team")]
	pub team: Team,
	#[serde(default = "League::unknown_league")]
	pub league: League,
	pub person: Person,
	pub sport: Sport,
	pub season: SeasonId,
}

#[derive(Builder)]
#[builder(derive(Into))]
pub struct StatLeadersEndpoint {
	stats: Vec<BaseballStatId>,
	stat_group: Option<StatGroup>,
	#[builder(into)]
	season: Option<SeasonId>,
	#[builder(into, default)]
	sport_id: SportId,
	stat_types: Vec<StatType>,
	start_date: Option<NaiveDate>,
	end_date: Option<NaiveDate>,
	pool: PlayerPool,

	/// Number of days to go back for data (starting from yesterday)
	days_back: Option<u32>,

	/// Limit on how many leaders to show per stat.
	/// Default is 5.
	limit: Option<u16>,
	/// Offset into results.
	offset: Option<u16>,

	/// [`None`] represents all game types.
	game_types: Option<Vec<GameType>>,
}

impl<S: stat_leaders_endpoint_builder::State> crate::endpoints::links::StatsAPIEndpointUrlBuilderExt for StatLeadersEndpointBuilder<S>
where
	S: stat_leaders_endpoint_builder::IsComplete,
{
	type Built = StatLeadersEndpoint;
}

impl Display for StatLeadersEndpoint {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"http://statsapi.mlb.com/api/v1/stats/leaders{params}",
			params = gen_params! {
				"leaderCategories": self.stats.iter().join(","),
				"statGroup"?: self.stat_group,
				"season"?: self.season,
				"sportId": self.sport_id,
				"stats": self.stat_types.iter().join(","),
				"startDate"?: self.start_date.map(|x| x.format(MLB_API_DATE_FORMAT)),
				"endDate"?: self.end_date.map(|x| x.format(MLB_API_DATE_FORMAT)),
				"playerPool": self.pool,
				"daysBack"?: self.days_back,
				"limit"?: self.limit,
				"offset"?: self.offset,
				"gameTypes"?: self.game_types.as_ref().map(|x| x.iter().join(",")),
			}
		)
	}
}

impl StatsAPIEndpointUrl for StatLeadersEndpoint {
	type Response = StatLeadersResponse;
}

#[cfg(test)]
mod tests {
	use crate::meta::MetaEndpoint;
	use crate::stats::leaders::StatLeadersEndpoint;
	use crate::types::PlayerPool;
	use crate::{BaseballStat, GameType, StatsAPIEndpointUrl};

	#[tokio::test]
	async fn test_stat_leaders() {
		let all_stats = MetaEndpoint::<BaseballStat>::new().get().await.unwrap().entries.into_iter().map(|x| x.id.clone()).collect::<Vec<_>>();
		let all_game_types = MetaEndpoint::<GameType>::new().get().await.unwrap().entries;

		let _ = crate::serde_path_to_error_parse(StatLeadersEndpoint::builder().stats(all_stats).pool(PlayerPool::All).limit(100).game_types(all_game_types).stat_types(vec![]).build()).await;
	}
}
