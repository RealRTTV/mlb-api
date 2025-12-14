use crate::requests::game::GameId;
use crate::requests::person::PersonId;
use crate::types::Copyright;
use bon::Builder;
use serde::Deserialize;
use std::fmt::{Display, Formatter};
use crate::request::StatsAPIRequestUrl;
use crate::stats;

stats! {
	pub struct SingleGameStats {
		[SimplifiedGameLog] = [Hitting, Pitching, Fielding],
		[VsPlayer5Y] = [Hitting, Pitching],
		[SimplifiedPlayLog] = [Hitting],
	}
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
pub struct PersonSingleGameStatsResponse {
	pub copyright: Copyright,
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
	use crate::requests::person::stats::PersonSingleGameStatsRequest;

	#[tokio::test]
	async fn single_sample() {
		let url = PersonSingleGameStatsRequest::builder()
			.person_id(660_271)
			.game_id(776_562)
			.build();
		let _ = crate::serde_path_to_error_parse(url).await;
	}
}
