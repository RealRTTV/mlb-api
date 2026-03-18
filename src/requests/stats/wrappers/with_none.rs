use derive_more::{Deref, DerefMut};
use serde::Deserialize;
use crate::stats::{RawStat, SingletonSplitStat};

#[derive(Debug, Deserialize, PartialEq, Eq, Clone, Deref, DerefMut)]
#[serde(from = "__WithNoneRaw<T>", bound = "T: RawStat")]
pub struct WithNone<T: RawStat> {
    pub stats: T,
}

#[derive(Deserialize)]
#[serde(untagged, bound = "T: RawStat")]
enum __WithNoneRaw<T: RawStat> {
    Wrapped {
        #[serde(rename = "stat")]
        stats: T
    },
    Inline(T),
}

impl<T: RawStat> From<__WithNoneRaw<T>> for WithNone<T> {
    fn from((__WithNoneRaw::Wrapped { stats } | __WithNoneRaw::Inline(stats)): __WithNoneRaw<T>) -> Self {
        Self { stats }
    }
}

impl<T: RawStat> Default for WithNone<T> {
    fn default() -> Self {
        Self {
            stats: T::default(),
        }
    }
}

impl<T: RawStat> SingletonSplitStat for WithNone<T> {

}
