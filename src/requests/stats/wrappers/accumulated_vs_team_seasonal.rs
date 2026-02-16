use derive_more::{AsMut, AsRef, Deref, DerefMut};
use serde::Deserialize;
use crate::game_types::GameType;
use crate::person::NamedPerson;
use crate::season::SeasonId;
use crate::stats::{RawStat, SingletonSplitStat};
use crate::team::NamedTeam;

#[derive(Debug, Deserialize, PartialEq, Eq, Clone, Deref, DerefMut, AsRef, AsMut)]
#[serde(rename_all = "camelCase")]
#[serde(bound = "T: RawStat")]
pub struct AccumulatedVsTeamSeasonalPitcherSplit<T: RawStat> {
	pub pitcher: NamedPerson,
	pub batter: NamedPerson,
	
	#[serde(rename = "opponent")]
	pub opposing_team: NamedTeam,
	#[as_ref] #[as_mut]
	pub game_type: GameType,
	#[as_ref] #[as_mut]
	pub team: NamedTeam,
	#[as_ref] #[as_mut]
	pub season: SeasonId,
	
	#[deref]
	#[deref_mut]
	#[serde(rename = "stat")]
	stats: T,
}

impl<T: RawStat> Default for AccumulatedVsTeamSeasonalPitcherSplit<T> {
	fn default() -> Self {
		Self {
			pitcher: NamedPerson::unknown_person(),
			batter: NamedPerson::unknown_person(),
			opposing_team: NamedTeam::unknown_team(),
			game_type: GameType::default(),
			team: NamedTeam::unknown_team(),
			season: SeasonId::current_season(),
			
			stats: T::default(),
		}
	}
}

impl<T: RawStat> SingletonSplitStat for AccumulatedVsTeamSeasonalPitcherSplit<T> {}
