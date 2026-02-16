#![allow(clippy::trait_duplication_in_bounds, reason = "serde")]

use crate::game_types::GameType;
use crate::person::PersonId;
use crate::request::RequestURL;
use crate::season::SeasonId;
use crate::sport::SportId;
use crate::stat_groups::StatGroup;
use crate::stat_types::StatType;
use crate::stats::Stats;
use crate::team::TeamId;
use crate::types::{Copyright, MLB_API_DATE_FORMAT};
use bon::Builder;
use chrono::NaiveDate;
use either::Either;
use itertools::Itertools;
use serde::Deserialize;
use std::fmt::{Display, Formatter};
use std::marker::PhantomData;

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(bound = "S: Stats")]
pub struct TeamsStatsResponse<S: Stats> {
	pub copyright: Copyright,
	pub stats: S,
}

#[derive(Builder)]
#[builder(derive(Into))]
#[builder(start_fn(vis = ""))]
pub struct TeamsStatsRequest<S: Stats> {
	#[builder(setters(vis = "", name = __id_internal))]
	pub id: Either<TeamId, SportId>,
	#[builder(into)]
	pub season: Option<SeasonId>,
	pub game_type: GameType,
	pub stat_types: Vec<StatType>,
	pub stat_groups: Vec<StatGroup>,
	pub start_date: Option<NaiveDate>,
	pub end_date: Option<NaiveDate>,
	#[builder(into)]
	pub opposing_player: Option<PersonId>,
	#[builder(into)]
	pub opposing_team: Option<PersonId>,
	#[builder(skip)]
	_marker: PhantomData<S>,
}

impl<S: Stats, State: teams_stats_request_builder::State + teams_stats_request_builder::IsComplete> crate::request::RequestURLBuilderExt for TeamsStatsRequestBuilder<S, State> {
    type Built = TeamsStatsRequest<S>;
}

impl<S: Stats> TeamsStatsRequest<S> {
	pub fn for_team(team_id: impl Into<TeamId>) -> TeamsStatsRequestBuilder<S, teams_stats_request_builder::SetId> {
		Self::builder().__id_internal(Either::Left(team_id.into()))
	}

	pub fn for_sport(sport_id: impl Into<SportId>) -> TeamsStatsRequestBuilder<S, teams_stats_request_builder::SetId> {
		Self::builder().__id_internal(Either::Right(sport_id.into()))
	}
}

impl<S: Stats> Display for TeamsStatsRequest<S> {
	#[allow(clippy::cognitive_complexity, reason = "still readable")]
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self.id {
			Either::Left(id) => write!(
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
			Either::Right(id) => write!(
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

impl<S: Stats> RequestURL for TeamsStatsRequest<S> {
	type Response = TeamsStatsResponse<S>;
}
