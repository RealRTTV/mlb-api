pub mod stats;

use std::fmt::{Display, Formatter};
use std::marker::PhantomData;
use crate::endpoints::positions::Position;
use crate::types::{Gender, Handedness, HeightMeasurement};
use chrono::NaiveDate;
use derive_more::{Deref, DerefMut, Display, From};
use serde::Deserialize;
use serde_with::DisplayFromStr;
use serde_with::serde_as;
use std::ops::{Deref, DerefMut};
use mlb_api_proc::{EnumTryAs, EnumTryAsMut, EnumTryInto};
use crate::cache::{EndpointEntryCache, HydratedCacheTable};
use crate::{rwlock_const_new, RwLock};
use crate::endpoints::draft::School;
use crate::endpoints::people::PeopleResponse;
use crate::endpoints::StatsAPIEndpointUrl;
use crate::endpoints::teams::team::Team;
use crate::hydrations::Hydrations;

#[serde_as]
#[derive(Debug, Deref, DerefMut, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(bound = "H: PersonHydrations")]
pub struct Ballplayer<H: PersonHydrations> {
	#[serde(deserialize_with = "crate::types::try_from_str")]
	#[serde(default)]
	pub primary_number: Option<u8>,
	pub current_age: u8,
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
	inner: HydratedPerson<H>,
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

#[serde_as]
#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BodyMeasurements {
	#[serde_as(as = "DisplayFromStr")]
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
		if let Some(char) = self.use_first_name.chars().next() {
			format!("{1}, {0}", char, self.use_last_name)
		} else {
			self.use_last_name.clone()
		}
	}

	#[must_use]
	pub fn name_first_initial_last(&self) -> String {
		if let Some(char) = self.use_first_name.chars().next() {
			format!("{0} {1}", char, self.use_last_name)
		} else {
			self.use_last_name.clone()
		}
	}

	#[must_use]
	pub fn name_fml(&self) -> String {
		if let Some(middle) = &self.middle_name {
			format!("{0} {1} {2}", self.use_first_name, middle, self.use_last_name)
		} else {
			format!("{0} {1}", self.use_first_name, self.use_last_name)
		}
	}

	#[must_use]
	pub fn name_lfm(&self) -> String {
		if let Some(middle) = &self.middle_name {
			format!("{2}, {0} {1}", self.use_first_name, middle, self.use_last_name)
		} else {
			format!("{1}, {0}", self.use_first_name, self.use_last_name)
		}
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
	hydrations: H
}

#[repr(transparent)]
#[derive(Debug, Deserialize, Deref, Display, PartialEq, Eq, Copy, Clone, Hash, From)]
pub struct PersonId(u32);

impl PersonId {
	#[must_use]
	pub const fn new(id: u32) -> Self {
		Self(id)
	}
}

#[derive(Debug, Deserialize, Eq, Clone, From, EnumTryAs, EnumTryAsMut, EnumTryInto)]
#[serde(untagged)]
#[serde(bound = "H: PersonHydrations")]
pub enum Person<H: PersonHydrations = ()> {
	Ballplayer(Ballplayer<H>),
	Hydrated(HydratedPerson<H>),
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

impl<H: PersonHydrations> Deref for Person<H> {
	type Target = IdentifiablePerson<H>;

	fn deref(&self) -> &Self::Target {
		match self {
			Self::Ballplayer(inner) => inner,
			Self::Hydrated(inner) => inner,
			Self::Named(inner) => inner,
			Self::Identifiable(inner) => inner,
		}
	}
}

impl<H: PersonHydrations> DerefMut for Person<H> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		match self {
			Self::Ballplayer(inner) => inner,
			Self::Hydrated(inner) => inner,
			Self::Named(inner) => inner,
			Self::Identifiable(inner) => inner,
		}
	}
}

pub struct PersonEndpoint<H: PersonHydrations> {
	id: PersonId,
	_marker: PhantomData<H>,
}

impl<H: PersonHydrations> PersonEndpoint<H> {
	#[must_use]
	pub const fn new(id: PersonId) -> Self {
		Self { id, _marker: PhantomData }
	}
}

impl<H: PersonHydrations> Display for PersonEndpoint<H> {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		if let Some(request_text) = H::request_text() {
			write!(f, "http://statsapimlb.com/api/v1/people/{}?hydrate={request_text}", self.id)
		} else {
			write!(f, "http://statsapimlb.com/api/v1/people/{}", self.id)
		}
	}
}

impl<H: PersonHydrations> StatsAPIEndpointUrl for PersonEndpoint<H> {
	type Response = PeopleResponse<H>;
}

#[serde_as]
#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PreferredTeamData {
	#[serde_as(as = "DisplayFromStr")]
	pub jersey_number: u8,
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
	#[serde(default, deserialize_with = "crate::types::try_from_str")]
	pub season: Option<u32>,
}

pub trait PersonHydrations: Hydrations {

}

impl PersonHydrations for () {}

