use derive_more::{Deref, DerefMut, From};
use mlb_api_proc::{EnumDeref, EnumDerefMut, EnumTryAs, EnumTryAsMut, EnumTryInto};
use serde::Deserialize;

string_id!(JobTypeId);

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IdentifiableJobType {
	#[serde(rename = "code")] pub id: JobTypeId,
}

#[derive(Debug, Deserialize, Deref, DerefMut, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HydratedJobType {
	pub job: String,

	#[deref]
	#[deref_mut]
	#[serde(flatten)]
	inner: IdentifiableJobType,
}

#[derive(Debug, Deserialize, Eq, Clone, From, EnumTryAs, EnumTryAsMut, EnumTryInto, EnumDeref, EnumDerefMut)]
#[serde(untagged)]
pub enum JobType {
	Hydrated(HydratedJobType),
	Identifiable(IdentifiableJobType),
}

id_only_eq_impl!(JobType, id);
meta_kind_impl!("jobTypes" => JobType);
tiered_request_entry_cache_impl!(JobType => HydratedJobType; id: JobTypeId);
test_impl!(JobType);
