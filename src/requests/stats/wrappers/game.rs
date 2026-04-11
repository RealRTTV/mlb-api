use chrono::{NaiveDate, Utc};
use derive_more::{Deref, DerefMut};
use serde::Deserialize;
use crate::TeamSide;
use crate::game::GameId;
use crate::season::SeasonId;
use crate::stats::{RawStat, SingletonSplitStat};
use crate::stats::wrappers::{GamePiece, OpposingTeamPiece, SeasonPiece};
use crate::team::NamedTeam;

#[derive(Debug, Deserialize, PartialEq, Eq, Clone, Deref, DerefMut)]
#[serde(rename_all = "camelCase")]
#[serde(bound = "T: RawStat")]
pub struct WithGame<T: RawStat> {
	pub opponent: NamedTeam,
	pub date: NaiveDate,
	#[serde(rename = "isHome", deserialize_with = "crate::deserialize_team_side_from_is_home")]
	pub team_side: TeamSide,
	pub game: GameId,
	pub season: SeasonId,

	#[deref]
	#[deref_mut]
	#[serde(rename = "stat")]
	pub stats: T,
}

impl<T: RawStat> SeasonPiece for WithGame<T> {
	fn season(&self) -> &SeasonId {
		&self.season
	}
}

impl<T: RawStat> OpposingTeamPiece for WithGame<T> {
	fn opposing_team(&self) -> &NamedTeam {
		&self.opponent
	}
}

impl<T: RawStat> GamePiece for WithGame<T> {
	fn game(&self) -> &GameId {
		&self.game
	}
}

impl<T: RawStat + Default> Default for WithGame<T> {
	fn default() -> Self {
		Self {
			opponent: NamedTeam::unknown_team(),
			date: Utc::now().date_naive(),
			team_side: TeamSide::Home,
			game: GameId::new(0),
			season: SeasonId::current_season(),
			stats: T::default(),
		}
	}
}

impl<T: RawStat + Default> SingletonSplitStat for WithGame<T> {}
