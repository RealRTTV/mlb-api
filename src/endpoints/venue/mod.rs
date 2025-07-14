use crate::endpoints::StatsAPIUrl;
use crate::endpoints::sports::SportId;
use crate::gen_params;
use crate::types::Copyright;
use derive_more::{Deref, DerefMut, Display, From};
use serde::Deserialize;
use serde_with::DisplayFromStr;
use serde_with::serde_as;
use std::fmt::{Display, Formatter};
use std::ops::{Deref, DerefMut};

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VenuesResponse {
	pub copyright: Copyright,
	pub venues: Vec<HydratedVenue>,
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
#[derive(Debug, Deserialize, Deref, Display, PartialEq, Eq, Copy, Clone)]
pub struct VenueId(u32);

#[derive(Debug, Deserialize, Eq, Clone, From)]
#[serde(untagged)]
pub enum Venue {
	Hydrated(HydratedVenue),
	Named(NamedVenue),
	Identifiable(IdentifiableVenue),
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
pub struct VenuesEndpointUrl {
	pub id: Option<SportId>,
	pub season: Option<u16>,
}

impl Display for VenuesEndpointUrl {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"http://statsapi.mlb.com/api/v1/venues{id}{params}",
			id = self.id.map_or(String::new(), |id| format!("/{id}")),
			params = gen_params! { "season"?: self.season }
		)
	}
}

impl StatsAPIUrl<VenuesResponse> for VenuesEndpointUrl {}

#[cfg(test)]
mod tests {
	use crate::endpoints::StatsAPIUrl;
	use crate::endpoints::venue::VenuesEndpointUrl;
	use chrono::{Datelike, Local};

	#[tokio::test]
	#[cfg_attr(not(feature = "_heavy_tests"), ignore)]
	async fn parse_all_venues_all_seasons() {
		for season in 1876..=Local::now().year() as _ {
			let _response = VenuesEndpointUrl { id: None, season: Some(season) }.get().await.unwrap();
		}
	}
	
	async fn parse_all_venues() {
		let _response = VenuesEndpointUrl { id: None, season: None }.get().await.unwrap();
	}
}
