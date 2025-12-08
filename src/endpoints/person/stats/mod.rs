use std::fmt::{Display, Formatter};
use crate::endpoints::game::GameId;
use crate::endpoints::person::PersonId;
use crate::endpoints::StatsAPIEndpointUrl;
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

pub struct PersonSingleGameStatsEndpoint {
	pub person_id: PersonId,
	pub game_id: GameId,
}

impl Display for PersonSingleGameStatsEndpoint {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "http://statsapi.mlb.com/api/v1/people/{}/stats/game/{}", self.person_id, self.game_id)
	}
}

impl StatsAPIEndpointUrl for PersonSingleGameStatsEndpoint {
	type Response = PersonSingleGameStatsResponse;
}

#[cfg(test)]
mod tests {
	use crate::endpoints::game::GameId;
	use crate::endpoints::person::PersonId;
	use crate::endpoints::person::stats::PersonSingleGameStatsEndpoint;

	#[tokio::test]
	async fn single_sample() {
		let url = PersonSingleGameStatsEndpoint {
			person_id: PersonId::new(660271),
			game_id: GameId::new(776562),
		};
		println!("{url}");
		let _ = crate::serde_path_to_error_parse(url).await;
	}
}
