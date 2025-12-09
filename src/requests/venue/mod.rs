use crate::cache::{RequestEntryCache, HydratedCacheTable};
use crate::seasons::season::SeasonId;
use crate::sports::SportId;
use crate::types::Copyright;
use crate::StatsAPIRequestUrl;
use crate::{gen_params, rwlock_const_new, RwLock};
use bon::Builder;
use derive_more::{Deref, DerefMut, Display, From};
use itertools::Itertools;
use mlb_api_proc::{EnumTryAs, EnumTryAsMut, EnumTryInto};
use serde::Deserialize;
use std::fmt::{Display, Formatter};
use std::ops::{Deref, DerefMut};

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

#[repr(transparent)]
#[derive(Debug, Deserialize, Deref, Display, PartialEq, Eq, Copy, Clone, Hash, From)]
pub struct VenueId(u32);

#[derive(Debug, Deserialize, Eq, Clone, From, EnumTryAs, EnumTryAsMut, EnumTryInto)]
#[serde(untagged)]
pub enum Venue {
	Hydrated(HydratedVenue),
	Named(NamedVenue),
	Identifiable(IdentifiableVenue),
}

impl Venue {
	#[must_use]
	pub fn unknown_venue() -> Self {
		Self::Named(NamedVenue::default())
	}
}

impl PartialEq for Venue {
	fn eq(&self, other: &Self) -> bool {
		self.id == other.id
	}
}

impl Deref for Venue {
	type Target = IdentifiableVenue;

	fn deref(&self) -> &Self::Target {
		match self {
			Self::Hydrated(inner) => inner,
			Self::Named(inner) => inner,
			Self::Identifiable(inner) => inner,
		}
	}
}

impl DerefMut for Venue {
	fn deref_mut(&mut self) -> &mut Self::Target {
		match self {
			Self::Hydrated(inner) => inner,
			Self::Named(inner) => inner,
			Self::Identifiable(inner) => inner,
		}
	}
}

#[derive(Builder)]
#[builder(derive(Into))]
pub struct VenuesRequest {
	#[builder(into)]
	sport_id: Option<SportId>,
	venue_ids: Option<Vec<VenueId>>,
	#[builder(into)]
	season: Option<SeasonId>,
}

impl<S: venues_request_builder::State + venues_request_builder::IsComplete> crate::requests::links::StatsAPIRequestUrlBuilderExt for VenuesRequestBuilder<S> {
	type Built = VenuesRequest;
}

impl Display for VenuesRequest {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "http://statsapi.mlb.com/api/v1/venues{}{}", self.sport_id.map_or(String::new(), |id| format!("/{id}")), gen_params! { "season"?: self.season, "venueIds"?: self.venue_ids.as_ref().map(|ids| ids.iter().join(",")) })
	}
}

impl StatsAPIRequestUrl for VenuesRequest {
	type Response = VenuesResponse;
}

static CACHE: RwLock<HydratedCacheTable<Venue>> = rwlock_const_new(HydratedCacheTable::new());

impl RequestEntryCache for Venue {
	type HydratedVariant = HydratedVenue;
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
	use crate::venue::VenuesRequest;
	use crate::StatsAPIRequestUrlBuilderExt;
	use chrono::{Datelike, Local};

	#[tokio::test]
	#[cfg_attr(not(feature = "_heavy_tests"), ignore)]
	async fn parse_all_venues_all_seasons() {
		for season in 1876..=Local::now().year() as _ {
			let _response = VenuesRequest::builder().season(season).build_and_get().await.unwrap();
		}
	}

	#[tokio::test]
	async fn parse_all_venues() {
		let _response = VenuesRequest::builder().build_and_get().await.unwrap();
	}
}