/// Creates hydrations for a person, ex:
/// ```rs
/// person_hydrations! {
///     pub struct ExampleHydrations {   ->    pub struct ExampleHydrations {
///         stats: MyStats,                    	  stats: MyStats,
///         awards,                            	  awards: Vec<Award>,
///         social,                            	  social: HashMap<String, Vec<String>>,
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
/// - `depth_charts`\
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
		    pub awards: ::std::vec::Vec<$crate::endpoints::awards::Award>,
		    pub current_team: $crate::endpoints::teams::team::Team,
			pub preferred_team: $crate::endpoints::person::PreferredTeamData,
		    // team: $crate::endpoints::teams::team::Team,
		    #[serde(default)]
		    pub roster_entries: ::std::vec::Vec<$crate::endpoints::teams::team::roster::RosterEntry>,
		    #[serde(default, rename = "jobEntries")]
		    jobs: ::std::vec::Vec<$crate::endpoints::jobs::EmployedPerson>,
		    #[serde(default)]
		    pub relatives: ::std::vec::Vec<$crate::endpoints::person::Relative>,
		    #[serde(default)]
		    pub transactions: ::std::vec::Vec<$crate::endpoints::transactions::Transaction>,
		    // possibly add a specific type? likely not as socials can always add more over time
		    #[serde(default)]
		    pub social: ::std::collections::HashMap<String, Vec<String>>,
		    #[serde(default)]
		    pub education: $crate::endpoints::person::Education,
		    #[serde(default, rename = "drafts")]
		    pub draft: ::std::vec::Vec<$crate::endpoints::draft::DraftPick>,
		    #[serde(default, rename = "xrefIds")]
		    pub xref_id: ::std::vec::Vec<ExternalReference>,
		    #[serde(default)]
		    pub nicknames: ::std::vec::Vec<String>,
		    #[serde(default)]
		    pub depth_charts: ::std::vec::Vec<$crate::endpoints::teams::team::roster::RosterEntry>,
	    }

	    impl $crate::hydrations::Hydrations for $hydrations_name {
		    fn request_text() -> ::core::option::Option<::std::borrow::Cow<'static, str>> {
			    let base = ::mlb_api_proc::concat_camel_case!($($hydration)*);
			    Some(::std::borrow::Cow::Owned(::std::string::String::from(base) + <$stats as $crate::endpoints::stats::Stats>::request_text()))
            }
	    }

	    impl $crate::endpoints::person::PersonHydrations for $hydrations_name {}
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
		    pub awards: ::std::vec::Vec<$crate::endpoints::awards::Award>,
		    pub current_team: $crate::endpoints::teams::team::Team,
			pub preferred_team: $crate::endpoints::person::PreferredTeamData,
		    // team: $crate::endpoints::teams::team::Team,
		    #[serde(default)]
		    pub roster_entries: ::std::vec::Vec<$crate::endpoints::teams::team::roster::RosterEntry>,
		    #[serde(default, rename = "jobEntries")]
		    jobs: ::std::vec::Vec<$crate::endpoints::jobs::EmployedPerson>,
		    #[serde(default)]
		    pub relatives: ::std::vec::Vec<$crate::endpoints::person::Relative>,
		    #[serde(default)]
		    pub transactions: ::std::vec::Vec<$crate::endpoints::transactions::Transaction>,
		    // possibly add a specific type? likely not as socials can always add more over time
		    #[serde(default)]
		    pub social: ::std::collections::HashMap<String, Vec<String>>,
		    #[serde(default)]
		    pub education: $crate::endpoints::person::Education,
		    #[serde(default, rename = "drafts")]
		    pub draft: ::std::vec::Vec<$crate::endpoints::draft::DraftPick>,
		    #[serde(default, rename = "xrefIds")]
		    pub xref_id: ::std::vec::Vec<ExternalReference>,
		    #[serde(default)]
		    pub nicknames: ::std::vec::Vec<String>,
		    #[serde(default)]
		    pub depth_charts: ::std::vec::Vec<$crate::endpoints::teams::team::roster::RosterEntry>,
	    }

		impl $crate::hydrations::Hydrations for $hydrations_name {
		    fn request_text() -> ::core::option::Option<::std::borrow::Cow<'static, str>> {
			    let base = ::mlb_api_proc::concat_camel_case!($($hydration)*);
			    Some(::std::borrow::Cow::Borrowed(base))
            }
	    }

	    impl $crate::endpoints::person::PersonHydrations for $hydrations_name {}
    }
}

static CACHE: RwLock<HydratedCacheTable<Person<()>>> = rwlock_const_new(HydratedCacheTable::new());

impl EndpointEntryCache for Person<()> {
	type HydratedVariant = HydratedPerson<()>;
	type Identifier = PersonId;
	type URL = PersonEndpoint<()>;

	fn into_hydrated_variant(self) -> Option<Self::HydratedVariant> {
		self.try_into_hydrated()
	}

	fn id(&self) -> &Self::Identifier {
		&self.id
	}

	fn url_for_id(id: &Self::Identifier) -> Self::URL {
		PersonEndpoint::new(id.clone())
	}

	fn get_entries(response: <Self::URL as StatsAPIEndpointUrl>::Response) -> impl IntoIterator<Item=Self>
	where
		Self: Sized
	{
		response.people
	}

	fn get_hydrated_cache_table() -> &'static RwLock<HydratedCacheTable<Self>>
	where
		Self: Sized
	{
		&CACHE
	}
}
