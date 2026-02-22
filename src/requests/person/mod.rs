//! A description of a person in baseball.
//!
//! Person works through multi-level definition.
//!
//! 1. [`PersonId`], which deserializes a:
//! ```json
//! "person": {
//!     "id": 660271,
//!     "link": "/api/v1/people/660271"
//! }
//! ```
//! 2. [`NamedPerson`], which deserializes a:
//! ```json
//! "person": {
//!     "id": 660271,
//!     "link": "/api/v1/people/660271",
//!     "fullName": "Shohei Ohtani"
//! }
//! ```
//! 3. [`Person`], which deserializes a lot of extra fields, see <http://statsapi.mlb.com/api/v1/people/660271>.
//! Technically, [`Person`] is actually an enum which separates fields supplied for [`Ballplayers`] (handedness, draft year, etc.), and fields available to people like coaches and umpires (such as last name, age, etc.) [`RegularPerson`].
//!
//! This module also contains [`person_hydrations`](crate::person_hydrations), which are used to get additional data about people when making requests.

#![allow(clippy::trait_duplication_in_bounds, reason = "serde duplicates it")]

pub mod free_agents;
pub mod stats;
pub mod people;
pub mod players;

use crate::cache::Requestable;
use crate::draft::School;
use crate::hydrations::Hydrations;
use crate::season::SeasonId;
use crate::{Gender, Handedness, HeightMeasurement};
use crate::request::RequestURL;
use bon::Builder;
use chrono::{Local, NaiveDate};
use derive_more::{Deref, DerefMut, Display, From};
use people::PeopleResponse;
use serde::de::Error;
use serde::{Deserialize, Deserializer};
use std::fmt::{Debug, Display, Formatter};
use std::hash::{Hash, Hasher};
use std::ops::{Deref, DerefMut};
use crate::meta::NamedPosition;
use crate::team::NamedTeam;

#[cfg(feature = "cache")]
use crate::{rwlock_const_new, RwLock, cache::CacheTable};

/// A baseball player.
///
/// [`Deref`]s to [`RegularPerson`]
#[derive(Debug, Deref, DerefMut, Deserialize, Eq, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(bound = "H: PersonHydrations")]
pub struct Ballplayer<H: PersonHydrations> {
	#[serde(deserialize_with = "crate::try_from_str")]
	#[serde(default)]
	pub primary_number: Option<u8>,
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
	inner: Box<RegularPerson<H>>,
}

/// A regular person; detailed-name stuff.
///
/// [`Deref`]s to [`NamedPerson`]
#[derive(Debug, Deserialize, Deref, DerefMut, Eq, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(bound = "H: PersonHydrations")]
pub struct RegularPerson<H: PersonHydrations> {
	pub primary_position: NamedPosition,
	// '? ? Brown' in 1920 does not have a first name or a middle name, rather than dealing with Option and making everyone hate this API, the better approach is an empty String.
	#[serde(default)]
	pub first_name: String,
	#[serde(rename = "nameSuffix")]
	pub suffix: Option<String>,
	#[serde(default)] // this is how their API does it, so I'll copy that.
	pub middle_name: String,
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
	inner: NamedPerson,

	#[serde(flatten)]
	pub extras: H,
}

impl<H: PersonHydrations> RegularPerson<H> {
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
		format!("{0} {1} {2}", self.use_first_name, self.middle_name, self.use_last_name)
	}

	#[must_use]
	pub fn name_lfm(&self) -> String {
		format!("{2}, {0} {1}", self.use_first_name, self.middle_name, self.use_last_name)
	}
}

/// A person with a name.
///
/// [`Deref`]s to [`PersonId`]
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NamedPerson {
	pub full_name: String,

	#[serde(flatten)]
	pub id: PersonId,
}

impl Hash for NamedPerson {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.id.hash(state);
	}
}

impl NamedPerson {
	#[must_use]
	pub(crate) fn unknown_person() -> Self {
		Self {
			full_name: "null".to_owned(),
			id: PersonId::new(0),
		}
	}

	#[must_use]
	pub fn is_unknown(&self) -> bool {
		*self.id == 0
	}
}

