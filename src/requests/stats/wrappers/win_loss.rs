use serde::Deserialize;
use thiserror::Error;
use crate::stats::{RawStat, Stat};
use crate::stats::wrappers::season::WithSeason;

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(bound = "T: RawStat")]
#[doc(hidden)]
pub struct __WinOrLossStruct<T: RawStat> {
	#[serde(flatten)]
	stats: WithSeason<T>,
	is_win: bool,
}

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct WithWinLoss<T: RawStat> {
	pub win: WithSeason<T>,
	pub loss: WithSeason<T>,
}

#[derive(Debug, Error)]
pub enum WinLossFromSplitWrappedVariantError {
	#[error("Did not find exactly two splits")]
	NotLen2,
	#[error("Found multiple win splits")]
	DuplicateWin,
	#[error("Found multiple loss splits")]
	DuplicateLoss,
}

impl<T: RawStat> Stat for WithWinLoss<T> {
	type Split = __WinOrLossStruct<T>;
	type TryFromSplitError = WinLossFromSplitWrappedVariantError;

	fn from_splits(splits: impl Iterator<Item=Self::Split>) -> Result<Self, Self::TryFromSplitError>
	where
		Self: Sized
	{
		use WinLossFromSplitWrappedVariantError as Error;

		let [a, b] = <Vec<Self::Split> as TryInto<[Self::Split; 2]>>::try_into(splits.collect()).map_err(|_| Error::NotLen2)?;
		if a.is_win == b.is_win {
			return Err(if a.is_win { Error::DuplicateWin } else { Error::DuplicateLoss })
		}

		if a.is_win {
			Ok(Self {
				win: a.stats,
				loss: b.stats,
			})
		} else {
			Ok(Self {
				win: b.stats,
				loss: a.stats,
			})
		}
	}
}
