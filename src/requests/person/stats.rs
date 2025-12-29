use std::borrow::Cow;
use crate::game::GameId;
use crate::person::PersonId;
use crate::types::Copyright;
use bon::Builder;
use serde::Deserialize;
use std::fmt::{Display, Formatter};
use serde::de::{Deserializer, Error};
use crate::hydrations::{HydrationText, Hydrations};
use crate::stats::{PossiblyFallback, StatTypeStats, __ParsedStats, make_stat_split, SimplifiedGameLogStats, SimplifiedPlayLogStats, VsPlayer5YStats};
use crate::request::StatsAPIRequestUrl;
use crate::stat_groups::StatGroup;

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
}

impl<S: person_single_game_stats_request_builder::State + person_single_game_stats_request_builder::IsComplete> crate::request::StatsAPIRequestUrlBuilderExt for PersonSingleGameStatsRequestBuilder<S> {
	type Built = PersonSingleGameStatsRequest;
}

impl Display for PersonSingleGameStatsRequest {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "http://statsapi.mlb.com/api/v1/people/{}/stats/game/{}", self.person_id, self.game_id)
	}
}

impl StatsAPIRequestUrl for PersonSingleGameStatsRequest {
	type Response = PersonSingleGameStatsResponse;
}

#[cfg(test)]
mod tests {
	use crate::person::stats::PersonSingleGameStatsRequest;
	use crate::request::StatsAPIRequestUrlBuilderExt;

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
	pub simplified_game_log: SingleGameStatsSimplifiedGameLogSplit,
	pub vs_player5_y: SingleGameStatsVsPlayer5YSplit,
	pub simplified_play_log: SingleGameStatsSimplifiedPlayLogSplit,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SingleGameStatsSimplifiedGameLogSplit {
	pub hitting: Box<PossiblyFallback<<SimplifiedGameLogStats as StatTypeStats>::Hitting>>,
	pub pitching: Box<PossiblyFallback<<SimplifiedGameLogStats as StatTypeStats>::Pitching>>,
	pub fielding: Box<PossiblyFallback<<SimplifiedGameLogStats as StatTypeStats>::Fielding>>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SingleGameStatsVsPlayer5YSplit {
	pub hitting: Box<PossiblyFallback<<VsPlayer5YStats as StatTypeStats>::Hitting>>,
	pub pitching: Box<PossiblyFallback<<VsPlayer5YStats as StatTypeStats>::Pitching>>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SingleGameStatsSimplifiedPlayLogSplit {
	pub hitting: Box<PossiblyFallback<<SimplifiedPlayLogStats as StatTypeStats>::Hitting>>,
}

impl<'de> Deserialize<'de> for SingleGameStats {
	fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error>
	where
		Self: Sized,
	{
		let mut parsed_stats: __ParsedStats = <__ParsedStats as Deserialize>::deserialize(deserializer)?;

		Ok(Self {
			simplified_game_log: SingleGameStatsSimplifiedGameLogSplit {
				hitting: Box::new(
					make_stat_split::<<SimplifiedGameLogStats as StatTypeStats>::Hitting>(
						&mut parsed_stats, "gameLog", StatGroup::Hitting,
					).map_err(D::Error::custom)?
				),
				pitching: Box::new(
					make_stat_split::<<SimplifiedGameLogStats as StatTypeStats>::Pitching>(
						&mut parsed_stats, "gameLog", StatGroup::Pitching,
					).map_err(D::Error::custom)?
				),
				fielding: Box::new(
					make_stat_split::<<SimplifiedGameLogStats as StatTypeStats>::Fielding>(
						&mut parsed_stats, "gameLog", StatGroup::Fielding,
					).map_err(D::Error::custom)?
				),
			},
			vs_player5_y: SingleGameStatsVsPlayer5YSplit {
				hitting: Box::new(
					make_stat_split::<<VsPlayer5YStats as StatTypeStats>::Hitting>(
						&mut parsed_stats, "vsPlayer5Y", StatGroup::Hitting,
					).map_err(D::Error::custom)?
				),
				pitching: Box::new(
					make_stat_split::<<VsPlayer5YStats as StatTypeStats>::Pitching>(
						&mut parsed_stats, "vsPlayer5Y", StatGroup::Pitching,
					).map_err(D::Error::custom)?
				),
			},
			simplified_play_log: SingleGameStatsSimplifiedPlayLogSplit {
				hitting: Box::new(
					make_stat_split::<<SimplifiedPlayLogStats as StatTypeStats>::Hitting>(
						&mut parsed_stats, "playLog", StatGroup::Hitting,
					).map_err(D::Error::custom)?
				),
			},
		})
	}
}

impl Hydrations for SingleGameStats {}

impl HydrationText for SingleGameStats {
	fn hydration_text() -> Cow<'static, str> {
		// actually works lol
		Cow::Borrowed("ERROR")
	}
}