id!(#[doc = "A [`u32`] that represents a person."] PersonId { id: u32 });

/// A complete person response for a hydrated request. Ballplayers have more fields.
#[derive(Debug, Deserialize, Clone, Eq, From)]
#[serde(untagged)]
#[serde(bound = "H: PersonHydrations")]
pub enum Person<H: PersonHydrations = ()> {
	Ballplayer(Ballplayer<H>),
	Regular(RegularPerson<H>),
}

impl<H: PersonHydrations> Person<H> {
	#[must_use]
	pub fn as_ballplayer(&self) -> Option<&Ballplayer<H>> {
		match self {
			Self::Ballplayer(x) => Some(x),
			Self::Regular(_) => None,
		}
	}
}

impl<H: PersonHydrations> Person<H> {
	#[must_use]
	pub fn as_ballplayer_mut(&mut self) -> Option<&mut Ballplayer<H>> {
		match self {
			Self::Ballplayer(x) => Some(x),
			Self::Regular(_) => None,
		}
	}
}

impl<H: PersonHydrations> Person<H> {
	#[must_use]
	pub fn into_ballplayer(self) -> Option<Ballplayer<H>> {
		match self {
			Self::Ballplayer(x) => Some(x),
			Self::Regular(_) => None,
		}
	}
}

impl<H: PersonHydrations> Deref for Person<H> {
	type Target = RegularPerson<H>;

	fn deref(&self) -> &Self::Target {
		match self {
			Self::Ballplayer(x) => x,
			Self::Regular(x) => x,
		}
	}
}

impl<H: PersonHydrations> DerefMut for Person<H> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		match self {
			Self::Ballplayer(x) => x,
			Self::Regular(x) => x,
		}
	}
}

impl<H1: PersonHydrations, H2: PersonHydrations> PartialEq<Person<H2>> for Person<H1> {
	fn eq(&self, other: &Person<H2>) -> bool {
		self.id == other.id
	}
}

impl<H1: PersonHydrations, H2: PersonHydrations> PartialEq<Ballplayer<H2>> for Ballplayer<H1> {
	fn eq(&self, other: &Ballplayer<H2>) -> bool {
		self.id == other.id
	}
}

impl<H1: PersonHydrations, H2: PersonHydrations> PartialEq<RegularPerson<H2>> for RegularPerson<H1> {
	fn eq(&self, other: &RegularPerson<H2>) -> bool {
		self.id == other.id
	}
}

id_only_eq_impl!(NamedPerson, id);

/// Returns a [`PeopleResponse`].
#[derive(Builder)]
#[builder(derive(Into))]
pub struct PersonRequest<H: PersonHydrations> {
	#[builder(into)]
	id: PersonId,

	#[builder(into)]
	hydrations: H::RequestData,
}

impl PersonRequest<()> {
	pub fn for_id(id: impl Into<PersonId>) -> PersonRequestBuilder<(), person_request_builder::SetHydrations<person_request_builder::SetId>> {
		Self::builder().id(id).hydrations(<() as Hydrations>::RequestData::default())
	}
}

impl<H: PersonHydrations, S: person_request_builder::State + person_request_builder::IsComplete> crate::request::RequestURLBuilderExt for PersonRequestBuilder<H, S> {
	type Built = PersonRequest<H>;
}

impl<H: PersonHydrations> Display for PersonRequest<H> {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		let hydration_text = H::hydration_text(&self.hydrations);
		if hydration_text.is_empty() {
			write!(f, "http://statsapi.mlb.com/api/v1/people/{}", self.id)
		} else {
			write!(f, "http://statsapi.mlb.com/api/v1/people/{}?hydrate={hydration_text}", self.id)
		}
	}
}

impl<H: PersonHydrations> RequestURL for PersonRequest<H> {
	type Response = PeopleResponse<H>;
}

/// The number on the back of a jersey, useful for radix sorts maybe??
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

/// Data regarding birthplace.
#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BirthData {
	pub birth_date: NaiveDate,
	pub birth_city: String,
	#[serde(rename = "birthStateProvince")]
	pub birth_state_or_province: Option<String>,
	pub birth_country: String,
}

impl BirthData {
	#[must_use]
	pub fn current_age(&self) -> u16 {
		Local::now().naive_local().date().years_since(self.birth_date).and_then(|x| u16::try_from(x).ok()).unwrap_or(0)
	}
}

/// Height and weight
#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BodyMeasurements {
	pub height: HeightMeasurement,
	pub weight: u16,
}

/// Strike zone dimensions, measured in feet from the ground.
#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StrikeZoneMeasurements {
	pub strike_zone_top: f64,
	pub strike_zone_bottom: f64,
}

impl Eq for StrikeZoneMeasurements {}

/// Data regarding preferred team, likely for showcasing the player with a certain look regardless of the time.
#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PreferredTeamData {
	pub jersey_number: JerseyNumber,
	pub position: NamedPosition,
	pub team: NamedTeam,
}

