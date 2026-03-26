//! A general feed of a game. Includes plays, linescore, etc. Typically your request unless you want to get more specific.

use std::fmt::{Display, Formatter};
use bon::Builder;
use derive_more::{Deref, DerefMut};
use fxhash::FxHashMap;
use serde::Deserialize;
use serde::de::IgnoredAny;
use crate::game::{Boxscore, Decisions, DoubleHeaderKind, GameDateTime, GameId, GameInfo, GameStatLeaders, GameTags, PlayAbout, Plays, ResourceUsage, ReviewData, WeatherConditions};
use crate::game::linescore::Linescore;
use crate::meta::{GameStatus, GameType};
use crate::meta::LogicalEventId;
use crate::person::{Ballplayer, NamedPerson, PersonId};
use crate::request::RequestURL;
use crate::season::SeasonId;
use crate::team::Team;
use crate::{Copyright, HomeAwaySplit};
use crate::venue::{Venue, VenueId};

/// See [`self`]
#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct LiveFeedResponse {
	pub copyright: Copyright,
	#[serde(rename = "gamePk")]
	pub id: GameId,
	#[serde(rename = "metaData")]
	pub meta: LiveFeedMetadata,
	#[serde(rename = "gameData")]
	pub data: LiveFeedData,
	#[serde(rename = "liveData")]
	pub live: LiveFeedLiveData,

	#[doc(hidden)]
	#[serde(rename = "link", default)]
	pub __link: IgnoredAny,
}

/// Metadata about the game, often not useful.
#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct LiveFeedMetadata {
	/// Recommended duration to send new requests (in seconds). Often 10.
	#[serde(rename = "wait")]
	pub recommended_poll_rate: u32,
	pub game_events: Vec<String>, // todo: what is this type
	pub logical_events: Vec<LogicalEventId>,

    #[doc(hidden)]
    #[serde(rename = "timeStamp", default)]
	pub __timestamp: IgnoredAny,
}

/// General information about the game
#[derive(Debug, Deserialize, PartialEq, Clone, Deref, DerefMut)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct LiveFeedData {
	#[deref]
	#[deref_mut]
	#[serde(rename = "game")]
	game: LiveFeedDataMeta,
	pub datetime: GameDateTime,
	pub status: GameStatus,
	pub teams: HomeAwaySplit<Team<()>>,
	#[serde(deserialize_with = "super::deserialize_players_cache")]
	pub players: FxHashMap<PersonId, Ballplayer<()>>,
	pub venue: Venue,
	pub official_venue: VenueId,
	pub weather: WeatherConditions,
	#[serde(rename = "gameInfo")]
	pub info: GameInfo,
	pub review: ReviewData,
	#[serde(rename = "flags")]
	pub live_tags: GameTags,
	pub probable_pitchers: HomeAwaySplit<NamedPerson>,
	pub official_scorer: NamedPerson,
	pub primary_datacaster: NamedPerson,
	pub mound_visits: HomeAwaySplit<ResourceUsage>,

    #[doc(hidden)]
    #[serde(rename = "alerts", default)]
	pub __alerts: IgnoredAny,
}

/// More specific information about the "game", child of [`LiveFeedData`]
#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct LiveFeedDataMeta {
	#[serde(rename = "pk")]
	pub id: GameId,
	#[serde(rename = "type")]
	pub game_type: GameType,
	pub double_header: DoubleHeaderKind,
	/// Will state `P` for [`GameType::Playoffs`] games rather than what playoff series it is, amongst other things
	pub gameday_type: GameType,
	#[serde(deserialize_with = "crate::from_yes_no")]
	pub tiebreaker: bool,
	/// No clue what this means
	pub game_number: u32,
	pub season: SeasonId,
	#[serde(rename = "seasonDisplay")]
	pub displayed_season: SeasonId,

	#[doc(hidden)]
	#[serde(rename = "id", default)]
	pub __id: IgnoredAny,
	#[doc(hidden)]
	#[serde(rename = "calendarEventID", default)]
	pub __calender_event_id: IgnoredAny,
}

/// Live data about the game -- i.e. stuff that changes as the game goes on.
/// 
/// Includes a lot of sub-requests within it, such as the [`super::PlayByPlay`] and [`super::Linescore`].
#[derive(Debug, Deserialize, PartialEq, Clone)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct LiveFeedLiveData {
	pub linescore: Linescore,
	pub boxscore: Boxscore,
	pub decisions: Decisions,
	pub leaders: GameStatLeaders,
	pub plays: Plays,
}

/// Returns a [`LiveFeedResponse`]
#[derive(Builder)]
#[builder(derive(Into))]
pub struct LiveFeedRequest {
	#[builder(into)]
	id: GameId,
}

impl<S: live_feed_request_builder::State + live_feed_request_builder::IsComplete> crate::request::RequestURLBuilderExt for LiveFeedRequestBuilder<S> {
	type Built = LiveFeedRequest;
}

impl Display for LiveFeedRequest {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "http://statsapi.mlb.com/api/v1.1/game/{}/feed/live", self.id)
	}
}

impl RequestURL for LiveFeedRequest {
	type Response = LiveFeedResponse;
}

#[cfg(test)]
mod tests {
	use crate::game::LiveFeedRequest;
	use crate::meta::GameType;
use crate::request::RequestURLBuilderExt;
    use crate::schedule::ScheduleRequest;
    use crate::season::{Season, SeasonsRequest};
    use crate::sport::SportId;

	#[tokio::test]
	async fn ws_gm7_2025_live_feed() {
		dbg!(LiveFeedRequest::builder().id(813_024).build().to_string());
		let response = LiveFeedRequest::builder().id(813_024).build_and_get().await.unwrap();
		dbg!(response);
	}

	#[tokio::test]
	async fn postseason_2025_live_feed() {
		let [season]: [Season; 1] = SeasonsRequest::builder().season(2025).sport_id(SportId::MLB).build_and_get().await.unwrap().seasons.try_into().unwrap();
		let postseason = season.postseason.expect("Expected the MLB to have a postseason");
		let games = ScheduleRequest::<()>::builder().date_range(postseason).sport_id(SportId::MLB).build_and_get().await.unwrap();
		let games = games.dates.into_iter().flat_map(|date| date.games).filter(|game| game.game_type.is_postseason()).map(|game| game.game_id).collect::<Vec<_>>();
		let mut has_errors = false;
		for game in games {
			if let Err(e) = LiveFeedRequest::builder().id(game).build_and_get().await {
				dbg!(e);
				has_errors = true;
			}
		}
		assert!(!has_errors, "Has errors.");
	}
	
    #[tokio::test]
    async fn regular_season_2025_live_feed() {
        let [season]: [Season; 1] = SeasonsRequest::builder().season(2025).sport_id(SportId::MLB).build_and_get().await.unwrap().seasons.try_into().unwrap();
        let regular_season = season.regular_season;
        let games = ScheduleRequest::<()>::builder().date_range(regular_season).sport_id(SportId::MLB).build_and_get().await.unwrap();
        let games = games.dates.into_iter().flat_map(|date| date.games).filter(|game| game.game_type == GameType::RegularSeason).collect::<Vec<_>>();
        let mut has_errors = false;
        for game in games {
            if let Err(e) = LiveFeedRequest::builder().id(game.game_id).build_and_get().await {
                dbg!(e);
                has_errors = true;
            }
        }
        assert!(!has_errors, "Has errors.");
    }
}
