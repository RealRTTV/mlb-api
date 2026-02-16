use serde::Deserialize;
use serde_json::Value;
use smallvec::SmallVec;
use thiserror::Error;
use crate::stat_groups::StatGroup;
use crate::stat_types::StatType;
use crate::stats::Stat;

#[derive(Deserialize)]
#[doc(hidden)]
struct __RawStats {
	#[serde(alias = "stat")]
	stats: Vec<__RawStatEntry>,
}

#[derive(Deserialize)]
#[serde(untagged)]
#[doc(hidden)]
enum __RawStatEntry {
	Depth0(__Depth0StatEntry),
	Depth1(__Depth1StatEntry),
}

pub type __Depth0StatEntry = __ParsedStatEntry;

#[derive(Deserialize)]
#[doc(hidden)]
struct __Depth1StatEntry {
	splits: Vec<__InlineStatEntry>
}

#[derive(Deserialize)]
#[doc(hidden)]
struct __InlineStatEntry {
	#[serde(rename = "type")]
	stat_type: StatType,
	#[serde(rename = "group")]
	stat_group: StatGroup,
	stat: Value,
}

impl From<__InlineStatEntry> for __ParsedStatEntry {
	fn from(value: __InlineStatEntry) -> Self {
		Self {
			stat_type: value.stat_type,
			stat_group: value.stat_group,
			splits: SmallVec::from_buf::<1>([value.stat]),
		}
	}
}

impl From<__Depth1StatEntry> for Vec<__Depth0StatEntry> {
	fn from(value: __Depth1StatEntry) -> Self {
		value.splits.into_iter().map(Into::into).collect()
	}
}

impl From<__RawStatEntry> for Vec<__ParsedStatEntry> {
	fn from(value: __RawStatEntry) -> Self {
		match value {
			__RawStatEntry::Depth0(x) => vec![x],
			__RawStatEntry::Depth1(x) => x.into(),
		}
	}
}

impl From<__RawStats> for __ParsedStats {
	fn from(value: __RawStats) -> Self {
		let mut entries = Vec::with_capacity(value.stats.len());
		for entry in value.stats {
			match entry {
				__RawStatEntry::Depth0(entry) => entries.push(entry),
				__RawStatEntry::Depth1(entry) => {
					entries.reserve(entry.splits.len());
					for entry in entry.splits {
						entries.push(__ParsedStatEntry::from(entry));
					}
				},
			}
		}
		Self {
			entries
		}
	}
}

#[doc(hidden)]
#[derive(Deserialize, Debug)]
#[serde(from = "__RawStats")]
pub struct __ParsedStats {
	entries: Vec<__ParsedStatEntry>
}

#[doc(hidden)]
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct __ParsedStatEntry {
	#[serde(rename = "type")]
	stat_type: StatType,
	#[serde(rename = "group")]
	stat_group: StatGroup,
	splits: SmallVec<Value, 1>,
}

#[doc(hidden)]
#[derive(Debug, Error)]
pub enum MakeStatSplitsError<S: Stat> {
	#[error("Failed to deserialize json into split type ({name}): {0}", name = core::any::type_name::<S>())]
	FailedPartialDeserialize(serde_json::Error),
	// FailedPartialDeserialize(serde_path_to_error::Error<serde_json::Error>),
	#[error("Failed to deserialize splits into greater split type ({name}): {0}", name = core::any::type_name::<S>())]
	FailedFullDeserialize(S::TryFromSplitError),
}

#[doc(hidden)]
pub fn make_stat_split<S: Stat>(stats: &mut __ParsedStats, target_stat_type_str: &'static str, target_stat_group: StatGroup) -> Result<S, MakeStatSplitsError<S>> {
	if let Some(idx) = stats.entries.iter().position(|entry| entry.stat_type.as_str().eq_ignore_ascii_case(target_stat_type_str) && entry.stat_group == target_stat_group) {
		let entry = stats.entries.remove(idx);
		let partially_deserialized = entry.splits
			.into_iter()
			.map(|split| {
				<<S as Stat>::Split as Deserialize>::deserialize(split)
				// serde_path_to_error::deserialize::<_, <S as Stat>::Split>(split)
			})
			.collect::<Result<Vec<S::Split>, _>>()
			.map_err(MakeStatSplitsError::FailedPartialDeserialize)?;
		let partially_deserialized_is_empty = partially_deserialized.is_empty();
		match <S as Stat>::from_splits(partially_deserialized.into_iter()) {
			Ok(s) => Ok(S::default()),
			Err(_) if partially_deserialized_is_empty => Ok(S::default()),
			Err(e) => Err(MakeStatSplitsError::FailedFullDeserialize(e)),
		}
	} else {
		Ok(S::default())
	}
}
