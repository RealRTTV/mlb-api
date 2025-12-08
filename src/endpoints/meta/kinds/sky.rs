use crate::endpoints::meta::{MetaEndpoint, MetaKind};
use derive_more::{Deref, DerefMut, Display, From};
use serde::Deserialize;
use std::ops::{Deref, DerefMut};
use mlb_api_proc::{EnumTryAs, EnumTryAsMut, EnumTryInto};
use crate::cache::{EndpointEntryCache, HydratedCacheTable};
use crate::{rwlock_const_new, RwLock};
use crate::endpoints::StatsAPIEndpointUrl;

#[derive(Debug, Deserialize, PartialEq, Eq, Copy, Clone)]
pub enum Sky {
	/// Day Game.
	#[serde(rename = "day")]
	Day,
	
	/// Night Game.
	#[serde(rename = "night")]
	Night,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
pub struct IdentifiableSkyDescription {
	#[serde(rename = "code")]
	pub id: SkyDescriptionId,
}

#[repr(transparent)]
#[derive(Debug, Deserialize, Deref, Display, PartialEq, Eq, Clone, Hash, From)]
pub struct SkyDescriptionId(String);

#[derive(Debug, Deserialize, Deref, DerefMut, PartialEq, Eq, Clone)]
pub struct HydratedSkyDescription {
	pub description: String,

	#[deref]
	#[deref_mut]
	#[serde(flatten)]
	inner: IdentifiableSkyDescription,
}

#[derive(Debug, Deserialize, Eq, Clone, From, EnumTryAs, EnumTryAsMut, EnumTryInto)]
#[serde(untagged)]
pub enum SkyDescription {
	Hydrated(HydratedSkyDescription),
	Identifiable(IdentifiableSkyDescription),
}

impl PartialEq for SkyDescription {
	fn eq(&self, other: &Self) -> bool {
		self.id == other.id
	}
}

impl Deref for SkyDescription {
	type Target = IdentifiableSkyDescription;

	fn deref(&self) -> &Self::Target {
		match self {
			Self::Hydrated(inner) => inner,
			Self::Identifiable(inner) => inner,
		}
	}
}

impl DerefMut for SkyDescription {
	fn deref_mut(&mut self) -> &mut Self::Target {
		match self {
			Self::Hydrated(inner) => inner,
			Self::Identifiable(inner) => inner,
		}
	}
}

impl MetaKind for SkyDescription {
	const ENDPOINT_NAME: &'static str = "sky";
}

static CACHE: RwLock<HydratedCacheTable<SkyDescription>> = rwlock_const_new(HydratedCacheTable::new());

impl EndpointEntryCache for SkyDescription {
	type HydratedVariant = HydratedSkyDescription;
	type Identifier = SkyDescriptionId;
	type URL = MetaEndpoint<Self>;

	fn into_hydrated_variant(self) -> Option<Self::HydratedVariant> {
		self.try_into_hydrated()
	}

	fn id(&self) -> &Self::Identifier {
		&self.id
	}

	fn url_for_id(_id: &Self::Identifier) -> Self::URL {
		MetaEndpoint::new()
	}

	fn get_entries(response: <Self::URL as StatsAPIEndpointUrl>::Response) -> impl IntoIterator<Item=Self>
	where
		Self: Sized
	{
		response.entries
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
	use crate::endpoints::meta::MetaEndpoint;

	#[tokio::test]
	async fn parse_meta() {
		let _response = MetaEndpoint::<super::SkyDescription>::new().get().await.unwrap();
	}
}
