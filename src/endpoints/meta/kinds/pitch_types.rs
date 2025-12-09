use crate::cache::{EndpointEntryCache, HydratedCacheTable};
use crate::meta::{MetaEndpoint, MetaKind};
use crate::StatsAPIEndpointUrl;
use crate::{rwlock_const_new, RwLock};
use derive_more::{Deref, DerefMut, Display, From};
use mlb_api_proc::{EnumTryAs, EnumTryAsMut, EnumTryInto};
use serde::Deserialize;
use std::ops::{Deref, DerefMut};

#[derive(Debug, Deserialize, Deref, DerefMut, PartialEq, Eq, Clone)]
pub struct HydratedPitchType {
	pub description: String,

	#[deref]
	#[deref_mut]
	#[serde(flatten)]
	inner: IdentifiablePitchType,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
pub struct IdentifiablePitchType {
	#[serde(rename = "code")] pub id: PitchTypeId,
}

#[repr(transparent)]
#[derive(Debug, Deserialize, Deref, Display, PartialEq, Eq, Clone, Hash, From)]
pub struct PitchTypeId(String);

#[derive(Debug, Deserialize, Eq, Clone, From, EnumTryAs, EnumTryAsMut, EnumTryInto)]
#[serde(untagged)]
pub enum PitchType {
	Hydrated(HydratedPitchType),
	Identifiable(IdentifiablePitchType),
}

impl PartialEq for PitchType {
	fn eq(&self, other: &Self) -> bool {
		self.id == other.id
	}
}

impl Deref for PitchType {
	type Target = IdentifiablePitchType;

	fn deref(&self) -> &Self::Target {
		match self {
			Self::Hydrated(inner) => inner,
			Self::Identifiable(inner) => inner,
		}
	}
}

impl DerefMut for PitchType {
	fn deref_mut(&mut self) -> &mut Self::Target {
		match self {
			Self::Hydrated(inner) => inner,
			Self::Identifiable(inner) => inner,
		}
	}
}

impl MetaKind for PitchType {
	const ENDPOINT_NAME: &'static str = "pitchTypes";
}

static CACHE: RwLock<HydratedCacheTable<PitchType>> = rwlock_const_new(HydratedCacheTable::new());

impl EndpointEntryCache for PitchType {
	type HydratedVariant = HydratedPitchType;
	type Identifier = PitchTypeId;
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
		let _response = MetaEndpoint::<super::PitchType>::new().get().await.unwrap();
	}
}