/// Relative to the ballplayer, father, son, etc.
#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Relative {
	pub has_stats: bool,
	pub relation: String,
	#[serde(flatten)]
	pub person: NamedPerson,
}

/// Schools the ballplayer went to.
#[derive(Debug, Deserialize, PartialEq, Eq, Clone, Default)]
pub struct Education {
	#[serde(default)]
	pub highschools: Vec<School>,
	#[serde(default)]
	pub colleges: Vec<School>,
}

/// More generalized than social media, includes retrosheet, fangraphs, etc.
#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
pub struct ExternalReference {
	#[serde(rename = "xrefId")]
	pub id: String,
	#[serde(rename = "xrefType")]
	pub xref_type: String,
	pub season: Option<SeasonId>,
}

/// A type that is made with [`person_hydrations!`](crate::person_hydrations)
pub trait PersonHydrations: Hydrations {}

impl PersonHydrations for () {}

/// Creates hydrations for a person
///```norun
/// person_hydrations! {
///     pub struct ExampleHydrations {  ->  pub struct ExampleHydrations {
///         awards,                     ->      awards: Vec<Award>,
///         social,                     ->      social: HashMap<String, Vec<String>>,
///         stats: MyStats,             ->      stats: MyStats,
///     }                               ->  }
/// }
///
/// person_hydrations! {
///     pub struct ExampleHydrations {        ->  pub struct ExampleHydrations {
///         stats: { [Season] = [Hitting] },  ->      stats: ExampleHydrationsInlineStats,
///     }                                     ->  }
/// }
///
/// let request = PersonRequest::<ExampleHydrations>::builder()
///     .id(660_271)
///     .hydrations(ExampleHydrations::builder())
///     .build();
///
/// let repsonse = request.get();
///```
///
/// ## Person Hydrations
/// <u>Note: Fields must appear in exactly this order (or be omitted)</u>
///
/// | Name             | Type                             |
/// |------------------|----------------------------------|
/// | `awards`         | [`Vec<Award>`]                   |
/// | `current_team`   | [`Team`]                         |
/// | `depth_charts`   | [`Vec<RosterEntry>`]             |
/// | `draft`          | [`Vec<DraftPick>`]               |
/// | `education`      | [`Education`]                    |
/// | `jobs`           | [`Vec<EmployedPerson>`]          |
/// | `nicknames`      | [`Vec<String>`]                  |
/// | `preferred_team` | [`Team`]                         |
/// | `relatives`      | [`Vec<Relative>`]                |
/// | `roster_entries` | [`Vec<RosterEntry>`]             |
/// | `transactions`   | [`Vec<Transaction>`]             |
/// | `social`         | [`HashMap<String, Vec<String>>`] |
/// | `stats`          | [`stats_type!`]                  |
/// | `xref_id`        | [`Vec<ExternalReference>`]       |
///
/// [`Vec<Award>`]: crate::awards::Award
/// [`Team`]: crate::team::Team
/// [`Vec<RosterEntry>`]: crate::team::roster::RosterEntry
/// [`Vec<DraftPick>`]: crate::draft::DraftPick
/// [`Education`]: Education
/// [`Vec<EmployedPerson>`]: crate::jobs::EmployedPerson
/// [`Vec<String>`]: String
/// [`Team`]: crate::team::Team
/// [`Vec<Relative>`]: Relative
/// [`Vec<RosterEntry>`]: crate::team::rosterRosterEntry
/// [`Vec<Transaction>`]: crate::transactions::Transaction
/// [`HashMap<String, Vec<String>>`]: std::collections::HashMap
/// [`stats_type!`]: crate::stats_type
/// [`Vec<ExternalReference>`]: ExternalReference
#[macro_export]
macro_rules! person_hydrations {
	(@ inline_structs [stats: { $($contents:tt)* } $(, $($rest:tt)*)?] $vis:vis struct $name:ident { $($field_tt:tt)* }) => {
        ::pastey::paste! {
            $crate::stats_type! {
                $vis struct [<$name InlineStats>] {
                    $($contents)*
                }
            }

            $crate::person_hydrations! { @ inline_structs [$($($rest)*)?]
                $vis struct $name {
                    $($field_tt)*
                    stats: [<$name InlineStats>],
                }
            }
        }
    };
    (@ inline_structs [$marker:ident : { $($contents:tt)* } $(, $($rest:tt)*)?] $vis:vis struct $name:ident { $($field_tt:tt)* }) => {
        compile_error!("Found unknown inline struct");
    };
    (@ inline_structs [$marker:ident $(: $value:path)? $(, $($rest:tt)*)?] $vis:vis struct $name:ident { $($field_tt:tt)* }) => {
        ::pastey::paste! {
            $crate::person_hydrations! { @ inline_structs [$($($rest)*)?]
                $vis struct $name {
                    $($field_tt)*
                    $marker $(: $value)?,
                }
            }
        }
    };
    (@ inline_structs [$(,)?] $vis:vis struct $name:ident { $($field_tt:tt)* }) => {
        ::pastey::paste! {
            $crate::person_hydrations! { @ actual
                $vis struct $name {
                    $($field_tt)*
                }
            }
        }
    };
    (@ actual
		$vis:vis struct $name:ident {
			$(awards $awards_comma:tt)?
			$(current_team $current_team_comma:tt)?
			$(depth_charts $depth_charts_comma:tt)?
			$(draft $draft_comma:tt)?
			$(education $education_comma:tt)?
			$(jobs $jobs_comma:tt)?
			$(nicknames $nicknames_comma:tt)?
			$(preferred_team $preferred_team_comma:tt)?
			$(relatives $relatives_comma:tt)?
			$(roster_entries $roster_entries_comma:tt)?
			$(transactions $transactions_comma:tt)?
			$(social $social_comma:tt)?
			$(stats: $stats:path ,)?
			$(xref_id $xref_id_comma:tt)?
		}
    ) => {
		::pastey::paste! {
			#[derive(::core::fmt::Debug, ::serde::Deserialize, ::core::cmp::PartialEq, ::core::cmp::Eq, ::core::clone::Clone)]
			#[serde(rename_all = "camelCase")]
			$vis struct $name {
				$(#[serde(default)] pub awards: ::std::vec::Vec<$crate::awards::Award> $awards_comma)?
				$(pub current_team: ::core::option::Option<$crate::team::NamedTeam> $current_team_comma)?
				$(#[serde(default)] pub depth_charts: ::std::vec::Vec<$crate::team::roster::RosterEntry> $depth_charts_comma)?
				$(#[serde(default, rename = "drafts")] pub draft: ::std::vec::Vec<$crate::draft::DraftPick> $draft_comma)?
				$(#[serde(default)] pub education: $crate::person::Education $education_comma)?
				$(#[serde(default, rename = "jobEntries")] pub jobs: ::std::vec::Vec<$crate::jobs::EmployedPerson> $jobs_comma)?
				$(#[serde(default)] pub nicknames: ::std::vec::Vec<String> $nicknames_comma)?
				$(pub preferred_team: ::core::option::Option<$crate::person::PreferredTeamData> $preferred_team_comma)?
				$(#[serde(default)] pub relatives: ::std::vec::Vec<$crate::person::Relative> $relatives_comma)?
				$(#[serde(default)] pub roster_entries: ::std::vec::Vec<$crate::team::roster::RosterEntry> $roster_entries_comma)?
				$(#[serde(default)] pub transactions: ::std::vec::Vec<$crate::transactions::Transaction> $transactions_comma)?
				$(#[serde(flatten)] pub stats: $stats ,)?
				$(#[serde(default)] pub social: ::std::collections::HashMap<String, Vec<String>> $social_comma)?
				$(#[serde(default, rename = "xrefIds")] pub xref_id: ::std::vec::Vec<ExternalReference> $xref_id_comma)?
			}

			impl $crate::person::PersonHydrations for $name {}

			impl $crate::hydrations::Hydrations for $name {
				type RequestData = [<$name RequestData>];

				fn hydration_text(_data: &Self::RequestData) -> ::std::borrow::Cow<'static, str> {
					let text = ::std::borrow::Cow::Borrowed(::std::concat!(
						$("awards," $awards_comma)?
						$("currentTeam," $current_team_comma)?
						$("depthCharts," $depth_charts_comma)?
						$("draft," $draft_comma)?
						$("education," $education_comma)?
						$("jobs," $jobs_comma)?
						$("nicknames," $nicknames_comma)?
						$("preferredTeam," $preferred_team_comma)?
						$("relatives," $relatives_comma)?
						$("rosterEntries," $roster_entries_comma)?
						$("transactions," $transactions_comma)?
						$("social," $social_comma)?
						$("xrefId," $xref_id_comma)?
					));

					$(
					let text = ::std::borrow::Cow::Owned(::std::format!("{text}stats({})", <$stats as $crate::hydrations::Hydrations>::hydration_text(&_data.stats)));
					)?

					text
				}
			}

			#[derive(::bon::Builder)]
			#[builder(derive(Into))]
			$vis struct [<$name RequestData>] {
				$(#[builder(into)] stats: <$stats as $crate::hydrations::Hydrations>::RequestData,)?
			}

			impl $name {
				#[allow(unused)]
				pub fn builder() -> [<$name RequestDataBuilder>] {
					[<$name RequestData>]::builder()
				}
			}

			impl ::core::default::Default for [<$name RequestData>]
			where
				$(for<'no_rfc_2056> <$stats as $crate::hydrations::Hydrations>::RequestData: ::core::default::Default,)?
			{
				fn default() -> Self {
					Self {
						$(stats: <<$stats as $crate::hydrations::Hydrations>::RequestData as ::core::default::Default>::default(),)?
					}
				}
			}
		}
    };
	($vis:vis struct $name:ident {
		$($tt:tt)*
	}) => {
		$crate::person_hydrations! { @ inline_structs [$($tt)*] $vis struct $name {} }
	};
}

#[cfg(feature = "cache")]
static CACHE: RwLock<CacheTable<Person<()>>> = rwlock_const_new(CacheTable::new());

impl Requestable for Person<()> {
	type Identifier = PersonId;
	type URL = PersonRequest<()>;

	fn id(&self) -> &Self::Identifier {
		&self.id
	}

	fn url_for_id(id: &Self::Identifier) -> Self::URL {
		PersonRequest::for_id(*id).build()
	}

	fn get_entries(response: <Self::URL as RequestURL>::Response) -> impl IntoIterator<Item = Self>
	where
		Self: Sized,
	{
		response.people.into_iter().map(Person::Ballplayer)
	}

	#[cfg(feature = "cache")]
	fn get_cache_table() -> &'static RwLock<CacheTable<Self>>
	where
		Self: Sized,
	{
		&CACHE
	}
}

entrypoint!(PersonId => Person);
entrypoint!(NamedPerson.id => Person);
entrypoint!(for < H > RegularPerson < H > . id => Person < > where H: PersonHydrations);
entrypoint!(for < H > Ballplayer < H > . id => Person < > where H: PersonHydrations);

#[cfg(test)]
mod tests {
	use crate::request::RequestURLBuilderExt;
	use crate::meta::RosterType;
	use super::*;
	use crate::team::roster::RosterRequest;
	use crate::team::TeamsRequest;
	use crate::TEST_YEAR;

	#[tokio::test]
	async fn no_hydrations() {
		person_hydrations! {
			pub struct EmptyHydrations {}
		}

		let _ = PersonRequest::<()>::for_id(665_489).build_and_get().await.unwrap();
		let _ = PersonRequest::<EmptyHydrations>::builder().id(665_489).hydrations(EmptyHydrationsRequestData::default()).build_and_get().await.unwrap();
	}

	#[tokio::test]
	async fn all_but_stats_hydrations() {
		person_hydrations! {
			pub struct AllButStatHydrations {
				awards,
				current_team,
				depth_charts,
				draft,
				education,
				jobs,
				nicknames,
				preferred_team,
				relatives,
				roster_entries,
				transactions,
				social,
				xref_id
			}
		}

		let _person = PersonRequest::<AllButStatHydrations>::builder().hydrations(AllButStatHydrationsRequestData::default()).id(665_489).build_and_get().await.unwrap().people.into_iter().next().unwrap();
	}

	#[rustfmt::skip]
	#[tokio::test]
	async fn only_stats_hydrations() {
		person_hydrations! {
			pub struct StatOnlyHydrations {
				stats: { [Sabermetrics] = [Pitching] },
			}
		}

		let toronto_blue_jays = TeamsRequest::mlb_teams()
			.season(TEST_YEAR)
			.build_and_get()
			.await
			.unwrap()
			.teams
			.into_iter()
			.find(|team| team.name.full_name == "Toronto Blue Jays")
			.unwrap();

		let roster = RosterRequest::<()>::for_team(toronto_blue_jays.id)
			.roster_type(RosterType::AllTime)
			.build_and_get()
			.await
			.unwrap();

		let player = roster
			.roster
			.into_iter()
			.find(|player| player.person.full_name == "Kevin Gausman")
			.unwrap();

		let request = PersonRequest::<StatOnlyHydrations>::builder()
			.id(player.person.id)
			.hydrations(StatOnlyHydrations::builder()
				.stats(StatOnlyHydrationsInlineStats::builder()
					.season(2023)
					// .situation(SituationCodeId::new("h"))
				)
			).build();
		println!("{request}");
		let player = request.get()
			.await
			.unwrap()
			.people
			.into_iter()
			.next()
			.unwrap();

		dbg!(&player.extras.stats);
	}
}
