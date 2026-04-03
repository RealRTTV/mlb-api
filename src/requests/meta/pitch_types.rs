use serde::{Deserialize, Deserializer};

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

pub fn fallback_pitch_type_deserializer<'de, D: Deserializer<'de>>(deserializer: D) -> Result<PitchType, D::Error> {
	Ok(PitchType::deserialize(deserializer).ok().unwrap_or_else(unknown_pitch_type))
}

#[must_use]
pub fn unknown_pitch_type() -> PitchType {
	PitchType {
		description: "Unknown".to_owned(),
		id: PitchTypeId::new("UN"),
	}
}

id_only_eq_impl!(PitchType, id);
meta_kind_impl!("pitchTypes" => PitchType);
tiered_request_entry_cache_impl!(PitchType.id: PitchTypeId);
test_impl!(PitchType);
