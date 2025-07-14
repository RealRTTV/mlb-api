use crate::endpoints::meta::kinds::MetaKind;
use derive_more::{Deref, DerefMut, From};
use serde::Deserialize;
use std::ops::{Deref, DerefMut};

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IdentifiableLanguage {
	#[serde(rename = "languageId")] id: LanguageId,
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

pub use id::*;

mod id {
	use derive_more::{Deref, Display};
	use serde::Deserialize;

	#[repr(transparent)]
	#[derive(Debug, Deserialize, Deref, Display, PartialEq, Eq, Copy, Clone)]
	pub struct LanguageId(pub(super) u32);

	impl LanguageId {
		#[must_use]
		pub const fn new(id: u32) -> Self {
			Self(id)
		}
	}
}

#[derive(Debug, Deserialize, Eq, Clone, From)]
#[serde(untagged)]
pub enum Language {
	Hydrated(HydratedLanguage),
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

#[cfg(test)]
mod tests {
	use crate::endpoints::meta::MetaEndpointUrl;
	use crate::endpoints::StatsAPIUrl;

	#[tokio::test]
	async fn parse_meta() {
		let _response = MetaEndpointUrl::<super::Language>::new().get().await.unwrap();
	}
}
