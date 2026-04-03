use crate::person::NamedPerson;
use crate::stats::SingletonSplitStat;
use crate::team::NamedTeam;
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FieldedMatchup {
	pub pitcher: NamedPerson,
	pub batter: NamedPerson,
	pub fielding_team: Option<NamedTeam>,
}

impl Default for FieldedMatchup {
	fn default() -> Self {
		Self {
			pitcher: NamedPerson::unknown_person(),
			batter: NamedPerson::unknown_person(),
			fielding_team: None,
		}
	}
}

impl SingletonSplitStat for FieldedMatchup {}
