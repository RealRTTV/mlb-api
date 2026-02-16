use derive_more::{Deref, DerefMut};
use serde::Deserialize;
use crate::game_types::GameType;
use crate::league::NamedLeague;
use crate::person::NamedPerson;
use crate::sport::SportId;
use crate::stats::{RawStat, SingletonSplitStat};
use crate::team::NamedTeam;

#[derive(Debug, Deserialize, PartialEq, Eq, Clone, Deref, DerefMut)]
#[serde(bound = "T: RawStat")]
#[serde(rename_all = "camelCase")]
pub struct Career<T: RawStat> {
	pub team: Option<NamedTeam>,
	pub player: NamedPerson,
	pub league: Option<NamedLeague>,
	pub sport: Option<SportId>,
	pub game_type: GameType,
	#[deref]
	#[deref_mut]
	#[serde(rename = "stat")]
	stats: T,
}

impl<T: RawStat> Default for Career<T> {
	fn default() -> Self {
		Self {
			team: None,
			player: NamedPerson::unknown_person(),
			league: None,
			sport: None,
			game_type: GameType::default(),
			stats: T::default(),
		}
	}
}

impl<T: RawStat> SingletonSplitStat for Career<T> {

}