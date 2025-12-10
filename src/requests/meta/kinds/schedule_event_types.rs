use crate::cache::{RequestEntryCache, HydratedCacheTable};
use crate::meta::{MetaRequest, MetaKind};
use crate::StatsAPIRequestUrl;
use crate::{rwlock_const_new, RwLock};
use derive_more::{Deref, DerefMut, Display, From};
use mlb_api_proc::{EnumTryAs, EnumTryAsMut, EnumTryInto};
use serde::Deserialize;
use std::ops::{Deref, DerefMut};

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
pub struct IdentifiableScheduleEventType {
	#[serde(rename = "code")] pub id: ScheduleEventTypeId,
}

#[repr(transparent)]
#[derive(Debug, Deserialize, Deref, Display, PartialEq, Eq, Clone, Hash, From)]
pub struct ScheduleEventTypeId(String);

#[derive(Debug, Deserialize, Deref, DerefMut, PartialEq, Eq, Clone)]
pub struct HydratedScheduleEventType {
	pub name: String,

	#[deref]
	#[deref_mut]
	#[serde(flatten)]
	inner: IdentifiableScheduleEventType,
}

#[derive(Debug, Deserialize, Eq, Clone, From, EnumTryAs, EnumTryAsMut, EnumTryInto)]
#[serde(untagged)]
pub enum ScheduleEventType {
	Hydrated(Box<HydratedScheduleEventType>),
	Identifiable(IdentifiableScheduleEventType),
}

impl PartialEq for ScheduleEventType {
	fn eq(&self, other: &Self) -> bool {
		self.id == other.id
	}
}

impl Deref for ScheduleEventType {
	type Target = IdentifiableScheduleEventType;

	fn deref(&self) -> &Self::Target {
		match self {
			Self::Hydrated(inner) => inner,
			Self::Identifiable(inner) => inner,
		}
	}
}

impl DerefMut for ScheduleEventType {
	fn deref_mut(&mut self) -> &mut Self::Target {
		match self {
			Self::Hydrated(inner) => inner,
			Self::Identifiable(inner) => inner,
		}
	}
}

impl MetaKind for ScheduleEventType {
	const ENDPOINT_NAME: &'static str = "scheduleEventTypes";
}

static CACHE: RwLock<HydratedCacheTable<ScheduleEventType>> = rwlock_const_new(HydratedCacheTable::new());

impl RequestEntryCache for ScheduleEventType {
	type HydratedVariant = Box<HydratedScheduleEventType>;
	type Identifier = ScheduleEventTypeId;
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
		let _response = MetaRequest::<super::ScheduleEventType>::new().get().await.unwrap();
	}
}
