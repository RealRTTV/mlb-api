use serde::Deserialize;

id!(#[doc = "A review on a play\n(These codes often are just single letters and mean nothing and only useful for lookup purposes.)"] ReviewReasonId { code: String });

/// A detailed `struct` representing a reviewable play.
///
/// ## Examples
/// ```
/// ReviewReason {
///     description: "Tag play".into(),
///     id: "A".into(), // see what I mean? meaningless code
/// }
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
