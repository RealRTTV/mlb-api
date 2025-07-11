use derive_more::Display;
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Eq, Copy, Clone, Display)]
#[serde(try_from = "__GameTypeStruct")]
pub enum GameType {
    #[display("Spring Training")]
    SpringTraining,
    
    #[display("Intrasquad")]
    Intrasquad,
    
    #[display("Exhibition")]
    Exhibition,
    
    #[display("Nineteenth Century Series")]
    NineteenthCenturySeries,
    
    #[display("Regular Season")]
    RegularSeason,

    #[display("All Star Game")]
    AllStarGame,
    
    #[display("Divisional Series")]
    DivisionalSeries,

    #[display("Wild Card Series")]
    WildCardSeries,
    
    #[display("Championship Series")]
    ChampionshipSeries,

    #[display("World Series")]
    WorldSeries,

    #[display("Playoffs")]
    Playoffs,
    
    #[display("Championship")]
    Championship,
}

#[derive(Deserialize)]
struct __GameTypeStruct {
    id: String,
}

impl TryFrom<__GameTypeStruct> for GameType {
    type Error = &'static str;

    fn try_from(value: __GameTypeStruct) -> Result<Self, Self::Error> {
        Ok(match &*value.id {
            "S" => GameType::SpringTraining,
            "I" => GameType::Intrasquad,
            "E" => GameType::Exhibition,
            "N" => GameType::NineteenthCenturySeries,
            "R" => GameType::RegularSeason,
            "A" => GameType::AllStarGame,
            "D" => GameType::DivisionalSeries,
            "F" => GameType::WildCardSeries,
            "L" => GameType::ChampionshipSeries,
            "W" => GameType::WorldSeries,
            "C" => GameType::Championship,
            "P" => GameType::Playoffs,
            _ => return Err("unknown game type"),
        })
    }
}
