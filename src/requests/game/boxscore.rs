//! Various live information & collection stats about the ongoing game.
//!
//! Teams have pitching, hitting, and fielding stats, rosters, batting orders, etc.
//!
//! Lists of umpires, top performers, etc.

use std::fmt::Display;

use bon::Builder;
use fxhash::FxHashMap;
use serde::{Deserialize, de::IgnoredAny};
use serde_with::{serde_as, DefaultOnError};

use crate::{Copyright, HomeAway, game::{BattingOrderIndex, GameId, LabelledValue, Official, PlayerGameStatusFlags, SectionedLabelledValues}, meta::NamedPosition, person::{Ballplayer, JerseyNumber, NamedPerson, PersonId}, request::RequestURL, stats::{StatTypeStats, stat_types::__BoxscoreStatTypeStats}, team::{NamedTeam, Team, TeamId, roster::RosterStatus}};

/// See [`self`]
#[serde_as]
#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "_debug", serde(deny_unknown_fields))]
pub struct Boxscore {
    #[serde(default)]
    pub copyright: Copyright,
    #[serde(rename = "info")]
    pub misc: Vec<LabelledValue>,
    #[serde_as(deserialize_as = "DefaultOnError")]
    pub top_performers: Option<[TopPerformer; 3]>,
    pub pitching_notes: Vec<String>,
    pub teams: HomeAway<TeamWithGameData>,
    pub officials: Vec<Official>,
}

impl Boxscore {
    /// Returns a [`PlayerWithGameData`] if present in the baseball game.
    #[must_use]
    pub fn find_player_with_game_data(&self, id: PersonId) -> Option<&PlayerWithGameData> {
        self.teams.home.players.get(&id).or_else(|| self.teams.away.players.get(&id))
    }
}

/// One of three "top performers" of the game, measured by game score.
///
/// Originally an enum but the amount of two-way-players that exist make it pointlessly annoying and easy to break.
#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "_debug", serde(deny_unknown_fields))]
pub struct TopPerformer {
    pub player: PlayerWithGameData,
    pub game_score: usize,
    #[serde(rename = "type")]
    pub player_kind: String,

    #[doc(hidden)]
    #[serde(rename = "pitchingGameScore", default)]
    pub __pitching_game_score: IgnoredAny,
    #[doc(hidden)]
    #[serde(rename = "hittingGameScore", default)]
    pub __hitting_game_score: IgnoredAny,
}

/// A person with some potentially useful information regarding their performance in the current game.
#[serde_as]
#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "_debug", serde(deny_unknown_fields))]
pub struct PlayerWithGameData {
	pub person: NamedPerson,
	#[serde(default)]
	#[serde_as(deserialize_as = "DefaultOnError")]
	pub jersey_number: Option<JerseyNumber>,
	pub position: NamedPosition,
	pub status: RosterStatus,
	#[serde(rename = "stats")]
	pub game_stats: BoxscoreStatCollection,
	pub season_stats: BoxscoreStatCollection,
	pub game_status: PlayerGameStatusFlags,
	#[serde(default)]
	pub all_positions: Vec<NamedPosition>,
	pub batting_order: Option<BattingOrderIndex>,

    #[doc(hidden)]
    #[serde(rename = "parentTeamId", default)]
	pub __parent_team_id: IgnoredAny,

	// only sometimes present
	#[doc(hidden)]
	#[serde(rename = "boxscoreName", default)]
	pub __boxscore_name: IgnoredAny,
}

/// A team with some potentially useful information regarding their performance in the current game.
#[serde_as]
#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "_debug", serde(deny_unknown_fields))]
pub struct TeamWithGameData {
    pub team: NamedTeam,
    pub team_stats: BoxscoreStatCollection,
    #[serde(deserialize_with = "super::deserialize_players_cache")]
    pub players: FxHashMap<PersonId, PlayerWithGameData>,
    pub batters: Vec<PersonId>,
    pub pitchers: Vec<PersonId>,
    pub bench: Vec<PersonId>,
    pub bullpen: Vec<PersonId>,
    #[serde_as(deserialize_as = "DefaultOnError")]
    pub batting_order: Option<[PersonId; 9]>,
    #[serde(rename = "info")]
    pub sectioned_labelled_values: Vec<SectionedLabelledValues>,
    #[serde(rename = "note")]
    pub notes: Vec<LabelledValue>,
}

/// Hitting, Pitching, and Fielding stats.
#[allow(private_interfaces, reason = "the underlying type is pub")]
#[serde_as]
#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "_debug", serde(deny_unknown_fields))]
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
    use crate::TEST_YEAR;
    use crate::game::BoxscoreRequest;
    use crate::meta::GameType;
    use crate::request::RequestURLBuilderExt;
    use crate::schedule::ScheduleRequest;
    use crate::season::{Season, SeasonsRequest};
    use crate::sport::SportId;

    #[tokio::test]
    async fn ws_gm7_2025_boxscore() {
        let _ = BoxscoreRequest::builder().id(813_024).build_and_get().await.unwrap();
    }

    #[tokio::test]
	async fn postseason_boxscore() {
		let [season]: [Season; 1] = SeasonsRequest::builder().season(TEST_YEAR).sport_id(SportId::MLB).build_and_get().await.unwrap().seasons.try_into().unwrap();
		let postseason = season.postseason.expect("Expected the MLB to have a postseason");
		let games = ScheduleRequest::<()>::builder().date_range(postseason).sport_id(SportId::MLB).build_and_get().await.unwrap();
		let games = games.dates.into_iter().flat_map(|date| date.games).filter(|game| game.game_type.is_postseason()).map(|game| game.game_id).collect::<Vec<_>>();
		let mut has_errors = false;
		for game in games {
			if let Err(e) = BoxscoreRequest::builder().id(game).build_and_get().await {
			    dbg!(e);
			    has_errors = true;
			}
		}
		assert!(!has_errors, "Has errors.");
	}

	#[cfg_attr(not(feature = "_heavy_tests"), ignore)]
    #[tokio::test]
    async fn regular_season_boxscore() {
        let [season]: [Season; 1] = SeasonsRequest::builder().season(TEST_YEAR).sport_id(SportId::MLB).build_and_get().await.unwrap().seasons.try_into().unwrap();
        let regular_season = season.regular_season;
        let games = ScheduleRequest::<()>::builder().date_range(regular_season).sport_id(SportId::MLB).build_and_get().await.unwrap();
        let games = games.dates.into_iter().flat_map(|date| date.games).filter(|game| game.game_type == GameType::RegularSeason).collect::<Vec<_>>();
        let mut has_errors = false;
        for game in games {
            if let Err(e) = BoxscoreRequest::builder().id(game.game_id).build_and_get().await {
                dbg!(e);
                has_errors = true;
            }
        }
        assert!(!has_errors, "Has errors.");
    }
}
