//! Gets the stats of a player for a single game.

use std::borrow::Cow;
use crate::game::GameId;
use crate::person::PersonId;
use crate::Copyright;
use bon::Builder;
use serde::Deserialize;
use std::fmt::{Display, Formatter};
use serde::de::{Deserializer, Error};
use crate::__stats__request_data;
use crate::hydrations::Hydrations;
use crate::stats::PlayStat;
use crate::request::RequestURL;
use crate::meta::StatGroup;
use crate::stats::parse::{__ParsedStats, make_stat_split};
use crate::stats::raw::{fielding, hitting, pitching};
use crate::stats::wrappers::{AccumulatedVsPlayerMatchup, WithNone};

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
pub struct PersonSingleGameStatsResponse {
	pub copyright: Copyright,
	#[serde(flatten)]
	pub stats: SingleGameStats,
}

#[derive(Builder)]
#[builder(derive(Into))]
pub struct PersonSingleGameStatsRequest {
	#[builder(into)]
	person_id: PersonId,
	#[builder(into)]
	game_id: GameId,
	#[builder(into)]
	#[builder(default)]
	bonus: SingleGameStatsRequestData,
}

impl<S: person_single_game_stats_request_builder::State + person_single_game_stats_request_builder::IsComplete> crate::request::RequestURLBuilderExt for PersonSingleGameStatsRequestBuilder<S> {
	type Built = PersonSingleGameStatsRequest;
}

impl Display for PersonSingleGameStatsRequest {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "http://statsapi.mlb.com/api/v1/people/{}/stats/game/{}?{}", self.person_id, self.game_id, self.bonus)
	}
}

impl RequestURL for PersonSingleGameStatsRequest {
	type Response = PersonSingleGameStatsResponse;
}

#[cfg(test)]
mod tests {
	use crate::person::stats::PersonSingleGameStatsRequest;
	use crate::request::RequestURLBuilderExt;

	#[tokio::test]
	async fn single_sample() {
		let _ = PersonSingleGameStatsRequest::builder()
			.person_id(660_271)
			.game_id(776_562)
			.build_and_get()
			.await
			.unwrap();
	}
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SingleGameStats {
	pub game_log: SingleGameStatsSimplifiedGameLogSplit,
	pub vs_player5_y: SingleGameStatsVsPlayer5YSplit,
	pub play_log: SingleGameStatsSimplifiedPlayLogSplit,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SingleGameStatsSimplifiedGameLogSplit {
	pub hitting: Box<WithNone<hitting::__SimplifiedGameLogStatsData>>,
	pub pitching: Box<WithNone<pitching::__SimplifiedGameLogStatsData>>,
	pub fielding: Box<WithNone<fielding::__SimplifiedGameLogStatsData>>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SingleGameStatsVsPlayer5YSplit {
	pub hitting: Box<AccumulatedVsPlayerMatchup<hitting::__VsPlayerStatsData>>,
	pub pitching: Box<AccumulatedVsPlayerMatchup<pitching::__VsPlayerStatsData>>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SingleGameStatsSimplifiedPlayLogSplit {
	pub hitting: Box<Vec<WithNone<PlayStat>>>,
}

impl<'de> Deserialize<'de> for SingleGameStats {
	fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error>
	where
		Self: Sized,
	{
		let mut parsed_stats: __ParsedStats = <__ParsedStats as Deserialize>::deserialize(deserializer)?;

		Ok(Self {
			game_log: SingleGameStatsSimplifiedGameLogSplit {
				hitting: Box::new(
					make_stat_split::<WithNone<hitting::__SimplifiedGameLogStatsData>>(
						&mut parsed_stats, "gameLog", StatGroup::Hitting,
					).map_err(D::Error::custom)?
				),
				pitching: Box::new(
					make_stat_split::<WithNone<pitching::__SimplifiedGameLogStatsData>>(
						&mut parsed_stats, "gameLog", StatGroup::Pitching,
					).map_err(D::Error::custom)?
				),
				fielding: Box::new(
					make_stat_split::<WithNone<fielding::__SimplifiedGameLogStatsData>>(
						&mut parsed_stats, "gameLog", StatGroup::Fielding,
					).map_err(D::Error::custom)?
				),
			},
			vs_player5_y: SingleGameStatsVsPlayer5YSplit {
				hitting: Box::new(
					make_stat_split::<AccumulatedVsPlayerMatchup<hitting::__VsPlayerStatsData>>(
						&mut parsed_stats, "vsPlayer5Y", StatGroup::Hitting,
					).map_err(D::Error::custom)?
				),
				pitching: Box::new(
					make_stat_split::<AccumulatedVsPlayerMatchup<pitching::__VsPlayerStatsData>>(
						&mut parsed_stats, "vsPlayer5Y", StatGroup::Pitching,
					).map_err(D::Error::custom)?
				),
			},
			play_log: SingleGameStatsSimplifiedPlayLogSplit {
				hitting: Box::new(
					make_stat_split::<Vec<WithNone<PlayStat>>>(
						&mut parsed_stats, "playLog", StatGroup::Hitting,
					).map_err(D::Error::custom)?
				),
			},
		})
	}
}

__stats__request_data!(pub SingleGameStats [Season]);

impl Hydrations for SingleGameStats {
	type RequestData = SingleGameStatsRequestData;

	fn hydration_text(_: &Self::RequestData) -> Cow<'static, str> {
		panic!("Hydrations::hydration_text() called on SingleGameStats. Must use `PersonSingleGameStatsRequest` instead.")
	}
}
