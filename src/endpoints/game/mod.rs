use derive_more::{Deref, Display, From};
use serde::Deserialize;
use std::ops::{Deref, DerefMut};
use strum::EnumTryAs;

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

#[repr(transparent)]
#[derive(Debug, Default, Deserialize, Deref, Display, PartialEq, Eq, Copy, Clone)]
pub struct GameId(u32);

impl GameId {
	#[must_use]
	pub const fn new(id: u32) -> Self {
		Self(id)
	}
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
pub struct IdentifiableGame {
	#[serde(rename = "gamePk")]
	pub id: GameId,
}

impl Default for IdentifiableGame {
	fn default() -> Self {
		Self { id: GameId::default() }
	}
}

#[derive(Debug, Deserialize, Eq, Clone, From, EnumTryAs)]
#[serde(untagged)]
pub enum Game {
	Identifiable(IdentifiableGame),
}

impl Default for Game {
	fn default() -> Self {
		Self::Identifiable(IdentifiableGame::default())
	}
}

impl Deref for Game {
	type Target = IdentifiableGame;

	fn deref(&self) -> &Self::Target {
		match self {
			Self::Identifiable(inner) => inner,
		}
	}
}

impl DerefMut for Game {
	fn deref_mut(&mut self) -> &mut Self::Target {
		match self {
			Self::Identifiable(inner) => inner,
		}
	}
}

impl PartialEq for Game {
	fn eq(&self, other: &Self) -> bool {
		self.id == other.id
	}
}
