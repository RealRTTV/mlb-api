use chrono::{NaiveDate, Utc};
use derive_more::{Deref, DerefMut};
use serde::Deserialize;
use crate::game::GameId;
use crate::person::NamedPerson;
use crate::season::SeasonId;
use crate::stats::{RawStat, SingletonSplitStat};
use crate::stats::wrappers::SeasonPiece;
use crate::team::NamedTeam;

#[derive(Debug, Deserialize, PartialEq, Eq, Clone, Deref, DerefMut)]
#[serde(rename_all = "camelCase")]
#[serde(bound = "T: RawStat")]
pub struct SingleMatchup<T: RawStat> {
	pub pitcher: NamedPerson,
	pub batter: NamedPerson,

	pub opponent: NamedTeam,
	pub date: NaiveDate,
	pub is_home: bool,
	pub game: GameId,
	pub season: SeasonId,

	#[deref]
	#[deref_mut]
	#[serde(rename = "stat")]
	pub stats: T,
}

impl<T: RawStat> SeasonPiece for SingleMatchup<T> {
	fn season(&self) -> &SeasonId {
		&self.season
	}
}

impl<T: RawStat> Default for SingleMatchup<T> {
	fn default() -> Self {
		Self {
			pitcher: NamedPerson::unknown_person(),
			batter: NamedPerson::unknown_person(),

			opponent: NamedTeam::unknown_team(),
			date: Utc::now().date_naive(),
			is_home: true,
			game: GameId::new(0),
			season: SeasonId::current_season(),			
			
			stats: T::default(),
		}
	}
}

impl<T: RawStat> SingletonSplitStat for SingleMatchup<T> {}
