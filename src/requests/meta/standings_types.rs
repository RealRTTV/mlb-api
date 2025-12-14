use derive_more::{Deref, DerefMut, From};
use mlb_api_proc::{EnumDeref, EnumDerefMut, EnumTryAs, EnumTryAsMut, EnumTryInto};
use serde::Deserialize;

string_id!(StandingsTypeId);

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
pub struct IdentifiableStandingsType {
	#[serde(rename = "name")]
	pub id: StandingsTypeId,
}

#[derive(Debug, Deserialize, Deref, DerefMut, PartialEq, Eq, Clone)]
pub struct HydratedStandingsType {
	pub description: String,

	#[deref]
	#[deref_mut]
	#[serde(flatten)]
	inner: IdentifiableStandingsType,
}

#[derive(Debug, Deserialize, Eq, Clone, From, EnumTryAs, EnumTryAsMut, EnumTryInto, EnumDeref, EnumDerefMut)]
#[serde(untagged)]
pub enum StandingsType {
	Hydrated(HydratedStandingsType),
	Identifiable(IdentifiableStandingsType),
}

id_only_eq_impl!(StandingsType, id);
meta_kind_impl!("standingsTypes" => StandingsType);
tiered_request_entry_cache_impl!(StandingsType => HydratedStandingsType; id: StandingsTypeId);
test_impl!(StandingsType);