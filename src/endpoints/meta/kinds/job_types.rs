use crate::endpoints::meta::{MetaEndpointUrl, MetaKind};
use derive_more::{Deref, DerefMut, Display, From};
use serde::Deserialize;
use std::ops::{Deref, DerefMut};
use strum::EnumTryAs;
use crate::cache::{EndpointEntryCache, HydratedCacheTable};
use crate::{rwlock_const_new, RwLock};
use crate::endpoints::StatsAPIUrl;

#[repr(transparent)]
#[derive(Debug, Deserialize, Deref, Display, PartialEq, Eq, Clone, Hash)]
pub struct JobTypeId(String);

impl JobTypeId {
	#[must_use]
	pub const fn new(id: String) -> Self {
		Self(id)
	}
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IdentifiableJobType {
	#[serde(rename = "code")] pub id: JobTypeId,
}

#[derive(Debug, Deserialize, Deref, DerefMut, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HydratedJobType {
	pub job: String,

	#[deref]
	#[deref_mut]
	#[serde(flatten)]
	inenr: IdentifiableJobType,
}

#[derive(Debug, Deserialize, Eq, Clone, From, EnumTryAs)]
#[serde(untagged)]
pub enum JobType {
	Hydrated(HydratedJobType),
	Identifiable(IdentifiableJobType),
}

impl PartialEq for JobType {
	fn eq(&self, other: &Self) -> bool {
		self.id == other.id
	}
}

impl Deref for JobType {
	type Target = IdentifiableJobType;

	fn deref(&self) -> &Self::Target {
		match self {
			Self::Hydrated(inner) => inner,
			Self::Identifiable(inner) => inner,
		}
	}
}

impl DerefMut for JobType {
	fn deref_mut(&mut self) -> &mut Self::Target {
		match self {
			Self::Hydrated(inner) => inner,
			Self::Identifiable(inner) => inner,
		}
	}
}

impl MetaKind for JobType {
	const ENDPOINT_NAME: &'static str = "jobTypes";
}

static CACHE: RwLock<HydratedCacheTable<JobType>> = rwlock_const_new(HydratedCacheTable::new());

impl EndpointEntryCache for JobType {
	type HydratedVariant = HydratedJobType;
	type Identifier = JobTypeId;
	type URL = MetaEndpointUrl<Self>;

	fn into_hydrated_entry(self) -> Option<Self::HydratedVariant> {
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
		let _response = MetaEndpointUrl::<super::JobType>::new().get().await.unwrap();
	}
}
