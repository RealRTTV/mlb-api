use serde::Deserialize;

id!(PitchCodeId { code: String });

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
	pub swing_made_contact: bool,
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
