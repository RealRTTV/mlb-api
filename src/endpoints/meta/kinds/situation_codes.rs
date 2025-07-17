use crate::endpoints::meta::{MetaEndpointUrl, MetaKind};
use derive_more::{Deref, DerefMut, Display, From};
use serde::Deserialize;
use std::ops::{Deref, DerefMut};
use strum::EnumTryAs;
use crate::cache::{EndpointEntryCache, HydratedCacheTable};
use crate::{rwlock_const_new, RwLock};
use crate::endpoints::StatsAPIUrl;

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
pub struct IdentifiableSituationCode {
	#[serde(rename = "code")] pub id: SituationCodeId,
}

#[repr(transparent)]
#[derive(Debug, Deserialize, Deref, Display, PartialEq, Eq, Clone, Hash)]
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

#[derive(Debug, Deserialize, Eq, Clone, From, EnumTryAs)]
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
		let _response = MetaEndpointUrl::<super::SituationCode>::new().get().await.unwrap();
	}
}
