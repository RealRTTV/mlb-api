use chrono::{NaiveDate, Utc};
use derive_more::{AsMut, AsRef, Deref, DerefMut};
use serde::Deserialize;
use crate::game::GameId;
use crate::season::SeasonId;
use crate::stats::{RawStat, SingletonSplitStat};
use crate::team::NamedTeam;

#[derive(Debug, Deserialize, PartialEq, Eq, Clone, Deref, DerefMut, AsRef, AsMut)]
#[serde(rename_all = "camelCase")]
#[serde(bound = "T: RawStat")]
pub struct WithGame<T: RawStat> {
	pub opponent: NamedTeam,
	pub date: NaiveDate,
	pub is_home: bool,
	#[as_ref] #[as_mut]
	pub game: GameId,
	#[as_ref] #[as_mut]
	pub season: SeasonId,

	#[deref]
	#[deref_mut]
	#[serde(rename = "stat")]
	stats: T,
}

impl<T: RawStat> Default for WithGame<T> {
	fn default() -> Self {
		Self {
			opponent: NamedTeam::unknown_team(),
			date: Utc::now().date_naive(),
			is_home: true,
			game: GameId::new(0),
			season: SeasonId::current_season(),
			stats: T::default(),
		}
	}
}

impl<T: RawStat> SingletonSplitStat for WithGame<T> {}
