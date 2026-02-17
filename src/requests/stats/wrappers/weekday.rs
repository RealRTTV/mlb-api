use chrono::Weekday;
use derive_more::{Deref, DerefMut};
use serde::{Deserialize, Deserializer};
use serde::de::Error;
use crate::season::SeasonId;
use crate::stats::{RawStat, SingletonSplitStat};
use crate::stats::wrappers::{SeasonPiece, WeekdayPiece};

#[derive(Debug, Deserialize, PartialEq, Eq, Clone, Deref, DerefMut)]
#[serde(rename_all = "camelCase")]
#[serde(bound = "T: RawStat")]
pub struct WithWeekday<T: RawStat> {
	#[serde(deserialize_with = "deserialize_day_of_week", rename = "dayOfWeek")]
	pub weekday: Weekday,
	pub season: SeasonId,

	#[deref]
	#[deref_mut]
	#[serde(rename = "stat")]
	stats: T,
}

impl<T: RawStat> SeasonPiece for WithWeekday<T> {
	fn season(&self) -> &SeasonId {
		&self.season
	}
}

impl<T: RawStat> WeekdayPiece for WithWeekday<T> {
	fn weekday(&self) -> &Weekday {
		&self.weekday
	}
}

impl<T: RawStat> Default for WithWeekday<T> {
	fn default() -> Self {
		Self {
			weekday: Weekday::Mon,
			season: SeasonId::current_season(),
			stats: T::default(),
		}
	}
}

impl<T: RawStat> SingletonSplitStat for WithWeekday<T> {}

fn deserialize_day_of_week<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Weekday, D::Error> {
	Weekday::try_from(u8::deserialize(deserializer)? - 1).map_err(D::Error::custom)
}
