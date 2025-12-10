use std::fmt::{Display, Formatter};
use crate::requests::game::GameId;
use crate::requests::person::PersonId;
use crate::requests::StatsAPIRequestUrl;
use crate::types::Copyright;
use serde::Deserialize;

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

pub struct PersonSingleGameStatsRequest {
	pub person_id: PersonId,
	pub game_id: GameId,
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
	use crate::requests::game::GameId;
	use crate::requests::person::PersonId;
	use crate::requests::person::stats::PersonSingleGameStatsRequest;

	#[tokio::test]
	async fn single_sample() {
		let url = PersonSingleGameStatsRequest {
			person_id: PersonId::new(660271),
			game_id: GameId::new(776562),
		};
		println!("{url}");
		let _ = crate::serde_path_to_error_parse(url).await;
	}
}
