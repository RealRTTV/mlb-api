//! Standings of a team, wins, losses, etc

use crate::divisions::{DivisionId, NamedDivision};
use crate::league::{LeagueId, NamedLeague};
use crate::meta::StandingsType;
use crate::request::{RequestURL, RequestURLBuilderExt};
use crate::season::SeasonId;
use crate::sport::SportId;
use crate::stats::ThreeDecimalPlaceRateStat;
use crate::team::NamedTeam;
use crate::Copyright;
use bon::Builder;
use chrono::{NaiveDate, NaiveDateTime};
use derive_more::{Add, AddAssign, Deref, DerefMut, Display};
use itertools::Itertools;
use serde::Deserialize;
use std::cmp::Ordering;
use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;
use crate::types::MLB_API_DATE_FORMAT;

/// A [`Vec`] of [`DivisionalStandings`]
///
/// The request divides the league into its divisions and then the divisions into their teams.
#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StandingsResponse {
    pub copyright: Copyright,
    #[serde(rename = "records")]
    pub divisions: Vec<DivisionalStandings>
}

/// [`TeamRecord`]s per division. `last_updated` field might be useful for caching
#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DivisionalStandings {
    pub standings_type: StandingsType,
    #[serde(rename = "league")]
    pub league_id: LeagueId,
    #[serde(rename = "division")]
    pub division_id: DivisionId,
    #[serde(rename = "sport")]
    pub sport_id: SportId,
    #[serde(deserialize_with = "crate::deserialize_datetime")]
    pub last_updated: NaiveDateTime,
    pub team_records: Vec<TeamRecord>,
}

/// Main bulk of the response; the team's record and standings information. Lots of stuff here.
#[derive(Debug, Deserialize, PartialEq, Eq, Clone, Deref, DerefMut)]
#[serde(rename_all = "camelCase")]
pub struct TeamRecord {
    pub team: NamedTeam,
    pub season: SeasonId,
    pub games_played: usize,
    pub runs_allowed: usize,
    pub runs_scored: usize,
    #[serde(rename = "divisionChamp")]
    pub is_divisional_champion: bool,
    #[serde(rename = "divisionLeader")]
    pub is_divisional_leader: bool,
    pub has_wildcard: bool,
    #[serde(deserialize_with = "crate::deserialize_datetime")]
    pub last_updated: NaiveDateTime,
    pub streak: Streak,
    #[serde(rename = "records")]
    pub splits: RecordSplits,

    #[serde(rename = "clinchIndicator", default)]
    pub clinch_kind: ClinchKind,
    pub games_back: GamesBack,
    pub wild_card_games_back: GamesBack,
    pub league_games_back: GamesBack,
    #[serde(rename = "springLeagueGamesBack")]
    pub spring_training_games_back: GamesBack,
    pub sport_games_back: GamesBack,
    pub division_games_back: GamesBack,
    pub conference_games_back: GamesBack,
    #[deref]
    #[deref_mut]
    #[serde(rename = "leagueRecord")]
    pub record: Record,

    #[serde(rename = "divisionRank", deserialize_with = "crate::try_from_str", default)]
    pub divisional_rank: Option<usize>,
    #[serde(deserialize_with = "crate::try_from_str", default)]
    pub league_rank: Option<usize>,
    #[serde(deserialize_with = "crate::try_from_str", default)]
    pub sport_rank: Option<usize>,
}

impl TeamRecord {
    /// Uses the pythagorean expected win loss pct formula
    #[must_use]
    pub fn expected_win_loss_pct(&self) -> ThreeDecimalPlaceRateStat {
        /// Some use 2, some use 1.82, some use 1.80.
        ///
        /// The people who use 2 are using an overall less precise version
        ///
        /// I have no clue about 1.83 vs 1.80, so I took the mean.
        const EXPONENT: f64 = 1.815;

        let exponentified_runs_scored: f64 = (self.runs_scored as f64).powf(EXPONENT);
        let exponentified_runs_allowed: f64 = (self.runs_allowed as f64).powf(EXPONENT);

        (exponentified_runs_scored / (exponentified_runs_scored + exponentified_runs_allowed)).into()
    }

