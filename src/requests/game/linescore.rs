use std::fmt::{Display, Formatter};

use bon::Builder;
use derive_more::{Deref, DerefMut};
use serde::Deserialize;

use crate::request::RequestURL;
use crate::{Copyright, HomeAwaySplit};
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
#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Linescore {
    #[serde(default)]
    pub copyright: Copyright,
    pub current_inning: Inning,
    pub inning_half: InningHalf,
    pub scheduled_innings: usize,
    pub innings: Vec<LinescoreInningRecord>,
    #[serde(rename = "teams")]
    pub rhe_totals: HomeAwaySplit<RHE>,
    pub offense: LinescoreOffense,
    pub defense: LinescoreDefense,
    #[serde(flatten)]
    pub count: AtBatCount,
}

/// A record of [`RHE`] from both teams in a single inning.
#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LinescoreInningRecord {
    #[serde(rename = "num")]
    pub inning: Inning,
    #[serde(flatten)]
    pub inning_record: HomeAwaySplit<RHE>,
}

/// Current offense in the linescore
#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LinescoreOffense {
    pub batter: NamedPerson,
    pub on_deck: NamedPerson,
    #[serde(rename = "inHole")]
    pub in_the_hole: NamedPerson,
    pub team: NamedTeam,
    /// Index of the current player in the batting order
    #[serde(rename = "battingOrder")]
    pub batting_order_index: usize,
}

/// Current defense in the linescore, note that it also contains their offense too.
#[derive(Debug, Deserialize, PartialEq, Eq, Clone, Deref, DerefMut)]
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
    use crate::game::LinescoreRequest;
    use crate::request::RequestURLBuilderExt;
    use crate::schedule::ScheduleRequest;
    use crate::season::{Season, SeasonsRequest};
    use crate::sport::SportId;

    #[tokio::test]
    async fn ws_gm7_2025_linescore() {
        let _ = LinescoreRequest::builder().id(813_024).build_and_get().await.unwrap();
    }
    
    #[tokio::test]
	async fn postseason_2025_linescore() {
		let [season]: [Season; 1] = SeasonsRequest::builder().season(2025).sport_id(SportId::MLB).build_and_get().await.unwrap().seasons.try_into().unwrap();
		let postseason = season.postseason.expect("Expected the MLB to have a postseason");
		let games = ScheduleRequest::<()>::builder().date_range(postseason).sport_id(SportId::MLB).build_and_get().await.unwrap();
		let games = games.dates.into_iter().flat_map(|date| date.games).filter(|game| game.game_type.is_postseason()).map(|game| game.game_id).collect::<Vec<_>>();
		for game in games {
			if let Err(e) = LinescoreRequest::builder().id(game).build_and_get().await {
			    dbg!(e);
			}
		}
	}
}
