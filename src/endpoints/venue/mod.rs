use crate::endpoints::StatsAPIEndpointUrl;
use crate::endpoints::sports::SportId;
use crate::{gen_params, rwlock_const_new, RwLock};
use crate::types::Copyright;
use derive_more::{Deref, DerefMut, Display, From};
use serde::Deserialize;
use serde_with::DisplayFromStr;
use serde_with::serde_as;
use std::fmt::{Display, Formatter};
use std::ops::{Deref, DerefMut};
use itertools::Itertools;
use mlb_api_proc::{EnumTryAs, EnumTryAsMut, EnumTryInto};
use crate::cache::{EndpointEntryCache, HydratedCacheTable};

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

#[serde_as]
#[derive(Debug, Deserialize, Deref, DerefMut, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HydratedVenue {
	pub active: bool,
	#[serde_as(as = "DisplayFromStr")]
	pub season: u16,

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

#[derive(Default)]
pub struct VenuesEndpoint {
	pub sport_id: Option<SportId>,
	pub venue_ids: Option<Vec<VenueId>>,
	pub season: Option<u16>,
}

impl Display for VenuesEndpoint {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "http://statsapi.mlb.com/api/v1/venues{}{}", self.sport_id.map_or(String::new(), |id| format!("/{id}")), gen_params! { "season"?: self.season, "venueIds"?: self.venue_ids.as_ref().map(|ids| ids.iter().join(",")) })
	}
}

impl StatsAPIEndpointUrl for VenuesEndpoint {
	type Response = VenuesResponse;
}

static CACHE: RwLock<HydratedCacheTable<Venue>> = rwlock_const_new(HydratedCacheTable::new());

impl EndpointEntryCache for Venue {
	type HydratedVariant = HydratedVenue;
	type Identifier = VenueId;
	type URL = VenuesEndpoint;

	fn into_hydrated_variant(self) -> Option<Self::HydratedVariant> {
		self.try_into_hydrated()
	}

	fn id(&self) -> &Self::Identifier {
		&self.id
	}

	fn url_for_id(id: &Self::Identifier) -> Self::URL {
		VenuesEndpoint {
			sport_id: None,
			venue_ids: Some(vec![id.clone()]),
			season: None,
		}
	}

	fn get_entries(response: <Self::URL as StatsAPIEndpointUrl>::Response) -> impl IntoIterator<Item=Self>
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
	use crate::endpoints::StatsAPIEndpointUrl;
	use crate::endpoints::venue::VenuesEndpoint;
	use chrono::{Datelike, Local};

	#[tokio::test]
	#[cfg_attr(not(feature = "_heavy_tests"), ignore)]
	async fn parse_all_venues_all_seasons() {
		for season in 1876..=Local::now().year() as _ {
			let _response = VenuesEndpoint { sport_id: None, season: Some(season), venue_ids: None }.get().await.unwrap();
		}
	}

	async fn parse_all_venues() {
		let _response = VenuesEndpoint { sport_id: None, season: None, venue_ids: None }.get().await.unwrap();
	}
}
