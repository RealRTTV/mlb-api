use derive_more::{Deref, DerefMut, From};
use mlb_api_proc::{EnumDeref, EnumDerefMut, EnumTryAs, EnumTryAsMut, EnumTryInto};
use serde::Deserialize;

// even though I can recite them all in my head, in the next 5-10 years, there definitely will be a new pitch type -- deathball?
string_id!(PitchTypeId);

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
pub struct IdentifiablePitchType {
	#[serde(rename = "code")] pub id: PitchTypeId,
}

#[derive(Debug, Deserialize, Deref, DerefMut, PartialEq, Eq, Clone)]
pub struct HydratedPitchType {
	pub description: String,

	#[deref]
	#[deref_mut]
	#[serde(flatten)]
	inner: IdentifiablePitchType,
}

#[derive(Debug, Deserialize, Eq, Clone, From, EnumTryAs, EnumTryAsMut, EnumTryInto, EnumDeref, EnumDerefMut)]
#[serde(untagged)]
pub enum PitchType {
	Hydrated(HydratedPitchType),
	Identifiable(IdentifiablePitchType),
}

id_only_eq_impl!(PitchType, id);
meta_kind_impl!("pitchTypes" => PitchType);
tiered_request_entry_cache_impl!(PitchType => HydratedPitchType; id: PitchTypeId);
test_impl!(PitchType);
