use crate::cache::{HydratedCacheTable, RequestEntryCache};
use crate::meta::{MetaKind, MetaRequest};
use crate::StatsAPIRequestUrl;
use crate::{rwlock_const_new, RwLock};
use derive_more::{Deref, DerefMut, Display, From};
use mlb_api_proc::{EnumTryAs, EnumTryAsMut, EnumTryInto};
use serde::Deserialize;
use std::ops::{Deref, DerefMut};

#[allow(clippy::struct_excessive_bools, reason = "false positive")]
#[derive(Debug, Deserialize, Deref, DerefMut, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HydratedPosition {
	pub short_name: String,
	pub full_name: String,
	pub formal_name: String,
	#[serde(rename = "pitcher")]
	pub is_pitcher: bool,
	#[serde(rename = "gamePosition")]
	pub is_game_position: bool,
	#[serde(rename = "fielder")]
	pub is_fielder: bool,
	#[serde(rename = "outfield")]
	pub is_outfield: bool,

	#[deref]
	#[deref_mut]
	#[serde(flatten)]
	inner: NamedPosition,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NamedPosition {
	pub code: PositionCode,
	#[serde(alias = "displayName")]
	pub name: String,
	#[serde(rename = "type")]
	pub r#type: String,
	#[serde(alias = "abbrev")]
	pub abbreviation: String,
}

#[repr(transparent)]
#[derive(Debug, Deserialize, Deref, Display, PartialEq, Eq, Clone, Hash)]
pub struct PositionCode(String);

#[derive(Debug, Deserialize, Eq, Clone, From, EnumTryAs, EnumTryAsMut, EnumTryInto)]
#[serde(untagged)]
pub enum Position {
	Hydrated(HydratedPosition),
	Named(NamedPosition),
}

impl PartialEq for Position {
	fn eq(&self, other: &Self) -> bool {
		self.code == other.code
	}
}

impl Deref for Position {
	type Target = NamedPosition;

	fn deref(&self) -> &Self::Target {
		match self {
			Self::Hydrated(inner) => inner,
			Self::Named(inner) => inner,
		}
	}
}

impl DerefMut for Position {
	fn deref_mut(&mut self) -> &mut Self::Target {
		match self {
			Self::Hydrated(inner) => inner,
			Self::Named(inner) => inner,
		}
	}
}

impl MetaKind for Position {
	const ENDPOINT_NAME: &'static str = "positions";
}

static CACHE: RwLock<HydratedCacheTable<Position>> = rwlock_const_new(HydratedCacheTable::new());

impl RequestEntryCache for Position {
	type HydratedVariant = HydratedPosition;
	type Identifier = PositionCode;
	type URL = MetaRequest<Self>;

	fn into_hydrated_variant(self) -> Option<Self::HydratedVariant> {
		self.try_into_hydrated()
	}

	fn id(&self) -> &Self::Identifier {
		&self.code
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
		let _response = MetaRequest::<super::Position>::new().get().await.unwrap();
	}
}
