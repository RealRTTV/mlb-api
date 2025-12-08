use crate::endpoints::sports::SportId;
use crate::endpoints::teams::team::TeamId;
use crate::endpoints::{GameType, StatGroup, StatType, StatsAPIEndpointUrl};
use crate::gen_params;
use crate::types::{Copyright, MLB_API_DATE_FORMAT};
use chrono::NaiveDate;
use itertools::Itertools;
use std::fmt::{Display, Formatter};
use std::marker::PhantomData;
use serde::Deserialize;
use crate::endpoints::person::PersonId;
use crate::endpoints::stats::Stats;

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(bound = "S: Stats")]
pub struct TeamsStatsResponse<S: Stats> {
	pub copyright: Copyright,
	pub stats: S,
}

pub struct TeamsStatsEndpoint<S: Stats> {
	/// Choice between either a [`TeamId`] or a [`SportId`]
	pub id: Result<TeamId, SportId>,
	pub season: Option<u16>,
	pub game_type: GameType,
	pub stat_types: Vec<StatType>,
	pub stat_groups: Vec<StatGroup>,
	pub start_date: Option<NaiveDate>,
	pub end_date: Option<NaiveDate>,
	pub opposing_player: Option<PersonId>,
	pub opposing_team: Option<PersonId>,
	_marker: PhantomData<S>,
}

impl<S: Stats> Display for TeamsStatsEndpoint<S> {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self.id {
			Ok(id) => write!(
				f,
				"http://statsapi.mlb.com/api/v1/teams/{id}/stats{params}",
				params = gen_params! {
					"season"?: self.season,
					"gameType": self.game_type,
					"stats": self.stat_types.iter().join(","),
					"group": self.stat_groups.iter().join(","),
					"startDate"?: self.start_date.map(|x| x.format(MLB_API_DATE_FORMAT)),
					"endDate"?: self.start_date.map(|x| x.format(MLB_API_DATE_FORMAT)),
					"opposingPlayerId"?: self.opposing_player,
					"opposingTeamId"?: self.opposing_team,
				}
			),
			Err(id) => write!(
				f,
				"http://statsapi.mlb.com/api/v1/teams/stats{params}",
				params = gen_params! {
					"season"?: self.season,
					"gameType": self.game_type,
					"stats": self.stat_types.iter().join(","),
					"group": self.stat_groups.iter().join(","),
					"sportId": id,
					"startDate"?: self.start_date.map(|x| x.format(MLB_API_DATE_FORMAT)),
					"endDate"?: self.start_date.map(|x| x.format(MLB_API_DATE_FORMAT)),
					"opposingPlayerId"?: self.opposing_player,
					"opposingTeamId"?: self.opposing_team,
				}
			),
		}
	}
}

impl<S: Stats> StatsAPIEndpointUrl for TeamsStatsEndpoint<S> {
	type Response = TeamsStatsResponse<S>;
}
