use crate::season::SeasonId;
use crate::types::Copyright;
use crate::request::StatsAPIRequestUrl;
use bon::Builder;
use derive_more::{Deref, DerefMut};
use itertools::Itertools;
use serde::Deserialize;
use std::fmt::{Display, Formatter};
use crate::cache::{CacheTable, RequestEntryCache};
use crate::{rwlock_const_new, RwLock};
use crate::sport::SportId;

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VenuesResponse {
	pub copyright: Copyright,
	pub venues: Vec<Venue>,
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
#[serde(rename_all = "camelCase")]
pub struct Venue {
	pub active: bool,
	pub season: SeasonId,

	#[deref]
	#[deref_mut]
	#[serde(flatten)]
	pub inner: NamedVenue,
}

id!(VenueId { id: u32 });
id_only_eq_impl!(NamedVenue, id);
id_only_eq_impl!(Venue, id);

static CACHE: RwLock<CacheTable<Venue>> = rwlock_const_new(CacheTable::new());

impl RequestEntryCache for Venue {
	type Identifier = VenueId;
	type URL = VenuesRequest;

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

	fn get_entries(response: <Self::URL as StatsAPIRequestUrl>::Response) -> impl IntoIterator<Item=Self>
	where
		Self: Sized
	{
		response.venues
	}

	fn get_cache_table() -> &'static RwLock<CacheTable<Self>>
	where
		Self: Sized
	{
		&CACHE
	}
}

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
