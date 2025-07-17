use crate::endpoints::meta::{MetaEndpointUrl, MetaKind};
use derive_more::{Deref, Display, From};
use serde::Deserialize;
use std::ops::{Deref, DerefMut};
use strum::EnumTryAs;
use crate::cache::{EndpointEntryCache, HydratedCacheTable};
use crate::{rwlock_const_new, RwLock};
use crate::endpoints::StatsAPIUrl;

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
pub struct IdentifiableLogicalEvent {
	#[serde(rename = "code")] pub id: LogicalEventId,
}

#[repr(transparent)]
#[derive(Debug, Deserialize, Deref, Display, PartialEq, Eq, Clone, Hash)]
pub struct LogicalEventId(String);

#[derive(Debug, Deserialize, Eq, Clone, From, EnumTryAs)]
#[serde(untagged)]
pub enum LogicalEvent {
	Identifiable(IdentifiableLogicalEvent),
}

impl PartialEq for LogicalEvent {
	fn eq(&self, other: &Self) -> bool {
		self.id == other.id
	}
}

impl Deref for LogicalEvent {
	type Target = IdentifiableLogicalEvent;

	fn deref(&self) -> &Self::Target {
		match self {
			Self::Identifiable(inner) => inner,
		}
	}
}

impl DerefMut for LogicalEvent {
	fn deref_mut(&mut self) -> &mut Self::Target {
		match self {
			Self::Identifiable(inner) => inner,
		}
	}
}

impl MetaKind for LogicalEvent {
	const ENDPOINT_NAME: &'static str = "logicalEvents";
}

static CACHE: RwLock<HydratedCacheTable<LogicalEvent>> = rwlock_const_new(HydratedCacheTable::new());

impl EndpointEntryCache for LogicalEvent {
	type HydratedVariant = IdentifiableLogicalEvent;
	type Identifier = LogicalEventId;
	type URL = MetaEndpointUrl<Self>;

	fn into_hydrated_entry(self) -> Option<Self::HydratedVariant> {
		self.try_as_identifiable()
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
		let _response = MetaEndpointUrl::<super::LogicalEvent>::new().get().await.unwrap();
	}
}
