use crate::cache::{RequestEntryCache, HydratedCacheTable};
use crate::meta::kinds::MetaKind;
use crate::meta::MetaRequest;
use crate::StatsAPIRequestUrl;
use crate::{rwlock_const_new, RwLock};
use derive_more::{Deref, DerefMut, Display, From};
use mlb_api_proc::{EnumTryAs, EnumTryAsMut, EnumTryInto};
use serde::Deserialize;
use std::ops::{Deref, DerefMut};

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IdentifiableLanguage {
	#[serde(rename = "languageId")]
	id: LanguageId,
}

#[derive(Debug, Deserialize, Deref, DerefMut, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HydratedLanguage {
	#[serde(rename = "languageCode")]
	pub code: String,
	pub name: String,
	pub locale: String,

	#[deref]
	#[deref_mut]
	#[serde(flatten)]
	inner: IdentifiableLanguage,
}

#[repr(transparent)]
#[derive(Debug, Deserialize, Deref, Display, PartialEq, Eq, Copy, Clone, Hash, From)]
pub struct LanguageId(pub(super) u32);

impl LanguageId {
	#[must_use]
	pub const fn new(id: u32) -> Self {
		Self(id)
	}
}

#[derive(Debug, Deserialize, Eq, Clone, From, EnumTryAs, EnumTryAsMut, EnumTryInto)]
#[serde(untagged)]
pub enum Language {
	Hydrated(Box<HydratedLanguage>),
	Identifiable(IdentifiableLanguage),
}

impl Language {
	#[must_use]
	pub fn id(&self) -> LanguageId {
		match self {
			Self::Hydrated(inner) => inner.id,
			Self::Identifiable(inner) => inner.id,
		}
	}
}

impl PartialEq for Language {
	fn eq(&self, other: &Self) -> bool {
		self.id() == other.id()
	}
}

impl Deref for Language {
	type Target = IdentifiableLanguage;

	fn deref(&self) -> &Self::Target {
		match self {
			Self::Hydrated(inner) => inner,
			Self::Identifiable(inner) => inner,
		}
	}
}

impl DerefMut for Language {
	fn deref_mut(&mut self) -> &mut Self::Target {
		match self {
			Self::Hydrated(inner) => inner,
			Self::Identifiable(inner) => inner,
		}
	}
}

impl MetaKind for Language {
	const ENDPOINT_NAME: &'static str = "languages";
}

static CACHE: RwLock<HydratedCacheTable<Language>> = rwlock_const_new(HydratedCacheTable::new());

impl RequestEntryCache for Language {
	type HydratedVariant = Box<HydratedLanguage>;
	type Identifier = LanguageId;
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
		let _response = MetaRequest::<super::Language>::new().get().await.unwrap();
	}
}
