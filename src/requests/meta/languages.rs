use derive_more::{Deref, DerefMut, From};
use mlb_api_proc::{EnumDeref, EnumDerefMut, EnumTryAs, EnumTryAsMut, EnumTryInto};
use serde::Deserialize;

integer_id!(LanguageId);

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IdentifiableLanguage {
	#[serde(rename = "languageId")]
	pub id: LanguageId,
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

#[derive(Debug, Deserialize, Eq, Clone, From, EnumTryAs, EnumTryAsMut, EnumTryInto, EnumDeref, EnumDerefMut)]
#[serde(untagged)]
pub enum Language {
	Hydrated(HydratedLanguage),
	Identifiable(IdentifiableLanguage),
}

id_only_eq_impl!(Language, id);
meta_kind_impl!("languages" => Language);
tiered_request_entry_cache_impl!(Language => HydratedLanguage; id: LanguageId);
test_impl!(Language);