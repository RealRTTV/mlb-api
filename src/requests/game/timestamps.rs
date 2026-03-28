//! A [`SimplifiedTimestamp`] list for all plays and events in a game.

use std::{fmt::Display, str::FromStr};

use bon::Builder;
use chrono::{DateTime, NaiveDate, NaiveDateTime, ParseError, Utc};
use derive_more::{From, Into};
use serde::Deserialize;
use serde::de::Error;
use serde_with::DisplayFromStr;

use crate::game::GameId;
use crate::request::RequestURL;

pub type GameTimestampsResponse = Vec<SimplifiedTimestamp>;

/// Represents a UTC datetime in a very simple format, ex: `20251231_235959`
#[derive(Debug, PartialEq, Eq, Clone, From, Into)]
pub struct SimplifiedTimestamp(DateTime<Utc>);

impl FromStr for SimplifiedTimestamp {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(NaiveDateTime::parse_from_str(s, "%Y%m%d_%H%M%S")?.and_utc()))
    }
}

impl Display for SimplifiedTimestamp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.format("%Y%d%m_%H%M%S"))
    }
}

impl<'de> Deserialize<'de> for SimplifiedTimestamp {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>
    {
        Self::from_str(&String::deserialize(deserializer)?).map_err(D::Error::custom)
    }
}

impl SimplifiedTimestamp {
    #[must_use]
    pub fn now() -> Self {
        Self(Utc::now())
    }
}

#[derive(Builder)]
#[builder(derive(Into))]
pub struct GameTimestampsRequest {
    #[builder(into)]
    id: GameId,
}

impl Display for GameTimestampsRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "http://statsapi.mlb.com/api/v1.1/game/{}/feed/live/timestamps", self.id)
    }
}

impl RequestURL for GameTimestampsRequest {
    type Response = GameTimestampsResponse;
}

impl<S: game_timestamps_request_builder::State + game_timestamps_request_builder::IsComplete> crate::request::RequestURLBuilderExt for GameTimestampsRequestBuilder<S> {
    type Built = GameTimestampsRequest;
}

#[cfg(test)]
mod tests {
    use crate::game::GameTimestampsRequest;
    use crate::request::RequestURLBuilderExt;

    #[tokio::test]
    async fn ws_gm7_2025_timestamps() {
        let _ =  GameTimestampsRequest::builder().id(813_024).build_and_get().await.unwrap();
    }
}

