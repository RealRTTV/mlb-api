use derive_more::{Deref, DerefMut};
use serde::Deserialize;
use crate::stats::{RawStat, SingletonSplitStat};

#[derive(Debug, Deserialize, PartialEq, Eq, Clone, Deref, DerefMut)]
#[serde(bound = "T: RawStat")]
pub struct WithNone<T: RawStat> {
    #[serde(rename = "stat")]
    pub stats: T,
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
