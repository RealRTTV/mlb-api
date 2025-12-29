use serde::Deserialize;

id!(JobTypeId { code: String });

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct JobType {
	pub job: String,
	#[serde(flatten)]
	pub id: JobTypeId,
}

id_only_eq_impl!(JobType, id);
meta_kind_impl!("jobTypes" => JobType);
tiered_request_entry_cache_impl!(JobType.id: JobTypeId);
test_impl!(JobType);
