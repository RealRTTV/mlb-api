use serde::Deserialize;

id!(ReviewReasonId { code: String });

#[derive(Debug, Deserialize, Clone)]
pub struct ReviewReason {
	pub description: String,
	#[serde(flatten)]
	pub id: ReviewReasonId,
}

id_only_eq_impl!(ReviewReason, id);
meta_kind_impl!("reviewReasons" => ReviewReason);
tiered_request_entry_cache_impl!(ReviewReason.id: ReviewReasonId);
test_impl!(ReviewReason);
