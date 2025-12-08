pub mod alumni; // done
pub mod coaches; // done
pub mod leaders;
pub mod personnel; // done
pub mod roster; // done
pub mod stats;
pub mod uniforms; // done

use crate::divisions::NamedDivision;
use crate::league::NamedLeague;
use crate::sports::NamedSport;
use crate::venue::NamedVenue;
use derive_more::{Deref, DerefMut, Display, From};
use serde::Deserialize;
use serde_with::DefaultOnError;
use serde_with::serde_as;
use std::ops::{Deref, DerefMut};
use mlb_api_proc::{EnumTryAs, EnumTryAsMut, EnumTryInto};
use crate::seasons::season::SeasonId;

#[serde_as]
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct RegularTeamRaw {
	#[serde(deserialize_with = "crate::types::from_yes_no")]
	#[serde(default)]
	all_star_status: bool,
	active: bool,
	season: SeasonId,
	#[serde_as(deserialize_as = "DefaultOnError")]
	#[serde(default)]
	venue: NamedVenue,
	#[serde(flatten)]
	name: TeamNameRaw,
	location_name: Option<String>,
	#[serde(default, deserialize_with = "crate::types::try_from_str")]
	first_year_of_play: Option<u16>,
	#[serde_as(deserialize_as = "DefaultOnError")]
	#[serde(default)]
	league: NamedLeague,
	division: Option<NamedDivision>,
	sport: NamedSport,
	#[serde(flatten)]
	parent_organization: Option<Organization>,
	#[serde(flatten)]
	inner: NamedTeam,
}

#[derive(Debug, Deserialize, Deref, DerefMut, PartialEq, Eq, Clone)]
#[serde(from = "RegularTeamRaw")]
pub struct RegularTeam {
	pub all_star_status: bool,
	pub active: bool,
	pub season: SeasonId,
	pub venue: NamedVenue,
	pub name: TeamName,
	pub location_name: Option<String>,
	pub first_year_of_play: u16,
	pub league: NamedLeague,
	pub division: Option<NamedDivision>,
	pub sport: NamedSport,
	pub parent_organization: Option<Organization>,

	#[deref]
	#[deref_mut]
	#[serde(flatten)]
	inner: NamedTeam,
}

impl From<RegularTeamRaw> for RegularTeam {
	fn from(value: RegularTeamRaw) -> Self {
		let RegularTeamRaw {
			all_star_status,
			active,
			season,
			venue,
			name,
			location_name,
			first_year_of_play,
			league,
			division,
			sport,
			parent_organization,
			inner,
		} = value;

		RegularTeam {
			all_star_status,
			active,
			season,
			venue,
			name: name.initialize(inner.id),
			location_name,
			first_year_of_play: first_year_of_play.unwrap_or(*season as _),
			league,
			division,
			sport,
			parent_organization,
			inner,
		}
	}
}

#[derive(Debug, Deserialize, Deref, DerefMut, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MLBTeam {
	pub spring_venue: NamedVenue,
	pub spring_league: NamedLeague,

	#[deref]
	#[deref_mut]
	#[serde(flatten)]
	pub inner: RegularTeam,
}

#[derive(Deserialize)]
struct NamedTeamRaw {
	#[serde(flatten)]
	name: TeamNameRaw,

	#[serde(flatten)]
	inner: IdentifiableTeam,
}

impl From<NamedTeamRaw> for NamedTeam {
	fn from(value: NamedTeamRaw) -> Self {
		let NamedTeamRaw { name, inner } = value;
		Self { name: name.initialize(inner.id), inner }
	}
}

#[derive(Debug, Deserialize, Deref, DerefMut, PartialEq, Eq, Clone)]
#[serde(from = "NamedTeamRaw")]
pub struct NamedTeam {
	pub name: TeamName,

	#[deref]
	#[deref_mut]
	inner: IdentifiableTeam,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
pub struct IdentifiableTeam {
	pub id: TeamId,
}

#[derive(Debug, Deserialize, Eq, Clone, From, EnumTryAs, EnumTryAsMut, EnumTryInto)]
#[serde(untagged)]
pub enum Team {
	MLB(MLBTeam),
	Regular(RegularTeam),
	Named(NamedTeam),
	Identifiable(IdentifiableTeam),
}

impl Team {
	#[must_use]
	pub(crate) const fn unknown_team() -> Self {
		Self::Identifiable(IdentifiableTeam { id: TeamId::new(0) })
	}
}

impl PartialEq for Team {
	fn eq(&self, other: &Self) -> bool {
		self.id == other.id
	}
}

impl Deref for Team {
	type Target = IdentifiableTeam;

