use crate::stats::raw::OmittedStatError;
use crate::stats::units::{CountingStat, InningsPitched, PercentageStat, ThreeDecimalPlaceRateStat, TwoDecimalPlaceRateStat};

type Result<T, E = OmittedStatError> = core::result::Result<T, E>;

macro_rules! wrap {
    ($expr:expr) => {
        (|| Ok($expr))().unwrap_or_default()
    };
}

/// # AVG (BA) - Batting Average
/// Describes the probability of a hit within an at bat, aka: the amount of hits per at bat
///
/// Hitters: Higher is better.
/// Pitchers: Lower is better.
#[must_use]
pub fn avg(hits: Result<CountingStat>, at_bats: Result<CountingStat>) -> ThreeDecimalPlaceRateStat {
    wrap!(ThreeDecimalPlaceRateStat::new(hits? as f64 / at_bats? as f64))
}

/// # SLG - Slugging
/// Describes the amount of bases averaged per each at bat.
///
/// Hitters: Higher is better.
/// Pitchers: Lower is better.
#[must_use]
pub fn slg(total_bases: Result<CountingStat>, at_bats: Result<CountingStat>) -> ThreeDecimalPlaceRateStat {
    wrap!(ThreeDecimalPlaceRateStat::new(total_bases? as f64 / at_bats? as f64))
}

/// # OBP - On-Base Percentage
/// Describes the probability of getting on base by any form, HBP, Walk, Intentional Walk, etc. per each PA.
///
/// Hitters: Higher is better.
/// Pitchers: Lower is better.
#[must_use]
pub fn obp(
    hits: Result<CountingStat>,
    base_on_balls: Result<CountingStat>,
    intentional_walks: Result<CountingStat>,
    hit_by_pitch: Result<CountingStat>,
    at_bats: Result<CountingStat>,
    sac_bunts: Result<CountingStat>,
    sac_hits: Result<CountingStat>,
) -> ThreeDecimalPlaceRateStat {
    wrap!(ThreeDecimalPlaceRateStat::new((hits? + base_on_balls? + intentional_walks? + hit_by_pitch?) as f64 / (at_bats? + base_on_balls? + intentional_walks? + hit_by_pitch? + sac_bunts? + sac_hits?) as f64))
}

/// # OPS - On-Base Plus Slugging
/// Adds OBP and SLG values together to make a new stat (yes, this means both components are weighted equally)
/// Typically this is used as a trivial way to rank performance, however if possible, using [`wOBAPiece::wOBA`]-like stats is recommended as they are generally more accurate.
///
/// Hitters: Higher is better.
/// Pitchers: Lower is better.
#[must_use]
pub fn ops(obp: Result<ThreeDecimalPlaceRateStat>, slg: Result<ThreeDecimalPlaceRateStat>) -> ThreeDecimalPlaceRateStat {
    wrap!(ThreeDecimalPlaceRateStat::new(*obp? + *slg?))
}

/// # SB% - Stolen Base Percentage
/// Describes the probability of a stolen base, given an attempt
///
/// Hitters: Higher is better.
/// Pitchers: Lower is better.
#[must_use]
pub fn stolen_base_pct(stolen_bases: Result<CountingStat>, caught_stealing: Result<CountingStat>) -> PercentageStat {
    wrap!(PercentageStat::new(stolen_bases? as f64 / (stolen_bases? + caught_stealing?) as f64))
}

/// # CS% - Caught Stealing Percentage
/// Describes the probability of failing to steal a base, given an attempt
///
/// Hitters: Lower is better.
/// Pitchers: Higher is better.
#[must_use]
pub fn caught_stealing_pct(stolen_bases: Result<CountingStat>, caught_stealing: Result<CountingStat>) -> PercentageStat {
    wrap!(PercentageStat::new(caught_stealing? as f64 / (stolen_bases? + caught_stealing?) as f64))
}

/// # BABIP - Batting Average on Balls in Play
/// Describes the batting average, only sampling balls that are in play.\
/// This stat is typically used as a "luck-indicator" stat. Being around .400 or greater is generally considered lucky, however below .300 or so is considered unlucky.\
/// Using expected stats (ex: `xwOBA` or `xAVG`) and comparing to the actual-outcome stats (ex: `wOBA` and `AVG`) generally gives a clearer indicator of luck, however these numbers are harder to find.
///
/// Hitters: Higher is better.
/// Pitchers: Lower is better.
#[must_use]
pub fn babip(
    hits: Result<CountingStat>,
    home_runs: Result<CountingStat>,
    at_bats: Result<CountingStat>,
    strikeouts: Result<CountingStat>,
    sac_flies: Result<CountingStat>,
) -> ThreeDecimalPlaceRateStat {
    wrap!(ThreeDecimalPlaceRateStat::new((hits? - home_runs?) as f64 / (at_bats? - strikeouts? - home_runs? - sac_flies?) as f64))
}

/// # BB%
/// Percentage of plate appearances that end in a walk (unintentional)
///
/// Hitters: Higher is better.
/// Pitchers: Lower is better.
#[must_use]
pub fn bb_pct(base_on_balls: Result<CountingStat>, plate_appearances: Result<CountingStat>) -> PercentageStat {
    wrap!(PercentageStat::new(base_on_balls? as f64 / plate_appearances? as f64))
}

/// # K%
/// Percentage of plate appearances that end in a strikeout
///
/// Hitters: Lower is better.
/// Pitchers: Higher is better.
#[must_use]
pub fn k_pct(strikeouts: Result<CountingStat>, plate_appearances: Result<CountingStat>) -> PercentageStat {
    wrap!(PercentageStat::new(strikeouts? as f64 / plate_appearances? as f64))
}

