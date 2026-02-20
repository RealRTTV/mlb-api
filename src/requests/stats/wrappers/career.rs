use derive_more::{Deref, DerefMut};
use serde::Deserialize;
use crate::meta::StandingsType;
use crate::league::NamedLeague;
use crate::person::NamedPerson;
use crate::sport::SportId;
use crate::stats::{RawStat, SingletonSplitStat};
use crate::stats::wrappers::{GameTypePiece, LeaguePiece, PlayerPiece, TeamPiece};
use crate::team::NamedTeam;

#[derive(Debug, Deserialize, PartialEq, Eq, Clone, Deref, DerefMut)]
#[serde(bound = "T: RawStat")]
#[serde(rename_all = "camelCase")]
pub struct Career<T: RawStat> {
	#[serde(default = "NamedTeam::unknown_team")]
	pub team: NamedTeam,
	pub player: NamedPerson,
	#[serde(default = "NamedLeague::unknown_league")]
	pub league: NamedLeague,
	pub sport: Option<SportId>,
	pub game_type: StandingsType,

	#[deref]
	#[deref_mut]
	#[serde(rename = "stat")]
	pub stats: T,
}

impl<T: RawStat> TeamPiece for Career<T> {
	fn team(&self) -> &NamedTeam {
		&self.team
	}
}

impl<T: RawStat> PlayerPiece for Career<T> {
	fn player(&self) -> &NamedPerson {
		&self.player
	}
}

impl<T: RawStat> GameTypePiece for Career<T> {
	fn game_type(&self) -> &StandingsType {
		&self.game_type
	}
}

impl<T: RawStat> LeaguePiece for Career<T> {
	fn league(&self) -> &NamedLeague {
		&self.league
	}
}

impl<T: RawStat> Default for Career<T> {
	fn default() -> Self {
		Self {
			team: NamedTeam::unknown_team(),
			player: NamedPerson::unknown_person(),
			league: NamedLeague::unknown_league(),
			sport: None,
			game_type: StandingsType::default(),
			stats: T::default(),
		}
	}
}

impl<T: RawStat> SingletonSplitStat for Career<T> {

}