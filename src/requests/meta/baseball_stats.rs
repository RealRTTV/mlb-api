use crate::requests::meta::stat_groups::StatGroup;
use derive_more::{Deref, DerefMut, From};
use mlb_api_proc::{EnumDeref, EnumDerefMut, EnumTryAs, EnumTryAsMut, EnumTryInto};
use serde::Deserialize;

string_id!(BaseballStatId);

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
pub struct IdentifiableBaseballStat {
	#[serde(rename = "name")]
	pub id: BaseballStatId,
}

#[derive(Debug, Deserialize, Deref, DerefMut, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HydratedBaseballStat {
	lookup_param: Option<String>,
	is_counting: bool,
	label: Option<String>,
	stat_groups: Vec<StatGroup>,

	#[deref]
	#[deref_mut]
	#[serde(flatten)]
	inner: IdentifiableBaseballStat,
}

#[derive(Debug, Deserialize, Eq, Clone, From, EnumTryAs, EnumTryAsMut, EnumTryInto, EnumDeref, EnumDerefMut)]
#[serde(untagged)]
pub enum BaseballStat {
	Hydrated(HydratedBaseballStat),
	Identifiable(IdentifiableBaseballStat),
}

id_only_eq_impl!(BaseballStat, id);
meta_kind_impl!("baseballStats" => BaseballStat);
tiered_request_entry_cache_impl!(BaseballStat => HydratedBaseballStat; id: BaseballStatId);
test_impl!(BaseballStat);
