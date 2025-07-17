use crate::endpoints::StatsAPIUrl;
use crate::endpoints::league::{IdentifiableLeague, LeagueId};
use crate::endpoints::sports::{IdentifiableSport, SportId};
use crate::gen_params;
use crate::types::Copyright;
use derive_more::{Deref, DerefMut, Display, From};
use serde::Deserialize;
use serde_with::DisplayFromStr;
use serde_with::serde_as;
use std::fmt::{Display, Formatter};
use std::ops::{Deref, DerefMut};

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DivisionsResponse {
	pub copyright: Copyright,
	pub divisions: Vec<HydratedDivision>,
}
#[derive(Debug, Deserialize, PartialEq, Eq, Copy, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IdentifiableDivision {
	pub id: DivisionId,
}

#[derive(Debug, Deserialize, Deref, DerefMut, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NamedDivision {
	pub name: String,

	#[deref]
	#[deref_mut]
	#[serde(flatten)]
	inner: IdentifiableDivision,
}

#[serde_as]
#[derive(Debug, Deserialize, Deref, DerefMut, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HydratedDivision {
	#[serde(rename = "nameShort")]
	pub short_name: String,
	#[serde_as(as = "DisplayFromStr")]
	pub season: u16,
	pub abbreviation: String,
	pub league: IdentifiableLeague,
	pub sport: IdentifiableSport,
	pub has_wildcard: bool,
	pub num_playoff_teams: Option<u8>,
	pub active: bool,

	#[deref]
	#[deref_mut]
	#[serde(flatten)]
	inner: NamedDivision,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone, From)]
#[serde(untagged)]
pub enum Division {
	Hydrated(HydratedDivision),
	Named(NamedDivision),
	Identifiable(IdentifiableDivision),
}

impl Deref for Division {
	type Target = IdentifiableDivision;

	fn deref(&self) -> &Self::Target {
		match self {
			Self::Hydrated(inner) => inner,
			Self::Named(inner) => inner,
			Self::Identifiable(inner) => inner,
		}
	}
}

impl DerefMut for Division {
	fn deref_mut(&mut self) -> &mut Self::Target {
		match self {
			Self::Hydrated(inner) => inner,
			Self::Named(inner) => inner,
			Self::Identifiable(inner) => inner,
		}
	}
}

#[repr(transparent)]
#[derive(Debug, Deserialize, Deref, Display, PartialEq, Eq, Copy, Clone)]
pub struct DivisionId(u32);

#[derive(Default)]
pub struct DivisionsEndpointUrl {
	pub division_id: Option<DivisionId>,
	pub league_id: Option<LeagueId>,
	pub sport_id: Option<SportId>,
	pub season: Option<u16>,
}

impl Display for DivisionsEndpointUrl {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"http://statsapi.mlb.com/api/v1/divisions{}",
			gen_params! { "divisionId"?: self.division_id, "leagueId"?: self.league_id, "sportId"?: self.sport_id, "season"?: self.season }
		)
	}
}

impl StatsAPIUrl for DivisionsEndpointUrl {
	type Response = DivisionsResponse;
}

#[cfg(test)]
mod tests {
	use crate::endpoints::StatsAPIUrl;
	use crate::endpoints::divisions::DivisionsEndpointUrl;

	#[tokio::test]
	async fn all_divisions_this_season() {
		let _response = DivisionsEndpointUrl::default().get().await.unwrap();
	}
}
