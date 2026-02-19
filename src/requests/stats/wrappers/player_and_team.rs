use derive_more::{Deref, DerefMut};
use serde::Deserialize;
use crate::meta::GameType;
use crate::person::NamedPerson;
use crate::season::SeasonId;
use crate::stats::{RawStat, SingletonSplitStat};
use crate::stats::wrappers::{GameTypePiece, PlayerPiece, SeasonPiece, TeamPiece};
use crate::team::NamedTeam;

#[derive(Debug, Deserialize, PartialEq, Eq, Clone, Deref, DerefMut)]
#[serde(rename_all = "camelCase")]
#[serde(bound = "T: RawStat")]
pub struct WithPlayerAndTeam<T: RawStat> {
    pub player: NamedPerson,
    pub game_type: GameType,
    pub season: SeasonId,

    pub team: NamedTeam,

    #[deref]
    #[deref_mut]
    #[serde(rename = "stat")]
    pub stats: T,
}

impl<T: RawStat> SeasonPiece for WithPlayerAndTeam<T> {
    fn season(&self) -> &SeasonId {
        &self.season
    }
}

impl<T: RawStat> PlayerPiece for WithPlayerAndTeam<T> {
    fn player(&self) -> &NamedPerson {
        &self.player
    }
}

impl<T: RawStat> GameTypePiece for WithPlayerAndTeam<T> {
    fn game_type(&self) -> &GameType {
        &self.game_type
    }
}

impl<T: RawStat> TeamPiece for WithPlayerAndTeam<T> {
    fn team(&self) -> &NamedTeam {
        &self.team
    }
}

impl<T: RawStat> Default for WithPlayerAndTeam<T> {
    fn default() -> Self {
        Self {
            player: NamedPerson::unknown_person(),
            game_type: GameType::default(),
            season: SeasonId::current_season(),
            team: NamedTeam::unknown_team(),
            stats: T::default(),
        }
    }
}

impl<T: RawStat> SingletonSplitStat for WithPlayerAndTeam<T> {}
