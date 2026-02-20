use serde::Deserialize;

id!(#[doc = "A [`u32`] representing a [`Language`]"]LanguageId { languageId: u32 });

// A language, such as English, French, etc.
// Likely used for locale-related purposes.
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Language {
	#[serde(rename = "languageCode")]
	pub code: String,
	pub name: String,
	pub locale: String,
	#[serde(flatten)]
	pub id: LanguageId,
}

id_only_eq_impl!(Language, id);
meta_kind_impl!("languages" => Language);
tiered_request_entry_cache_impl!(Language.id: LanguageId);
test_impl!(Language);