    /// Assumes 162 total games. Recommended to use the other function if available
    ///
    /// See [`Self::expected_end_of_season_record_with_total_games`]
    #[must_use]
    pub fn expected_end_of_season_record(&self) -> Record {
        self.expected_end_of_season_record_with_total_games(162)
    }

    /// Expected record at the end of the season considering the games already played and the expected win loss pct.
    #[must_use]
    pub fn expected_end_of_season_record_with_total_games(&self, total_games: usize) -> Record {
        let expected_pct: f64 = self.expected_win_loss_pct().into();
        let remaining_games = total_games.saturating_sub(self.record.games_played());
        let wins = (remaining_games as f64 * expected_pct).round() as usize;
        let losses = remaining_games - wins;

        self.record + Record { wins, losses }
    }

    /// Net runs scored for the team
    #[must_use]
    pub fn run_differential(&self) -> isize {
        self.runs_scored as isize - self.runs_allowed as isize
    }
}

/// Different record splits depending on the Division, League, [`RecordSplitKind`], etc.
#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
pub struct RecordSplits {
    #[serde(rename = "splitRecords", default)]
    pub record_splits: Vec<RecordSplit>,
    #[serde(rename = "divisionRecords", default)]
    pub divisional_record_splits: Vec<DivisionalRecordSplit>,
    #[serde(rename = "leagueRecords", default)]
    pub league_record_splits: Vec<LeagueRecordSplit>,
    #[serde(rename = "overallRecords", default)]
    pub basic_record_splits: Vec<RecordSplit>,
    #[serde(rename = "expectedRecords", default)]
    pub expected_record_splits: Vec<RecordSplit>,
}

/// Different indicators for clinching the playoffs.
///
/// Note: This assumes the modern postseason format, if you are dealing with older formats the predicates below are not guaranteed to work.
#[repr(u8)]
#[derive(Debug, Deserialize, PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Default)]
pub enum ClinchKind {
    /// The Team has clinched a top seed guaranteeing a bye.
    #[serde(rename = "z")]
    Bye = 4,

    /// The team has clinched their position in the division.
    #[serde(rename = "y")]
    Divisional = 3,

    /// The team has clinched a wild card position.
    #[serde(rename = "w")]
    WildCard = 2,

    /// The team has clinched a position in the postseason, however that specific placement is unknown.
    #[serde(rename = "x")]
    Postseason = 1,

    /// Team has not clinched the postseason.
    #[default]
    #[serde(skip)]
    None = 0,

    // doesn't exist?
    // #[serde(rename = "e")]
    // Eliminated = -1,
}

impl ClinchKind {
    /// Whether a team is guaranteed to play in the postseason.
    ///
    /// ## Examples
    /// ```
    /// use mlb_api::standings::ClinchKind;
    ///
    /// assert!(ClinchKind::Bye.clinched_postseason());
    /// assert!(ClinchKind::WildCard.clinched_postseason());
    /// assert!(ClinchKind::Postseason.clinched_postseason());
    /// assert!(!ClinchKind::None.clinched_postseason());
    /// ```
    #[must_use]
    pub fn clinched_postseason(self) -> bool {
        self as u8 >= Self::Postseason as u8
    }

    /// Whether the [`ClinchKind`] is a final decision and cannot be changed.
    ///
    /// ## Examples
    /// ```
    /// use mlb_api::standings::ClinchKind;
    ///
    /// assert!(ClinchKind::Bye.is_final());
    /// assert!(ClinchKind::WildCard.is_final());
    /// assert!(!ClinchKind::Postseason.is_final());
    /// assert!(!ClinchKind::None.is_final());
    /// ```
    #[must_use]
    pub fn is_final(self) -> bool {
        self as u8 >= Self::WildCard as u8
    }

    /// Whether the team will play in a Wild Card Series.
    ///
    /// If the postseason decision [is not final](Self::is_final), the team is considered to *not* play in the wild card round. If you want different behavior use [`Self::guaranteed_in_wildcard`].
    /// ## Examples
    /// ```
    ///
    /// use mlb_api::standings::ClinchKind;
    ///
    /// assert!(!ClinchKind::Bye.guaranteed_in_wildcard());
    /// assert!(ClinchKind::Divisional.guaranteed_in_wildcard());
    /// assert!(ClinchKind::WildCard.guaranteed_in_wildcard());
    /// assert!(!ClinchKind::None.guaranteed_in_wildcard());
    /// assert!(!ClinchKind::Postseason.guaranteed_in_wildcard());
    /// ```
    #[must_use]
    pub fn guaranteed_in_wildcard(self) -> bool {
        matches!(self, Self::WildCard | Self::Divisional)
    }

