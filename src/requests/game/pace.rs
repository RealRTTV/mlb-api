//! Conglomerate pace data about teams, leagues, and sports. Such as total hits, innings, game duration, etc.

use std::fmt::Display;

use bon::Builder;
use chrono::TimeDelta;
use derive_more::{Deref, DerefMut};
use serde::{Deserialize, de::IgnoredAny};
use itertools::Itertools;

use crate::{Copyright, league::{LeagueId, NamedLeague}, request::RequestURL, season::SeasonId, sport::SportId, team::{NamedTeam, TeamId}};

#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "_debug", serde(deny_unknown_fields))]
pub struct GamePace {
    #[serde(deserialize_with = "crate::deserialize_time_delta_from_hms", default)]
    pub total_game_time: TimeDelta,
    /// Can have a .5 for a half-inning played
    #[serde(rename = "totalInningsPlayed", default)]
    pub innings_played: f64,
    #[serde(rename = "totalHits", default)]
    pub hits: usize,
    #[serde(rename = "totalRuns", default)]
    pub runs: usize,
    #[serde(rename = "totalPlateAppearances", default)]
    pub plate_appearances: usize,
    #[serde(rename = "totalPitchers",default)]
    pub num_pitchers: usize,
    #[serde(rename = "totalPitches", default)]
    pub num_pitches: usize,
    #[serde(rename = "totalGames", default)]
    pub games: usize,

    #[serde(rename = "total9InnGames", default)]
    pub nine_inning_games: usize,
    #[serde(rename = "total9InnGamesCompletedEarly", default)]
    pub nine_inning_games_completed_early: usize,
    #[serde(rename = "total9InnGamesScheduled", default)]
    pub nine_inning_games_scheduled: usize,
    #[serde(rename = "total9InnGamesWithoutExtraInn", default)]
    pub nine_inning_games_without_extra_innings: usize,
    
    #[serde(rename = "total7InnGames", default)]
    pub seven_inning_games: usize,
    #[serde(rename = "total7InnGamesCompletedEarly", default)]
    pub seven_inning_games_completed_early: usize,
    #[serde(rename = "total7InnGamesScheduled", default)]
    pub seven_inning_games_scheduled: usize,
    #[serde(rename = "total7InnGamesWithoutExtraInn", default)]
    pub seven_inning_games_without_extra_innings: usize,
    
    #[serde(rename = "totalExtraInnGames", default)]
    pub extra_inning_games: usize,
    
    pub season: SeasonId,
    #[serde(rename = "timePer7InnGame", deserialize_with = "crate::deserialize_time_delta_from_hms", default)]
    pub time_per_seven_inning_game: TimeDelta,

    #[doc(hidden)]
    #[serde(rename = "prPortalCalculatedFields", default)]
    pub __pr_portal_calculated_fields: IgnoredAny,

    #[doc(hidden)]
    #[serde(rename = "hitsPer9Inn", default)]
    pub __hits_per_nine: IgnoredAny,

    #[doc(hidden)]
    #[serde(rename = "runsPer9Inn", default)]
    pub __runs_per_nine: IgnoredAny,

    #[doc(hidden)]
    #[serde(rename = "pitchesPer9Inn", default)]
    pub __pitches_per_nine: IgnoredAny,

    #[doc(hidden)]
    #[serde(rename = "plateAppearancesPer9Inn", default)]
    pub __plate_appearances_per_nine: IgnoredAny,

    #[doc(hidden)]
    #[serde(rename = "hitsPerGame", default)]
    pub __hits_per_game: IgnoredAny,

    #[doc(hidden)]
    #[serde(rename = "runsPerGame", default)]
    pub __runs_per_game: IgnoredAny,

    #[doc(hidden)]
    #[serde(rename = "inningsPlayedPerGame", default)]
    pub __innings_played_per_game: IgnoredAny,

    #[doc(hidden)]
    #[serde(rename = "pitchesPerGame", default)]
    pub __pitches_per_game: IgnoredAny,

