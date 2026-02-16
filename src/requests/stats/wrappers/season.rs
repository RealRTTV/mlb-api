use derive_more::{AsMut, AsRef, Deref, DerefMut};
use serde::Deserialize;
use crate::season::SeasonId;
use crate::stats::{RawStat, SingletonSplitStat};

#[derive(Debug, Deserialize, PartialEq, Eq, Clone, Deref, DerefMut, AsRef, AsMut)]
#[serde(bound = "T: RawStat")]
pub struct WithSeason<T: RawStat> {
	#[as_ref] #[as_mut]
	pub season: SeasonId,

	#[deref]
	#[deref_mut]
	#[serde(rename = "stat")]
	stats: T,
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