    /// Whether the team has a possibility of playing in the Wild Card Series.
    ///
    /// ## Examples
    /// ```
    ///
    /// use mlb_api::standings::ClinchKind;
    ///
    /// assert!(!ClinchKind::Bye.guaranteed_in_wildcard());
    /// assert!(ClinchKind::Divisional.guaranteed_in_wildcard());
    /// assert!(ClinchKind::WildCard.guaranteed_in_wildcard());
    /// assert!(ClinchKind::None.guaranteed_in_wildcard());
    /// assert!(ClinchKind::Postseason.guaranteed_in_wildcard());
    /// ```
    #[must_use]
    pub fn potentially_in_wildcard(self) -> bool {
        matches!(self, Self::WildCard | Self::Divisional | Self::Postseason | Self::None)
    }
}

#[derive(Deserialize, PartialEq, Eq, Clone)]
#[serde(try_from = "&str")]
pub struct GamesBack {
    /// How many games back a team is from the target spot.
    ///
    /// If negative, then `-games` is the amount of games to the target spot.
    /// If positive, the amount of games ahead of the target spot (ex: WC1 compared to WC3).
    /// If zero, you are matched with the target spot in terms of record, tiebreakers apply.
    games: isize,

    /// Whether the team has finished and won a game and their opponents have not, leading to being a half game ahead.
    half: bool,
}

impl PartialOrd for GamesBack {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for GamesBack {
    fn cmp(&self, other: &Self) -> Ordering {
        self.games.cmp(&other.games).then_with(|| self.half.cmp(&other.half))
    }
}

impl Display for GamesBack {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.games > 0 {
            write!(f, "+")?;
        }

        if self.games != 0 {
            write!(f, "{}", self.games.abs())?;
        } else {
            write!(f, "-")?;
        }

        write!(f, ".{c}", c = self.half.then_some('5').unwrap_or('0'))?;

        Ok(())
    }
}

impl Debug for GamesBack {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        <Self as Display>::fmt(self, f)
    }
}

impl<'a> TryFrom<&'a str> for GamesBack {
    type Error = <Self as FromStr>::Err;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        <Self as FromStr>::from_str(value)
    }
}

impl FromStr for GamesBack {
    type Err = &'static str;

    fn from_str(mut s: &str) -> Result<Self, Self::Err> {
        if s == "-" { return Ok(Self { games: 0, half: false }) }

        let sign: isize = match s.strip_prefix("+") {
            Some(s2) => {
                s = s2;
                1
            }
            None => -1,
        };

        let (games, half) = s.split_once('.').unwrap_or((s, ""));
        let games = games.parse::<usize>().map_err(|_| "invalid game quantity")?;
        let half = half == "5";

        Ok(Self {
            games: games as isize * sign,
            half,
        })
    }
}

#[derive(Debug, Deserialize, PartialEq, Eq, Copy, Clone, Add, AddAssign)]
pub struct Record {
    wins: usize,
    losses: usize,
}

impl Record {
    /// % of games that end in a win
    #[must_use]
    pub fn pct(self) -> ThreeDecimalPlaceRateStat {
        (self.wins as f64 / self.games_played() as f64).into()
    }

    /// Number of games played
    #[must_use]
    pub fn games_played(self) -> usize {
        self.wins + self.losses
    }
}

// A repetition of a kind of game outcome; ex: W5 (last 5 games were wins), L1 (last 1 game was a loss).
#[derive(Debug, Deserialize, PartialEq, Eq, Copy, Clone)]
pub struct Streak {
    #[serde(rename = "streakNumber")]
    pub quantity: usize,
    #[serde(rename = "streakType")]
    pub kind: StreakKind,
}

impl Display for Streak {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.kind, self.quantity)
    }
}

// todo: hook into [`GameOutcome`]?
/// A game outcome for streak purposes
#[derive(Debug, Deserialize, PartialEq, Eq, Copy, Clone, Display)]
pub enum StreakKind {
    /// A game that ended in a win for this team.
    #[serde(rename = "wins")]
    #[display("W")]
    Win,
    /// A game that ended in a loss for this team.
    #[serde(rename = "losses")]
    #[display("L")]
    Loss,
}

