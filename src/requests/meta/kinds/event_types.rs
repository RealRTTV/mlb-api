use crate::cache::{HydratedCacheTable, RequestEntryCache};
use crate::meta::{MetaKind, MetaRequest};
use crate::{rwlock_const_new, RwLock};
use crate::{string_id, StatsAPIRequestUrl};
use derive_more::{Deref, DerefMut, From};
use mlb_api_proc::{EnumTryAs, EnumTryAsMut, EnumTryInto};
use serde::Deserialize;
use std::ops::{Deref, DerefMut};

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IdentifiableEventType {
	#[serde(rename = "code")] pub id: EventTypeId,
}

string_id!(EventTypeId);

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

impl RequestEntryCache for EventType {
	type HydratedVariant = HydratedEventType;
	type Identifier = EventTypeId;
	type URL = MetaRequest<Self>;

	fn into_hydrated_variant(self) -> Option<Self::HydratedVariant> {
		self.try_into_hydrated()
	}

	fn id(&self) -> &Self::Identifier {
		&self.id
	}

	fn url_for_id(_id: &Self::Identifier) -> Self::URL {
		MetaRequest::new()
	}

	fn get_entries(response: <Self::URL as StatsAPIRequestUrl>::Response) -> impl IntoIterator<Item=Self>
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
	use crate::meta::MetaRequest;
	use crate::StatsAPIRequestUrl;

	#[tokio::test]
	async fn parse_meta() {
		let _response = MetaRequest::<super::EventType>::new().get().await.unwrap();
	}
}