	fn deref(&self) -> &Self::Target {
		match self {
			Self::MLB(inner) => inner,
			Self::Regular(inner) => inner,
			Self::Named(inner) => inner,
			Self::Identifiable(inner) => inner,
		}
	}
}

impl DerefMut for Team {
	fn deref_mut(&mut self) -> &mut Self::Target {
		match self {
			Self::MLB(inner) => inner,
			Self::Regular(inner) => inner,
			Self::Named(inner) => inner,
			Self::Identifiable(inner) => inner,
		}
	}
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct TeamNameRaw {
	pub name: String,
	pub team_code: Option<String>,
	pub file_code: Option<String>,
	pub abbreviation: Option<String>,
	pub team_name: Option<String>,
	pub short_name: Option<String>,
	pub franchise_name: Option<String>,
	pub club_name: Option<String>,
}

#[derive(Debug, Eq, Clone, From)]
pub enum TeamName {
	Hydrated(HydratedTeamName),
	Unhydrated(UnhydratedTeamName),
}

impl PartialEq for TeamName {
	fn eq(&self, other: &Self) -> bool {
		self.name == other.name
	}
}

impl Deref for TeamName {
	type Target = UnhydratedTeamName;

	fn deref(&self) -> &Self::Target {
		match self {
			Self::Hydrated(inner) => inner,
			Self::Unhydrated(inner) => inner,
		}
	}
}

impl DerefMut for TeamName {
	fn deref_mut(&mut self) -> &mut Self::Target {
		match self {
			Self::Hydrated(inner) => inner,
			Self::Unhydrated(inner) => inner,
		}
	}
}

#[derive(Debug, Deref, DerefMut, PartialEq, Eq, Clone)]
pub struct UnhydratedTeamName {
	pub name: String,
}

#[derive(Debug, Deref, DerefMut, PartialEq, Eq, Clone)]
pub struct HydratedTeamName {
	#[deref]
	#[deref_mut]
	inner: UnhydratedTeamName,

	pub team_code: String,
	pub file_code: String,
	pub abbreviation: String,
	pub team_name: String,
	pub short_name: String,
	pub franchise_name: String,
	pub club_name: String,
}

impl TeamNameRaw {
	fn initialize(self, id: TeamId) -> TeamName {
		let Self {
			name,
			team_code,
			file_code,
			abbreviation,
			team_name,
			short_name,
			franchise_name,
			club_name,
		} = self;

		let inner = UnhydratedTeamName { name };
		if let Some(team_code) = team_code
			&& let Some(abbreviation) = abbreviation
			&& let Some(team_name) = team_name
			&& let Some(short_name) = short_name
		{
			TeamName::Hydrated(HydratedTeamName {
				file_code: file_code.unwrap_or_else(|| format!("t{id}")),
				franchise_name: franchise_name.unwrap_or_else(|| short_name.clone()),
				club_name: club_name.unwrap_or_else(|| team_name.clone()),
				team_code,
				abbreviation,
				team_name,
				short_name,
				inner,
			})
		} else {
			TeamName::Unhydrated(inner)
		}
	}
}

#[repr(transparent)]
#[derive(Debug, Deserialize, Deref, Display, PartialEq, Eq, Copy, Clone, Hash, From)]
pub struct TeamId(u32);

impl TeamId {
	#[must_use]
	pub const fn new(id: u32) -> Self {
		Self(id)
	}
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NamedOrganization {
	#[serde(rename = "parentOrgName")]
	pub name: String,
	#[serde(rename = "parentOrgId")]
	pub id: OrganizationId,
}

#[repr(transparent)]
#[derive(Debug, Deserialize, Deref, Display, PartialEq, Eq, Copy, Clone, Hash, From)]
pub struct OrganizationId(u16);

#[derive(Debug, Deserialize, Eq, Clone, From, EnumTryAs, EnumTryAsMut, EnumTryInto)]
#[serde(untagged)]
pub enum Organization {
	NamedOrganization(NamedOrganization),
}

impl PartialEq for Organization {
	fn eq(&self, other: &Self) -> bool {
		self.id == other.id
	}
}

impl Deref for Organization {
	type Target = NamedOrganization;

	fn deref(&self) -> &Self::Target {
		match self {
			Self::NamedOrganization(inner) => inner,
		}
	}
}

impl DerefMut for Organization {
	fn deref_mut(&mut self) -> &mut Self::Target {
		match self {
			Organization::NamedOrganization(inner) => inner,
		}
	}
}
