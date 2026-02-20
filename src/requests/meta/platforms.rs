use serde::Deserialize;

id!(#[doc = "A [`String`] representing an electronic platform"] PlatformId { platformCode: String });

/// A detailed `struct` representing a Platform
///
/// ## Examples
/// Platform {
///     name: "iOS Phone".into(),
///     id: "ios-phone".into(),
/// }
#[derive(Debug, Deserialize, Clone)]
pub struct Platform {
	#[serde(rename = "platformDescription")]
	pub name: String,
	#[serde(flatten)]
	pub id: PlatformId,
}

id_only_eq_impl!(Platform, id);
meta_kind_impl!("platforms" => Platform);
tiered_request_entry_cache_impl!(Platform.id: PlatformId);
test_impl!(Platform);
