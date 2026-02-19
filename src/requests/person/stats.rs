use std::borrow::Cow;
use crate::game::GameId;
use crate::person::PersonId;
use crate::Copyright;
use bon::Builder;
use serde::Deserialize;
use std::fmt::{Display, Formatter};
use serde::de::{Deserializer, Error};
use crate::__stats__request_data;
use crate::hydrations::{HydrationText, Hydrations};
use crate::stats::{StatTypeStats, PlayStat};
use crate::request::RequestURL;
use crate::meta::StatGroup;
use crate::stats::parse::{__ParsedStats, make_stat_split};
use crate::stats::raw::{fielding, hitting, pitching};
use crate::stats::stat_types::__VsPlayer5YStatTypeStats;
use crate::stats::wrappers::WithNone;

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
	pub hitting: Box<<GameLogStats as StatTypeStats>::Hitting>,
	pub pitching: Box<<GameLogStats as StatTypeStats>::Pitching>,
	pub fielding: Box<<GameLogStats as StatTypeStats>::Fielding>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SingleGameStatsVsPlayer5YSplit {
	pub hitting: Box<<__VsPlayer5YStatTypeStats as StatTypeStats>::Hitting>,
	pub pitching: Box<<__VsPlayer5YStatTypeStats as StatTypeStats>::Pitching>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SingleGameStatsSimplifiedPlayLogSplit {
	pub hitting: Box<<PlayLogStats as StatTypeStats>::Hitting>,
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
					make_stat_split::<<__VsPlayer5YStatTypeStats as StatTypeStats>::Hitting>(
						&mut parsed_stats, "vsPlayer5Y", StatGroup::Hitting,
					).map_err(D::Error::custom)?
				),
				pitching: Box::new(
					make_stat_split::<<__VsPlayer5YStatTypeStats as StatTypeStats>::Pitching>(
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
	type Hitting = WithNone<hitting::__SimplifiedGameLogStatsData>;
	type Pitching = WithNone<pitching::__SimplifiedGameLogStatsData>;
	type Fielding = WithNone<fielding::__SimplifiedGameLogStatsData>;
	type Catching = ();
}

pub struct PlayLogStats;

impl StatTypeStats for PlayLogStats {
	type Hitting = Vec<WithNone<PlayStat>>;
	type Pitching = Vec<WithNone<PlayStat>>;
	type Fielding = Vec<WithNone<PlayStat>>;
	type Catching = ();
}

impl Hydrations for SingleGameStats {}

__stats__request_data!(pub SingleGameStats [Season]);

impl HydrationText for SingleGameStats {
	type RequestData = SingleGameStatsRequestData;

	fn hydration_text(_: &Self::RequestData) -> Cow<'static, str> {
		panic!("HydrationText::hydration_text() called on SingleGameStats. Must use `PersonSingleGameStatsRequest` instead.")
	}
}
