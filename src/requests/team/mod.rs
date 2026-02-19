///! Numerous endpoints for teams; [`roster`], [`history`], [`affiliates`], etc.

pub mod alumni;
pub mod coaches;
pub mod leaders;
pub mod personnel;
pub mod roster;
pub mod stats;
pub mod uniforms;
pub mod history;
pub mod affiliates;
pub mod teams;

use std::hash::{Hash, Hasher};
use serde_with::DefaultOnError;
use crate::divisions::NamedDivision;
use crate::league::{LeagueId, NamedLeague};
use crate::season::SeasonId;
use crate::venue::{NamedVenue, VenueId};
use derive_more::{Deref, DerefMut};
use serde::Deserialize;
use serde_with::serde_as;
use crate::sport::SportId;

#[serde_as]
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct TeamRaw {
	#[serde(default)]
	all_star_status: AllStarStatus,
	active: bool,
	season: u32,
	#[serde(default)]
	venue: Option<NamedVenue>,
	location_name: Option<String>,
	#[serde(default, deserialize_with = "crate::try_from_str")]
	first_year_of_play: Option<u32>,
	#[serde(default)]
	#[serde_as(deserialize_as = "DefaultOnError")]
	league: Option<NamedLeague>,
	#[serde(default)]
	#[serde_as(deserialize_as = "DefaultOnError")]
	division: Option<NamedDivision>,
	sport: SportId,
	#[serde(flatten)]
	parent_organization: Option<NamedOrganization>,
	#[serde(flatten)]
	name: TeamNameRaw,
	spring_venue: Option<VenueId>,
	spring_league: Option<LeagueId>,
	#[serde(flatten)]
	inner: NamedTeam,
}

#[derive(Debug, Deserialize, Deref, DerefMut, Clone)]
#[serde(from = "TeamRaw")]
pub struct Team {
	pub all_star_status: AllStarStatus,
	pub active: bool,
	pub season: SeasonId,
	pub venue: NamedVenue,
	pub location_name: Option<String>,
	pub first_year_of_play: SeasonId,
	pub league: NamedLeague,
	pub division: Option<NamedDivision>,
	pub sport: SportId,
	pub parent_organization: Option<NamedOrganization>,
	pub name: TeamName,
	pub spring_venue: Option<VenueId>,
	pub spring_league: Option<LeagueId>,

	#[deref]
	#[deref_mut]
	#[serde(flatten)]
	inner: NamedTeam,
}

impl From<TeamRaw> for Team {
	fn from(value: TeamRaw) -> Self {
		let TeamRaw {
			all_star_status,
			active,
			season,
			venue,
			location_name,
			first_year_of_play,
			league,
			division,
			sport,
			parent_organization,
			name,
			spring_venue,
			spring_league,
			inner,
		} = value;

		Self {
			all_star_status,
			active,
			season: SeasonId::new(season),
			venue: venue.unwrap_or_else(NamedVenue::unknown_venue),
			location_name,
			first_year_of_play: first_year_of_play.unwrap_or(season).into(),
			league: league.unwrap_or_else(NamedLeague::unknown_league),
			division,
			sport,
			parent_organization,
			spring_venue,
			spring_league,
			name: name.initialize(inner.id, inner.full_name.clone()),
			inner,
		}
	}
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NamedTeam {
	#[serde(alias = "name")]
	pub full_name: String,
	#[serde(flatten)]
	pub id: TeamId,
}


impl Hash for NamedTeam {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.id.hash(state);
	}
}

impl NamedTeam {
	#[must_use]
	pub(crate) fn unknown_team() -> Self {
		Self {
			full_name: "null".to_owned(),
			id: TeamId::new(0),
		}
	}

	#[must_use]
	pub fn is_unknown(&self) -> bool {
		*self.id == 0
	}
}

id_only_eq_impl!(NamedTeam, id);
id_only_eq_impl!(Team, id);

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct TeamNameRaw {
	pub team_code: String,
	pub abbreviation: String,
	pub team_name: String,
	pub short_name: String,
	#[serde(default)]
	pub file_code: Option<String>,
	#[serde(default)]
	pub franchise_name: Option<String>,
	#[serde(default)]
	pub club_name: Option<String>,
}

#[derive(Debug, PartialEq, Eq, Deref, DerefMut, Clone)]
pub struct TeamName {
	pub team_code: String,
	pub file_code: String,
	pub abbreviation: String,
	pub team_name: String,
	pub short_name: String,
	pub franchise_name: String,
	pub club_name: String,
	#[deref]
	#[deref_mut]
	pub full_name: String,
}

impl TeamNameRaw {
	fn initialize(self, id: TeamId, full_name: String) -> TeamName {
		let Self {
			team_code,
			abbreviation,
			team_name,
			short_name,
			file_code,
			franchise_name,
			club_name,
		} = self;


		TeamName {
			file_code: file_code.unwrap_or_else(|| format!("t{id}")),
			franchise_name: franchise_name.unwrap_or_else(|| short_name.clone()),
			club_name: club_name.unwrap_or_else(|| team_name.clone()),
			team_code,
			abbreviation,
			team_name,
			short_name,
			full_name,
		}
	}
}

id!(TeamId { id: u32 });

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NamedOrganization {
	#[serde(rename = "parentOrgName")]
	pub name: String,
	#[serde(rename = "parentOrgId")]
	pub id: OrganizationId,
}

id!(OrganizationId { id: u32 });

#[derive(Debug, Deserialize, PartialEq, Eq, Copy, Clone, Default)]
pub enum AllStarStatus {
	#[serde(rename = "Y")]
	Yes,
	#[default]
	#[serde(rename = "N")]
	No,
	#[serde(rename = "F")]
	F,
	#[serde(rename = "O")]
	O,
}
