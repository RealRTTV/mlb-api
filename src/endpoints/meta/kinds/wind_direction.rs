use crate::cache::{EndpointEntryCache, HydratedCacheTable};
use crate::meta::{MetaEndpoint, MetaKind};
use crate::StatsAPIEndpointUrl;
use crate::{rwlock_const_new, RwLock};
use derive_more::{Deref, DerefMut, Display, From};
use mlb_api_proc::{EnumTryAs, EnumTryAsMut, EnumTryInto};
use serde::Deserialize;
use std::ops::{Deref, DerefMut};

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
pub struct IdentifiableWindDirection {
	#[serde(rename = "code")] pub id: WindDirectionId,
}

#[repr(transparent)]
#[derive(Debug, Deserialize, Deref, Display, PartialEq, Eq, Clone, Hash, From)]
pub struct WindDirectionId(String);

#[derive(Debug, Deserialize, Deref, DerefMut, PartialEq, Eq, Clone)]
pub struct HydratedWindDirection {
	pub description: String,

	#[deref]
	#[deref_mut]
	#[serde(flatten)]
	inner: IdentifiableWindDirection,
}

#[derive(Debug, Deserialize, Eq, Clone, From, EnumTryAs, EnumTryAsMut, EnumTryInto)]
#[serde(untagged)]
pub enum WindDirection {
	Hydrated(HydratedWindDirection),
	Identifiable(IdentifiableWindDirection),
}

impl PartialEq for WindDirection {
	fn eq(&self, other: &Self) -> bool {
		self.id == other.id
	}
}

impl Deref for WindDirection {
	type Target = IdentifiableWindDirection;

	fn deref(&self) -> &Self::Target {
		match self {
			Self::Hydrated(inner) => inner,
			Self::Identifiable(inner) => inner,
		}
	}
}

impl DerefMut for WindDirection {
	fn deref_mut(&mut self) -> &mut Self::Target {
		match self {
			Self::Hydrated(inner) => inner,
			Self::Identifiable(inner) => inner,
		}
	}
}

impl MetaKind for WindDirection {
	const ENDPOINT_NAME: &'static str = "windDirection";
}

static CACHE: RwLock<HydratedCacheTable<WindDirection>> = rwlock_const_new(HydratedCacheTable::new());

impl EndpointEntryCache for WindDirection {
	type HydratedVariant = HydratedWindDirection;
	type Identifier = WindDirectionId;
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
    use crate::meta::MetaEndpoint;
    use crate::StatsAPIEndpointUrl;

    #[tokio::test]
	async fn parse_meta() {
		let _response = MetaEndpoint::<super::WindDirection>::new().get().await.unwrap();
	}
}
