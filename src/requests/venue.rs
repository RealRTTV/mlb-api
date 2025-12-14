use crate::season::SeasonId;
use crate::types::Copyright;
use crate::request::StatsAPIRequestUrl;
use bon::Builder;
use derive_more::{Deref, DerefMut, From};
use itertools::Itertools;
use mlb_api_proc::{EnumDeref, EnumDerefMut, EnumTryAs, EnumTryAsMut, EnumTryInto};
use serde::Deserialize;
use std::fmt::{Display, Formatter};
use crate::sports::SportId;

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VenuesResponse {
	pub copyright: Copyright,
	pub venues: Vec<Venue>,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Copy, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IdentifiableVenue {
	pub id: VenueId,
}

impl Default for IdentifiableVenue {
	fn default() -> Self {
		Self { id: VenueId(0) }
	}
}

#[derive(Debug, Deserialize, Deref, DerefMut, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NamedVenue {
	pub name: String,

	#[deref]
	#[deref_mut]
	#[serde(flatten)]
	inner: IdentifiableVenue,
}

impl Default for NamedVenue {
	fn default() -> Self {
		Self {
			name: "null".to_owned(),
			inner: IdentifiableVenue::default(),
		}
	}
}

#[derive(Debug, Deserialize, Deref, DerefMut, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HydratedVenue {
	pub active: bool,
	pub season: SeasonId,

	#[deref]
	#[deref_mut]
	#[serde(flatten)]
	pub inner: NamedVenue,
}

integer_id!(VenueId);

#[derive(Debug, Deserialize, Eq, Clone, From, EnumTryAs, EnumTryAsMut, EnumTryInto, EnumDeref, EnumDerefMut)]
#[serde(untagged)]
pub enum Venue {
	Hydrated(Box<HydratedVenue>),
	Named(NamedVenue),
	Identifiable(IdentifiableVenue),
}

impl Venue {
	#[must_use]
	pub fn unknown_venue() -> Self {
		Self::Named(NamedVenue::default())
	}
}

id_only_eq_impl!(Venue, id);

#[derive(Builder)]
#[builder(derive(Into))]
pub struct VenuesRequest {
	#[builder(into, default)]
	sport_id: SportId,
	venue_ids: Option<Vec<VenueId>>,
	#[builder(into)]
	season: Option<SeasonId>,
}

impl<S: venues_request_builder::State + venues_request_builder::IsComplete> crate::request::StatsAPIRequestUrlBuilderExt for VenuesRequestBuilder<S> {
	type Built = VenuesRequest;
}

impl Display for VenuesRequest {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "http://statsapi.mlb.com/api/v1/venues{}", gen_params! { "season"?: self.season, "sportId": self.sport_id, "venueIds"?: self.venue_ids.as_ref().map(|ids| ids.iter().join(",")) })
	}
}

impl StatsAPIRequestUrl for VenuesRequest {
	type Response = VenuesResponse;
}

tiered_request_entry_cache_impl!(VenuesRequest => |id: VenueId| { VenuesRequest::builder().venue_ids(vec![*id]).build() }.venues => Venue => Box<HydratedVenue>);

#[cfg(test)]
mod tests {
	use crate::request::StatsAPIRequestUrlBuilderExt;
	use crate::TEST_YEAR;
	use crate::venue::VenuesRequest;

	#[tokio::test]
	#[cfg_attr(not(feature = "_heavy_tests"), ignore)]
	async fn parse_all_venues_all_seasons() {
		for season in 1876..=TEST_YEAR {
			let _response = VenuesRequest::builder().season(season).build_and_get().await.unwrap();
		}
	}

	#[tokio::test]
	async fn parse_all_venues() {
		let _response = VenuesRequest::builder().build_and_get().await.unwrap();
	}
}
