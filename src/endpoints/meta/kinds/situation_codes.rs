use crate::meta::{MetaEndpoint, MetaKind};
use derive_more::{Deref, DerefMut, Display, From};
use serde::Deserialize;
use std::ops::{Deref, DerefMut};
use mlb_api_proc::{EnumTryAs, EnumTryAsMut, EnumTryInto};
use crate::cache::{EndpointEntryCache, HydratedCacheTable};
use crate::{rwlock_const_new, RwLock};
use crate::StatsAPIEndpointUrl;

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
pub struct IdentifiableSituationCode {
	#[serde(rename = "code")] pub id: SituationCodeId,
}

#[repr(transparent)]
#[derive(Debug, Deserialize, Deref, Display, PartialEq, Eq, Clone, Hash, From)]
pub struct SituationCodeId(String);

#[derive(Debug, Deserialize, Deref, DerefMut, PartialEq, Eq, Clone)]
pub struct HydratedSituationCode {
	#[serde(rename = "navigationMenu")]
	pub navigation_menu_kind: String,
	pub description: String,
	#[serde(rename = "team")]
	pub is_team_active: bool,
	#[serde(rename = "batting")]
	pub is_batting_active: bool,
	#[serde(rename = "fielding")]
	pub is_fielding_active: bool,
	#[serde(rename = "pitching")]
	pub is_pitching_active: bool,

	#[deref]
	#[deref_mut]
	#[serde(flatten)]
	inner: IdentifiableSituationCode,
}

#[derive(Debug, Deserialize, Eq, Clone, From, EnumTryAs, EnumTryAsMut, EnumTryInto)]
#[serde(untagged)]
pub enum SituationCode {
	Hydrated(HydratedSituationCode),
	Identifiable(IdentifiableSituationCode),
}

impl PartialEq for SituationCode {
	fn eq(&self, other: &Self) -> bool {
		self.id == other.id
	}
}

impl Deref for SituationCode {
	type Target = IdentifiableSituationCode;

	fn deref(&self) -> &Self::Target {
		match self {
			Self::Hydrated(inner) => inner,
			Self::Identifiable(inner) => inner,
		}
	}
}

impl DerefMut for SituationCode {
	fn deref_mut(&mut self) -> &mut Self::Target {
		match self {
			Self::Hydrated(inner) => inner,
			Self::Identifiable(inner) => inner,
		}
	}
}

impl MetaKind for SituationCode {
	const ENDPOINT_NAME: &'static str = "situationCodes";
}

static CACHE: RwLock<HydratedCacheTable<SituationCode>> = rwlock_const_new(HydratedCacheTable::new());

impl EndpointEntryCache for SituationCode {
	type HydratedVariant = HydratedSituationCode;
	type Identifier = SituationCodeId;
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
	use crate::StatsAPIEndpointUrl;
	use crate::meta::MetaEndpoint;

	#[tokio::test]
	async fn parse_meta() {
		let _response = MetaEndpoint::<super::SituationCode>::new().get().await.unwrap();
	}
}
