use serde::Deserialize;

id!(SkyDescriptionId { code: String });

#[derive(Debug, Deserialize, Clone)]
pub struct SkyDescription {
	pub description: String,
	#[serde(flatten)]
	pub id: SkyDescriptionId,
}


id_only_eq_impl!(SkyDescription, id);
meta_kind_impl!("sky" => SkyDescription);
tiered_request_entry_cache_impl!(SkyDescription.id: SkyDescriptionId);
test_impl!(SkyDescription);

#[derive(Debug, Deserialize, PartialEq, Eq, Copy, Clone)]
pub enum Sky {
	/// Day Game.
	#[serde(rename = "day")]
	Day,

	/// Night Game.
	#[serde(rename = "night")]
	Night,
}
