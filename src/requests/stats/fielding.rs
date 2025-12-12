use crate::requests::stats::units::{InningsPitched, ThreeDecimalPlaceStat, TwoDecimalPlaceStat};
use derive_more::{Add, AddAssign};
use serde::Deserialize;
use serde_with::serde_as;
use serde_with::DisplayFromStr;

#[serde_as]
#[derive(Debug, Deserialize, PartialEq, Eq, Clone, Add, AddAssign, Default)]
#[serde(rename_all = "camelCase")]
pub struct FieldingStats {
	pub games_played: u32,
	pub games_started: u32,
	pub assists: u32,
	#[serde(rename = "putOuts")]
	pub putouts: u32,
	pub errors: u32,
	#[serde_as(as = "DisplayFromStr")]
	pub innings: InningsPitched,
	pub games: u32,
	pub double_plays: u32,
	pub triple_plays: u32,
	pub throwing_errors: u32,
}

impl FieldingStats {
	/// # Fielding %
	/// The probability of a play being made successfully
	#[must_use]
	pub fn fielding_pct(&self) -> ThreeDecimalPlaceStat {
		(self.errors as f64 / self.chances() as f64).into()
	}

	/// # Range Factor per Game
	/// Basic stat that describes the amount of successful plays per game played
	#[must_use]
	pub fn range_factor_per_game(&self) -> TwoDecimalPlaceStat {
		((self.putouts + self.assists) as f64 / self.games as f64).into()
	}

	/// # Range Factor per 9 Innings
	/// Basic stat that describes the amount of successful plays 9 innings of play
	#[must_use]
	pub fn range_factor_per_nine_innings(&self) -> TwoDecimalPlaceStat {
		((self.putouts + self.assists) as f64 / self.innings.as_fraction() * 9.0).into()
	}
}

impl FieldingStats {
	#[must_use]
	pub const fn chances(&self) -> u32 {
		self.putouts + self.errors + self.assists
	}
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone, Add, AddAssign, Default)]
#[serde(rename_all = "camelCase")]
pub struct SimplifiedGameLogFieldingStats {
	pub games_started: u32,
	pub caught_stealing: u32,
	pub stolen_bases: u32,
	pub assists: u32,
	#[serde(rename = "putOuts")]
	pub putouts: u32,
	pub errors: u32,
	#[serde(rename = "passedBall")]
	pub passed_balls: u32,
	pub pickoffs: u32,
}

impl SimplifiedGameLogFieldingStats {
	/// # Fielding %
	/// The probability of a play being made successfully
	#[must_use]
	pub fn fielding_pct(&self) -> ThreeDecimalPlaceStat {
		(self.errors as f64 / self.chances() as f64).into()
	}

	/// # Stolen Base Percentage
	/// Describes the probability of a stolen base, given an attempt
	#[must_use]
	pub fn stolen_base_pct(&self) -> ThreeDecimalPlaceStat {
		(self.stolen_bases as f64 / self.stolen_base_attempts() as f64).into()
	}

	/// # Caught Stealing Percentage
	/// Describes the probability of failing to steal a base, given an attempt
	#[must_use]
	pub fn caught_stealing_pct(&self) -> ThreeDecimalPlaceStat {
		(self.caught_stealing as f64 / self.stolen_base_attempts() as f64).into()
	}
}

impl SimplifiedGameLogFieldingStats {
	#[must_use]
	pub const fn chances(&self) -> u32 {
		self.putouts + self.errors + self.assists
	}

	#[must_use]
	pub const fn stolen_base_attempts(&self) -> u32 {
		self.stolen_bases + self.caught_stealing
	}
}
