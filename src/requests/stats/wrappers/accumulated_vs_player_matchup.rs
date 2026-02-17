use derive_more::{Deref, DerefMut};
use serde::Deserialize;
use crate::game_types::GameType;
use crate::person::NamedPerson;
use crate::stats::{RawStat, SingletonSplitStat};
use crate::stats::wrappers::{AccumulatedMatchup, BatterPiece, GameTypePiece, OpposingTeamPiece, PitcherPiece, TeamPiece};
use crate::team::NamedTeam;

#[derive(Debug, Deserialize, PartialEq, Eq, Clone, Deref, DerefMut)]
#[serde(rename_all = "camelCase")]
#[serde(bound = "T: RawStat")]
pub struct AccumulatedVsPlayerMatchup<T: RawStat> {
	pub pitcher: NamedPerson,
	pub batter: NamedPerson,

	#[deref]
	#[deref_mut]
	#[serde(flatten)]
	inner: AccumulatedMatchup<T>,
}

impl<T: RawStat> OpposingTeamPiece for AccumulatedVsPlayerMatchup<T> {
	fn opposing_team(&self) -> &NamedTeam {
		&self.opposing_team
	}
}

impl<T: RawStat> GameTypePiece for AccumulatedVsPlayerMatchup<T> {
	fn game_type(&self) -> &GameType {
		&self.game_type
	}
}

impl<T: RawStat> TeamPiece for AccumulatedVsPlayerMatchup<T> {
	fn team(&self) -> &NamedTeam {
		&self.team
	}
}

impl<T: RawStat> PitcherPiece for AccumulatedVsPlayerMatchup<T> {
	fn pitcher(&self) -> &NamedPerson {
		&self.pitcher
	}
}

impl<T: RawStat> BatterPiece for AccumulatedVsPlayerMatchup<T> {
	fn batter(&self) -> &NamedPerson {
		&self.batter
	}
}

impl<T: RawStat> Default for AccumulatedVsPlayerMatchup<T> {
	fn default() -> Self {
		Self {
			pitcher: NamedPerson::unknown_person(),
			batter: NamedPerson::unknown_person(),
			
			inner: AccumulatedMatchup::default(),
		}
	}
}

impl<T: RawStat> SingletonSplitStat for AccumulatedVsPlayerMatchup<T> {}
