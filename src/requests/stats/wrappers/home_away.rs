use serde::Deserialize;
use thiserror::Error;
use crate::stats::{RawStat, Stat};
use crate::stats::wrappers::season::WithSeason;

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(bound = "T: RawStat")]
#[doc(hidden)]
pub struct __HomeOrAwayStruct<T: RawStat> {
	#[serde(flatten)]
	stats: WithSeason<T>,
	is_home: bool,
}

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct WithHomeAndAway<T: RawStat> {
	pub home: WithSeason<T>,
	pub away: WithSeason<T>,
}

#[derive(Debug, Error)]
pub enum HomeAndAwayFromSplitWrappedVariantError {
	#[error("Did not find exactly two splits")]
	NotLen2,
	#[error("Found multiple home splits")]
	DuplicateHome,
	#[error("Found multiple away splits")]
	DuplicateAway,
}

impl<T: RawStat> Stat for WithHomeAndAway<T> {
	type Split = __HomeOrAwayStruct<T>;
	type TryFromSplitError = HomeAndAwayFromSplitWrappedVariantError;

	fn from_splits(splits: impl Iterator<Item=Self::Split>) -> Result<Self, Self::TryFromSplitError>
	where
		Self: Sized
	{
		use HomeAndAwayFromSplitWrappedVariantError as Error;

		let [a, b] = <Vec<Self::Split> as TryInto<[Self::Split; 2]>>::try_into(splits.collect()).map_err(|_| Error::NotLen2)?;
		if a.is_home == b.is_home {
			return Err(if a.is_home { Error::DuplicateHome } else { Error::DuplicateAway })
		}

		if a.is_home {
			Ok(Self {
				home: a.stats,
				away: b.stats,
			})
		} else {
			Ok(Self {
				home: b.stats,
				away: a.stats,
			})
		}
	}
}
