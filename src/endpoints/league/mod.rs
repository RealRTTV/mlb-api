pub mod all_star_ballot;
pub mod all_star_final_vote;
pub mod all_star_write_ins;

use crate::endpoints::seasons::season::{Season, SeasonState};
use crate::endpoints::sports::NamedSport;
use derive_more::{Deref, DerefMut, Display, From};
use serde::Deserialize;
use std::ops::{Deref, DerefMut};
use strum::EnumTryAs;

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
#[derive(Debug, Deserialize, Deref, Display, PartialEq, Eq, Copy, Clone)]
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
