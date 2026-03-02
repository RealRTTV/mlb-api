//! Endpoints related to teams; [`roster`], [`history`], [`affiliates`], etc.

pub mod alumni;
pub mod coaches;
pub mod leaders;
pub mod personnel;
pub mod roster;
pub mod stats;
pub mod uniforms;
pub mod history;
pub mod affiliates;
// pub mod teams;

use std::fmt::{Debug, Display, Formatter};
use std::hash::{Hash, Hasher};
use bon::Builder;
use serde_with::DefaultOnError;
use crate::divisions::NamedDivision;
use crate::league::{LeagueId, NamedLeague};
use crate::season::SeasonId;
use crate::venue::{NamedVenue, VenueId};
use derive_more::{Deref, DerefMut};
use serde::de::DeserializeOwned;
use serde::Deserialize;
use serde_with::serde_as;
use crate::Copyright;
use crate::hydrations::Hydrations;
use crate::request::RequestURL;
use crate::sport::SportId;

#[serde_as]
#[derive(Deserialize)]
#[serde(rename_all = "camelCase", bound = "H: TeamHydrations")]
struct __TeamRaw<H: TeamHydrations> {
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
	sport: H::Sport,
	#[serde(flatten)]
	parent_organization: Option<NamedOrganization>,
	#[serde(flatten)]
	name: __TeamNameRaw,
	spring_venue: Option<VenueId>,
	spring_league: Option<LeagueId>,
	#[serde(flatten)]
	inner: NamedTeam,
	#[serde(flatten)]
	extras: H,
}

/// A detailed `struct` representing a baseball team.
///
/// ## Examples
/// ```
/// Team {
///     all_star_status: AllStarStatus::Yes,
///     active: true,
///     season: 2025,
///     venue: NamedVenue { name: "Rogers Centre", id: 14 },
///     location_name: Some("Toronto"),
///     first_year_of_play: 1977,
///     league: NamedLeague { name: "American League", id: 103 },
///     division: Some(NamedDivision { name: "American League East", id: 201 }),
///     sport: SportId::MLB,
///     parent_organization: None,
///     name: TeamName {
///         team_code: "tor",
///         file_code: "tor",
///         abbreviation: "TOR",
///         team_name: "Blue Jays",
///         short_name: "Toronto",
///         franchise_name: "Toronto",
///         club_name: "Blue Jays",
///         full_name: "Toronto Blue Jays",
///     },
///     spring_venue: Some(VenueId::new(2536)),
///     spring_league: Some(LeagueId::new(115)),
///     id: 141,
/// }
/// ```
#[derive(Debug, Deserialize, Deref, DerefMut, Clone)]
#[serde(from = "__TeamRaw<H>", bound = "H: TeamHydrations")]
pub struct Team<H: TeamHydrations> {
	pub all_star_status: AllStarStatus,
	pub active: bool,
	pub season: SeasonId,
	pub venue: NamedVenue,
	pub location_name: Option<String>,
	pub first_year_of_play: SeasonId,
	pub league: NamedLeague,
	pub division: Option<NamedDivision>,
	pub sport: H::Sport,
	pub parent_organization: Option<NamedOrganization>,
	pub name: TeamName,
	pub spring_venue: Option<VenueId>,
	pub spring_league: Option<LeagueId>,

	#[deref]
	#[deref_mut]
	#[serde(flatten)]
	inner: NamedTeam,

	pub extras: H,
}

impl<H: TeamHydrations> From<__TeamRaw<H>> for Team<H> {
	fn from(value: __TeamRaw<H>) -> Self {
		let __TeamRaw {
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
			extras,
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
			extras,
		}
	}
}

/// A team with a name and [id](TeamId)
/// 
/// ## Examples
/// ```
/// use mlb_api::team::NamedTeam;
///
/// NamedTeam {
///     full_name: "Toronto Blue Jays".into(),
///     id: 141.into(),
/// }
/// ```
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

impl<H: TeamHydrations> PartialEq for Team<H> {
	fn eq(&self, other: &Self) -> bool {
		self.id == other.id
	}
}

