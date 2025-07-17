use crate::endpoints::meta::{MetaEndpointUrl, MetaKind};
use derive_more::{Deref, DerefMut, Display, From};
use serde::Deserialize;
use std::ops::{Deref, DerefMut};
use strum::EnumTryAs;
use crate::cache::{EndpointEntryCache, HydratedCacheTable};
use crate::{rwlock_const_new, RwLock};
use crate::endpoints::StatsAPIUrl;

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
pub struct IdentifiableSkyDescription {
	pub id: SkyDescriptionId,
}

#[repr(transparent)]
#[derive(Debug, Deserialize, Deref, Display, PartialEq, Eq, Clone, Hash)]
pub struct SkyDescriptionId(String);

#[derive(Debug, Deserialize, Deref, DerefMut, PartialEq, Eq, Clone)]
pub struct HydratedSkyDescription {
	pub description: String,

	#[deref]
	#[deref_mut]
	#[serde(flatten)]
	inner: IdentifiableSkyDescription,
}

#[derive(Debug, Deserialize, Eq, Clone, From, EnumTryAs)]
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
	type URL = MetaEndpointUrl<Self>;

	fn into_hydrated_variant(self) -> Option<Self::HydratedVariant> {
		self.try_as_hydrated()
	}

	fn id(&self) -> &Self::Identifier {
		&self.id
	}

	fn url_for_id(_id: &Self::Identifier) -> Self::URL {
		MetaEndpointUrl::new()
	}

	fn get_entries(response: <Self::URL as StatsAPIUrl>::Response) -> impl IntoIterator<Item=Self>
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
	use crate::endpoints::StatsAPIUrl;
	use crate::endpoints::meta::MetaEndpointUrl;

	#[tokio::test]
	async fn parse_meta() {
		let _response = MetaEndpointUrl::<super::SkyDescription>::new().get().await.unwrap();
	}
}
