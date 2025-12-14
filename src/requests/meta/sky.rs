use derive_more::{Deref, DerefMut, From};
use mlb_api_proc::{EnumDeref, EnumDerefMut, EnumTryAs, EnumTryAsMut, EnumTryInto};
use serde::Deserialize;

string_id!(SkyDescriptionId);

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
pub struct IdentifiableSkyDescription {
	#[serde(rename = "code")]
	pub id: SkyDescriptionId,
}

#[derive(Debug, Deserialize, Deref, DerefMut, PartialEq, Eq, Clone)]
pub struct HydratedSkyDescription {
	pub description: String,

	#[deref]
	#[deref_mut]
	#[serde(flatten)]
	inner: IdentifiableSkyDescription,
}

#[derive(Debug, Deserialize, Eq, Clone, From, EnumTryAs, EnumTryAsMut, EnumTryInto, EnumDeref, EnumDerefMut)]
#[serde(untagged)]
pub enum SkyDescription {
	Hydrated(HydratedSkyDescription),
	Identifiable(IdentifiableSkyDescription),
}

id_only_eq_impl!(SkyDescription, id);
meta_kind_impl!("sky" => SkyDescription);
tiered_request_entry_cache_impl!(SkyDescription => HydratedSkyDescription; id: SkyDescriptionId);
test_impl!(SkyDescription);

#[derive(Debug, Deserialize, PartialEq, Eq, Copy, Clone)]
pub enum Sky {
	/// Day Game.
	#[serde(rename = "day")]
	Day,

	/// Night Game.
	#[serde(rename = "night")]
	Night,
}
