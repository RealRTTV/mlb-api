use std::fmt::Formatter;
use derive_more::{AsMut, AsRef, Deref, DerefMut};
use serde::{Deserialize, Deserializer};
use serde::de::{Error, Visitor};
use crate::season::SeasonId;
use crate::stats::{RawStat, SingletonSplitStat};

#[derive(Debug, Deserialize, PartialEq, Eq, Clone, Deref, DerefMut, AsRef, AsMut)]
#[serde(rename_all = "camelCase")]
#[serde(bound = "T: RawStat")]
pub struct WithWeekday<T: RawStat> {
	#[as_ref] #[as_mut]
	#[serde(deserialize_with = "deserialize_day_of_week")]
	pub weekday: chrono::Weekday,
	#[as_ref] #[as_mut]
	pub season: SeasonId,

	#[deref]
	#[deref_mut]
	#[serde(rename = "stat")]
	stats: T,
}

impl<T: RawStat> Default for WithWeekday<T> {
	fn default() -> Self {
		Self {
			weekday: chrono::Weekday::Mon,
			season: SeasonId::current_season(),
			stats: T::default(),
		}
	}
}

impl<T: RawStat> SingletonSplitStat for WithWeekday<T> {}

fn deserialize_day_of_week<'de, D: Deserializer<'de>>(deserializer: D) -> Result<chrono::Weekday, D::Error> {
	struct WeekdayVisitor;

	impl Visitor<'_> for WeekdayVisitor {
		type Value = chrono::Weekday;

		fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
			formatter.write_str("an integer between 0 and 6 representing the day of the week")
		}

		#[allow(clippy::cast_sign_loss, reason = "needlessly pedantic")]
		fn visit_i8<E>(self, v: i8) -> Result<Self::Value, E>
		where
			E: Error,
		{
			chrono::Weekday::try_from(v as u8 - 1).map_err(E::custom)
		}
	}

	deserializer.deserialize_any(WeekdayVisitor)
}
