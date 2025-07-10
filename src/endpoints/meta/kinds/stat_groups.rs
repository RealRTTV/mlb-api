use serde::Deserialize;
use thiserror::Error;

#[derive(Debug, Deserialize, PartialEq, Eq, Copy, Clone)]
#[serde(try_from = "StatGroupStruct")]
pub enum StatGroup {
    Hitting,
    Pitching,
    Fielding,
    Catching,
    Running,
    Game,
    Team,
    Streak,
}

#[derive(Deserialize)]
struct StatGroupStruct(String);

#[derive(Debug, Error)]
pub enum StatGroupError {
    #[error("Invalid stat group '{0}'")]
    InvalidName(String),
}

impl TryFrom<StatGroupStruct> for StatGroup {
    type Error = StatGroupError;
    
    fn try_from(value: StatGroupStruct) -> Result<Self, Self::Error> {
        Ok(match &*value.0 {
            "hitting" => Self::Hitting,
            "pitching" => Self::Pitching,
            "fielding" => Self::Fielding,
            "catching" => Self::Catching,
            "running" => Self::Running,
            "game" => Self::Game,
            "team" => Self::Team,
            "streak" => Self::Streak,
            _ => return Err(StatGroupError::InvalidName(value.0)),
        })
    }
}
