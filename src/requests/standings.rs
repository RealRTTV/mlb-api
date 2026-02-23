//! Standings of a team

use crate::divisions::DivisionId;
use crate::league::LeagueId;
use crate::meta::StandingsType;
use crate::request::{RequestURL, RequestURLBuilderExt};
use crate::season::SeasonId;
use crate::sport::SportId;
use crate::stats::ThreeDecimalPlaceRateStat;
use crate::team::NamedTeam;
use crate::Copyright;
use bon::Builder;
use chrono::{NaiveDate, NaiveDateTime};
use derive_more::{Deref, DerefMut};
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

/// A team's record and standings information. Lots of stuff here.
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

#[derive(Debug, Deserialize, PartialEq, Eq, Copy, Clone)]
pub struct Record {
    wins: usize,
    losses: usize,
    ties: usize,
}

impl Record {
    /// % of games that end in a win
    #[must_use]
    pub fn pct(self) -> ThreeDecimalPlaceRateStat {
        (self.wins as f64 / (self.wins + self.losses + self.ties) as f64).into()
    }
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
