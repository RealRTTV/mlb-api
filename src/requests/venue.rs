//! Venues; Yankee Stadium, Rogers Centre, Dodger Stadium, etc.

use crate::season::SeasonId;
use crate::Copyright;
use crate::request::RequestURL;
use bon::Builder;
use derive_more::{Deref, DerefMut};
use itertools::Itertools;
use serde::Deserialize;
use std::fmt::{Display, Formatter};
use std::marker::PhantomData;
use crate::cache::{Requestable};
use crate::sport::SportId;

#[cfg(feature = "cache")]
use crate::{rwlock_const_new, RwLock, cache::CacheTable};
use crate::hydrations::Hydrations;

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase", bound = "H: VenueHydrations")]
pub struct VenuesResponse<H: VenueHydrations> {
	pub copyright: Copyright,
	pub venues: Vec<Venue<H>>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NamedVenue {
	pub name: String,
	#[serde(flatten)]
	pub id: VenueId,
}

impl NamedVenue {
	pub(crate) fn unknown_venue() -> Self {
		Self {
			name: "null".to_owned(),
			id: VenueId::new(0),
		}
	}

	#[must_use]
	pub fn is_unknown(&self) -> bool {
		*self.id == 0
	}
}

#[derive(Debug, Deserialize, Deref, DerefMut, Clone)]
#[serde(rename_all = "camelCase", bound = "H: VenueHydrations")]
pub struct Venue<H: VenueHydrations = ()> {
	pub active: bool,
	pub season: SeasonId,

	#[deref]
	#[deref_mut]
	#[serde(flatten)]
	pub inner: NamedVenue,

