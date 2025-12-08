use crate::endpoints::StatsAPIEndpointUrl;
use crate::endpoints::league::{IdentifiableLeague, LeagueId};
use crate::endpoints::sports::{IdentifiableSport, SportId};
use crate::{gen_params, rwlock_const_new, RwLock};
use crate::types::Copyright;
use derive_more::{Deref, DerefMut, Display, From};
use serde::Deserialize;
use serde_with::DisplayFromStr;
use serde_with::serde_as;
use std::fmt::{Display, Formatter};
use std::ops::{Deref, DerefMut};
use mlb_api_proc::{EnumTryAs, EnumTryAsMut, EnumTryInto};
use crate::cache::{EndpointEntryCache, HydratedCacheTable};

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DivisionsResponse {
	pub copyright: Copyright,
	pub divisions: Vec<Division>,
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

#[derive(Debug, Deserialize, Eq, Clone, From, EnumTryAs, EnumTryAsMut, EnumTryInto)]
#[serde(untagged)]
pub enum Division {
	Hydrated(HydratedDivision),
	Named(NamedDivision),
	Identifiable(IdentifiableDivision),
}

impl PartialEq for Division {
	fn eq(&self, other: &Self) -> bool {
		self.id == other.id
	}
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
#[derive(Debug, Deserialize, Deref, Display, PartialEq, Eq, Copy, Clone, Hash, From)]
pub struct DivisionId(u32);

#[derive(Default)]
pub struct DivisionsEndpoint {
	pub division_id: Option<DivisionId>,
	pub league_id: Option<LeagueId>,
	pub sport_id: Option<SportId>,
	pub season: Option<u16>,
}

impl Display for DivisionsEndpoint {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"http://statsapi.mlb.com/api/v1/divisions{}",
			gen_params! { "divisionId"?: self.division_id, "leagueId"?: self.league_id, "sportId"?: self.sport_id, "season"?: self.season }
		)
	}
}

impl StatsAPIEndpointUrl for DivisionsEndpoint {
	type Response = DivisionsResponse;
}

static CACHE: RwLock<HydratedCacheTable<Division>> = rwlock_const_new(HydratedCacheTable::new());

impl EndpointEntryCache for Division {
	type HydratedVariant = HydratedDivision;
	type Identifier = DivisionId;
	type URL = DivisionsEndpoint;

	fn into_hydrated_variant(self) -> Option<Self::HydratedVariant> {
		self.try_into_hydrated()
	}

	fn id(&self) -> &Self::Identifier {
		&self.id
	}

	fn url_for_id(id: &Self::Identifier) -> Self::URL {
		DivisionsEndpoint {
			division_id: Some(id.clone()),
			league_id: None,
			sport_id: None,
			season: None,
		}
	}

	fn get_entries(response: <Self::URL as StatsAPIEndpointUrl>::Response) -> impl IntoIterator<Item=Self>
	where
		Self: Sized
	{
		response.divisions
	}

	fn get_hydrated_cache_table() -> &'static RwLock<HydratedCacheTable<Self>>
	where
		Self: Sized
	{
		&CACHE
	}
}

#[cfg(test)]
mod tests {
	use crate::endpoints::StatsAPIEndpointUrl;
	use crate::endpoints::divisions::DivisionsEndpoint;

	#[tokio::test]
	async fn all_divisions_this_season() {
		let _response = DivisionsEndpoint::default().get().await.unwrap();
	}
}
