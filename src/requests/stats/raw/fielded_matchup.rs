use serde::Deserialize;
use crate::person::NamedPerson;
use crate::stats::RawStat;
use crate::team::NamedTeam;

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FieldedMatchup {
	pub pitcher: NamedPerson,
	pub batter: NamedPerson,
	pub fielding_team: NamedTeam,
}

impl Default for FieldedMatchup {
	fn default() -> Self {
		Self {
			pitcher: NamedPerson::unknown_person(),
			batter: NamedPerson::unknown_person(),
			fielding_team: NamedTeam::unknown_team(),
		}
	}
}

impl RawStat for FieldedMatchup {}
