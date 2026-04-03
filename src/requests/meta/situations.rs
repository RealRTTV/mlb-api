use derive_more::Display;
use serde::Deserialize;

/// Filters searching through situation codes to filter with multiple situation codes.
///
/// Note that with 0 [`SituationCode`](SituationCodeId)s [`SituationCodeFilter::Any`] returns `false` and [`SituationCodeFilter::All`] returns `true`.
#[derive(Copy, Clone, Default, PartialEq, Eq)]
pub enum SituationCodeFilter {
	/// Display results that match <u>all</u> the [`SituationCode`]s selected.
	All,

	/// Display results that match <u>any</u> the [`SituationCode`]s selected.
	#[default]
	Any,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Copy, Clone, Display)]
pub enum SituationCodeCategory {
	/// The environment the game is played in, turf, grass, day, night, home, away.
	#[serde(rename = "Game")]
	#[display("Game")]
	Game,

	/// The month the game is played in
	#[serde(rename = "Month")]
	#[display("Month")]
	Month,

	/// The time in the schedule the game is played, pre-ASG, last 30 days, etc.
	#[serde(rename = "Timeframe")]
	#[display("Timeframe")]
	Timeframe,

	/// Day of the week
	#[serde(rename = "Day")]
	#[display("Day")]
	DayOfWeek,

	/// What kind of opponent you're facing, divisional, league, etc.
	#[serde(rename = "Opponent")]
	#[display("Opponent")]
	Opponent,

	/// Venue-related filters.
	#[serde(rename = "Venue")]
	#[display("Venue")]
	Venue,

	/// General At-Bat filters; LHB, RHP, etc.
	#[serde(rename = "At-Bat")]
	#[display("At-Bat")]
	AtBat,

	/// Scoring data, leading, trailing, how many runs they lead by, etc.
	#[serde(rename = "Score")]
	#[display("Score")]
	GameScore,

	/// Inning filters; extra innings, first inning, etc.
	#[serde(rename = "Inning")]
	#[display("Inning")]
	Inning,

	/// Game result; win, loss, previous was a win, etc.
	#[serde(rename = "Result")]
	#[display("Result")]
	GameResult,

	/// Batting order position
	#[serde(rename = "Order")]
	#[display("Order")]
	BattingOrder,

	/// Runner positions on the bases
	#[serde(rename = "Runners")]
	#[display("Runners")]
	Runners,

	/// Fielding Position
	#[serde(rename = "Position")]
	#[display("Position")]
	Position,

	/// Days of rest, starting, relieving, etc.
	#[serde(rename = "Pitching")]
	#[display("Pitching")]
	Pitching,

	/// Pitch count filters.
	#[serde(rename = "Pitch Count")]
	#[display("Pitch Count")]
	PitchCount,

	/// None out, One out, Two out
	#[serde(rename = "Outs")]
	#[display("Outs")]
	Outs,

	/// 0-0 Count, 3-2 Count, etc.
	#[serde(rename = "Count")]
	#[display("Count")]
	AtBatCount,

	/// Type of pitch thrown
	#[serde(rename = "Pitch Type")]
	#[display("Pitch Type")]
	PitchType,
}

id!(#[doc = "A [`String`] representing a situation, such as c00 for a 0-0 count"] SituationCodeId { code: String });

/// A specific situation in a game
///
/// ## Examples
/// ```
/// SituationCode {
///     category: Some(SituationCodeCategory::AtBatCount),
///     is_team_active: true,
///     is_batting_active: true,
///     is_fielding_active: false,
///     is_pitching_active: true,
/// }
/// ```
#[allow(clippy::struct_excessive_bools, reason = "false positive")]
#[derive(Debug, Deserialize, Clone)]
pub struct SituationCode {
	#[serde(rename = "navigationMenu", default)]
	pub category: Option<SituationCodeCategory>,
	pub description: String,
	/// If the [`SituationCode`] can be applied to team stats
	#[serde(rename = "team")]
	pub is_team_active: bool,
	/// If the [`SituationCode`] can be applied to hitting stats
	#[serde(rename = "batting")]
	pub is_batting_active: bool,
	/// If the [`SituationCode`] can be applied to fielding stats
	#[serde(rename = "fielding")]
	pub is_fielding_active: bool,
	/// If the [`SituationCode`] can be applied to pitching stats
	#[serde(rename = "pitching")]
	pub is_pitching_active: bool,
	#[serde(flatten)]
	pub id: SituationCodeId,
}

id_only_eq_impl!(SituationCode, id);
meta_kind_impl!("situationCodes" => SituationCode);
tiered_request_entry_cache_impl!(SituationCode.id: SituationCodeId);
test_impl!(SituationCode);
