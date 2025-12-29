use crate::meta::stat_groups::StatGroup;
use serde::Deserialize;

id!(BaseballStatId { name: String });

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BaseballStat {
	pub lookup_param: Option<String>,
	pub is_counting: bool,
	pub label: Option<String>,
	pub stat_groups: Vec<StatGroup>,
	#[serde(flatten)]
	pub id: BaseballStatId,
}

id_only_eq_impl!(BaseballStat, id);
meta_kind_impl!("baseballStats" => BaseballStat);
tiered_request_entry_cache_impl!(BaseballStat.id: BaseballStatId);
test_impl!(BaseballStat);
