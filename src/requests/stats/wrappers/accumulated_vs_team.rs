use crate::game_types::GameType;
use crate::person::NamedPerson;
use crate::stats::wrappers::{AccumulatedMatchup, BatterPiece, GameTypePiece, OpposingTeamPiece, TeamPiece};
use crate::stats::{RawStat, SingletonSplitStat};
use crate::team::NamedTeam;
use derive_more::{Deref, DerefMut};
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Eq, Clone, Deref, DerefMut)]
#[serde(rename_all = "camelCase")]
#[serde(bound = "T: RawStat")]
pub struct AccumulatedVsTeamTotalMatchup<T: RawStat> {
	pub batter: NamedPerson,

	#[deref]
	#[deref_mut]
	inner: AccumulatedMatchup<T>,
}

impl<T: RawStat> OpposingTeamPiece for AccumulatedVsTeamTotalMatchup<T> {
	fn opposing_team(&self) -> &NamedTeam {
		&self.opposing_team
	}
}

impl<T: RawStat> GameTypePiece for AccumulatedVsTeamTotalMatchup<T> {
	fn game_type(&self) -> &GameType {
		&self.game_type
	}
}

impl<T: RawStat> TeamPiece for AccumulatedVsTeamTotalMatchup<T> {
	fn team(&self) -> &NamedTeam {
		&self.team
	}
}

impl<T: RawStat> BatterPiece for AccumulatedVsTeamTotalMatchup<T> {
	fn batter(&self) -> &NamedPerson {
		&self.batter
	}
}

impl<T: RawStat> Default for AccumulatedVsTeamTotalMatchup<T> {
	fn default() -> Self {
		Self {
			batter: NamedPerson::unknown_person(),

			inner: AccumulatedMatchup::default(),
		}
	}
}

impl<T: RawStat> SingletonSplitStat for AccumulatedVsTeamTotalMatchup<T> {}