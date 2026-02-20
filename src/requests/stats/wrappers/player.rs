use derive_more::{Deref, DerefMut};
use serde::Deserialize;
use crate::meta::StandingsType;
use crate::person::NamedPerson;
use crate::season::SeasonId;
use crate::stats::{RawStat, SingletonSplitStat};
use crate::stats::wrappers::SeasonPiece;

#[derive(Debug, Deserialize, PartialEq, Eq, Clone, Deref, DerefMut)]
#[serde(rename_all = "camelCase")]
#[serde(bound = "T: RawStat")]
pub struct WithPlayer<T: RawStat> {
	pub player: NamedPerson,
	pub game_type: StandingsType,
	pub season: SeasonId,

	#[deref]
	#[deref_mut]
	#[serde(rename = "stat")]
	pub stats: T,
}

impl<T: RawStat> SeasonPiece for WithPlayer<T> {
	fn season(&self) -> &SeasonId {
		&self.season
	}
}

impl<T: RawStat> Default for WithPlayer<T> {
	fn default() -> Self {
		Self {
			player: NamedPerson::unknown_person(),
			game_type: StandingsType::default(),
			season: SeasonId::current_season(),
			stats: T::default(),
		}
	}
}

impl<T: RawStat> SingletonSplitStat for WithPlayer<T> {}
