use crate::game_types::GameType;
use crate::person::NamedPerson;
use crate::season::SeasonId;
use crate::stats::wrappers::{AccumulatedVsPlayerMatchup, BatterPiece, GameTypePiece, OpposingTeamPiece, PitcherPiece, SeasonPiece, TeamPiece};
use crate::stats::{RawStat, SingletonSplitStat};
use crate::team::NamedTeam;
use derive_more::{Deref, DerefMut};
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Eq, Clone, Deref, DerefMut)]
#[serde(rename_all = "camelCase")]
#[serde(bound = "T: RawStat")]
pub struct AccumulatedVsTeamSeasonalPitcherSplit<T: RawStat> {
	pub season: SeasonId,

	#[deref]
	#[deref_mut]
	inner: AccumulatedVsPlayerMatchup<T>,
}

impl<T: RawStat> OpposingTeamPiece for AccumulatedVsTeamSeasonalPitcherSplit<T> {
	fn opposing_team(&self) -> &NamedTeam {
		&self.opposing_team
	}
}

impl<T: RawStat> GameTypePiece for AccumulatedVsTeamSeasonalPitcherSplit<T> {
	fn game_type(&self) -> &GameType {
		&self.game_type
	}
}

impl<T: RawStat> TeamPiece for AccumulatedVsTeamSeasonalPitcherSplit<T> {
	fn team(&self) -> &NamedTeam {
		&self.team
	}
}

impl<T: RawStat> PitcherPiece for AccumulatedVsTeamSeasonalPitcherSplit<T> {
	fn pitcher(&self) -> &NamedPerson {
		&self.pitcher
	}
}

impl<T: RawStat> BatterPiece for AccumulatedVsTeamSeasonalPitcherSplit<T> {
	fn batter(&self) -> &NamedPerson {
		&self.batter
	}
}

impl<T: RawStat> SeasonPiece for AccumulatedVsTeamSeasonalPitcherSplit<T> {
	fn season(&self) -> &SeasonId {
		&self.season
	}
}

impl<T: RawStat> Default for AccumulatedVsTeamSeasonalPitcherSplit<T> {
	fn default() -> Self {
		Self {
			season: SeasonId::current_season(),
			
			inner: AccumulatedVsPlayerMatchup::default(),
		}
	}
}

impl<T: RawStat> SingletonSplitStat for AccumulatedVsTeamSeasonalPitcherSplit<T> {}
