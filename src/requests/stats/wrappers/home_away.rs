use serde::Deserialize;
use thiserror::Error;
use crate::{HomeAway, TeamSide};
use crate::stats::{RawStat, Stat};
use crate::stats::wrappers::season::WithSeason;

#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(bound = "T: RawStat")]
#[doc(hidden)]
pub struct __HomeOrAwayStruct<T: RawStat> {
	#[serde(flatten)]
	stats: WithSeason<T>,
	#[serde(rename = "isHome", deserialize_with = "crate::deserialize_team_side_from_is_home")]
	team_side: TeamSide,
}

pub type WithHomeAndAway<T> = HomeAway<WithSeason<T>>;

#[derive(Debug, Error)]
pub enum HomeAndAwayFromSplitWrappedVariantError {
	#[error("Did not find exactly two splits")]
	NotLen2,
	#[error("Found multiple home splits")]
	DuplicateHome,
	#[error("Found multiple away splits")]
	DuplicateAway,
}

impl<T: RawStat + Default> Stat for WithHomeAndAway<T> {
	type Split = __HomeOrAwayStruct<T>;
	type TryFromSplitError = HomeAndAwayFromSplitWrappedVariantError;

	fn from_splits(splits: impl Iterator<Item=Self::Split>) -> Result<Self, Self::TryFromSplitError>
	where
		Self: Sized
	{
		use HomeAndAwayFromSplitWrappedVariantError as Error;

		let [a, b] = <Vec<Self::Split> as TryInto<[Self::Split; 2]>>::try_into(splits.collect()).map_err(|_| Error::NotLen2)?;
		if a.team_side == b.team_side {
			return Err(if a.team_side.is_home() { Error::DuplicateHome } else { Error::DuplicateAway })
		}

		let mut split = Self {
			home: a.stats,
			away: b.stats,
		};

		if a.team_side.is_away() {
			split = split.swap();
		}

		Ok(split)
	}
}
