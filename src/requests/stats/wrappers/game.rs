use chrono::{NaiveDate, Utc};
use derive_more::{Deref, DerefMut};
use serde::Deserialize;
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
	pub is_home: bool,
	pub game: GameId,
	pub season: SeasonId,

	#[deref]
	#[deref_mut]
	#[serde(rename = "stat")]
	stats: T,
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

impl<T: RawStat> Default for WithGame<T> {
	fn default() -> Self {
		Self {
			opponent: NamedTeam::unknown_team(),
			date: Utc::now().date_naive(),
			is_home: true,
			game: GameId::new(0),
			season: SeasonId::current_season(),
			stats: T::default(),
		}
	}
}

impl<T: RawStat> SingletonSplitStat for WithGame<T> {}
