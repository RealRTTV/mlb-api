mod career;
mod map;
mod season;
mod position_and_season;
mod team;
mod game;
mod player;
mod player_and_team;
mod single_matchup;
mod accumulated_matchup;
mod accumulated_vs_player_matchup;
mod accumulated_vs_team;
mod accumulated_vs_team_seasonal;
mod month;
mod weekday;
mod home_away;
mod win_loss;
mod with_none;

pub use career::*;
pub use map::*;
pub use season::*;
pub use position_and_season::*;
pub use team::*;
pub use game::*;
pub use player::*;
pub use player_and_team::*;
pub use single_matchup::*;
pub use accumulated_matchup::*;
pub use accumulated_vs_player_matchup::*;
pub use accumulated_vs_team::*;
pub use accumulated_vs_team_seasonal::*;
pub use month::*;
pub use weekday::*;
pub use home_away::*;
pub use win_loss::*;
pub use with_none::*;

use std::convert::Infallible;
use chrono::{Month, Weekday};
use crate::game::GameId;
use crate::game_types::GameType;
use crate::league::NamedLeague;
use crate::person::NamedPerson;
use crate::positions::NamedPosition;
use crate::season::SeasonId;
use crate::stats::{SingletonSplitStat, Stat};
use crate::team::NamedTeam;

impl<T: SingletonSplitStat> Stat for Vec<T> {
	type Split = T;
	type TryFromSplitError = Infallible;

	fn from_splits(splits: impl Iterator<Item=Self::Split>) -> Result<Self, Self::TryFromSplitError>
	where
		Self: Sized
	{
		Ok(Self::from_iter(splits))
	}
}

macro_rules! piece {
    ($name:ident => $ty:ty) => {
		::pastey::paste! {
			pub trait [<$name Piece>] {
				fn [<$name:snake>](&self) -> &$ty;
			}

			pub struct [<By $name>];

			impl<T: [<$name Piece>]> MapKey<T> for [<By $name>] {
				type Key = $ty;

				fn get_key(this: &T) -> Self::Key { this.[<$name:snake>]().clone() }
			}
		}
	};
}

piece!(Season => SeasonId);
piece!(Month => Month);
piece!(Weekday => Weekday);
piece!(Position => NamedPosition);
piece!(OpposingTeam => NamedTeam);
piece!(GameType => GameType);
piece!(Team => NamedTeam);
piece!(Pitcher => NamedPerson);
piece!(Batter => NamedPerson);
piece!(Player => NamedPerson);
piece!(Game => GameId);
piece!(League => NamedLeague);
