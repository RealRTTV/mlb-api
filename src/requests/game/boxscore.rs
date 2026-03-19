//! Various live information & collection stats about the ongoing game.
//!
//! Teams have pitching, hitting, and fielding stats, rosters, batting orders, etc.
//!
//! Lists of umpires, top performers, etc.

use std::fmt::Display;

use bon::Builder;
use fxhash::FxHashMap;
use serde::Deserialize;
use serde_with::{serde_as, DefaultOnError};

use crate::{Copyright, HomeAwaySplit, game::{GameId, LabelledValue, PlayerGameStatusFlags, SectionedLabelledValues}, meta::NamedPosition, person::{Ballplayer, JerseyNumber, NamedPerson, PersonId}, request::RequestURL, stats::{StatTypeStats, stat_types::__BoxscoreStatTypeStats}, team::{NamedTeam, Team, TeamId, roster::RosterStatus}};

/// See [`self`]
#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Boxscore {
    #[serde(default)]
    pub copyright: Copyright,
    #[serde(rename = "info")]
    pub misc: Vec<LabelledValue>,
    pub top_performers: Option<[TopPerformer; 3]>,
    pub pitching_notes: Vec<String>,
    pub teams: HomeAwaySplit<TeamWithGameData>,
}

/// One of three "top performers" of the game, measured by game score.
#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(tag = "type")]
pub enum TopPerformer {
    #[serde(rename = "hitter", rename_all = "camelCase")]
    Hitter {
        player: PlayerWithGameData,
        game_score: usize,
    },
    #[serde(rename = "starter", rename_all = "camelCase")]
    Starter {
        player: PlayerWithGameData,
        game_score: usize,
    },
}

/// A person with some potentially useful information regarding their performance in the current game.
#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PlayerWithGameData {
	pub person: NamedPerson,
	pub jersey_number: JerseyNumber,
	pub position: NamedPosition,
	pub status: RosterStatus,
	#[serde(rename = "parentTeamId")]
	pub parent_team: TeamId,
	pub stats: BoxscoreStatCollection,
	/// Uses the active game's [`GameType`], not the regular season stats.
	pub season_stats: BoxscoreStatCollection,
	pub game_status: PlayerGameStatusFlags,
	#[serde(default)]
	pub all_positions: Vec<NamedPosition>,
}

/// A team with some potentially useful information regarding their performance in the current game.
#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TeamWithGameData {
    pub team: NamedTeam,
    pub team_stats: BoxscoreStatCollection,
    #[serde(deserialize_with = "super::deserialize_players_cache")]
    pub players: FxHashMap<PersonId, PlayerWithGameData>,
    pub batters: Vec<PersonId>,
    pub pitchers: Vec<PersonId>,
    pub bench: Vec<PersonId>,
    pub bullpen: Vec<PersonId>,
    pub batting_order: [PersonId; 9],
    #[serde(rename = "info")]
    pub sectioned_labelled_values: Vec<SectionedLabelledValues>,
    #[serde(rename = "note")]
    pub notes: Vec<LabelledValue>,
}

/// Hitting, Pitching, and Fielding stats.
#[allow(private_interfaces, reason = "the underlying type is pub")]
#[serde_as]
#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BoxscoreStatCollection {
    #[serde(rename = "batting")]
    #[serde_as(deserialize_as = "DefaultOnError")]
    pub hitting: <__BoxscoreStatTypeStats as StatTypeStats>::Hitting,
    #[serde_as(deserialize_as = "DefaultOnError")]
    pub fielding: <__BoxscoreStatTypeStats as StatTypeStats>::Fielding,
    #[serde_as(deserialize_as = "DefaultOnError")]
    pub pitching: <__BoxscoreStatTypeStats as StatTypeStats>::Pitching,
}

#[derive(Builder)]
#[builder(derive(Into))]
pub struct BoxscoreRequest {
    #[builder(into)]
    id: GameId,
}

impl<S: boxscore_request_builder::State + boxscore_request_builder::IsComplete> crate::request::RequestURLBuilderExt for BoxscoreRequestBuilder<S> {
    type Built = BoxscoreRequest;
}

impl Display for BoxscoreRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "http://statsapi.mlb.com/api/v1/game/{}/boxscore", self.id)
    }
}

impl RequestURL for BoxscoreRequest {
    type Response = Boxscore;
}

#[cfg(test)]
mod tests {
    use crate::game::BoxscoreRequest;
    use crate::request::RequestURLBuilderExt;
    use crate::schedule::ScheduleRequest;
    use crate::season::{Season, SeasonsRequest};
    use crate::sport::SportId;

    #[tokio::test]
    async fn ws_gm7_2025_boxscore() {
        let _ = BoxscoreRequest::builder().id(813_024).build_and_get().await.unwrap();
    }

    #[tokio::test]
	async fn postseason_2025_boxscore() {
		let [season]: [Season; 1] = SeasonsRequest::builder().season(2025).sport_id(SportId::MLB).build_and_get().await.unwrap().seasons.try_into().unwrap();
		let postseason = season.postseason.expect("Expected the MLB to have a postseason");
		let games = ScheduleRequest::<()>::builder().date_range(postseason).sport_id(SportId::MLB).build_and_get().await.unwrap();
		let games = games.dates.into_iter().flat_map(|date| date.games).filter(|game| game.game_type.is_postseason()).map(|game| game.game_id).collect::<Vec<_>>();
		for game in games {
			if let Err(e) = BoxscoreRequest::builder().id(game).build_and_get().await {
			    dbg!(e);
			}
		}
	}
}
