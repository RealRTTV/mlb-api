use chrono::Month;
use derive_more::{Deref, DerefMut};
use serde::{Deserialize, Deserializer};
use serde::de::Error;
use crate::season::SeasonId;
use crate::stats::{RawStat, SingletonSplitStat};
use crate::stats::wrappers::{MonthPiece, SeasonPiece};

#[derive(Debug, Deserialize, PartialEq, Eq, Clone, Deref, DerefMut)]
#[serde(rename_all = "camelCase")]
#[serde(bound = "T: RawStat")]
pub struct WithMonth<T: RawStat> {
	#[serde(deserialize_with = "deserialize_month")]
	pub month: Month,
	pub season: SeasonId,

	#[deref]
	#[deref_mut]
	#[serde(rename = "stat")]
	stats: T,
}

fn deserialize_month<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Month, D::Error> {
	Month::try_from(u8::deserialize(deserializer)?).map_err(D::Error::custom)
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
			month: Month::January,

			stats: T::default(),
		}
	}
}

impl<T: RawStat> SingletonSplitStat for WithMonth<T> {}
