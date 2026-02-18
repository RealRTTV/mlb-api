use std::hash::{Hash, Hasher};
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

impl Hash for NamedPosition {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.code.hash(state);
	}
}

impl NamedPosition {
	#[must_use]
	pub fn unknown_position() -> Self {
		Self {
			code: PositionCode::new("X"),
			name: "Unknown".to_owned(),
			r#type: "Unknown".to_owned(),
			abbreviation: "X".to_owned(),
		}
	}
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

impl Hash for Position {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.code.hash(state);
	}
}

impl Position {
	#[must_use]
	pub fn unknown_position() -> Self {
		Self {
			short_name: "Unknown".to_owned(),
			full_name: "Unknown".to_owned(),
			formal_name: "Unknown".to_owned(),
			is_pitcher: false,
			is_game_position: false,
			is_fielder: false,
			is_outfield: false,
			inner: NamedPosition::unknown_position(),
		}
	}
}

id_only_eq_impl!(Position, code);
id_only_eq_impl!(NamedPosition, code);
meta_kind_impl!("positions" => Position);
tiered_request_entry_cache_impl!([Position, NamedPosition].code: PositionCode);
test_impl!(Position);
