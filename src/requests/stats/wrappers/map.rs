use crate::stats::{RawStat, SingletonSplitStat, Stat};
use derive_more::{Deref, DerefMut};
use fxhash::FxHashMap;
use std::collections::hash_map::Entry;
use std::fmt::{Debug, Formatter};
use std::hash::Hash;
use thiserror::Error;

#[derive(Debug, PartialEq, Eq, Clone, Deref, DerefMut)]
pub struct Map<T, A: Hash + Clone + Debug + Eq> where T: AsRef<A> {
	inner: FxHashMap<A, T>,
}

#[derive(Debug, Error)]
pub enum MapFromSplitWrappedVariantError<A: Hash + Clone + Debug + Eq> {
	#[error("Duplicate entry for key {key:?} found")]
	DuplicateEntry { key: A },
}

impl<A: Hash + Clone + Debug + Eq, T: AsRef<A>> Default for Map<T, A> {
	fn default() -> Self {
		Self { inner: FxHashMap::default() }
	}
}

impl<A: Hash + Clone + Debug + Eq, T: AsRef<A> + SingletonSplitStat> Stat for Map<T, A> {
	type Split = T;
	type TryFromSplitError = MapFromSplitWrappedVariantError<A>;

	fn from_splits(splits: impl Iterator<Item=T>) -> Result<Self, Self::TryFromSplitError>
	where
		Self: Sized
	{
		let mut this = Self::default();
		for split in splits {
			let id: &A = T::as_ref(&split);
			let id = id.clone();
			match this.inner.entry(id.clone()) {
				Entry::Occupied(_) => return Err(Self::TryFromSplitError::DuplicateEntry { key: id }),
				Entry::Vacant(slot) => {
					slot.insert(split);
				}
			}
		}
		Ok(this)
	}
}

#[derive(Debug, PartialEq, Eq, Clone, Deref, DerefMut)]
pub struct Map2D<T: SingletonSplitStat, A: Hash + Clone + Eq + Debug, B: Hash + Clone + Eq + Debug> where T: AsRef<A> + AsRef<B> {
	inner: FxHashMap<A, FxHashMap<B, T>>,
}

impl<A: Hash + Clone + Eq + Debug, B: Hash + Clone + Eq + Debug, T: AsRef<A> + AsRef<B> + SingletonSplitStat> Default for Map2D<T, A, B> {
	fn default() -> Self {
		Self { inner: FxHashMap::default() }
	}
}

impl<A: Hash + Clone + Eq + Debug, B: Hash + Clone + Eq + Debug, T: AsRef<A> + AsRef<B> + SingletonSplitStat> Stat for Map2D<T, A, B> {
	type Split = T;
	type TryFromSplitError = MapFromSplitWrappedVariantError<B>;

	fn from_splits(splits: impl Iterator<Item=Self::Split>) -> Result<Self, Self::TryFromSplitError>
	where
		Self: Sized
	{
		let mut this = Self::default();
		for split in splits {
			let id: A = <T as AsRef<A>>::as_ref(&split).clone();
			let inner_id: B = <T as AsRef<B>>::as_ref(&split).clone();
			match this.inner.entry(id).or_insert_with(FxHashMap::default).entry(inner_id.clone()) {
				Entry::Occupied(_) => return Err(Self::TryFromSplitError::DuplicateEntry { key: inner_id }),
				Entry::Vacant(slot) => {
					slot.insert(split);
				}
			}
		}
		Ok(this)
	}
}
