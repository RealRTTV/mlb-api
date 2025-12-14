use derive_more::From;
use mlb_api_proc::{EnumDeref, EnumDerefMut, EnumTryAs, EnumTryAsMut, EnumTryInto};
use serde::Deserialize;

pub mod boxscore;
pub mod changes;
pub mod color;
pub mod content;
pub mod context_metrics;
pub mod diff;
pub mod linescore;
pub mod pace;
pub mod pbp;
pub mod timestamps;
pub mod uniforms;
pub mod win_probability;

integer_id!(GameId);

#[derive(Debug, Deserialize, PartialEq, Eq, Clone, Default)]
pub struct IdentifiableGame {
	#[serde(rename = "gamePk")]
	pub id: GameId,
}

#[derive(Debug, Deserialize, Eq, Clone, From, EnumTryAs, EnumTryAsMut, EnumTryInto, EnumDeref, EnumDerefMut)]
#[serde(untagged)]
pub enum Game {
	Identifiable(IdentifiableGame),
}

id_only_eq_impl!(Game, id);

#[derive(Debug, Deserialize, PartialEq, Eq, Copy, Clone)]
pub enum DoubleHeaderKind {
	#[serde(rename = "N")]
	/// Not a doubleheader
	Not,

	#[serde(rename = "Y")]
	/// First game in a double-header
	FirstGame,

	#[serde(rename = "S")]
	/// Second game in a double-header.
	SecondGame,
}

impl DoubleHeaderKind {
	#[must_use]
	pub const fn is_double_header(self) -> bool {
		matches!(self, Self::FirstGame | Self::SecondGame)
	}
}
