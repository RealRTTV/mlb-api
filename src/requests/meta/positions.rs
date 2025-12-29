use derive_more::{Deref, DerefMut};
use serde::Deserialize;

id!(PositionCode { code: String });

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NamedPosition {
	pub code: PositionCode,
	#[serde(alias = "displayName")]
	pub name: String,
	#[serde(rename = "type")]
	pub r#type: String,
	#[serde(alias = "abbrev")]
	pub abbreviation: String,
}

#[allow(clippy::struct_excessive_bools, reason = "false positive")]
#[derive(Debug, Deserialize, Deref, DerefMut, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Position {
	pub short_name: String,
	pub full_name: String,
	pub formal_name: String,
	#[serde(rename = "pitcher")]
	pub is_pitcher: bool,
	#[serde(rename = "gamePosition")]
	pub is_game_position: bool,
	#[serde(rename = "fielder")]
	pub is_fielder: bool,
	#[serde(rename = "outfield")]
	pub is_outfield: bool,

	#[deref]
	#[deref_mut]
	#[serde(flatten)]
	inner: NamedPosition,
}

id_only_eq_impl!(Position, code);
id_only_eq_impl!(NamedPosition, code);
meta_kind_impl!("positions" => Position);
tiered_request_entry_cache_impl!([Position, NamedPosition].code: PositionCode);
test_impl!(Position);
