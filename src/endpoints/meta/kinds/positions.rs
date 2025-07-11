use crate::endpoints::meta::MetaKind;
use derive_more::{Deref, DerefMut, From};
use serde::Deserialize;
use std::ops::{Deref, DerefMut};

#[derive(Debug, Deserialize, Deref, DerefMut, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HydratedPosition {
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

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NamedPosition {
	pub code: String,
	#[serde(alias = "displayName")]
	pub name: String,
	#[serde(rename = "type")]
	pub r#type: String,
	#[serde(alias = "abbrev")]
	pub abbreviation: String,
}

#[derive(Debug, Deserialize, Eq, Clone, From)]
#[serde(untagged)]
pub enum Position {
	Hydrated(HydratedPosition),
	Named(NamedPosition),
}

impl PartialEq for Position {
	fn eq(&self, other: &Self) -> bool {
		self.code == other.code
	}
}

impl Deref for Position {
	type Target = NamedPosition;

	fn deref(&self) -> &Self::Target {
		match self {
			Self::Hydrated(inner) => inner,
			Self::Named(inner) => inner,
		}
	}
}

impl DerefMut for Position {
	fn deref_mut(&mut self) -> &mut Self::Target {
		match self {
			Self::Hydrated(inner) => inner,
			Self::Named(inner) => inner,
		}
	}
}

impl MetaKind for Position {
	const ENDPOINT_NAME: &'static str = "positions";
}