    #[doc(hidden)]
    #[serde(rename = "pitchersPerGame", default)]
    pub __pitchers_per_game: IgnoredAny,

    #[doc(hidden)]
    #[serde(rename = "plateAppearancesPerGame", default)]
    pub __plate_appearances_per_game: IgnoredAny,

    #[doc(hidden)]
    #[serde(rename = "timePerGame", default)]
    pub __time_per_game: IgnoredAny,

    #[doc(hidden)]
    #[serde(rename = "timePerPitch", default)]
    pub __time_per_pitch: IgnoredAny,

    #[doc(hidden)]
    #[serde(rename = "timePerHit", default)]
    pub __time_per_hit: IgnoredAny,

    #[doc(hidden)]
    #[serde(rename = "timePerRun", default)]
    pub __time_per_run: IgnoredAny,

    #[doc(hidden)]
    #[serde(rename = "timePerPlateAppearance", default)]
    pub __time_per_plate_appearance: IgnoredAny,

    #[doc(hidden)]
    #[serde(rename = "timePer9Inn", default)]
    pub __time_per_nine: IgnoredAny,

    #[doc(hidden)]
    #[serde(rename = "timePer77PlateAppearances", default)]
    pub __time_per_77_plate_appearances: IgnoredAny,

    #[doc(hidden)]
    #[serde(rename = "totalExtraInnTime", default)]
    pub __total_extra_inning_time: IgnoredAny,

    #[doc(hidden)]
    #[serde(rename = "timePer7InnGameWithoutExtraInn", default)]
    pub __time_per_seven_inning_game_without_extra_inning: IgnoredAny,

    #[doc(hidden)]
    #[serde(rename = "hitsPerRun", default)]
    pub __hits_per_run: IgnoredAny,

    #[doc(hidden)]
    #[serde(rename = "pitchesPerPitcher", default)]
    pub __pitches_per_pitcher: IgnoredAny,
}

#[derive(Debug, Deserialize, PartialEq, Clone, Deref, DerefMut)]
#[cfg_attr(feature = "_debug", serde(deny_unknown_fields))]
pub struct TeamGamePace {
    #[deref]
    #[deref_mut]
    #[serde(flatten)]
    pub pace: GamePace,

    pub team: NamedTeam,
    pub league: NamedLeague,
    pub sport: SportId,
}

#[derive(Debug, Deserialize, PartialEq, Clone, Deref, DerefMut)]
#[cfg_attr(feature = "_debug", serde(deny_unknown_fields))]
pub struct LeagueGamePace {
    #[deref]
    #[deref_mut]
    #[serde(flatten)]
    pub pace: GamePace,

    pub league: NamedLeague,

    // only sometimes present
    #[doc(hidden)]
    #[serde(rename = "sport", default)]
    pub __sport: IgnoredAny,
}

#[derive(Debug, Deserialize, PartialEq, Clone, Deref, DerefMut)]
#[cfg_attr(feature = "_debug", serde(deny_unknown_fields))]
pub struct SportGamePace {
    #[deref]
    #[deref_mut]
    #[serde(flatten)]
    pub pace: GamePace,

    pub sport: SportId,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
#[cfg_attr(feature = "_debug", serde(deny_unknown_fields))]
pub struct SportGamePaceResponse {
    pub copyright: Copyright,
    pub sports: Vec<SportGamePace>,

    #[doc(hidden)]
    #[serde(rename = "teams", default)]
    pub __teams: IgnoredAny,

    #[doc(hidden)]
    #[serde(rename = "leagues", default)]
    pub __leagues: IgnoredAny,
}

#[derive(Builder)]
#[builder(derive(Into))]
pub struct SportGamePaceRequest {
    #[builder(into)]
    season: SeasonId,
    #[builder(into)]
    sport: Option<SportId>,
}

impl Display for SportGamePaceRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "http://statsapi.mlb.com/api/v1/gamePace{}", gen_params! {
            "season": self.season,
            "sportId"?: self.sport,
        })
    }
}

