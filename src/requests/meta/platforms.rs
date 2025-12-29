use serde::Deserialize;

id!(PlatformId { platformCode: String });

#[derive(Debug, Deserialize, Clone)]
pub struct Platform {
	#[serde(rename = "platformDescription")]
	pub name: String,
	#[serde(flatten)]
	pub id: PlatformId,
}

id_only_eq_impl!(Platform, id);
meta_kind_impl!("platforms" => Platform);
tiered_request_entry_cache_impl!(Platform.id: PlatformId);
test_impl!(Platform);
