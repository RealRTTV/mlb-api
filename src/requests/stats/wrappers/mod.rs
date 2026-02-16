mod career;
mod map;
mod season;
mod position_and_season;
mod team;
mod game;
mod player;
mod single_matchup;
mod accumulated_matchup;
mod accumulated_vs_player_matchup;
mod accumulated_vs_team;
mod accumulated_vs_team_seasonal;
mod month;
mod weekday;
mod home_away;
mod win_loss;

pub use career::*;
pub use map::*;
pub use season::*;
pub use position_and_season::*;
pub use team::*;
pub use game::*;
pub use player::*;
pub use single_matchup::*;
pub use accumulated_matchup::*;
pub use accumulated_vs_player_matchup::*;
pub use accumulated_vs_team::*;
pub use accumulated_vs_team_seasonal::*;
pub use month::*;
pub use weekday::*;
pub use home_away::*;
pub use win_loss::*;

use std::convert::Infallible;
use crate::stats::{SingletonSplitStat, Stat};

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
