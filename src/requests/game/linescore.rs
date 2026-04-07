use std::fmt::{Display, Formatter};

use bon::Builder;
use derive_more::{Deref, DerefMut};
use serde::Deserialize;
use serde::de::IgnoredAny;
use serde_with::{serde_as, DefaultOnError};

use crate::request::RequestURL;
use crate::{Copyright, HomeAway};
use crate::game::{AtBatCount, GameId, Inning, InningHalf, RHE};
use crate::person::NamedPerson;
use crate::team::NamedTeam;

/// An inning by inning record of the game's scoring.
/// 
/// This is pretty much a 1:1 correlation of the:
/// ```
///     | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10| 11|| R | H | E |
/// LAD | 0 | 0 | 0 | 1 | 0 | 1 | 0 | 1 | 1 | 0 | 1 || 5 | 11| 0 |
/// TOR | 0 | 0 | 3 | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 0 || 4 | 14| 0 |
/// ````
/// You're used to seeing.
#[serde_as]
#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "_debug", serde(deny_unknown_fields))]
pub struct Linescore {
    #[serde(default)]
    pub copyright: Copyright,
    #[serde(default = "Inning::starting")]
    pub current_inning: Inning,
    #[serde(default = "InningHalf::starting")]
    pub inning_half: InningHalf,
    pub scheduled_innings: usize,
    pub innings: Vec<LinescoreInningRecord>,
    #[serde(rename = "teams")]
    pub rhe_totals: HomeAway<RHE>,
    #[serde_as(deserialize_as = "DefaultOnError")]
    pub offense: Option<LinescoreOffense>,
    #[serde_as(deserialize_as = "DefaultOnError")]
    pub defense: Option<LinescoreDefense>,
    pub note: Option<String>,
    #[serde(flatten)]
    pub count: AtBatCount,

    #[doc(hidden)]
    #[serde(rename = "currentInningOrdinal", default)]
    pub __current_inning_ordinal: IgnoredAny,
    #[doc(hidden)]
    #[serde(rename = "inningState", default)]
    pub __inning_state: IgnoredAny,
    #[doc(hidden)]
    #[serde(rename = "isTopInning", default)]
    pub __is_top_inning: IgnoredAny,
}

/// A record of [`RHE`] from both teams in a single inning.
#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "_debug", serde(deny_unknown_fields))]
pub struct LinescoreInningRecord {
    #[serde(rename = "num")]
    pub inning: Inning,
    #[serde(flatten)]
    pub inning_record: HomeAway<RHE>,

    #[doc(hidden)]
    #[serde(rename = "ordinalNum", default)]
    pub __ordinal_num: IgnoredAny,
}

/// Current offense in the linescore
#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "_debug", serde(deny_unknown_fields))]
pub struct LinescoreOffense {
    pub batter: NamedPerson,
    pub on_deck: NamedPerson,
    #[serde(rename = "inHole")]
    pub in_the_hole: NamedPerson,
    pub team: NamedTeam,
    /// Index of the current player in the batting order
    #[serde(rename = "battingOrder")]
    pub batting_order_index: usize,

    #[doc(hidden)]
    #[serde(rename = "pitcher", default)]
    pub __pitcher: IgnoredAny,
    #[doc(hidden)]
    #[serde(rename = "catcher", default)]
    pub __catcher: IgnoredAny,
    #[doc(hidden)]
    #[serde(rename = "first", default)]
    pub __first_baseman: IgnoredAny,
    #[doc(hidden)]
    #[serde(rename = "second", default)]
    pub __second_baseman: IgnoredAny,
    #[doc(hidden)]
    #[serde(rename = "third", default)]
    pub __third_baseman: IgnoredAny,
    #[doc(hidden)]
    #[serde(rename = "shortstop", default)]
    pub __shortstop: IgnoredAny,
    #[doc(hidden)]
    #[serde(rename = "left", default)]
    pub __leftfielder: IgnoredAny,
    #[doc(hidden)]
    #[serde(rename = "center", default)]
    pub __centerfielder: IgnoredAny,
    #[doc(hidden)]
    #[serde(rename = "right", default)]
    pub __rightfielder: IgnoredAny,
}

