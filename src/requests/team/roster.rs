//! Returns a list of [`RosterPlayer`]s on a teams roster.

use crate::person::NamedPerson;
use crate::season::SeasonId;
use crate::team::TeamId;
use crate::{Copyright, MLB_API_DATE_FORMAT};
use bon::Builder;
use chrono::NaiveDate;
use serde::Deserialize;
use std::fmt::{Debug, Display, Formatter};
use serde::de::DeserializeOwned;
use crate::hydrations::Hydrations;
use crate::meta::NamedPosition;
use crate::request::RequestURL;
use crate::meta::RosterType;
use crate::team::NamedTeam;

/// Returns a [`Vec`] of [`RosterPlayer`]s for a team.
#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase", bound = "H: RosterHydrations")]
pub struct RosterResponse<H: RosterHydrations = ()> {
    pub copyright: Copyright,
    #[serde(default)]
    pub roster: Vec<RosterPlayer<H>>,
    pub team_id: TeamId,
    pub roster_type: RosterType,
}

// A [`NamedPerson`] on a roster, has an assigned position.
#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RosterPlayer<H: RosterHydrations = ()> {
    pub person: H::Person,
    #[serde(deserialize_with = "crate::try_from_str")]
    pub jersey_number: Option<u8>,
    pub position: NamedPosition,
    pub status: RosterStatus,
    pub parent_team_id: Option<TeamId>,
}

/// Status on the roster
#[derive(Debug, Deserialize, PartialEq, Eq, Copy, Clone)]
#[serde(try_from = "__RosterStatusStruct")]
pub enum RosterStatus {
    Active,
    FortyMan,
    Claimed,
    ReassignedToMinors,
    Released,
    MinorLeagueContract,
    InjuryLeave7Day,
    InjuryLeave10Day,
    InjuryLeave15Day,
    InjuryLeave60Day,
    Traded,
    DesignatedForAssignment,
    FreeAgent,
    RestrictedList,
    AssignedToNewTeam,
    RehabAssignment,
    NonRosterInvitee,
    Waived,
    Deceased,
    VoluntarilyRetired,
}

#[derive(Deserialize)]
#[doc(hidden)]
struct __RosterStatusStruct {
    code: String,
    description: String,
}

impl TryFrom<__RosterStatusStruct> for RosterStatus {
    type Error = String;

    fn try_from(value: __RosterStatusStruct) -> Result<Self, Self::Error> {
        Ok(match &*value.code {
            "A" => Self::Active,
            "40M" => Self::FortyMan,
            "CL" => Self::Claimed,
            "RM" => Self::ReassignedToMinors,
            "RL" => Self::Released,
            "MIN" => Self::MinorLeagueContract,
            "D7" => Self::InjuryLeave7Day,
            "D10" => Self::InjuryLeave10Day,
            "D15" => Self::InjuryLeave15Day,
            "D60" => Self::InjuryLeave60Day,
            "TR" => Self::Traded,
            "DES" => Self::DesignatedForAssignment,
            "FA" => Self::FreeAgent,
            "RST" => Self::RestrictedList,
            "ASG" => Self::AssignedToNewTeam,
            "RA" => Self::RehabAssignment,
            "NRI" => Self::NonRosterInvitee,
            "WA" => Self::Waived,
            "DEC" => Self::Deceased,
            "RET" => Self::VoluntarilyRetired,
            code => return Err(format!("Invalid code '{code}' (desc: {})", value.description)),
        })
    }
}

/// Returns a [`RosterResponse`]
#[derive(Builder)]
#[builder(derive(Into))]
#[allow(unused)]
pub struct RosterRequest<H: RosterHydrations = ()> {
    #[builder(into)]
    team_id: TeamId,
    #[builder(into)]
    season: Option<SeasonId>,
    date: Option<NaiveDate>,
    #[builder(into, default)]
    roster_type: RosterType,
    #[builder(into)]
    hydrations: H::RequestData,
}

impl<H: RosterHydrations, S: roster_request_builder::State + roster_request_builder::IsComplete> crate::request::RequestURLBuilderExt for RosterRequestBuilder<H, S> {
    type Built = RosterRequest<H>;
}

