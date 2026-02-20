use serde::Deserialize;

id!(#[doc = "A [`String`] representing a [`JobType`]."] JobTypeId { code: String });

/// Different types of baseball jobs; Umpires, Coaches, etc.
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct JobType {
	/// Director of Instant Replay, Bench Coach, etc.
	pub job: String,
	/// UMPR, MNGR, etc.
	#[serde(flatten)]
	pub id: JobTypeId,
}

id_only_eq_impl!(JobType, id);
meta_kind_impl!("jobTypes" => JobType);
tiered_request_entry_cache_impl!(JobType.id: JobTypeId);
test_impl!(JobType);
