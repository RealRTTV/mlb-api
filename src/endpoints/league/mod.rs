pub mod all_star_ballot;
pub mod all_star_final_vote;
pub mod all_star_write_ins;

use std::fmt::{Display, Formatter};
use crate::endpoints::seasons::season::{Season, SeasonState};
use crate::endpoints::sports::{NamedSport, SportId};
use derive_more::{Deref, DerefMut, Display, From};
use serde::Deserialize;
use std::ops::{Deref, DerefMut};
use itertools::Itertools;
use strum::EnumTryAs;
use crate::endpoints::StatsAPIUrl;
use crate::{gen_params, rwlock_const_new, RwLock};
use crate::cache::{EndpointEntryCache, HydratedCacheTable};

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LeagueResponse {
	pub copyright: String,
	pub leagues: Vec<League>,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Copy, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IdentifiableLeague {
	pub id: LeagueId,
}

impl Default for IdentifiableLeague {
	fn default() -> Self {
		Self { id: LeagueId(0) }
	}
}

#[derive(Debug, Deserialize, Deref, DerefMut, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NamedLeague {
	pub name: String,

	#[deref]
	#[deref_mut]
	#[serde(flatten)]
	inner: IdentifiableLeague,
}

impl Default for NamedLeague {
	fn default() -> Self {
		Self {
			name: "null".to_owned(),
			inner: IdentifiableLeague::default(),
		}
	}
}

#[derive(Debug, Deserialize, Deref, DerefMut, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HydratedLeague {
	pub abbreviation: String,
	#[serde(rename = "nameShort")]
	pub short_name: Option<String>,
	#[serde(rename = "orgCode")]
	pub code: String,
	pub season_state: SeasonState,
	#[serde(flatten, deserialize_with = "bad_league_season_schema_deserializer")]
	#[serde(rename = "seasonDateInfo")]
	pub season: Season,
	#[serde(default)]
	pub has_split_season: bool,
	pub num_games: u8,
	pub has_playoff_points: Option<bool>,
	pub num_teams: u8,
	pub num_wildcard_teams: Option<u8>,
	#[serde(rename = "conferencesInUse")]
	pub has_conferences: bool,
	#[serde(rename = "divisionsInUse")]
	pub has_divisions: bool,
	pub sport: Option<NamedSport>,
	pub active: bool,

	#[deref]
	#[deref_mut]
	#[serde(flatten)]
	inner: NamedLeague,
}

// this is annoying me that it exists
#[derive(Deserialize)]
struct BadLeagueSeasonSchema {
	#[serde(rename = "hasWildCard")]
	has_wildcard: bool,
	#[serde(rename = "seasonDateInfo")]
	rest: Season,
}

fn bad_league_season_schema_deserializer<'de, D: serde::Deserializer<'de>>(deserializer: D) -> Result<Season, D::Error> {
	let BadLeagueSeasonSchema { has_wildcard, mut rest } = BadLeagueSeasonSchema::deserialize(deserializer)?;
	rest.has_wildcard = has_wildcard;
	Ok(rest)
}

#[repr(transparent)]
#[derive(Debug, Deserialize, Deref, Display, PartialEq, Eq, Copy, Clone, Hash)]
pub struct LeagueId(u32);

impl LeagueId {
	#[must_use]
	pub const fn new(id: u32) -> Self {
		Self(id)
	}
}

#[derive(Debug, Deserialize, Eq, Clone, From, EnumTryAs)]
#[serde(untagged)]
pub enum League {
	Hydrated(HydratedLeague),
	Named(NamedLeague),
	Identifiable(IdentifiableLeague),
}

impl PartialEq for League {
	fn eq(&self, other: &Self) -> bool {
		self.id == other.id
	}
}

impl Deref for League {
	type Target = IdentifiableLeague;

	fn deref(&self) -> &Self::Target {
		match self {
			Self::Hydrated(inner) => inner,
			Self::Named(inner) => inner,
			Self::Identifiable(inner) => inner,
		}
	}
}

impl DerefMut for League {
	fn deref_mut(&mut self) -> &mut Self::Target {
		match self {
			Self::Hydrated(inner) => inner,
			Self::Named(inner) => inner,
			Self::Identifiable(inner) => inner,
		}
	}
}

pub struct LeagueEndpointUrl {
	pub sport_id: Option<SportId>,
	pub league_ids: Option<Vec<LeagueId>>,
}

impl Display for LeagueEndpointUrl {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "http://statsapi.mlb.com/api/v1/leagues{}", gen_params! {
			"sportId"?: self.sport_id,
			"leagueIds"?: self.league_ids.as_ref().map(|ids| ids.iter().copied().join(",")),
		})
	}
}

impl StatsAPIUrl for LeagueEndpointUrl {
	type Response = LeagueResponse;
}

static CACHE: RwLock<HydratedCacheTable<League>> = rwlock_const_new(HydratedCacheTable::new());

impl EndpointEntryCache for League {
	type HydratedVariant = HydratedLeague;
	type Identifier = LeagueId;
	type URL = LeagueEndpointUrl;

	fn into_hydrated_variant(self) -> Option<Self::HydratedVariant> {
		self.try_as_hydrated()
	}

	fn id(&self) -> &Self::Identifier {
		&self.id
	}

	fn url_for_id(id: &Self::Identifier) -> Self::URL {
		LeagueEndpointUrl {
			sport_id: None,
			league_ids: Some(vec![id.clone()]),
		}
	}

	fn get_entries(response: <Self::URL as StatsAPIUrl>::Response) -> impl IntoIterator<Item=Self>
	where
		Self: Sized
	{
		response.leagues
	}

	fn get_hydrated_cache_table() -> &'static RwLock<HydratedCacheTable<Self>>
	where
		Self: Sized
	{
		&CACHE
	}
}