/// A team's record, filtered by the [`RecordSplitKind`].
#[derive(Debug, Deserialize, PartialEq, Eq, Copy, Clone, Deref, DerefMut)]
pub struct RecordSplit {
    #[deref]
    #[deref_mut]
    #[serde(flatten)]
    pub record: Record,
    #[serde(rename = "type")]
    pub kind: RecordSplitKind,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone, Deref, DerefMut)]
pub struct DivisionalRecordSplit {
    #[deref]
    #[deref_mut]
    #[serde(flatten)]
    pub record: Record,
    pub division: NamedDivision,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone, Deref, DerefMut)]
pub struct LeagueRecordSplit {
    #[deref]
    #[deref_mut]
    #[serde(flatten)]
    pub record: Record,
    pub league: NamedLeague,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Copy, Clone, Hash)]
#[serde(rename_all = "camelCase")]
pub enum RecordSplitKind {
    /// Games as the home team
    Home,
    /// Games as the away team
    Away,
    /// Games in which you are "left" (?)

    /// A
    Left,
    /// Games in which you are "left" && you are the home team
    LeftHome,
    /// Games in which you are "left" && you are the away team
    LeftAway,
    /// Games in which you are "right" (?)

    Right,
    /// Games in which you are "right" and the home team
    RightHome,
    /// Games in which you are "right" and the away team
    RightAway,

    /// Last 10 games of the season as of the current date.
    /// Note that early in the season, [`Record::games_played`] may be < 10
    LastTen,
    /// Games that went to extra innings.
    #[serde(rename = "extraInning")]
    ExtraInnings,
    /// Games won or lost by one run
    OneRun,

    /// what?
    Winners,

    /// Day games
    Day,
    /// Night games
    Night,

    /// Games played on grass
    Grass,
    /// Games played on turf
    Turf,

    /// Expected record using pythagorean expected win loss pct
    ///
    /// This value is calculated as a percentage and multiplied by the number of games that <u>have been</u> played.
    #[allow(non_camel_case_types)]
    #[serde(rename = "xWinLoss")]
    xWinLoss,

    /// Expected record for the season using pythagorean expected win loss pct
    ///
    /// This value is calculated as a percentage and multiplied by the number of games <u>in the season</u>.
    #[allow(non_camel_case_types)]
    #[serde(rename = "xWinLossSeason")]
    xWinLossSeason,
}

// todo: hydrations
/// Returns a [`StandingsResponse`].
#[derive(Builder)]
#[builder(derive(Into))]
pub struct StandingsRequest {
    #[builder(into)]
    league_id: LeagueId,
    #[builder(into, default)]
    season: SeasonId,
    standings_types: Option<Vec<StandingsType>>,
    #[builder(into)]
    date: Option<NaiveDate>,
}

impl<S: standings_request_builder::State + standings_request_builder::IsComplete> RequestURLBuilderExt for StandingsRequestBuilder<S> {
    type Built = StandingsRequest;
}

impl Display for StandingsRequest {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "http://statsapi.mlb.com/api/v1/standings{params}", params = gen_params! {
            "leagueId": self.league_id,
            "season": self.season,
            "standingsTypes"?: self.standings_types.as_ref().map(|x| x.iter().copied().join(",")),
            "date"?: self.date.map(|x| x.format(MLB_API_DATE_FORMAT)),
        })
    }
}

impl RequestURL for StandingsRequest {
    type Response = StandingsResponse;
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;
    use crate::league::LeagueId;
    use crate::request::RequestURLBuilderExt;
    use crate::standings::StandingsRequest;
    use crate::TEST_YEAR;

    #[tokio::test]
    async fn all_mlb_leagues_2025() {
        for league_id in [LeagueId::new(103), LeagueId::new(104)] {
            let _ = StandingsRequest::builder().season(TEST_YEAR).league_id(league_id).build_and_get().await.unwrap();
            let _ = StandingsRequest::builder().season(TEST_YEAR).date(NaiveDate::from_ymd_opt(TEST_YEAR as _, 09, 26).unwrap()).league_id(league_id).build_and_get().await.unwrap();
        }
    }
}
