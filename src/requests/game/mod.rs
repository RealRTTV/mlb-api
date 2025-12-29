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

id!(GameId { gamePk: u32 });

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
