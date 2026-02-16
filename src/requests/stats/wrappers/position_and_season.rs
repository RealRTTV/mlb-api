use derive_more::{Deref, DerefMut};
use serde::Deserialize;
use crate::positions::NamedPosition;
use crate::season::SeasonId;
use crate::stats::{RawStat, SingletonSplitStat};
use crate::stats::wrappers::{PositionPiece, SeasonPiece};

#[derive(Debug, Deserialize, PartialEq, Eq, Clone, Deref, DerefMut)]
#[serde(bound = "T: RawStat")]
pub struct WithPositionAndSeason<T: RawStat> {
	pub position: NamedPosition,
	pub season: SeasonId,

	#[deref]
	#[deref_mut]
	#[serde(rename = "stat")]
	stats: T,
}

impl<T: RawStat> SeasonPiece for WithPositionAndSeason<T> {
	fn season(&self) -> &SeasonId {
		&self.season
	}
}

impl<T: RawStat> PositionPiece for WithPositionAndSeason<T> {
	fn position(&self) -> &NamedPosition {
		&self.position
	}
}

impl<T: RawStat> Default for WithPositionAndSeason<T> {
	fn default() -> Self {
		Self {
			position: NamedPosition::unknown_position(),
			season: SeasonId::current_season(),
			stats: T::default(),
		}
	}
}

impl<T: RawStat> SingletonSplitStat for WithPositionAndSeason<T> {}
	