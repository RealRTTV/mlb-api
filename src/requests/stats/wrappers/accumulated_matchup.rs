use derive_more::{Deref, DerefMut};
use serde::Deserialize;
use crate::game_types::GameType;
use crate::stats::{RawStat, SingletonSplitStat};
use crate::stats::wrappers::{GameTypePiece, OpposingTeamPiece, TeamPiece};
use crate::team::NamedTeam;

#[derive(Debug, Deserialize, PartialEq, Eq, Clone, Deref, DerefMut)]
#[serde(rename_all = "camelCase")]
#[serde(bound = "T: RawStat")]
pub struct AccumulatedMatchup<T: RawStat> {
	#[serde(rename = "opponent")]
	pub opposing_team: NamedTeam,
	pub game_type: GameType,
	pub team: NamedTeam,
	
	#[deref]
	#[deref_mut]
	#[serde(rename = "stat")]
	pub stats: T,
}

impl<T: RawStat> OpposingTeamPiece for AccumulatedMatchup<T> {
	fn opposing_team(&self) -> &NamedTeam {
		&self.opposing_team
	}
}

impl<T: RawStat> GameTypePiece for AccumulatedMatchup<T> {
	fn game_type(&self) -> &GameType {
		&self.game_type
	}
}

impl<T: RawStat> TeamPiece for AccumulatedMatchup<T> {
	fn team(&self) -> &NamedTeam {
		&self.team
	}
}

impl<T: RawStat> Default for AccumulatedMatchup<T> {
	fn default() -> Self {
		Self {
			opposing_team: NamedTeam::unknown_team(),
			game_type: GameType::default(),
			team: NamedTeam::unknown_team(),
			
			stats: T::default(),
		}
	}
}

impl<T: RawStat> SingletonSplitStat for AccumulatedMatchup<T> {}
