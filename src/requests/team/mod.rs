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

use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};
use bon::Builder;
use serde_with::DefaultOnError;
use crate::divisions::NamedDivision;
use crate::league::{LeagueId, NamedLeague};
use crate::season::SeasonId;
use crate::venue::{NamedVenue, VenueId};
use derive_more::{Deref, DerefMut};
use serde::Deserialize;
use serde_with::serde_as;
use crate::Copyright;
use crate::request::RequestURL;
use crate::sport::SportId;

#[serde_as]
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct __TeamRaw {
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
	name: __TeamNameRaw,
	spring_venue: Option<VenueId>,
	spring_league: Option<LeagueId>,
	#[serde(flatten)]
	inner: NamedTeam,
}

/// A detailed `struct` representing a baseball team.
///
/// ## Examples
/// ```
/// Team {
///     all_star_status: AllStarStatus::Yes,
///     active: true,
///     season: 2025.into(),
///     venue: NamedVenue { name: "Rogers Centre".into(), id: 14.into() },
///     location_name: Some("Toronto".to_owned()),
///     first_year_of_play: 1977.into(),
///     league: NamedLeague { name: "American League".into(), id: 103.into() },
///     division: Some(NamedDivision { name: "American League East".into(), id: 201.into() }),
///     sport: SportId::MLB,
///     parent_organization: None,
///     name: TeamName {
///         team_code: "tor".into(),
///         file_code: "tor".into(),
///         abbreviation: "TOR".into(),
///         team_name: "Blue Jays".into(),
///         short_name: "Toronto".into(),
///         franchise_name: "Toronto".into(),
///         club_name: "Blue Jays".into(),
///         full_name: "Toronto Blue Jays",
///     },
///     spring_venue: Some(VenueId::new(2536)),
///     spring_league: Some(LeagueId::new(115)),
///     id: 141.into(),
/// }
/// ```
#[derive(Debug, Deserialize, Deref, DerefMut, Clone)]
#[serde(from = "__TeamRaw")]
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

impl From<__TeamRaw> for Team {
	fn from(value: __TeamRaw) -> Self {
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
id_only_eq_impl!(Team, id);

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
///
/// Hydrations:
/// * `previousSchedule`
/// * `nextSchedule`
/// * `venue`
/// * `springVenue`
/// * `social`
/// * `deviceProperties`
/// * `game(promotions)`
/// * `game(atBatPromotions)`
/// * `game(tickets)`
/// * `game(atBatTickets)`
/// * `game(sponsorships)`
/// * `league`
/// * `person`
/// * `sport`
/// * `standings`
/// * `division`
/// * `xrefId`
/// * `location`
#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TeamsResponse {
	pub copyright: Copyright,
	pub teams: Vec<Team>,
}

/// Returns a [`TeamsResponse`].
#[derive(Builder)]
#[builder(start_fn(vis = ""))]
#[builder(derive(Into))]
pub struct TeamsRequest {
	#[builder(setters(vis = "", name = "sport_id_internal"))]
	sport_id: Option<SportId>,
	#[builder(into)]
	season: Option<SeasonId>,
}

impl TeamsRequest {
	pub fn for_sport(sport_id: SportId) -> TeamsRequestBuilder<teams_request_builder::SetSportId> {
		Self::builder().sport_id_internal(sport_id)
	}

	pub fn mlb_teams() -> TeamsRequestBuilder<teams_request_builder::SetSportId> {
		Self::for_sport(SportId::MLB)
	}

	pub fn all_sports() -> TeamsRequestBuilder {
		Self::builder()
	}
}

impl<S: teams_request_builder::State + teams_request_builder::IsComplete> crate::request::RequestURLBuilderExt for TeamsRequestBuilder<S> {
	type Built = TeamsRequest;
}

impl Display for TeamsRequest {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "http://statsapi.mlb.com/api/v1/teams{}", gen_params! { "sportId"?: self.sport_id, "season"?: self.season })
	}
}

impl RequestURL for TeamsRequest {
	type Response = TeamsResponse;
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