impl RosterRequest {
    pub fn for_team(team_id: impl Into<TeamId>) -> RosterRequestBuilder<(), roster_request_builder::SetHydrations<roster_request_builder::SetTeamId>> {
        Self::builder().team_id(team_id).hydrations(<() as Hydrations>::RequestData::default())
    }
}

impl<H: RosterHydrations> Display for RosterRequest<H> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let hydrations = Some(<H as Hydrations>::hydration_text(&self.hydrations)).filter(|s| !s.is_empty());
        write!(f, "http://statsapi.mlb.com/api/v1/teams/{}/roster{}", self.team_id, gen_params! { "season"?: self.season, "date"?: self.date.as_ref().map(|date| date.format(MLB_API_DATE_FORMAT)), "rosterType": &self.roster_type, "hydrate"?: hydrations })
    }
}

impl<H: RosterHydrations> RequestURL for RosterRequest<H> {
    type Response = RosterResponse<H>;
}

/// A [`Person`](crate::person::Person)s entry on a roster.
#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RosterEntry {
    pub position: NamedPosition,
    pub status: RosterStatus,
    pub team: NamedTeam,
    pub is_active: bool,
    pub is_active_forty_man: bool,
    pub start_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
    pub status_date: NaiveDate,
}

/// A type that is made with [`roster_hydrations!`](crate::roster_hydrations)
pub trait RosterHydrations: Hydrations {
    /// [`NamedPerson`] when no hydrations are present and [`Person`](crate::person::Person) when they are.
    type Person: Debug + DeserializeOwned + Eq + Clone;
}

impl RosterHydrations for () {
    type Person = NamedPerson;
}

