use crate::endpoints::meta::{MetaEndpointUrl, MetaKind};
use derive_more::{Deref, DerefMut, Display, From};
use serde::Deserialize;
use std::ops::{Deref, DerefMut};
use strum::EnumTryAs;
use crate::cache::{EndpointEntryCache, HydratedCacheTable};
use crate::{rwlock_const_new, RwLock};
use crate::endpoints::StatsAPIUrl;

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IdentifiableGameStatus {
	#[serde(rename = "detailedState")] pub id: GameStatusId,
}

#[repr(transparent)]
#[derive(Debug, Deserialize, Deref, Display, PartialEq, Eq, Clone, Hash)]
pub struct GameStatusId(String);

#[derive(Debug, Deserialize, Deref, DerefMut, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HydratedGameStatus {
	pub abstract_game_state: String,
	pub coded_game_state: String,
	pub status_code: String,
	pub reason: Option<String>,
	pub abstract_game_code: String,

	#[deref]
	#[deref_mut]
	#[serde(flatten)]
	inner: IdentifiableGameStatus,
}

#[derive(Debug, Deserialize, Eq, Clone, From, EnumTryAs)]
#[serde(untagged)]
pub enum GameStatus {
	Hydrated(HydratedGameStatus),
	Identifiable(IdentifiableGameStatus),
}

impl Deref for GameStatus {
	type Target = IdentifiableGameStatus;

	fn deref(&self) -> &Self::Target {
		match self {
			Self::Hydrated(inner) => inner,
			Self::Identifiable(inner) => inner,
		}
	}
}

impl DerefMut for GameStatus {
	fn deref_mut(&mut self) -> &mut Self::Target {
		match self {
			Self::Hydrated(inner) => inner,
			Self::Identifiable(inner) => inner,
		}
	}
}

impl PartialEq for GameStatus {
	fn eq(&self, other: &Self) -> bool {
		self.id == other.id
	}
}

impl MetaKind for GameStatus {
	const ENDPOINT_NAME: &'static str = "gameStatus";
}

static CACHE: RwLock<HydratedCacheTable<GameStatus>> = rwlock_const_new(HydratedCacheTable::new());

impl EndpointEntryCache for GameStatus {
	type HydratedVariant = HydratedGameStatus;
	type Identifier = GameStatusId;
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
		let _response = MetaEndpointUrl::<super::GameStatus>::new().get().await.unwrap();
	}
}
