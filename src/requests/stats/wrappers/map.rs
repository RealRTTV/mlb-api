use crate::stats::{SingletonSplitStat, Stat};
use derive_more::{Deref, DerefMut};
use fxhash::FxHashMap;
use std::collections::hash_map::Entry;
use std::fmt::{Debug, Formatter};
use std::hash::Hash;
use thiserror::Error;

#[derive(Deref, DerefMut)]
pub struct Map<T: SingletonSplitStat, A: MapKey<T>> {
	inner: FxHashMap<A::Key, T>,
}

impl<T: SingletonSplitStat, A: MapKey<T>> Debug for Map<T, A> {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		self.inner.fmt(f)
	}
}

impl<T: SingletonSplitStat, A: MapKey<T>> PartialEq for Map<T, A> {
	fn eq(&self, other: &Self) -> bool {
		self.inner == other.inner
	}
}

impl<T: SingletonSplitStat, A: MapKey<T>> Eq for Map<T, A> {}

impl<T: SingletonSplitStat, A: MapKey<T>> Clone for Map<T, A> {
	fn clone(&self) -> Self {
		Self {
			inner: self.inner.clone(),
		}
	}
}

#[derive(Debug, Error)]
pub enum MapFromSplitWrappedVariantError<T, A: MapKey<T>> {
	#[error("Duplicate entry for key {key:?} found")]
	DuplicateEntry { key: A::Key },
}

impl<T: SingletonSplitStat, A: MapKey<T>> Default for Map<T, A> {
	fn default() -> Self {
		Self { inner: FxHashMap::default() }
	}
}

impl<T: SingletonSplitStat, A: MapKey<T>> Stat for Map<T, A> {
	type Split = T;
	type TryFromSplitError = MapFromSplitWrappedVariantError<T, A>;

	fn from_splits(splits: impl Iterator<Item=T>) -> Result<Self, Self::TryFromSplitError>
	where
		Self: Sized
	{
		let mut this = Self::default();
		for split in splits {
			let id: A::Key = A::get_key(&split);
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

#[derive(Deref, DerefMut)]
pub struct Map2D<T: SingletonSplitStat, A: MapKey<T>, B: MapKey<T>> {
	inner: FxHashMap<A::Key, FxHashMap<B::Key, T>>,
}

impl<T: SingletonSplitStat, A: MapKey<T>, B: MapKey<T>> Clone for Map2D<T, A, B> {
	fn clone(&self) -> Self {
		Self {
			inner: self.inner.clone(),
		}
	}
}

impl<T: SingletonSplitStat, A: MapKey<T>, B: MapKey<T>> Debug for Map2D<T, A, B> {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		self.inner.fmt(f)
	}
}

impl<T: SingletonSplitStat, A: MapKey<T>, B: MapKey<T>> PartialEq for Map2D<T, A, B> {
	fn eq(&self, other: &Self) -> bool {
		self.inner == other.inner
	}
}

impl<T: SingletonSplitStat, A: MapKey<T>, B: MapKey<T>> Eq for Map2D<T, A, B> {}

impl<T: SingletonSplitStat, A: MapKey<T>, B: MapKey<T>> Default for Map2D<T, A, B> {
	fn default() -> Self {
		Self { inner: FxHashMap::default() }
	}
}

impl<T: SingletonSplitStat, A: MapKey<T>, B: MapKey<T>> Stat for Map2D<T, A, B> {
	type Split = T;
	type TryFromSplitError = MapFromSplitWrappedVariantError<T, B>;

	fn from_splits(splits: impl Iterator<Item=Self::Split>) -> Result<Self, Self::TryFromSplitError>
	where
		Self: Sized
	{
		let mut this = Self::default();
		for split in splits {
			let id: A::Key = A::get_key(&split);
			let inner_id: B::Key = B::get_key(&split);
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

pub trait MapKey<T> {
	type Key: Hash + Clone + Debug + Eq;

	fn get_key(this: &T) -> Self::Key;
}