impl<S: sport_game_pace_request_builder::State + sport_game_pace_request_builder::IsComplete> crate::request::RequestURLBuilderExt for SportGamePaceRequestBuilder<S> {
    type Built = SportGamePaceRequest;
}

impl RequestURL for SportGamePaceRequest {
    type Response = SportGamePaceResponse;
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
#[cfg_attr(feature = "_debug", serde(deny_unknown_fields))]
pub struct TeamGamePaceResponse {
    pub copyright: Copyright,
    pub teams: Vec<TeamGamePace>,

    #[doc(hidden)]
    #[serde(rename = "sports", default)]
    pub __sports: IgnoredAny,

    #[doc(hidden)]
    #[serde(rename = "leagues", default)]
    pub __leagues: IgnoredAny,
}

#[derive(Builder)]
#[builder(derive(Into))]
pub struct TeamGamePaceRequest {
    #[builder(into)]
    season: SeasonId,
    teams: Vec<TeamId>,
}

impl Display for TeamGamePaceRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "http://statsapi.mlb.com/api/v1/gamePace{}", gen_params! {
            "season": self.season,
            "teamIds": self.teams.iter().join(","),
        })
    }
}

impl<S: team_game_pace_request_builder::State + team_game_pace_request_builder::IsComplete> crate::request::RequestURLBuilderExt for TeamGamePaceRequestBuilder<S> {
    type Built = TeamGamePaceRequest;
}

impl RequestURL for TeamGamePaceRequest {
    type Response = TeamGamePaceResponse;
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
#[cfg_attr(feature = "_debug", serde(deny_unknown_fields))]
pub struct LeagueGamePaceResponse {
    pub copyright: Copyright,
    pub leagues: Vec<LeagueGamePace>,

    #[doc(hidden)]
    #[serde(rename = "teams", default)]
    pub __teams: IgnoredAny,

    #[doc(hidden)]
    #[serde(rename = "sports", default)]
    pub __sports: IgnoredAny,
}

#[derive(Builder)]
#[builder(derive(Into))]
pub struct LeagueGamePaceRequest {
    #[builder(into)]
    season: SeasonId,
    leagues: Vec<LeagueId>,
}

impl Display for LeagueGamePaceRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "http://statsapi.mlb.com/api/v1/gamePace{}", gen_params! {
            "season": self.season,
            "leagueIds": self.leagues.iter().join(","),
        })
    }
}

impl<S: league_game_pace_request_builder::State + league_game_pace_request_builder::IsComplete> crate::request::RequestURLBuilderExt for LeagueGamePaceRequestBuilder<S> {
    type Built = LeagueGamePaceRequest;
}

impl RequestURL for LeagueGamePaceRequest {
    type Response = LeagueGamePaceResponse;
}

#[cfg(test)]
mod tests {
    use crate::{TEST_YEAR, game::{LeagueGamePaceRequest, SportGamePaceRequest, TeamGamePaceRequest}, request::RequestURLBuilderExt, sport::SportId, team::TeamsRequest};

    #[tokio::test]
    async fn sport_game_pace() {
        for season in 1901..=TEST_YEAR {
            let _ = SportGamePaceRequest::builder().season(season).build_and_get().await.unwrap();
        }
    }

    #[tokio::test]
    async fn team_game_pace() {
        let teams = TeamsRequest::<()>::builder().sport_id(SportId::MLB).build_and_get().await.unwrap().teams;
        let _ = TeamGamePaceRequest::builder().season(TEST_YEAR).teams(teams.into_iter().map(|team| team.id).collect()).build_and_get().await.unwrap();
    }

    #[tokio::test]
    async fn league_game_pace() {
        for season in 1901..=TEST_YEAR {
            let _ = LeagueGamePaceRequest::builder().season(season).leagues(vec![103.into(), 104.into()]).build_and_get().await.unwrap();
        }
    }
}

