use chrono::Month;
use derive_more::{Deref, DerefMut};
use serde::Deserialize;
use crate::season::SeasonId;
use crate::stats::{RawStat, SingletonSplitStat};
use crate::stats::wrappers::{MonthPiece, SeasonPiece};

#[derive(Debug, Deserialize, PartialEq, Eq, Clone, Deref, DerefMut)]
#[serde(rename_all = "camelCase")]
#[serde(bound = "T: RawStat")]
pub struct WithMonth<T: RawStat> {
	pub month: chrono::Month,
	pub season: SeasonId,

	#[deref]
	#[deref_mut]
	#[serde(rename = "stat")]
	stats: T,
}

impl<T: RawStat> SeasonPiece for WithMonth<T> {
	fn season(&self) -> &SeasonId {
		&self.season
	}
}

impl<T: RawStat> MonthPiece for WithMonth<T> {
	fn month(&self) -> &Month {
		&self.month
	}
}

impl<T: RawStat> Default for WithMonth<T> {
	fn default() -> Self {
		Self {
			season: SeasonId::current_season(),
			month: chrono::Month::January,

			stats: T::default(),
		}
	}
}

impl<T: RawStat> SingletonSplitStat for WithMonth<T> {}
