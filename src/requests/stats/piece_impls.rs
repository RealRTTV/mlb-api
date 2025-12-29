#![allow(clippy::cast_lossless, reason = "this crate is not obsessed with producing the most accurate floating point representations of data, it's close enough for what we need.")]

use crate::stats::pieces::{AtBatPiece, BaseOnBallsPiece, DecisionsPiece, EarnedRunsPiece, ExtraBaseHitsPiece, GamesPitchedPiece, HitByPitchPiece, HitsPiece, InningsPitchedPiece, IntentionalWalksPiece, PitchQuantityPiece, PlateAppearancePiece, RunsPiece, SacrificeHitsPiece, StealingPiece, StrikeoutsPiece, StrikesPiece, SwingDataPiece, TotalBasesPiece};
use crate::stats::units::{PercentageStat, ThreeDecimalPlaceStat, TwoDecimalPlaceStat};

pub trait AVGPiece {
	/// # Batting Average
	/// Describes the probability of a hit within an at bat, aka: the amount of hits per at bat
	#[must_use]
	fn avg(&self) -> ThreeDecimalPlaceStat;
}

impl<T: HitsPiece + AtBatPiece> AVGPiece for T {
	fn avg(&self) -> ThreeDecimalPlaceStat {
		(self.hits() as f64 / self.at_bats() as f64).into()
	}
}

pub trait SLGPiece {
	/// # Slugging
	/// Describes the amount of bases averaged per each at bat.
	#[must_use]
	fn slg(&self) -> ThreeDecimalPlaceStat;
}

impl<T: TotalBasesPiece + AtBatPiece> SLGPiece for T {
	fn slg(&self) -> ThreeDecimalPlaceStat {
		(self.total_bases() as f64 / self.at_bats() as f64).into()
	}
}

pub trait OBPPiece {
	/// # On-Base Percentage
	/// Describes the probability of getting on base by any form, HBP, Walk, Intentional Walk, etc. per each PA.
	#[must_use]
	fn obp(&self) -> ThreeDecimalPlaceStat;
}

impl<T: TimesOnBasePiece + OBPPlateAppearancesPiece> OBPPiece for T {
	fn obp(&self) -> ThreeDecimalPlaceStat {
		(self.times_on_base() as f64 / self.obp_plate_appearances() as f64).into()
	}
}

pub trait SinglesPiece {
	#[must_use]
	fn singles(&self) -> u32;
}

impl<T: HitsPiece + ExtraBaseHitsPiece> SinglesPiece for T {
	fn singles(&self) -> u32 {
		self.hits() - self.doubles() - self.triples() - self.home_runs()
	}
}

pub trait TimesOnBasePiece {
	#[must_use]
	fn times_on_base(&self) -> u32;
}

impl<T: HitsPiece + BaseOnBallsPiece + IntentionalWalksPiece + HitByPitchPiece> TimesOnBasePiece for T {
	fn times_on_base(&self) -> u32 {
		self.hits() + self.base_on_balls() + self.intentional_walks() + self.hit_by_pitch()
	}
}

pub trait OBPPlateAppearancesPiece {
	#[must_use]
	fn obp_plate_appearances(&self) -> u32;
}

impl<T: AtBatPiece + BaseOnBallsPiece + IntentionalWalksPiece + HitByPitchPiece + SacrificeHitsPiece> OBPPlateAppearancesPiece for T {
	fn obp_plate_appearances(&self) -> u32 {
		self.at_bats() + self.base_on_balls() + self.intentional_walks() + self.hit_by_pitch() + self.sac_bunts() + self.sac_flies()
	}
}

pub trait OPSPiece {
	/// # On-Base Plus Slugging
	/// Adds OBP and SLG values together to make a new stat (yes, this means both components are weighted equally)
	/// Typically this is used as a trivial way to rank performance, however if possible, using [`wOBAPiece::wOBA`]-like stats is recommended as they are generally more accurate.
	#[must_use]
	fn ops(&self) -> ThreeDecimalPlaceStat;
}

impl<T: OBPPiece + SLGPiece> OPSPiece for T {
	fn ops(&self) -> ThreeDecimalPlaceStat {
		self.obp() + self.slg()
	}
}

pub trait TripleSlash: AVGPiece + OBPPiece + SLGPiece {
	#[must_use]
	fn triple_slash(&self) -> (ThreeDecimalPlaceStat, ThreeDecimalPlaceStat, ThreeDecimalPlaceStat) {
		(self.avg(), self.obp(), self.slg())
	}
}

pub trait StolenBasePctPiece {
	/// # Stolen Base Percentage
	/// Describes the probability of a stolen base, given an attempt
	#[must_use]
	fn stolen_base_pct(&self) -> ThreeDecimalPlaceStat;

	/// # Caught Stealing Percentage
	/// Describes the probability of failing to steal a base, given an attempt
	#[must_use]
	fn caught_stealing_pct(&self) -> ThreeDecimalPlaceStat;
}

