use serde::Deserialize;

id!(WindDirectionId { code: String });

#[derive(Debug, Deserialize, Clone)]
pub struct WindDirection {
	pub description: String,
	#[serde(flatten)]
	pub id: WindDirectionId,
}

id_only_eq_impl!(WindDirection, id);
meta_kind_impl!("windDirection" => WindDirection);
tiered_request_entry_cache_impl!(WindDirection.id: WindDirectionId);
test_impl!(WindDirection);
