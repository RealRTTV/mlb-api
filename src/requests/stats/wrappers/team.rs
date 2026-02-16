use derive_more::{AsMut, AsRef, Deref, DerefMut};
use serde::Deserialize;
use crate::stats::{RawStat, SingletonSplitStat};
use crate::team::NamedTeam;

#[derive(Debug, Deserialize, PartialEq, Eq, Clone, Deref, DerefMut, AsRef, AsMut)]
#[serde(bound = "T: RawStat")]
pub struct WithTeam<T: RawStat> {
	#[as_ref] #[as_mut]
	pub team: NamedTeam,
	
	#[deref]
	#[deref_mut]
	#[serde(rename = "stat")]
	stats: T,
}

impl<T: RawStat> Default for WithTeam<T> {
	fn default() -> Self {
		Self {
			stats: T::default(),
			team: NamedTeam::unknown_team(),
		}
	}
}


impl<T: RawStat> SingletonSplitStat for WithTeam<T> {}