impl<T: StealingPiece> StolenBasePctPiece for T {
	fn stolen_base_pct(&self) -> ThreeDecimalPlaceStat {
		(self.stolen_bases() as f64 / self.stolen_base_attempts() as f64).into()
	}

	fn caught_stealing_pct(&self) -> ThreeDecimalPlaceStat {
		(self.caught_stealing() as f64 / self.stolen_base_attempts() as f64).into()
	}
}

pub trait StolenBaseAttemptsPiece {
	#[must_use]
	fn stolen_base_attempts(&self) -> u32;
}

impl<T: StealingPiece> StolenBaseAttemptsPiece for T {
	fn stolen_base_attempts(&self) -> u32 {
		self.stolen_bases() + self.caught_stealing()
	}
}

pub trait BAFieldPiece {
	/// # Batting Average on Balls in Play
	/// Describes the batting average, only sampling balls that are in play.\
	/// This stat is typically used as a "luck-indicator" stat. Being around .400 or greater is generally considered lucky, however below .300 or so is considered unlucky.\
	/// Using expected stats (ex: `xwOBA` or `xAVG`) and comparing to the actual-outcome stats (ex: `wOBA` and `AVG`) generally gives a clearer indicator of luck, however these numbers are harder to find.
	#[must_use]
	fn babip(&self) -> ThreeDecimalPlaceStat;
}

impl<T: HitsPiece + ExtraBaseHitsPiece + AtBatPiece + StrikeoutsPiece + SacrificeHitsPiece> BAFieldPiece for T {
	fn babip(&self) -> ThreeDecimalPlaceStat {
		// would be more accurate if we could account for inside-the-park home-runs
		((self.hits() - self.home_runs()) as f64 / (self.at_bats() - self.strikeouts() - self.home_runs() + self.sac_flies()) as f64).into()
	}
}

pub trait BBPctPiece {
	/// # BB%
	/// Percentage of plate appearances that end in a walk (unintentional)
	#[must_use]
	fn walk_pct(&self) -> PercentageStat;
}

impl<T: BaseOnBallsPiece + PlateAppearancePiece> BBPctPiece for T {
	fn walk_pct(&self) -> PercentageStat {
		(self.base_on_balls() as f64 / self.plate_appearances() as f64).into()
	}
}

pub trait KPctPiece {
	/// # K%
	/// Percentage of plate appearances that end in a strikeout
	#[must_use]
	fn strikeout_pct(&self) -> PercentageStat;
}

impl<T: StrikeoutsPiece + PlateAppearancePiece> KPctPiece for T {
	fn strikeout_pct(&self) -> PercentageStat {
		(self.strikeouts() as f64 / self.plate_appearances() as f64).into()
	}
}

pub trait ExtraBasesPiece {
	#[must_use]
	fn extra_bases(&self) -> u32;
}

impl<T: ExtraBaseHitsPiece> ExtraBasesPiece for T {
	fn extra_bases(&self) -> u32 {
		self.doubles() + self.triples() * 2 + self.home_runs() * 3
	}
}

pub trait ISOPiece {
	/// # Isolated Power
	/// Describes the amount of extra bases hit per at bat.
	#[must_use]
	fn iso(&self) -> ThreeDecimalPlaceStat;
}

impl<T: AtBatPiece + ExtraBaseHitsPiece> ISOPiece for T {
	fn iso(&self) -> ThreeDecimalPlaceStat {
		(self.extra_bases() as f64 / self.at_bats() as f64).into()
	}
}

pub trait StrikeoutToWalkRatioPiece {
	/// # K/BB Ratio (Strikeout to Walk Ratio)
	/// Ratio between strikeouts and walks
	#[must_use]
	fn strikeout_to_walk_ratio(&self) -> TwoDecimalPlaceStat;
}

impl<T: StrikeoutsPiece + BaseOnBallsPiece> StrikeoutToWalkRatioPiece for T {
	fn strikeout_to_walk_ratio(&self) -> TwoDecimalPlaceStat {
		(self.base_on_balls() as f64 / self.strikeouts() as f64).into()
	}
}

pub trait WhiffPctPiece {
	/// # Whiff%
	/// Percentage of swings that miss.
	#[must_use]
	fn whiff_pct(&self) -> PercentageStat;
}

impl<T: SwingDataPiece> WhiffPctPiece for T {
	fn whiff_pct(&self) -> PercentageStat {
		(self.whiffs() as f64 / self.total_swings() as f64).into()
	}
}

pub trait BallsPiece {
	/// # Balls
	/// The total amount of balls thrown by the pitcher
	#[must_use]
	fn balls(&self) -> u32;
}

impl<T: StrikesPiece + PitchQuantityPiece> BallsPiece for T {
	fn balls(&self) -> u32 {
		self.num_pitches() - self.strikes()
	}
}

pub trait ERAPiece {
	/// # Earned Run Average
	/// The expected number of earned runs to be given up over nine innings of pitching.
	#[must_use]
	fn era(&self) -> ThreeDecimalPlaceStat;
}

