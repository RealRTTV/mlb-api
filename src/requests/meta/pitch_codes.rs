use serde::Deserialize;

id!(#[doc = "A [`String`] representing a pitch, such as \"F\" for Foul, \"S\" for Swinging Strike, etc."] PitchCodeId { code: String });

/// A detailed `struct` representing everything logic-related that happened with a pitch.
///
/// ## Examples
/// ```
/// PitchCode {
///     id: "F".into(),
///     description: "Strike - Foul".into(),
///     has_swing: true,
///     is_whiff: false,
///     made_contact_fair: false,
///     is_strike: true,
///     is_ball: false,
///     is_pitch: true,
///     pitch_result_text: "Foul Ball".into(),
///     is_bunt_attempt: false,
///     made_contact: true
/// }
/// ```
#[allow(clippy::struct_excessive_bools, reason = "false positive")]
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PitchCode {
	pub description: String,
	#[serde(rename = "swingStatus")]
	pub has_swing: bool,
	#[serde(rename = "swingMissStatus")]
	pub is_whiff: bool,
	#[serde(rename = "swingContactStatus")]
	pub made_contact_fair: bool,
	#[serde(rename = "strikeStatus")]
	pub is_strike: bool,
	#[serde(rename = "ballStatus")]
	pub is_ball: bool,
	#[serde(rename = "pitchStatus")]
	pub is_pitch: bool,
	pub pitch_result_text: String,
	#[serde(rename = "buntAttemptStatus")]
	pub is_bunt_attempt: bool,
	#[serde(rename = "contactStatus")]
	pub made_contact: bool,
	#[serde(flatten)]
	pub id: PitchCodeId,
}

id_only_eq_impl!(PitchCode, id);
meta_kind_impl!("pitchCodes" => PitchCode);
tiered_request_entry_cache_impl!(PitchCode.id: PitchCodeId);
test_impl!(PitchCode);
