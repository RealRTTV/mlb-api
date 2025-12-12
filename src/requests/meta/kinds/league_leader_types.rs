use crate::cache::{HydratedCacheTable, RequestEntryCache};
use crate::meta::{MetaKind, MetaRequest};
use crate::{rwlock_const_new, RwLock};
use crate::{string_id, StatsAPIRequestUrl};
use derive_more::From;
use mlb_api_proc::{EnumTryAs, EnumTryAsMut, EnumTryInto};
use serde::Deserialize;
use std::ops::{Deref, DerefMut};

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
pub struct IdentifiableLeagueLeaderType {
	#[serde(rename = "displayName")]
	pub id: LeagueLeaderTypeId,
}

string_id!(LeagueLeaderTypeId);

#[derive(Debug, Deserialize, Eq, Clone, From, EnumTryAs, EnumTryAsMut, EnumTryInto)]
#[serde(untagged)]
pub enum LeagueLeaderType {
	Identifiable(IdentifiableLeagueLeaderType),
}

impl PartialEq for LeagueLeaderType {
	fn eq(&self, other: &Self) -> bool {
		self.id == other.id
	}
}

impl Deref for LeagueLeaderType {
	type Target = IdentifiableLeagueLeaderType;

	fn deref(&self) -> &Self::Target {
		match self {
			Self::Identifiable(inner) => inner,
		}
	}
}

impl DerefMut for LeagueLeaderType {
	fn deref_mut(&mut self) -> &mut Self::Target {
		match self {
			Self::Identifiable(inner) => inner,
		}
	}
}

impl MetaKind for LeagueLeaderType {
	const ENDPOINT_NAME: &'static str = "leagueLeaderTypes";
}

static CACHE: RwLock<HydratedCacheTable<LeagueLeaderType>> = rwlock_const_new(HydratedCacheTable::new());

impl RequestEntryCache for LeagueLeaderType {
	type HydratedVariant = IdentifiableLeagueLeaderType;
	type Identifier = LeagueLeaderTypeId;
	type URL = MetaRequest<Self>;

	fn into_hydrated_variant(self) -> Option<Self::HydratedVariant> {
		self.try_into_identifiable()
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
		let _response = MetaRequest::<super::LeagueLeaderType>::new().get().await.unwrap();
	}
}
