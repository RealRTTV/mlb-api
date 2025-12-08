use derive_more::{Add, AddAssign};
use serde::Deserialize;
use crate::endpoints::stats::units::{ThreeDecimalPlaceStat, TwoDecimalPlaceStat};

#[derive(Debug, Deserialize, PartialEq, Eq, Clone, Add, AddAssign, Default)]
#[serde(rename_all = "camelCase")]
pub struct CatchingStats {
	pub at_bats: u32,
	pub base_on_balls: u32,
	pub batters_faced: u32,
	pub catchers_interference: u32,
	pub caught_stealing: u32,
	pub earned_runs: u32,
	pub games_pitched: u32,
	pub games_played: u32,
	pub hit_by_pitch: u32,
	pub hits: u32,
	pub home_runs: u32,
	pub intentional_walks: u32,
	#[serde(rename = "passedBall")]
	pub passed_balls: u32,
	pub pickoffs: u32,
	pub runs: u32,
	pub sac_bunts: u32,
	pub sac_flies: u32,
	pub stolen_bases: u32,
	#[serde(rename = "strikeOuts")]
	pub strikeouts: u32,
	pub total_bases: u32,
	pub wild_pitches: u32,
}

impl CatchingStats {
	/// # Batting Average
	/// Describes the probability of a hit within an at bat, aka: the amount of hits per at bat
	#[must_use]
	pub fn avg(&self) -> ThreeDecimalPlaceStat {
		(self.hits as f64 / self.at_bats as f64).into()
	}

	/// # Slugging
	/// Describes the amount of bases averaged per each at bat.
	#[must_use]
	pub fn slg(&self) -> ThreeDecimalPlaceStat {
		(self.total_bases as f64 / self.at_bats as f64).into()
	}

	/// # On-Base Percentage
	/// Describes the probability of getting on base by any form, HBP, Walk, Intentional Walk, etc. per each PA.
	#[must_use]
	pub fn obp(&self) -> ThreeDecimalPlaceStat {
		((self.hits + self.base_on_balls + self.intentional_walks + self.hit_by_pitch) as f64 / (self.at_bats + self.base_on_balls + self.intentional_walks + self.hit_by_pitch + self.sac_bunts + self.sac_flies) as f64).into()
	}

	/// # On-Base Plus Slugging
	/// Adds OBP and SLG values together to make a new stat (yes, this means both components are weighted equally)
	/// Typically this is used as a trivial way to rank performance, however if possible, using [`HittingStats::wOBA`]-like stats is recommended as they are generally more accurate.
	#[must_use]
	pub fn ops(&self) -> ThreeDecimalPlaceStat {
		self.obp() + self.slg()
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

	/// # K/BB Ratio (Strikeout to Walk Ratio)
	/// Ratio between strikeouts and walks
	#[must_use]
	pub fn strikeout_to_walk_ratio(&self) -> TwoDecimalPlaceStat {
		(self.base_on_balls as f64 / self.strikeouts as f64).into()
	}
}

impl CatchingStats {
	#[must_use]
	pub const fn stolen_base_attempts(&self) -> u32 {
		self.stolen_bases + self.caught_stealing
	}
}
