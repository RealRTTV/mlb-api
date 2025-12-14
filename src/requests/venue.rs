use crate::cache::{HydratedCacheTable, RequestEntryCache};
use crate::season::SeasonId;
use crate::types::Copyright;
use crate::{rwlock_const_new, RwLock};
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

static CACHE: RwLock<HydratedCacheTable<Venue>> = rwlock_const_new(HydratedCacheTable::new());

impl RequestEntryCache for Venue {
	type HydratedVariant = Box<HydratedVenue>;
	type Identifier = VenueId;
	type URL = VenuesRequest;

	fn into_hydrated_variant(self) -> Option<Self::HydratedVariant> {
		self.try_into_hydrated()
	}

	fn id(&self) -> &Self::Identifier {
		&self.id
	}

	fn url_for_id(id: &Self::Identifier) -> Self::URL {
		VenuesRequest::builder().venue_ids(vec![*id]).build()
	}

	fn get_entries(response: <Self::URL as StatsAPIRequestUrl>::Response) -> impl IntoIterator<Item=Self>
	where
		Self: Sized
	{
		response.venues
	}

	fn get_hydrated_cache_table() -> &'static RwLock<HydratedCacheTable<Self>>
	where
		Self: Sized
	{
		&CACHE
	}
}

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