/// Current defense in the linescore, note that it also contains their offense too.
#[derive(Debug, Deserialize, PartialEq, Clone, Deref, DerefMut)]
#[cfg_attr(feature = "_debug", serde(deny_unknown_fields))]
pub struct LinescoreDefense {
    pub pitcher: NamedPerson,
    pub catcher: NamedPerson,
    #[serde(rename = "first")]
    pub first_baseman: NamedPerson,
    #[serde(rename = "second")]
    pub second_baseman: NamedPerson,
    #[serde(rename = "third")]
    pub third_baseman: NamedPerson,
    pub shortstop: NamedPerson,
    #[serde(rename = "left")]
    pub leftfielder: NamedPerson,
    #[serde(rename = "center")]
    pub centerfielder: NamedPerson,
    #[serde(rename = "right")]
    pub rightfielder: NamedPerson,
    
    #[deref]
    #[deref_mut]
    #[serde(flatten)]
    pub offense: LinescoreOffense,
}

/// Returns a [`Linescore`]
#[derive(Builder)]
#[builder(derive(Into))]
pub struct LinescoreRequest {
    #[builder(into)]
    id: GameId
}

impl<S: linescore_request_builder::State + linescore_request_builder::IsComplete> crate::request::RequestURLBuilderExt for LinescoreRequestBuilder<S> {
    type Built = LinescoreRequest;
}

impl Display for LinescoreRequest {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "http://statsapi.mlb.com/api/v1/game/{}/linescore", self.id)
    }
}

impl RequestURL for LinescoreRequest {
    type Response = Linescore;
}

#[cfg(test)]
mod tests {
    use crate::TEST_YEAR;
    use crate::game::LinescoreRequest;
    use crate::meta::GameType;
    use crate::request::RequestURLBuilderExt;
    use crate::schedule::ScheduleRequest;
    use crate::season::{Season, SeasonsRequest};
    use crate::sport::SportId;

    #[tokio::test]
    async fn ws_gm7_2025_linescore() {
        let _ = LinescoreRequest::builder().id(813_024).build_and_get().await.unwrap();
    }
    
    #[tokio::test]
	async fn postseason_linescore() {
		let [season]: [Season; 1] = SeasonsRequest::builder().season(TEST_YEAR).sport_id(SportId::MLB).build_and_get().await.unwrap().seasons.try_into().unwrap();
		let postseason = season.postseason.expect("Expected the MLB to have a postseason");
		let games = ScheduleRequest::<()>::builder().date_range(postseason).sport_id(SportId::MLB).build_and_get().await.unwrap();
		let games = games.dates.into_iter().flat_map(|date| date.games).filter(|game| game.game_type.is_postseason()).map(|game| game.game_id).collect::<Vec<_>>();
		let mut has_errors = false;
		for game in games {
			if let Err(e) = LinescoreRequest::builder().id(game).build_and_get().await {
			    dbg!(e);
			    has_errors = true;
			}
		}
		assert!(!has_errors, "Has errors.");
	}

	#[cfg_attr(not(feature = "_heavy_tests"), ignore)]
    #[tokio::test]
    async fn regular_season_linescore() {
        let [season]: [Season; 1] = SeasonsRequest::builder().season(TEST_YEAR).sport_id(SportId::MLB).build_and_get().await.unwrap().seasons.try_into().unwrap();
        let regular_season = season.regular_season;
        let games = ScheduleRequest::<()>::builder().date_range(regular_season).sport_id(SportId::MLB).build_and_get().await.unwrap();
        let games = games.dates.into_iter().flat_map(|date| date.games).filter(|game| game.game_type == GameType::RegularSeason).collect::<Vec<_>>();
        let mut has_errors = false;
        for game in games {
            if let Err(e) = LinescoreRequest::builder().id(game.game_id).build_and_get().await {
                dbg!(e);
                has_errors = true;
            }
        }
        assert!(!has_errors, "Has errors.");
    }
}
