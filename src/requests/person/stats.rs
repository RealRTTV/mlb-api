use std::borrow::Cow;
use crate::game::GameId;
use crate::person::PersonId;
use crate::types::Copyright;
use bon::Builder;
use serde::Deserialize;
use std::fmt::{Display, Formatter};
use serde::de::{Deserializer, Error};
use crate::__stats__request_data;
use crate::hydrations::{HydrationText, Hydrations};
use crate::stats::{PossiblyFallback, StatTypeStats, __ParsedStats, make_stat_split, Multiple, PlayStat, VsPlayer5YStats};
use crate::request::StatsAPIRequestUrl;
use crate::stat_groups::StatGroup;
use crate::stats::catching::CatchingStats;
use crate::stats::fielding::SimplifiedGameLogFieldingStats;
use crate::stats::hitting::SimplifiedGameLogHittingStats;
use crate::stats::pitching::SimplifiedGameLogPitchingStats;

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

impl<S: person_single_game_stats_request_builder::State + person_single_game_stats_request_builder::IsComplete> crate::request::StatsAPIRequestUrlBuilderExt for PersonSingleGameStatsRequestBuilder<S> {
	type Built = PersonSingleGameStatsRequest;
}

impl Display for PersonSingleGameStatsRequest {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "http://statsapi.mlb.com/api/v1/people/{}/stats/game/{}?{}", self.person_id, self.game_id, self.bonus)
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
	pub game_log: SingleGameStatsSimplifiedGameLogSplit,
	pub vs_player5_y: SingleGameStatsVsPlayer5YSplit,
	pub play_log: SingleGameStatsSimplifiedPlayLogSplit,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SingleGameStatsSimplifiedGameLogSplit {
	pub hitting: Box<PossiblyFallback<<GameLogStats as StatTypeStats>::Hitting>>,
	pub pitching: Box<PossiblyFallback<<GameLogStats as StatTypeStats>::Pitching>>,
	pub fielding: Box<PossiblyFallback<<GameLogStats as StatTypeStats>::Fielding>>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SingleGameStatsVsPlayer5YSplit {
	pub hitting: Box<PossiblyFallback<<VsPlayer5YStats as StatTypeStats>::Hitting>>,
	pub pitching: Box<PossiblyFallback<<VsPlayer5YStats as StatTypeStats>::Pitching>>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SingleGameStatsSimplifiedPlayLogSplit {
	pub hitting: Box<PossiblyFallback<<PlayLogStats as StatTypeStats>::Hitting>>,
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
					make_stat_split::<<GameLogStats as StatTypeStats>::Hitting>(
						&mut parsed_stats, "gameLog", StatGroup::Hitting,
					).map_err(D::Error::custom)?
				),
				pitching: Box::new(
					make_stat_split::<<GameLogStats as StatTypeStats>::Pitching>(
						&mut parsed_stats, "gameLog", StatGroup::Pitching,
					).map_err(D::Error::custom)?
				),
				fielding: Box::new(
					make_stat_split::<<GameLogStats as StatTypeStats>::Fielding>(
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
			play_log: SingleGameStatsSimplifiedPlayLogSplit {
				hitting: Box::new(
					make_stat_split::<<PlayLogStats as StatTypeStats>::Hitting>(
						&mut parsed_stats, "playLog", StatGroup::Hitting,
					).map_err(D::Error::custom)?
				),
			},
		})
	}
}

pub struct GameLogStats;

impl StatTypeStats for GameLogStats {
	type Hitting = Option<SimplifiedGameLogHittingStats>;
	type Pitching = Option<SimplifiedGameLogPitchingStats>;
	type Fielding = Option<SimplifiedGameLogFieldingStats>;
	type Catching = Option<CatchingStats>;
}

pub struct PlayLogStats;

impl StatTypeStats for PlayLogStats {
	type Hitting = Multiple<PlayStat>;
	type Pitching = Multiple<PlayStat>;
	type Fielding = Multiple<PlayStat>;
	type Catching = Multiple<PlayStat>;
}

impl Hydrations for SingleGameStats {}

__stats__request_data!(pub SingleGameStats [Season]);

impl HydrationText for SingleGameStats {
	type RequestData = SingleGameStatsRequestData;

	fn hydration_text(_: &Self::RequestData) -> Cow<'static, str> {
		// actually works for us lol
		Cow::Borrowed("ERROR")
	}
}