	#[serde(flatten)]
	pub extras: H,
}

id!(VenueId { id: u32 });
id_only_eq_impl!(NamedVenue, id);

impl<H: VenueHydrations> PartialEq for Venue<H> {
	fn eq(&self, other: &Self) -> bool {
		self.id == other.id
	}
}
impl<H: VenueHydrations> Eq for Venue<H> {}

pub trait VenueHydrations: Hydrations<RequestData=()> {}

impl VenueHydrations for () {}

/// Creates hydrations for a venue
///
/// ## Examples
/// ```no_run
/// ```
///
/// ## Venue Hydrations
/// | Name                  | Type                       |
/// |-----------------------|----------------------------|
/// | `location`            | [`Location`]               |
/// | `timezone`            | [`TimeZoneData`]           |
/// | `field_info`          | [`FieldInfo`]              |
/// | `external_references` | [`Vec<ExternalReference>`] |
/// | `tracking_system`     | [`TrackingSystem`]         |
///
/// [`Location`]: crate::types::Location
/// [`TimeZoneData`]: crate::types::TimeZoneData
/// [`FieldInfo`]: crate::types::FieldInfo
/// [`Vec<ExternalReference>`]: crate::types::ExternalReference
/// [`TrackingSystem`]: crate::types::TrackingSystem
#[macro_export]
macro_rules! venue_hydrations {
    (@ inline_structs [$field:ident $(: $value: path)? $(, $($tt:tt)*)?] $vis:vis struct $name:ident { $($field_tt:tt)* }) => {
	    $crate::venue_hydrations! { @ inline_structs [$($($tt)*)?] $vis struct $name { $($field_tt)* $field $(: $value)?, } }
    };
	(@ inline_structs [$(,)?] $vis:vis struct $name:ident { $($field_tt:tt)* }) => {
		$crate::venue_hydrations! { @ actual $vis struct $name { $($field_tt)* } }
	};
	(@ actual $vis:vis struct $name:ident {
		$(location $location_comma:tt)?
		$(timezone $timezone_comma:tt)?
		$(field_info $field_info_comma:tt)?
		$(external_references $external_references_comma:tt)?
		$(tracking_system $tracking_system_comma:tt)?
	}) => {
		#[derive(::core::fmt::Debug, ::serde::Deserialize, ::core::cmp::PartialEq, ::core::cmp::Eq, ::core::clone::Clone)]
		#[serde(rename_all = "camelCase")]
		$vis struct $name {
			$(#[serde(default)] pub location: $crate::types::Location $location_comma)?
			$(#[serde(rename = "timeZone")] pub timezone: $crate::types::TimeZoneData $timezone_comma)?
			$(#[serde(rename = "fieldInfo")] pub field_info: $crate::types::FieldInfo $field_info_comma)?
			$(#[serde(default, rename = "xrefIds")] pub external_references: ::std::vec::Vec<$crate::types::ExternalReference> $external_references_comma)?
			$(#[serde(rename = "trackingVersion")] pub tracking_system: ::core::option::Option<$crate::types::TrackingSystem> $tracking_system_comma)?
		}

		impl $crate::venue::VenueHydrations for $name {}

		impl $crate::hydrations::Hydrations for $name {
			type RequestData = ();

			fn hydration_text(&(): &Self::RequestData) -> ::std::borrow::Cow<'static, str> {
				::std::borrow::Cow::Borrowed(::std::concat!(
					$("location," $location_comma)?
					$("timezone," $timezone_comma)?
					$("fieldInfo," $field_info_comma)?
					$("xrefId," $external_references_comma)?
					$("trackingVersion," $tracking_system_comma)?
				))
			}
		}
	};
	($vis:vis struct $name:ident { $($tt:tt)* }) => {
		venue_hydrations! { @ inline_structs [$($tt)*] $vis struct $name {} }
	};
}

#[cfg(feature = "cache")]
static CACHE: RwLock<CacheTable<Venue<()>>> = rwlock_const_new(CacheTable::new());

impl Requestable for Venue<()> {
	type Identifier = VenueId;
	type URL = VenuesRequest<()>;

	fn id(&self) -> &Self::Identifier {
		&self.id
	}

	#[cfg(feature = "aggressive_cache")]
	fn url_for_id(_id: &Self::Identifier) -> Self::URL {
		VenuesRequest::builder().build()
	}

	#[cfg(not(feature = "aggressive_cache"))]
	fn url_for_id(id: &Self::Identifier) -> Self::URL {
		VenuesRequest::builder().venue_ids(vec![*id]).build()
	}

	fn get_entries(response: <Self::URL as RequestURL>::Response) -> impl IntoIterator<Item=Self>
	where
		Self: Sized
	{
		response.venues
	}

	#[cfg(feature = "cache")]
	fn get_cache_table() -> &'static RwLock<CacheTable<Self>>
	where
		Self: Sized
	{
		&CACHE
	}
}

#[derive(Builder)]
#[builder(derive(Into))]
pub struct VenuesRequest<H: VenueHydrations> {
	#[builder(into, default)]
	sport_id: SportId,
	venue_ids: Option<Vec<VenueId>>,
	#[builder(into)]
	season: Option<SeasonId>,
	#[builder(skip)]
	_marker: PhantomData<H>,
}

impl VenuesRequest<()> {
	pub fn for_sport(sport_id: impl Into<SportId>) -> VenuesRequestBuilder<(), venues_request_builder::SetSportId> {
		Self::builder().sport_id(sport_id)
	}
}

impl<H: VenueHydrations, S: venues_request_builder::State + venues_request_builder::IsComplete> crate::request::RequestURLBuilderExt for VenuesRequestBuilder<H, S> {
	type Built = VenuesRequest<H>;
}

impl<H: VenueHydrations> Display for VenuesRequest<H> {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		let hydrations = Some(H::hydration_text(&())).filter(|s| !s.is_empty());
		write!(f, "http://statsapi.mlb.com/api/v1/venues{}", gen_params! { "season"?: self.season, "sportId": self.sport_id, "venueIds"?: self.venue_ids.as_ref().map(|ids| ids.iter().join(",")), "hydrate"?: hydrations })
	}
}

impl<H: VenueHydrations> RequestURL for VenuesRequest<H> {
	type Response = VenuesResponse<H>;
}

#[cfg(test)]
mod tests {
	use crate::request::RequestURLBuilderExt;
	use crate::sport::SportId;
	use crate::TEST_YEAR;
	use crate::venue::VenuesRequest;

	#[tokio::test]
	#[cfg_attr(not(feature = "_heavy_tests"), ignore)]
	async fn parse_all_venues_all_seasons() {
		for season in 1876..=TEST_YEAR {
			let _response = VenuesRequest::<()>::builder().season(season).build_and_get().await.unwrap();
		}
	}

	#[tokio::test]
	async fn parse_all_venues() {
		let _response = VenuesRequest::<()>::builder().build_and_get().await.unwrap();
	}

	#[tokio::test]
	async fn parse_all_mlb_venues_hydrated() {
		venue_hydrations! {
			pub struct ExampleHydrations {
				location,
				timezone,
				field_info,
				external_references,
				tracking_system,
			}
		}

		let _response = VenuesRequest::<ExampleHydrations>::builder().sport_id(SportId::MLB).build_and_get().await.unwrap();
	}
}
