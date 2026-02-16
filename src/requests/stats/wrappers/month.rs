use derive_more::{AsMut, AsRef, Deref, DerefMut};
use serde::Deserialize;
use crate::season::SeasonId;
use crate::stats::{RawStat, SingletonSplitStat};

#[derive(Debug, Deserialize, PartialEq, Eq, Clone, Deref, DerefMut, AsRef, AsMut)]
#[serde(rename_all = "camelCase")]
#[serde(bound = "T: RawStat")]
pub struct WithMonth<T: RawStat> {
	#[as_ref] #[as_mut]
	pub month: chrono::Month,
	#[as_ref] #[as_mut]
	pub season: SeasonId,

	#[deref]
	#[deref_mut]
	#[serde(rename = "stat")]
	stats: T,
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
