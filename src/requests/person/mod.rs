#![allow(clippy::trait_duplication_in_bounds, reason = "serde duplicates it")]

pub mod free_agents;
pub mod stats;
// done
pub mod people;
pub mod players;

use crate::cache::{HydratedCacheTable, RequestEntryCache};
use crate::draft::School;
use crate::hydrations::Hydrations;
use crate::requests::meta::positions::Position;
use crate::season::SeasonId;
use crate::requests::team::Team;
use crate::types::{Gender, Handedness, HeightMeasurement};
use crate::request::StatsAPIRequestUrl;
use crate::{rwlock_const_new, RwLock};
use bon::Builder;
use chrono::NaiveDate;
use derive_more::{Deref, DerefMut, Display, From};
use mlb_api_proc::{EnumDeref, EnumDerefMut, EnumTryAs, EnumTryAsMut, EnumTryInto};
use people::PeopleResponse;
use serde::de::Error;
use serde::{Deserialize, Deserializer};
use std::fmt::{Display, Formatter};
use std::marker::PhantomData;

#[derive(Debug, Deref, DerefMut, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(bound = "H: PersonHydrations")]
pub struct Ballplayer<H>
where
	H: PersonHydrations,
{
	#[serde(deserialize_with = "crate::types::try_from_str")]
	#[serde(default)]
	pub primary_number: Option<u8>,
	pub current_age: u16,
	#[serde(flatten)]
	pub birth_data: BirthData,
	#[serde(flatten)]
	pub body_measurements: BodyMeasurements,
	pub gender: Gender,
	pub draft_year: Option<u16>,
	#[serde(rename = "mlbDebutDate")]
	pub mlb_debut: Option<NaiveDate>,
	pub bat_side: Handedness,
	pub pitch_hand: Handedness,
	#[serde(flatten)]
	pub strike_zone: StrikeZoneMeasurements,
	#[serde(rename = "nickName")]
	pub nickname: Option<String>,

	#[deref]
	#[deref_mut]
	#[serde(flatten)]
	inner: Box<HydratedPerson<H>>,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BirthData {
	pub birth_date: NaiveDate,
	pub birth_city: String,
	#[serde(rename = "birthStateProvince")]
	pub birth_state_or_province: Option<String>,
	pub birth_country: String,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BodyMeasurements {
	pub height: HeightMeasurement,
	pub weight: u16,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StrikeZoneMeasurements {
	pub strike_zone_top: f64,
	pub strike_zone_bottom: f64,
}

impl Eq for StrikeZoneMeasurements {}

#[derive(Debug, Deref, DerefMut, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(bound = "H: PersonHydrations")]
pub struct HydratedPerson<H: PersonHydrations> {
	pub primary_position: Position,
	// '? ? Brown' in 1920 does not have a first name or a middle name, rather than dealing with Option and making everyone hate this API, the better approach is an empty String.
	#[serde(default)]
	pub first_name: String,
	pub middle_name: Option<String>,
	#[serde(rename = "nameSuffix")]
	pub suffix: Option<String>,
	#[serde(default)]
	pub last_name: String,
	#[serde(default)]
	#[serde(rename = "useName")]
	pub use_first_name: String,
	#[serde(default)]
	pub use_last_name: String,
	#[serde(default)]
	pub boxscore_name: String,

	pub is_player: bool,
	#[serde(default)]
	pub is_verified: bool,
	pub active: bool,

	#[deref]
	#[deref_mut]
	#[serde(flatten)]
	inner: NamedPerson<H>,
}

impl<H: PersonHydrations> HydratedPerson<H> {
	#[must_use]
	pub fn name_first_last(&self) -> String {
		format!("{0} {1}", self.use_first_name, self.use_last_name)
	}

	#[must_use]
	pub fn name_last_first(&self) -> String {
		format!("{1}, {0}", self.use_first_name, self.use_last_name)
	}

	#[must_use]
	pub fn name_last_first_initial(&self) -> String {
		self.use_first_name.chars().next().map_or_else(|| self.use_last_name.clone(), |char| format!("{1}, {0}", char, self.use_last_name))
	}

	#[must_use]
	pub fn name_first_initial_last(&self) -> String {
		self.use_first_name.chars().next().map_or_else(|| self.use_last_name.clone(), |char| format!("{0} {1}", char, self.use_last_name))
	}

	#[must_use]
	pub fn name_fml(&self) -> String {
		self.middle_name
			.as_ref()
			.map_or_else(|| format!("{0} {1}", self.use_first_name, self.use_last_name), |middle| format!("{0} {1} {2}", self.use_first_name, middle, self.use_last_name))
	}

	#[must_use]
	pub fn name_lfm(&self) -> String {
		self.middle_name
			.as_ref()
			.map_or_else(|| format!("{1}, {0}", self.use_first_name, self.use_last_name), |middle| format!("{2}, {0} {1}", self.use_first_name, middle, self.use_last_name))
	}
}

#[derive(Debug, Deserialize, Deref, DerefMut, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(bound = "H: PersonHydrations")]
pub struct NamedPerson<H: PersonHydrations> {
	pub full_name: String,

	#[deref]
	#[deref_mut]
	#[serde(flatten)]
	inner: IdentifiablePerson<H>,
}

#[derive(Debug, Deserialize, Deref, DerefMut, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(bound = "H: PersonHydrations")]
pub struct IdentifiablePerson<H: PersonHydrations> {
	pub id: PersonId,

	#[serde(flatten)]
	#[deref]
	#[deref_mut]
	hydrations: H,
}

integer_id!(PersonId);

#[repr(transparent)]
#[derive(Debug, Deref, Display, PartialEq, Eq, Copy, Clone, Hash, From)]
pub struct JerseyNumber(u8);

impl<'de> Deserialize<'de> for JerseyNumber {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		String::deserialize(deserializer)?.parse::<u8>().map(JerseyNumber).map_err(D::Error::custom)
	}
}

#[derive(Debug, Deserialize, Eq, Clone, From, EnumTryAs, EnumTryAsMut, EnumTryInto, EnumDeref, EnumDerefMut)]
#[serde(untagged)]
#[serde(bound = "H: PersonHydrations")]
pub enum Person<H: PersonHydrations = ()> {
	Ballplayer(Box<Ballplayer<H>>),
	Hydrated(Box<HydratedPerson<H>>),
	Named(NamedPerson<H>),
	Identifiable(IdentifiablePerson<H>),
}

impl Person<()> {
	#[must_use]
	pub(crate) const fn unknown_person() -> Self {
		Self::Identifiable(IdentifiablePerson { id: PersonId::new(0), hydrations: () })
	}
}

impl<H1: PersonHydrations, H2: PersonHydrations> PartialEq<Person<H2>> for Person<H1> {
	fn eq(&self, other: &Person<H2>) -> bool {
		self.id == other.id
	}
}

#[derive(Debug, Deserialize, Eq, Clone, From, EnumTryAs, EnumTryAsMut, EnumTryInto, EnumDeref, EnumDerefMut)]
#[serde(untagged)]
#[serde(bound = "H: PersonHydrations")]
pub enum NamedOrBetterPerson<H: PersonHydrations = ()> {
	Ballplayer(Box<Ballplayer<H>>),
	Hydrated(Box<HydratedPerson<H>>),
	Named(NamedPerson<H>),
}

impl NamedOrBetterPerson<()> {
	#[must_use]
	pub(crate) const fn unknown_person() -> Self {
		Self::Named(NamedPerson {
			full_name: String::new(),
			inner: IdentifiablePerson { id: PersonId::new(0), hydrations: () },
		})
	}
}

impl<H1: PersonHydrations, H2: PersonHydrations> PartialEq<NamedOrBetterPerson<H2>> for NamedOrBetterPerson<H1> {
	fn eq(&self, other: &NamedOrBetterPerson<H2>) -> bool {
		self.id == other.id
	}
}

#[derive(Debug, Deserialize, Eq, Clone, From, EnumTryAs, EnumTryAsMut, EnumTryInto, EnumDeref, EnumDerefMut)]
#[serde(untagged)]
#[serde(bound = "H: PersonHydrations")]
pub enum HydratedOrBetterPerson<H: PersonHydrations = ()> {
	Ballplayer(Box<Ballplayer<H>>),
	Hydrated(Box<HydratedPerson<H>>),
}

impl<H1: PersonHydrations, H2: PersonHydrations> PartialEq<HydratedOrBetterPerson<H2>> for HydratedOrBetterPerson<H1> {
	fn eq(&self, other: &HydratedOrBetterPerson<H2>) -> bool {
		self.id == other.id
	}
}

#[derive(Builder)]
#[builder(derive(Into))]
pub struct PersonRequest<H: PersonHydrations> {
	#[builder(start_fn)]
	#[builder(into)]
	id: PersonId,
	#[builder(skip)]
	_marker: PhantomData<H>,
}

impl<H: PersonHydrations, S: person_request_builder::State + person_request_builder::IsComplete> crate::request::StatsAPIRequestUrlBuilderExt for PersonRequestBuilder<H, S> {
	type Built = PersonRequest<H>;
}

impl<H: PersonHydrations> Display for PersonRequest<H> {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		if let Some(request_text) = H::request_text() {
			write!(f, "http://statsapi.mlb.com/api/v1/people/{}?hydrate={request_text}", self.id)
		} else {
			write!(f, "http://statsapi.mlb.com/api/v1/people/{}", self.id)
		}
	}
}

impl<H: PersonHydrations> StatsAPIRequestUrl for PersonRequest<H> {
	type Response = PeopleResponse<H>;
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PreferredTeamData {
	pub jersey_number: JerseyNumber,
	pub position: Position,
	pub team: Team,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Relative {
	pub has_stats: bool,
	pub relation: String,
	#[serde(flatten)]
	pub person: Person<()>,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone, Default)]
pub struct Education {
	#[serde(default)]
	pub highschools: Vec<School>,
	#[serde(default)]
	pub colleges: Vec<School>,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
pub struct ExternalReference {
	#[serde(rename = "xrefId")]
	pub id: String,
	#[serde(rename = "xrefType")]
	pub xref_type: String,
	pub season: Option<SeasonId>,
}

pub trait PersonHydrations: Hydrations {}

impl PersonHydrations for () {}

/// Creates hydrations for a person, ex:
/// ```rs
/// person_hydrations! {
///     pub struct ExampleHydrations {   ->    pub struct ExampleHydrations {
///         stats: MyStats,                        stats: MyStats,
///         awards,                                awards: Vec<Award>,
///         social,                                social: HashMap<String, Vec<String>>,
///     }                                      }
/// }
/// ```
///
/// The list of valid hydrations are:
/// - `stats`
/// - `awards`
/// - `current_team`
/// - `preferred_team`
/// - `roster_entries`
/// - `relatives`
/// - `transactions`
/// - `social`
/// - `education`
/// - `draft`
/// - `xref_id`
/// - `nicknames`
/// - `depth_charts`
///
/// Note: the others can appear in any order, but stats must go first (and is also the only one with a type assigned)
#[macro_export]
macro_rules! person_hydrations {
    (
		$vis:vis struct $hydrations_name:ident {
			stats: $stats:path, $($hydration:ident),* $(,)?
		}
    ) => {

	    #[::mlb_api_proc::filter_fields]
	    #[keep($($hydration),*)]
	    #[derive(::core::fmt::Debug, ::serde::Deserialize, ::core::cmp::PartialEq, ::core::cmp::Eq, ::core::clone::Clone)]
	    #[serde(rename_all = "camelCase")]
	    $vis struct $hydrations_name {
		    #[keep]
		    pub stats: $stats,
		    #[serde(default)]
		    pub awards: ::std::vec::Vec<$crate::awards::Award>,
		    pub current_team: $crate::requests::team::Team,
			pub preferred_team: $crate::person::PreferredTeamData,
		    // team: $crate::teams::team::Team,
		    #[serde(default)]
		    pub roster_entries: ::std::vec::Vec<$crate::requests::team::roster::RosterEntry>,
		    #[serde(default, rename = "jobEntries")]
		    pub jobs: ::std::vec::Vec<$crate::jobs::EmployedPerson>,
		    #[serde(default)]
		    pub relatives: ::std::vec::Vec<$crate::person::Relative>,
		    #[serde(default)]
		    pub transactions: ::std::vec::Vec<$crate::transactions::Transaction>,
		    #[serde(default)]
		    pub social: ::std::collections::HashMap<String, Vec<String>>,
		    #[serde(default)]
		    pub education: $crate::person::Education,
		    #[serde(default, rename = "drafts")]
		    pub draft: ::std::vec::Vec<$crate::draft::DraftPick>,
		    #[serde(default, rename = "xrefIds")]
		    pub xref_id: ::std::vec::Vec<ExternalReference>,
		    #[serde(default)]
		    pub nicknames: ::std::vec::Vec<String>,
		    #[serde(default)]
		    pub depth_charts: ::std::vec::Vec<$crate::requests::team::roster::RosterEntry>,
	    }

	    impl $crate::hydrations::Hydrations for $hydrations_name {
		    fn request_text() -> ::core::option::Option<::std::borrow::Cow<'static, str>> {
			    let base = ::mlb_api_proc::concat_camel_case!($($hydration)*);
			    Some(::std::borrow::Cow::Owned(::std::string::String::from(base) + "stats(" + <$stats as $crate::stats::Stats>::request_text() + ")"))
            }
	    }

	    impl $crate::person::PersonHydrations for $hydrations_name {}
    };
	(
		$vis:vis struct $hydrations_name:ident {
			$($hydration:ident),* $(,)?
		}
    ) => {
		#[::mlb_api_proc::filter_fields]
	    #[keep($($hydration),*)]
	    #[derive(::core::fmt::Debug, ::serde::Deserialize, ::core::cmp::PartialEq, ::core::cmp::Eq, ::core::clone::Clone)]
	    #[serde(rename_all = "camelCase")]
	    $vis struct $hydrations_name {
		    #[serde(default)]
		    pub awards: ::std::vec::Vec<$crate::awards::Award>,
		    pub current_team: $crate::requests::team::Team,
			pub preferred_team: $crate::person::PreferredTeamData,
		    // team: $crate::teams::team::Team,
		    #[serde(default)]
		    pub roster_entries: ::std::vec::Vec<$crate::requests::team::roster::RosterEntry>,
		    #[serde(default, rename = "jobEntries")]
		    pub jobs: ::std::vec::Vec<$crate::jobs::EmployedPerson>,
		    #[serde(default)]
		    pub relatives: ::std::vec::Vec<$crate::person::Relative>,
		    #[serde(default)]
		    pub transactions: ::std::vec::Vec<$crate::transactions::Transaction>,
		    #[serde(default)]
		    pub social: ::std::collections::HashMap<String, Vec<String>>,
		    #[serde(default)]
		    pub education: $crate::person::Education,
		    #[serde(default, rename = "drafts")]
		    pub draft: ::std::vec::Vec<$crate::draft::DraftPick>,
		    #[serde(default, rename = "xrefIds")]
		    pub xref_id: ::std::vec::Vec<ExternalReference>,
		    #[serde(default)]
		    pub nicknames: ::std::vec::Vec<String>,
		    #[serde(default)]
		    pub depth_charts: ::std::vec::Vec<$crate::requests::team::roster::RosterEntry>,
	    }

		impl $crate::hydrations::Hydrations for $hydrations_name {
		    fn request_text() -> ::core::option::Option<::std::borrow::Cow<'static, str>> {
			    let base = ::mlb_api_proc::concat_camel_case!($($hydration)*);
			    Some(::std::borrow::Cow::Borrowed(base))
            }
	    }

	    impl $crate::person::PersonHydrations for $hydrations_name {}
    }
}

static CACHE: RwLock<HydratedCacheTable<Person<()>>> = rwlock_const_new(HydratedCacheTable::new());

impl RequestEntryCache for Person<()> {
	type HydratedVariant = Box<HydratedPerson<()>>;
	type Identifier = PersonId;
	type URL = PersonRequest<()>;

	fn into_hydrated_variant(self) -> Option<Self::HydratedVariant> {
		self.try_into_hydrated()
	}

	fn id(&self) -> &Self::Identifier {
		&self.id
	}

	fn url_for_id(id: &Self::Identifier) -> Self::URL {
		PersonRequest::builder(*id).build()
	}

	fn get_entries(response: <Self::URL as StatsAPIRequestUrl>::Response) -> impl IntoIterator<Item = Self>
	where
		Self: Sized,
	{
		response.people
	}

	fn get_hydrated_cache_table() -> &'static RwLock<HydratedCacheTable<Self>>
	where
		Self: Sized,
	{
		&CACHE
	}
}

#[cfg(test)]
mod tests {
	use crate::request::StatsAPIRequestUrlBuilderExt;
	use crate::roster_types::RosterType;
	use super::*;
	use crate::requests::team::roster::RosterRequest;
	use crate::requests::team::teams::TeamsRequest;
	use crate::TEST_YEAR;

	#[tokio::test]
	async fn no_hydrations() {
		let _ = PersonRequest::<()>::builder(665_489).build_and_get().await.unwrap();
	}

	#[tokio::test]
	async fn all_but_stats_hydrations() {
		person_hydrations! {
			pub struct AllButStatHydrations {
				awards,
				current_team,
				preferred_team,
				roster_entries,
				relatives,
				transactions,
				social,
				education,
				draft,
				xref_id,
				nicknames,
				depth_charts,
			}
		}

		let _ = PersonRequest::<AllButStatHydrations>::builder(665_489).build_and_get().await.unwrap();
	}

	#[rustfmt::skip]
	#[tokio::test]
	async fn only_stats_hydrations() {
		stats! {
			pub struct BasicStats {
				[Career] = [Hitting]
			}
		}

		person_hydrations! {
			pub struct StatOnlyHydrations {
				stats: BasicStats,
			}
		}

		let toronto_blue_jays = TeamsRequest::mlb_teams()
			.season(TEST_YEAR)
			.build_and_get()
			.await
			.unwrap()
			.teams
			.into_iter()
			.find(|team| team.try_as_named().is_some_and(|team| team.name.name == "Toronto Blue Jays"))
			.unwrap();

		let roster = RosterRequest::builder()
			.team_id(toronto_blue_jays.id)
			.roster_type(RosterType::AllTime)
			.build_and_get()
			.await
			.unwrap();

		let bautista = roster
			.roster
			.into_iter()
			.find(|player| player.person.try_as_named().is_some_and(|person| person.full_name.rsplit_once(' ').is_some_and(|(_, last_name)| last_name == "Bautista")))
			.unwrap();

		let player = PersonRequest::<StatOnlyHydrations>::builder(bautista.person.id)
			.build_and_get()
			.await
			.unwrap()
			.people
			.into_iter()
			.next()
			.unwrap();
	
		let _ = player.stats;
	}
}
