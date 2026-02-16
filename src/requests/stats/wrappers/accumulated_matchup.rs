use derive_more::{AsMut, AsRef, Deref, DerefMut};
use serde::Deserialize;
use crate::game_types::GameType;
use crate::stats::{RawStat, SingletonSplitStat};
use crate::team::NamedTeam;

#[derive(Debug, Deserialize, PartialEq, Eq, Clone, Deref, DerefMut, AsRef, AsMut)]
#[serde(rename_all = "camelCase")]
#[serde(bound = "T: RawStat")]
pub struct AccumulatedMatchup<T: RawStat> {
	#[serde(rename = "opponent")]
	pub opposing_team: NamedTeam,
	#[as_ref] #[as_mut]
	pub game_type: GameType,
	#[as_ref] #[as_mut]
	pub team: NamedTeam,
	
	#[deref]
	#[deref_mut]
	#[serde(rename = "stat")]
	stats: T,
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
