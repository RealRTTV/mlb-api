use derive_more::{Deref, DerefMut};
use serde::Deserialize;
use crate::season::SeasonId;
use crate::stats::{RawStat, SingletonSplitStat};
use crate::stats::wrappers::SeasonPiece;

#[derive(Debug, Deserialize, PartialEq, Eq, Clone, Deref, DerefMut)]
#[serde(bound = "T: RawStat")]
pub struct WithSeason<T: RawStat> {
	pub season: SeasonId,

	#[deref]
	#[deref_mut]
	#[serde(rename = "stat")]
	pub stats: T,
}

impl<T: RawStat> SeasonPiece for WithSeason<T> {
	fn season(&self) -> &SeasonId {
		&self.season
	}
}

impl<T: RawStat> Default for WithSeason<T> {
	fn default() -> Self {
		Self {
			season: SeasonId::current_season(),
			stats: T::default(),
		}
	}
}

impl<T: RawStat> SingletonSplitStat for WithSeason<T> {

}