/// Creates hydrations for a [`RosterRequest`].
///
/// ## Roster Hydrations
/// | Name     | Type                                             |
/// |----------|--------------------------------------------------|
/// | `person` | [`person_hydrations!`](crate::person_hydrations) |
///
/// ## Examples
/// ```norun
/// person_hydrations! {
///     pub struct ExamplePersonHydrations {
///         nicknames
///     }
/// }
///
/// roster_hydrations! {
///     pub struct ExampleRosterHydrations {
///         person: ExamplePersonHydrations
///     }
/// }
///
/// // alternatively you can inline these hydrations
/// roster_hydrations! {
///     pub struct ExampleRosterHydrations {
///         person: { nicknames }
///     }
/// }
///
/// let request = RosterRequest::<ExamplePersonHydrations>::builder()
///     .team_id(141)
///     .hydrations(ExampleRosterHydrations::builder()
///         .person(ExamplePersonHydrations::builder()
///             .build())
///         .build())
///     .build();
/// let response = request.get();
///
/// // note that assuming there isn't anything required to be specified, Default can be used on these builders
/// let request = RosterRequest::<ExamplePersonHydrations>::builder()
///     .team_id(141)
///     .hydrations(ExampleRosterHydrationsRequestData::default())
///     .build();
/// ```
#[macro_export]
macro_rules! roster_hydrations {
    (@ inline_structs [person: { $($contents:tt)* } $(, $($rest:tt)*)?] $vis:vis struct $name:ident { $($field_tt:tt)* }) => {
        ::pastey::paste! {
            $crate::person_hydrations! {
                $vis struct [<$name InlinePersonHydrations>] {
                    $($contents)*
                }
            }

            $crate::roster_hydrations! { @ inline_structs [$($($rest)*)?]
                $vis struct $name {
                    $($field_tt)*
                    person: [<$name InlinePersonHydrations>],
                }
            }
        }
    };
    (@ inline_structs [$marker:ident : { $($contents:tt)* } $(, $($rest:tt)*)?] $vis:vis struct $name:ident { $($field_tt:tt)* }) => {
        compile_error!("Found unknown inline struct");
    };
    (@ inline_structs [$marker:ident $(: $value:path)? $(, $($rest:tt)*)?] $vis:vis struct $name:ident { $($field_tt:tt)* }) => {
        ::pastey::paste! {
            $crate::roster_hydrations! { @ inline_structs [$($($rest)*)?]
                $vis struct $name {
                    $($field_tt)*
                    $marker $(: $value)?,
                }
            }
        }
    };
    (@ inline_structs [$(,)?] $vis:vis struct $name:ident { $($field_tt:tt)* }) => {
        ::pastey::paste! {
            $crate::roster_hydrations! { @ actual
                $vis struct $name {
                    $($field_tt)*
                }
            }
        }
    };
    ($vis:vis struct $name:ident {
        $($contents:tt)*
    }) => {
        $crate::roster_hydrations! { @ inline_structs [$($contents)*] $vis struct $name {} }
    };
    (@ person_type [person: $person:ident $(,)?]) => {
        $crate::person::Person<$person>
    };
    (@ person_type [$_01:ident (: $_02:path) $(, $($rest:tt)*)?]) => {
        $crate::roster_hydrations! { @ person_type [$($($rest)*)?] }
    };
    (@ person_type [$(,)?]) => {
        $crate::person::NamedPerson
    };
    (@ actual $vis:vis struct $name:ident {
        $(person: $person:ident ,)?
    }) => {
        ::pastey::paste! {
            #[derive(::core::fmt::Debug, ::serde::Deserialize, ::core::cmp::PartialEq, ::core::cmp::Eq, ::core::clone::Clone)]
            #[serde(rename_all = "camelCase")]
            $vis struct $name {}

            impl $crate::team::roster::RosterHydrations for $name {
                type Person = $crate::roster_hydrations!(@ person_type [$(person: $person ,)?]);
            }

            impl $crate::hydrations::Hydrations for $name {
                type RequestData = [<$name RequestData>];

                #[allow(unused_variables)]
                fn hydration_text(_data: &Self::RequestData) -> ::std::borrow::Cow<'static, str> {
                    let text = ::std::borrow::Cow::Borrowed("");

                    $(
                    let text = ::std::borrow::Cow::Owned(::std::format!("person({})", <$person as $crate::hydrations::Hydrations>::hydration_text(&_data.person)));
                    )?

                    text
                }
            }

            #[derive(::bon::Builder)]
            #[builder(derive(Into))]
            $vis struct [<$name RequestData>] {
                $(#[builder(into)] person: <$person as $crate::hydrations::Hydrations>::RequestData,)?
            }

            impl $name {
				#[allow(unused)]
				pub fn builder() -> [<$name RequestDataBuilder>] {
					[<$name RequestData>]::builder()
				}
			}

            impl ::core::default::Default for [<$name RequestData>]
			where
				$(for<'no_rfc_2056> <$person as $crate::hydrations::Hydrations>::RequestData: ::core::default::Default,)?
			{
				fn default() -> Self {
					Self {
						$(person: <<$person as $crate::hydrations::Hydrations>::RequestData as ::core::default::Default>::default(),)?
					}
				}
			}
        }
    };
}

#[cfg(test)]
mod tests {
	use crate::meta::MetaRequest;
    use crate::request::{RequestURL, RequestURLBuilderExt};
    use crate::meta::RosterType;
    use crate::team::roster::RosterRequest;
	use crate::team::TeamsRequest;
    use crate::TEST_YEAR;

    #[tokio::test]
    #[cfg_attr(not(feature = "_heavy_tests"), ignore)]
    async fn test_this_year_all_mlb_teams_all_roster_types() {
        let season = TEST_YEAR;
        let teams = TeamsRequest::mlb_teams().season(season).build_and_get().await.unwrap().teams;
        let roster_types = MetaRequest::<RosterType>::new().get().await.unwrap().entries;
        for team in teams {
            for roster_type in &roster_types {
                let _ = RosterRequest::<()>::for_team(team.id).season(season).roster_type(*roster_type).build_and_get().await.unwrap();
            }
        }
    }

    #[tokio::test]
    async fn hydrations_test() {
        roster_hydrations! {
            pub struct ExampleHydrations {
                person: {
                    nicknames
                },
            }
        }

        let request = RosterRequest::<ExampleHydrations>::builder().hydrations(ExampleHydrationsRequestData::default()).team_id(141).season(TEST_YEAR).roster_type(RosterType::default()).build();
        // println!("Request: {request}");
        let _response = request.get().await.unwrap();
        /*for entry in _response.roster {
            if let Person::Ballplayer(ballplayer) = entry.person {
                println!("Name: {}, Nicknames: {:?}", ballplayer.full_name, ballplayer.extras.nicknames);
            }
        }*/
    }
}

