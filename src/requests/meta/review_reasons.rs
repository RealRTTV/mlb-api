use derive_more::{Deref, DerefMut, From};
use mlb_api_proc::{EnumDeref, EnumDerefMut, EnumTryAs, EnumTryAsMut, EnumTryInto};
use serde::Deserialize;

string_id!(ReviewReasonId);

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
pub struct IdentifiableReviewReason {
	#[serde(rename = "code")] pub id: ReviewReasonId,
}

#[derive(Debug, Deserialize, Deref, DerefMut, PartialEq, Eq, Clone)]
pub struct HydratedReviewReason {
	pub description: String,

	#[deref]
	#[deref_mut]
	#[serde(flatten)]
	inner: IdentifiableReviewReason,
}

#[derive(Debug, Deserialize, Eq, Clone, From, EnumTryAs, EnumTryAsMut, EnumTryInto, EnumDeref, EnumDerefMut)]
#[serde(untagged)]
pub enum ReviewReason {
	Hydrated(HydratedReviewReason),
	Identifiable(IdentifiableReviewReason),
}

id_only_eq_impl!(ReviewReason, id);
meta_kind_impl!("reviewReasons" => ReviewReason);
tiered_request_entry_cache_impl!(ReviewReason => HydratedReviewReason; id: ReviewReasonId);
test_impl!(ReviewReason);
