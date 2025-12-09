pub mod players;

use crate::cache::{EndpointEntryCache, HydratedCacheTable};
use crate::types::Copyright;
use crate::StatsAPIEndpointUrl;
use crate::{gen_params, rwlock_const_new, RwLock};
use bon::Builder;
use derive_more::{Deref, DerefMut, Display, From};
use mlb_api_proc::{EnumTryAs, EnumTryAsMut, EnumTryInto};
use serde::Deserialize;
use std::fmt::{Display, Formatter};
use std::ops::{Deref, DerefMut};

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SportsResponse {
	pub copyright: Copyright,
	pub sports: Vec<Sport>,
}

#[repr(transparent)]
#[derive(Debug, Deserialize, Deref, Display, PartialEq, Eq, Copy, Clone, Hash, From)]
pub struct SportId(pub(super) u32);

impl SportId {
	#[must_use]
	pub const fn new(id: u32) -> Self {
		Self(id)
	}

	/// This is here because we can rest assured that it won't ever go away.
	pub const MLB: Self = Self::new(1);
}

impl Default for SportId {
	fn default() -> Self {
		Self::MLB
	}
}

#[derive(Builder)]
#[builder(derive(Into))]
pub struct SportsEndpoint {
	#[builder(into)]
	id: Option<SportId>,
}

impl<S: sports_endpoint_builder::State> crate::endpoints::links::StatsAPIEndpointUrlBuilderExt for SportsEndpointBuilder<S> where S: sports_endpoint_builder::IsComplete {
	type Built = SportsEndpoint;
}

impl Display for SportsEndpoint {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "http://statsapi.mlb.com/api/v1/sports{}", gen_params! { "sportId"?: self.id })
	}
}

impl StatsAPIEndpointUrl for SportsEndpoint {
	type Response = SportsResponse;
}

#[derive(Debug, Deserialize, PartialEq, Eq, Copy, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IdentifiableSport {
	pub id: SportId,
}

#[derive(Debug, Deserialize, Deref, DerefMut, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NamedSport {
	pub name: String,

	#[deref]
	#[deref_mut]
	#[serde(flatten)]
	pub(super) inner: IdentifiableSport,
}

#[derive(Debug, Deserialize, Deref, DerefMut, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HydratedSport {
	pub code: String,
	pub abbreviation: String,
	#[serde(rename = "activeStatus")]
	pub active: bool,

	#[deref]
	#[deref_mut]
	#[serde(flatten)]
	pub(super) inner: NamedSport,
}

#[derive(Debug, Deserialize, Eq, Clone, From, EnumTryAs, EnumTryAsMut, EnumTryInto)]
#[serde(untagged)]
pub enum Sport {
	Hydrated(HydratedSport),
	Named(NamedSport),
	Identifiable(IdentifiableSport),
}

impl PartialEq for Sport {
	fn eq(&self, other: &Self) -> bool {
		self.id == other.id
	}
}

impl Deref for Sport {
	type Target = IdentifiableSport;

	fn deref(&self) -> &Self::Target {
		match self {
			Self::Hydrated(inner) => inner,
			Self::Named(inner) => inner,
			Self::Identifiable(inner) => inner,
		}
	}
}

impl DerefMut for Sport {
	fn deref_mut(&mut self) -> &mut Self::Target {
		match self {
			Self::Hydrated(inner) => inner,
			Self::Named(inner) => inner,
			Self::Identifiable(inner) => inner,
		}
	}
}

static CACHE: RwLock<HydratedCacheTable<Sport>> = rwlock_const_new(HydratedCacheTable::new());

impl EndpointEntryCache for Sport {
	type HydratedVariant = HydratedSport;
	type Identifier = SportId;
	type URL = SportsEndpoint;

	fn into_hydrated_variant(self) -> Option<Self::HydratedVariant> {
		self.try_into_hydrated()
	}

	fn id(&self) -> &Self::Identifier {
		&self.id
	}

	fn url_for_id(id: &Self::Identifier) -> Self::URL {
		SportsEndpoint::builder().id(id.clone()).build()
	}

	fn get_entries(response: <Self::URL as StatsAPIEndpointUrl>::Response) -> impl IntoIterator<Item=Self>
	where
		Self: Sized
	{
		response.sports
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
	use super::*;
	use crate::StatsAPIEndpointUrlBuilderExt;

	#[tokio::test]
	async fn parse_all_sports() {
		let _result = SportsEndpoint::builder().build_and_get().await.unwrap();
	}
}
