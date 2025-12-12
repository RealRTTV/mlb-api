use crate::cache::{HydratedCacheTable, RequestEntryCache};
use crate::meta::{MetaKind, MetaRequest};
use crate::{rwlock_const_new, RwLock};
use crate::{string_id, StatsAPIRequestUrl};
use derive_more::{Deref, DerefMut, From};
use mlb_api_proc::{EnumTryAs, EnumTryAsMut, EnumTryInto};
use serde::Deserialize;
use std::ops::{Deref, DerefMut};

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
pub struct IdentifiableStandingsType {
	#[serde(rename = "name")]
	pub id: StandingsTypeId,
}

string_id!(StandingsTypeId);

#[derive(Debug, Deserialize, Deref, DerefMut, PartialEq, Eq, Clone)]
pub struct HydratedStandingsType {
	pub description: String,

	#[deref]
	#[deref_mut]
	#[serde(flatten)]
	inner: IdentifiableStandingsType,
}

#[derive(Debug, Deserialize, Eq, Clone, From, EnumTryAs, EnumTryAsMut, EnumTryInto)]
#[serde(untagged)]
pub enum StandingsType {
	Hydrated(HydratedStandingsType),
	Identifiable(IdentifiableStandingsType),
}

impl PartialEq for StandingsType {
	fn eq(&self, other: &Self) -> bool {
		self.id == other.id
	}
}

impl Deref for StandingsType {
	type Target = IdentifiableStandingsType;

	fn deref(&self) -> &Self::Target {
		match self {
			Self::Hydrated(inner) => inner,
			Self::Identifiable(inner) => inner,
		}
	}
}

impl DerefMut for StandingsType {
	fn deref_mut(&mut self) -> &mut Self::Target {
		match self {
			Self::Hydrated(inner) => inner,
			Self::Identifiable(inner) => inner,
		}
	}
}

impl MetaKind for StandingsType {
	const ENDPOINT_NAME: &'static str = "standingsTypes";
}

static CACHE: RwLock<HydratedCacheTable<StandingsType>> = rwlock_const_new(HydratedCacheTable::new());

impl RequestEntryCache for StandingsType {
	type HydratedVariant = HydratedStandingsType;
	type Identifier = StandingsTypeId;
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
		let _response = MetaRequest::<super::StandingsType>::new().get().await.unwrap();
	}
}
