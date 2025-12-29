use serde::Deserialize;

id!(StandingsTypeId { name: String });

#[derive(Debug, Deserialize, Clone)]
pub struct StandingsType {
	pub description: String,
	#[serde(flatten)]
	pub id: StandingsTypeId,
}

id_only_eq_impl!(StandingsType, id);
meta_kind_impl!("standingsTypes" => StandingsType);
tiered_request_entry_cache_impl!(StandingsType.id: StandingsTypeId);
test_impl!(StandingsType);