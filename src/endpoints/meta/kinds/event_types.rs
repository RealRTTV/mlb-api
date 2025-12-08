use std::ops::{Deref, DerefMut};
use crate::endpoints::meta::{MetaEndpoint, MetaKind};
use derive_more::{Deref, DerefMut, Display, From};
use serde::Deserialize;
use mlb_api_proc::{EnumTryAs, EnumTryAsMut, EnumTryInto};
use crate::cache::{EndpointEntryCache, HydratedCacheTable};
use crate::{rwlock_const_new, RwLock};
use crate::endpoints::StatsAPIEndpointUrl;

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IdentifiableEventType {
	#[serde(rename = "code")] pub id: EventTypeId,
}

#[repr(transparent)]
#[derive(Debug, Deserialize, Deref, Display, PartialEq, Eq, Clone, Hash, From)]
pub struct EventTypeId(String);

#[derive(Debug, Deserialize, Deref, DerefMut, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HydratedEventType {
	plate_appearance: bool,
	hit: bool,
	base_running_event: bool,
	description: String,

	#[deref]
	#[deref_mut]
	#[serde(flatten)]
	inner: IdentifiableEventType,
}

#[derive(Debug, Deserialize, Eq, Clone, From, EnumTryAs, EnumTryAsMut, EnumTryInto)]
#[serde(untagged)]
pub enum EventType {
	Hydrated(HydratedEventType),
	Identifiable(IdentifiableEventType),
}

impl PartialEq for EventType {
	fn eq(&self, other: &Self) -> bool {
		self.id == other.id
	}
}

impl Deref for EventType {
	type Target = IdentifiableEventType;

	fn deref(&self) -> &Self::Target {
		match self {
			Self::Hydrated(inner) => inner,
			Self::Identifiable(inner) => inner,
		}
	}
}

impl DerefMut for EventType {
	fn deref_mut(&mut self) -> &mut Self::Target {
		match self {
			Self::Hydrated(inner) => inner,
			Self::Identifiable(inner) => inner,
		}
	}
}

impl MetaKind for EventType {
	const ENDPOINT_NAME: &'static str = "eventTypes";
}

static CACHE: RwLock<HydratedCacheTable<EventType>> = rwlock_const_new(HydratedCacheTable::new());

impl EndpointEntryCache for EventType {
	type HydratedVariant = HydratedEventType;
	type Identifier = EventTypeId;
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
		let _response = MetaEndpoint::<super::EventType>::new().get().await.unwrap();
	}
}
