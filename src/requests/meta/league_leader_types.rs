use derive_more::From;
use mlb_api_proc::{EnumDeref, EnumDerefMut, EnumTryAs, EnumTryAsMut, EnumTryInto};
use serde::Deserialize;

string_id!(LeagueLeaderTypeId);

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
pub struct IdentifiableLeagueLeaderType {
	#[serde(rename = "displayName")]
	pub id: LeagueLeaderTypeId,
}

#[derive(Debug, Deserialize, Eq, Clone, From, EnumTryAs, EnumTryAsMut, EnumTryInto, EnumDeref, EnumDerefMut)]
#[serde(untagged)]
pub enum LeagueLeaderType {
	Identifiable(IdentifiableLeagueLeaderType),
}

impl LeagueLeaderType {
	#[must_use]
	pub fn try_into_hydrated(self) -> Option<IdentifiableLeagueLeaderType> {
		self.try_into_identifiable()
	}
}

id_only_eq_impl!(LeagueLeaderType, id);
meta_kind_impl!("leagueLeaderTypes" => LeagueLeaderType);
tiered_request_entry_cache_impl!(LeagueLeaderType => IdentifiableLeagueLeaderType; id: LeagueLeaderTypeId);
test_impl!(LeagueLeaderType);
