use serde::Deserialize;

id!(#[doc = "A [`String`] describing the conditions of the sky"] SkyDescriptionId { code: String });

/// A detailed `struct` representing the sky conditions
///
/// ## Examples
/// ```
/// SkyDescription {
///     description: "Clear".into(),
///     id: "Clear".into(),
/// }
/// ```
#[derive(Debug, Deserialize, Clone)]
pub struct SkyDescription {
	pub description: String,
	#[serde(flatten)]
	pub id: SkyDescriptionId,
}

id_only_eq_impl!(SkyDescription, id);
meta_kind_impl!("sky" => SkyDescription);
tiered_request_entry_cache_impl!(SkyDescription.id: SkyDescriptionId);
test_impl!(SkyDescription);

/// Whether the sky shows daytime or nighttime
#[derive(Debug, Deserialize, PartialEq, Eq, Copy, Clone)]
pub enum DayNight {
	/// Day Game.
	#[serde(rename = "day")]
	Day,

	/// Night Game.
	#[serde(rename = "night")]
	Night,
}
