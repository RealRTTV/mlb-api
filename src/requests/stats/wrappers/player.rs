use derive_more::{AsMut, AsRef, Deref, DerefMut};
use serde::Deserialize;
use crate::game_types::GameType;
use crate::person::NamedPerson;
use crate::season::SeasonId;
use crate::stats::{RawStat, SingletonSplitStat};

#[derive(Debug, Deserialize, PartialEq, Eq, Clone, Deref, DerefMut, AsRef, AsMut)]
#[serde(rename_all = "camelCase")]
#[serde(bound = "T: RawStat")]
pub struct WithPlayer<T: RawStat> {
	#[as_ref] #[as_mut]
	pub player: NamedPerson,
	#[as_ref] #[as_mut]
	pub game_type: GameType,
	#[as_ref] #[as_mut]
	pub season: SeasonId,

	#[deref]
	#[deref_mut]
	#[serde(rename = "stat")]
	stats: T,
}

impl<T: RawStat> Default for WithPlayer<T> {
	fn default() -> Self {
		Self {
			player: NamedPerson::unknown_person(),
			game_type: GameType::default(),
			season: SeasonId::current_season(),
			stats: T::default(),
		}
	}
}

impl<T: RawStat> SingletonSplitStat for WithPlayer<T> {}