/// # Extra Bases
pub fn extra_bases(doubles: Result<CountingStat>, triples: Result<CountingStat>, home_runs: Result<CountingStat>) -> Result<CountingStat> {
    Ok(doubles? + triples? * 2 + home_runs? * 3)
}

/// # ISO - Isolated Power
/// Describes the amount of extra bases hit per at bat.
///
/// Hitters: Higher is better.
/// Pitchers: Lower is better.
#[must_use]
pub fn iso(extra_bases: Result<CountingStat>, at_bats: Result<CountingStat>) -> ThreeDecimalPlaceRateStat {
    wrap!(ThreeDecimalPlaceRateStat::new(extra_bases? as f64 / at_bats? as f64))
}

/// # K/BB Ratio (Strikeout to Walk Ratio)
/// Ratio between strikeouts and walks
///
/// Hitters: Lower is better.
/// Pitchers: Higher is better.
#[must_use]
pub fn strikeout_to_walk_ratio(strikeouts: Result<CountingStat>, base_on_balls: Result<CountingStat>) -> TwoDecimalPlaceRateStat {
    wrap!(TwoDecimalPlaceRateStat::new(strikeouts? as f64 / base_on_balls? as f64))
}

/// # Whiff%
/// Percentage of swings that miss.
///
/// Hitters: Lower is better.
/// Pitchers: Higher is better.
#[must_use]
pub fn whiff_pct(whiffs: Result<CountingStat>, total_swings: Result<CountingStat>) -> PercentageStat {
    wrap!(PercentageStat::new(whiffs? as f64 / total_swings? as f64))
}

/// # ERA - Earned Run Average
/// The expected number of earned runs to be given up over nine innings of pitching.
///
/// Pitchers: Lower is better.
#[must_use]
pub fn era(earned_runs: Result<CountingStat>, innings_pitched: Result<InningsPitched>) -> TwoDecimalPlaceRateStat {
    wrap!(TwoDecimalPlaceRateStat::new(earned_runs? as f64 * 27.0 / innings_pitched?.as_outs() as f64))
}

/// # WHIP - Walks & Hits per Inning Pitched
/// Described in title.
///
/// Pitchers: Lower is better.
#[must_use]
pub fn whip(hits: Result<CountingStat>, base_on_balls: Result<CountingStat>, innings_pitched: Result<InningsPitched>) -> TwoDecimalPlaceRateStat {
    wrap!(TwoDecimalPlaceRateStat::new(((base_on_balls? + hits?) as f64) / innings_pitched?.as_fraction()))
}

/// # WPCT - Win %
/// Percentage of decisions that are pitcher wins
///
/// Pitchers: Higher is better.
#[must_use]
pub fn win_pct(wins: Result<CountingStat>, losses: Result<CountingStat>) -> ThreeDecimalPlaceRateStat {
    wrap!(ThreeDecimalPlaceRateStat::new(wins? as f64 / (wins? + losses?) as f64))
}

/// # P/IP - Pitches per Inning Pitched
/// Described in title.
///
/// Pitchers: Lower is better.*
///
/// \* High K% pitchers often have higher P/IP compared to GB% pitchers - even if you find similar wOBA and ERA values for both.
#[must_use]
pub fn pitches_per_inning_pitched(number_of_pitches: Result<CountingStat>, innings_pitched: Result<InningsPitched>) -> TwoDecimalPlaceRateStat {
    wrap!(TwoDecimalPlaceRateStat::new(number_of_pitches? as f64 / innings_pitched?.as_fraction()))
}

/// # K/9 - Strikeouts per 9 Innings Pitched
/// Described in title.
/// K% is often preferred due to better edge cases.
///
/// Pitchers: Higher is better.
#[must_use]
pub fn k_per_9(strikeouts: Result<CountingStat>, innings_pitched: Result<InningsPitched>) -> TwoDecimalPlaceRateStat {
    wrap!(TwoDecimalPlaceRateStat::new(strikeouts? as f64 * 27.0 / innings_pitched?.as_outs() as f64))
}

/// # BB/9 - Walks per 9 Innings Pitched
/// Described in title.
/// BB% is often preferred due to better edge cases.
///
/// Pitchers: Lower is better.
#[must_use]
pub fn bb_per_9(base_on_balls: Result<CountingStat>, innings_pitched: Result<InningsPitched>) -> TwoDecimalPlaceRateStat {
    wrap!(TwoDecimalPlaceRateStat::new(base_on_balls? as f64 * 27.0 / innings_pitched?.as_outs() as f64))
}

/// # H/9 - Hits per 9 Innings
/// Described in title.
///
/// Pitchers: Lower is better.
#[must_use]
pub fn hits_per_9(hits: Result<CountingStat>, innings_pitched: Result<InningsPitched>) -> TwoDecimalPlaceRateStat {
    wrap!(TwoDecimalPlaceRateStat::new(hits? as f64 * 27.0 / innings_pitched?.as_outs() as f64))
}

/// # RA/9 - Runs scored per 9 Innings Pitched
/// Described in title.
///
/// Pitchers: Lower is better.
#[must_use]
pub fn runs_scored_per_9(runs: Result<CountingStat>, innings_pitched: Result<InningsPitched>) -> TwoDecimalPlaceRateStat {
    wrap!(TwoDecimalPlaceRateStat::new(runs? as f64 * 27.0 / innings_pitched?.as_outs() as f64))
}

/// # HR/9 - Home Runs per 9 Innings
/// Described in title.
///
/// Pitchers: Lower is better.
#[must_use]
pub fn home_runs_per_9(home_runs: Result<CountingStat>, innings_pitched: Result<InningsPitched>) -> TwoDecimalPlaceRateStat {
    wrap!(TwoDecimalPlaceRateStat::new(home_runs? as f64 / innings_pitched?.as_outs() as f64))
}