impl<T: EarnedRunsPiece + InningsPitchedPiece> ERAPiece for T {
	fn era(&self) -> ThreeDecimalPlaceStat {
		(self.earned_runs() as f64 / self.innings_pitched().as_fraction() * 9.0).into()
	}
}

pub trait WHIPPiece {
	/// # Walks & Hits per Inning Pitched
	/// Described in title.
	#[must_use]
	fn whip(&self) -> ThreeDecimalPlaceStat;
}

impl<T: BaseOnBallsPiece + HitsPiece + InningsPitchedPiece> WHIPPiece for T {
	fn whip(&self) -> ThreeDecimalPlaceStat {
		((self.base_on_balls() + self.hits()) as f64 / self.innings_pitched().as_fraction()).into()
	}
}

pub trait StrikePct {
	/// # Strike %
	/// Percentage of pitches that are strikes.
	#[must_use]
	fn strike_pct(&self) -> PercentageStat;
}

impl<T: StrikesPiece + PitchQuantityPiece> StrikePct for T {
	fn strike_pct(&self) -> PercentageStat {
		(self.strikes() as f64 / self.num_pitches() as f64).into()
	}
}

pub trait WinLossPct {
	/// # Win %
	/// Percentage of decisions that are pitcher wins
	#[must_use]
	fn win_pct(&self) -> PercentageStat;
}

impl<T: DecisionsPiece> WinLossPct for T {
	fn win_pct(&self) -> PercentageStat {
		(self.wins() as f64 / (self.wins() + self.losses()) as f64).into()
	}
}

pub trait PitchesPerInningsPitchedPiece {
	/// # Pitches per Inning Pitched
	/// Described in title.
	#[must_use]
	fn pitches_per_inning(&self) -> TwoDecimalPlaceStat;
}

impl<T: PitchQuantityPiece + InningsPitchedPiece> PitchesPerInningsPitchedPiece for T {
	fn pitches_per_inning(&self) -> TwoDecimalPlaceStat {
		(self.num_pitches() as f64 / self.innings_pitched().as_fraction()).into()
	}
}

pub trait StrikeoutsPerNineInningsPitched {
	/// # Strikeouts per 9 Innings
	/// Described in title.
	#[must_use]
	fn strikeouts_per_nine_innings(&self) -> TwoDecimalPlaceStat;
}

impl<T: StrikeoutsPiece + InningsPitchedPiece> StrikeoutsPerNineInningsPitched for T {
	fn strikeouts_per_nine_innings(&self) -> TwoDecimalPlaceStat {
		(self.strikeouts() as f64 / self.innings_pitched().as_fraction() * 9.0).into()
	}
}

pub trait WalksPerNineInningsPitched {
	/// # Walks per 9 Innings
	/// Described in title.
	#[must_use]
	fn walks_per_nine_innings(&self) -> TwoDecimalPlaceStat;
}

impl<T: BaseOnBallsPiece + InningsPitchedPiece> WalksPerNineInningsPitched for T {
	fn walks_per_nine_innings(&self) -> TwoDecimalPlaceStat {
		(self.base_on_balls() as f64 / self.innings_pitched().as_fraction() * 9.0).into()
	}
}

pub trait HitsPerNineInningsPitched {
	/// # Hits per 9 Innings
	/// Described in title.
	#[must_use]
	fn hits_per_nine_innings(&self) -> TwoDecimalPlaceStat;
}

impl<T: HitsPiece + InningsPitchedPiece> HitsPerNineInningsPitched for T {
	fn hits_per_nine_innings(&self) -> TwoDecimalPlaceStat {
		(self.hits() as f64 / self.innings_pitched().as_fraction() * 9.0).into()
	}
}

pub trait RunAveragePerNineInningsPitched {
	/// # Runs scored per 9 Innings
	/// Described in title.
	#[must_use]
	fn runs_scored_per_nine_innings(&self) -> TwoDecimalPlaceStat;
}

impl<T: RunsPiece + InningsPitchedPiece> RunAveragePerNineInningsPitched for T {
	fn runs_scored_per_nine_innings(&self) -> TwoDecimalPlaceStat {
		(self.runs() as f64 / self.innings_pitched().as_fraction() * 9.0).into()
	}
}

pub trait HomeRunsPerNineInningsPitched {
	/// # Home Runs per 9 Innings
	/// Described in title.
	#[must_use]
	fn home_runs_per_nine_innings(&self) -> TwoDecimalPlaceStat;
}

impl<T: ExtraBaseHitsPiece + InningsPitchedPiece> HomeRunsPerNineInningsPitched for T {
	fn home_runs_per_nine_innings(&self) -> TwoDecimalPlaceStat {
		(self.home_runs() as f64 / self.innings_pitched().as_fraction() * 9.0).into()
	}
}

pub trait NoDecisionsPiece {
	#[must_use]
	fn no_decisions(&self) -> u32;
}

impl<T: DecisionsPiece + GamesPitchedPiece> NoDecisionsPiece for T {
	fn no_decisions(&self) -> u32 {
		self.games_pitched() - self.decisions()
	}
}
