use chrono::{Datelike, Local};
use serde::Deserialize;

/// Shared types across multiple endpoints
#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct Copyright(pub String);

impl Default for Copyright {
    fn default() -> Self {
        let year = Local::now().year();
        Self(format!("Copyright {year} MLB Advanced Media, L.P.  Use of any content on this page acknowledges agreement to the terms posted here http://gdx.mlb.com/components/copyright.txt"))
    }
}
