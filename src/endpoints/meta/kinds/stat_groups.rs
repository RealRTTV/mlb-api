use derive_more::FromStr;
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Eq, Copy, Clone, FromStr)]
#[serde(try_from = "__StatGroupStruct")]
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
#[serde(rename_all = "camelCase")]
struct __StatGroupStruct {
    display_name: String,
}

impl TryFrom<__StatGroupStruct> for StatGroup {
    type Error = derive_more::FromStrError;
    
    fn try_from(value: __StatGroupStruct) -> Result<Self, Self::Error> {
        value.display_name.parse::<Self>()
    }
}