impl<H: TeamHydrations> Eq for Team<H> {}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct __TeamNameRaw {
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

/// A detailed description of a team's name.
///
/// ## Table of MLB [`TeamName`] data
/// | `full_name`           | `team_code` | `file_code` | `abbreviation` |`team_name`| `club_name`  |`franchise_name`| `short_name`  |
/// |-----------------------|-------------|-------------|----------------|-----------|--------------|----------------|---------------|
/// | Athletics             | `ath`       | `ath`       | `ATH`          | Athletics | Athletics    | Athletics      | Athletics     |
/// | Pittsburgh Pirates    | `pit`       | `pit`       | `PIT`          | Pirates   | Pirates      | Pittsburgh     | Pittsburgh    |
/// | San Diego Padres      | `sdn`       | `sd`        | `SD`           | Padres    | Padres       | San Diego      | San Diego     |
/// | Seattle Mariners      | `sea`       | `sea`       | `SEA`          | Mariners  | Mariners     | Seattle        | Seattle       |
/// | San Francisco Giants  | `sfn`       | `sf`        | `SF`           | Giants    | Giants       | San Francisco  | San Francisco |
/// | St. Louis Cardinals   | `sln`       | `stl`       | `STL`          | Cardinals | Cardinals    | St. Louis      | St. Louis     |
/// | Tampa Bay Rays        | `tba`       | `tb`        | `TB`           | Rays      | Rays         | Tampa Bay      | Tampa Bay     |
/// | Texas Rangers         | `tex`       | `tex`       | `TEX`          | Rangers   | Rangers      | Texas          | Texas         |
/// | Toronto Blue Jays     | `tor`       | `tor`       | `TOR`          | Blue Jays | Blue Jays    | Toronto        | Toronto       |
/// | Minnesota Twins       | `min`       | `min`       | `MIN`          | Twins     | Twins        | Minnesota      | Minnesota     |
/// | Philadelphia Phillies | `phi`       | `phi`       | `PHI`          | Phillies  | Phillies     | Philadelphia   | Philadelphia  |
/// | Atlanta Braves        | `atl`       | `atl`       | `ATL`          | Braves    | Braves       | Atlanta        | Atlanta       |
/// | Chicago White Sox     | `cha`       | `cws`       | `CWS`          | White Sox | White Sox    | Chicago        | Chi White Sox |
/// | Miami Marlins         | `mia`       | `mia`       | `MIA`          | Marlins   | Marlins      | Miami          | Miami         |
/// | New York Yankees      | `nya`       | `nyy`       | `NYY`          | Yankees   | Yankees      | New York       | NY Yankees    |
/// | Milwaukee Brewers     | `mil`       | `mil`       | `MIL`          | Brewers   | Brewers      | Milwaukee      | Milwaukee     |
/// | Los Angeles Angels    | `ana`       | `ana`       | `LAA`          | Angels    | Angels       | Los Angeles    | LA Angels     |
/// | Arizona Diamondbacks  | `ari`       | `ari`       | `AZ`           | D-backs   | Diamondbacks | Arizona        | Arizona       |
/// | Baltimore Orioles     | `bal`       | `bal`       | `BAL`          | Orioles   | Orioles      | Baltimore      | Baltimore     |
/// | Boston Red Sox        | `bos`       | `bos`       | `BOS`          | Red Sox   | Red Sox      | Boston         | Boston        |
/// | Chicago Cubs          | `chn`       | `chc`       | `CHC`          | Cubs      | Cubs         | Chicago        | Chi Cubs      |
/// | Cincinnati Reds       | `cin`       | `cin`       | `CIN`          | Reds      | Reds         | Cincinnati     | Cincinnati    |
/// | Cleveland Guardians   | `cle`       | `cle`       | `CLE`          | Guardians | Guardians    | Cleveland      | Cleveland     |
/// | Colorado Rockies      | `col`       | `col`       | `COL`          | Rockies   | Rockies      | Colorado       | Colorado      |
/// | Detroit Tigers        | `det`       | `det`       | `DET`          | Tigers    | Tigers       | Detroit        | Detroit       |
/// | Houston Astros        | `hou`       | `hou`       | `HOU`          | Astros    | Astros       | Houston        | Houston       |
/// | Kansas City Royals    | `kca`       | `kc`        | `KC`           | Royals    | Royals       | Kansas City    | Kansas City   |
/// | Los Angeles Dodgers   | `lan`       | `la`        | `LAD`          | Dodgers   | Dodgers      | Los Angeles    | LA Dodgers    |
/// | Washington Nationals  | `was`       | `was`       | `WSH`          | Nationals | Nationals    | Washington     | Washington    |
/// | New York Mets         | `nyn`       | `nym`       | `NYM`          | Mets      | Mets         | New York       | NY Mets       |
#[derive(Debug, PartialEq, Eq, Deref, DerefMut, Clone)]
pub struct TeamName {
	/// Typically 3 characters and all lowercase.
	pub team_code: String,
	pub file_code: String,
	pub abbreviation: String,
	pub team_name: String,
	/// Effectively `franchise_name` but has changes for duplicates like 'New York'
	pub short_name: String,
	pub franchise_name: String,
	pub club_name: String,
	#[deref]
	#[deref_mut]
	pub full_name: String,
}

impl __TeamNameRaw {
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

id!(#[doc = "A [`u32`] representing a team's ID."] TeamId { id: u32 });

/// A named organization.
#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NamedOrganization {
	#[serde(rename = "parentOrgName")]
	pub name: String,
	#[serde(rename = "parentOrgId")]
	pub id: OrganizationId,
}

id!(#[doc = "ID of a parent organization -- still don't know what this is."] OrganizationId { id: u32 });

/// Honestly, no clue. Would love to know.
#[derive(Debug, Deserialize, PartialEq, Eq, Copy, Clone, Default)]
pub enum AllStarStatus {
	/// 'tis an All-Star team (?)
	#[serde(rename = "Y")]
	Yes,
	/// 'tis not an All-Star team (?)
	#[default]
	#[serde(rename = "N")]
	No,
	/// No clue.
	#[serde(rename = "F")]
	F,
	/// No clue.
	#[serde(rename = "O")]
	O,
}

/// A [`Vec`] of [`Team`]s
#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase", bound = "H: TeamHydrations")]
pub struct TeamsResponse<H: TeamHydrations> {
	pub copyright: Copyright,
	pub teams: Vec<Team<H>>,
}

pub trait TeamHydrations: Hydrations {
	/// By default [`SportId`], with [`sport`] hydration, [`Sport`](crate::sport::Sport)
	type Sport: Debug + DeserializeOwned + Eq + Clone;
}

impl TeamHydrations for () {
	type Sport = SportId;
}

/// | Name                    | Type                             |
/// |-------------------------|----------------------------------|
/// | `previousSchedule`      |                                  |
/// | `nextSchedule`          |                                  |
/// | `venue`                 |                                  |
/// | `springVenue`           |                                  |
/// | `social`                | [`HashMap<String, Vec<String>>`] |
/// | `game`                  |                                  |
/// | `league`                |                                  |
/// | `sport`                 | [`sports_hydrations!`]           |
/// | `standings`             |                                  |
/// | `division`              |                                  |
/// | `xrefId`                | [`ExternalReference`]            |
///
/// [`sports_hydrations!`]: crate::sports_hydrations
/// [`ExternalReference`]: crate::types::ExternalReference
/// [`HashMap<String, Vec<String>>`]: std::collections::HashMap
#[macro_export]
macro_rules! team_hydrations {
	(@ inline_structs [sport: { $($sport_tt:tt)* } $(, $($tt:tt)+)?] $vis:vis struct $name:ident { $($field_tt:tt)* }) => {
		::pastey::paste! {
			$crate::sports_hydrations! {
				$vis struct [<$name InlineSport>] {
					$($sport_tt)*
				}
			}

			$crate::team_hydrations! { @ inline_structs [$($($tt)+)?]
				$vis struct $name {
					$($field_tt)*
					sport: [<$name InlineSport>],
				}
			}
		}
	};
	(@ inline_structs [$_01:ident : { $($_02:tt)* } $(, $($tt:tt)+)?] $vis:vis struct $name:ident { $($field_tt:tt)* }) => {
		::core::compile_error!("Found unknown inline struct");
	};
	(@ inline_structs [$field:ident $(, $($tt:tt)+)?] $vis:vis struct $name:ident { $($field_tt:tt)* }) => {
		$crate::team_hydrations! { @ inline_structs [$($($tt)+)?]
			$vis struct $name {
				$($field_tt)*
				$field,
			}
		}
	};
	(@ inline_structs [] $vis:vis struct $name:ident { $($field_tt:tt)* }) => {
		$crate::team_hydrations! { @ actual
			$vis struct $name {
				$($field_tt)*
			}
		}
	};
	(@ sport_type) => {
		$crate::sport::SportId
	};
	(@ sport_type $hydrations:path) => {
		$crate::sport::Sport<$hydrations>
	};
	(@ actual $vis:vis struct $name:ident {
		$(previous_schedule $previous_schedule:path ,)?
		$(next_schedule $next_schedule:path ,)?
		$(venue $venue:path ,)?
		$(spring_venue $spring_venue:path ,)?
		$(social $social_comma:tt)?
		$(league $league:path, )?
		$(sport $sport:path ,)?
		$(standings $standings:path ,)?
		$(division $division:path ,)?
		$(external_references $external_references_comma:tt)?
		$(location $location_comma:tt)?
	}) => {
		::pastey::paste! {
			#[derive(::core::fmt::Debug, ::serde::Deserialize, ::core::cmp::PartialEq, ::core::cmp::Eq, ::core::clone::Clone)]
			#[serde(rename_all = "camelCase")]
			$vis struct $name {
				$(#[serde(rename = "xrefIds")] external_references: ::std::vec::Vec<$crate::types::ExternalReference> $external_references_comma)?
				$(#[serde(default, rename = "social")] socials: ::std::collections::HashMap<::std::string::String, ::std::vec::Vec<::std::string::String> $social_comma>)?
			}

			impl $crate::team::TeamHydrations for $name {
				type Sport = $crate::team_hydrations!(@ sport_type $($sport)?);
			}
		}
	};
    ($vis:vis struct $name:ident {
		$($tt:tt)*
	}) => {
		$crate::team_hydrations! { @ inline_structs [$($tt)*] $vis struct $name {} }
	};
}

/// Returns a [`TeamsResponse`].
#[derive(Builder)]
#[builder(derive(Into))]
pub struct TeamsRequest<H: TeamHydrations> {
	#[builder(into)]
	sport_id: Option<SportId>,
	#[builder(into)]
	season: Option<SeasonId>,
	#[builder(into)]
	hydrations: H::RequestData,
}

impl TeamsRequest<()> {
	pub fn for_sport(sport_id: impl Into<SportId>) -> TeamsRequestBuilder<(), teams_request_builder::SetHydrations<teams_request_builder::SetSportId>> {
		Self::builder().sport_id(sport_id).hydrations(())
	}

	pub fn mlb_teams() -> TeamsRequestBuilder<(), teams_request_builder::SetHydrations<teams_request_builder::SetSportId>> {
		Self::for_sport(SportId::MLB)
	}

	pub fn all_sports() -> TeamsRequestBuilder<(), teams_request_builder::SetHydrations> {
		Self::builder().hydrations(())
	}
}

impl<H: TeamHydrations, S: teams_request_builder::State + teams_request_builder::IsComplete> crate::request::RequestURLBuilderExt for TeamsRequestBuilder<H, S> {
	type Built = TeamsRequest<H>;
}

impl<H: TeamHydrations> Display for TeamsRequest<H> {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		let hydrations = Some(H::hydration_text(&self.hydrations)).filter(|s| !s.is_empty());
		write!(f, "http://statsapi.mlb.com/api/v1/teams{}", gen_params! { "sportId"?: self.sport_id, "season"?: self.season, "hydrate"?: hydrations })
	}
}

impl<H: TeamHydrations> RequestURL for TeamsRequest<H> {
	type Response = TeamsResponse<H>;
}

#[cfg(test)]
mod tests {
	use crate::request::RequestURLBuilderExt;
	use crate::TEST_YEAR;
	use super::*;

	#[tokio::test]
	#[cfg_attr(not(feature = "_heavy_tests"), ignore)]
	async fn parse_all_teams_all_seasons() {
		for season in 1871..=TEST_YEAR {
			let _response = TeamsRequest::all_sports().season(season).build_and_get().await.unwrap();
		}
	}

	#[tokio::test]
	async fn parse_all_mlb_teams_this_season() {
		let _ = TeamsRequest::mlb_teams().build_and_get().await.unwrap();
	}
}
