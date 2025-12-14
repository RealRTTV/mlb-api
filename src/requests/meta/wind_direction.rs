use derive_more::{Deref, DerefMut, From};
use mlb_api_proc::{EnumDeref, EnumDerefMut, EnumTryAs, EnumTryAsMut, EnumTryInto};
use serde::Deserialize;

string_id!(WindDirectionId);

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
pub struct IdentifiableWindDirection {
	#[serde(rename = "code")] pub id: WindDirectionId,
}

#[derive(Debug, Deserialize, Deref, DerefMut, PartialEq, Eq, Clone)]
pub struct HydratedWindDirection {
	pub description: String,

	#[deref]
	#[deref_mut]
	#[serde(flatten)]
	inner: IdentifiableWindDirection,
}

#[derive(Debug, Deserialize, Eq, Clone, From, EnumTryAs, EnumTryAsMut, EnumTryInto, EnumDeref, EnumDerefMut)]
#[serde(untagged)]
pub enum WindDirection {
	Hydrated(HydratedWindDirection),
	Identifiable(IdentifiableWindDirection),
}

id_only_eq_impl!(WindDirection, id);
meta_kind_impl!("windDirection" => WindDirection);
tiered_request_entry_cache_impl!(WindDirection => HydratedWindDirection; id: WindDirectionId);
test_impl!(WindDirection);
