use serde::Deserialize;

// even though I can recite them all in my head, in the next 5-10 years, there definitely will be a new pitch type -- death-ball?
id!(#[doc = "A [`String`] representing pitch types, `\"FF\"` for Four-Seam Fastball, `\"CU\"` for Curveball, etc."] PitchTypeId { code: String });

/// A detailed `struct` describing a pitch type.
///
/// ## Examples
/// ```
/// PitchType {
///     description: "Four-Seam Fastball".into(),
///     id: "FF".into(),
/// }
/// ```
#[derive(Debug, Deserialize, Clone)]
pub struct PitchType {
	pub description: String,
	#[serde(flatten)]
	pub id: PitchTypeId,
}

id_only_eq_impl!(PitchType, id);
meta_kind_impl!("pitchTypes" => PitchType);
tiered_request_entry_cache_impl!(PitchType.id: PitchTypeId);
test_impl!(PitchType);
