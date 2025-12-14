use derive_more::{Deref, DerefMut, From};
use mlb_api_proc::{EnumDeref, EnumDerefMut, EnumTryAs, EnumTryAsMut, EnumTryInto};
use serde::Deserialize;

string_id!(PitchCodeId);

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
pub struct IdentifiablePitchCode {
	#[serde(rename = "code")] pub id: PitchCodeId,
}

#[allow(clippy::struct_excessive_bools, reason = "false positive")]
#[derive(Debug, Deserialize, Deref, DerefMut, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HydratedPitchCode {
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

	#[deref]
	#[deref_mut]
	#[serde(flatten)]
	inner: IdentifiablePitchCode,
}

#[derive(Debug, Deserialize, Eq, Clone, From, EnumTryAs, EnumTryAsMut, EnumTryInto, EnumDeref, EnumDerefMut)]
#[serde(untagged)]
pub enum PitchCode {
	Hydrated(HydratedPitchCode),
	Identifiable(IdentifiablePitchCode),
}

id_only_eq_impl!(PitchCode, id);
meta_kind_impl!("pitchCodes" => PitchCode);
tiered_request_entry_cache_impl!(PitchCode => HydratedPitchCode; id: PitchCodeId);
test_impl!(PitchCode);
