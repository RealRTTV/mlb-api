use derive_more::{Display, FromStr};
use serde::Deserialize;

/// Hitting, Pitching, etc.
#[derive(Debug, Deserialize, PartialEq, Eq, Copy, Clone, FromStr, Hash, Display)]
#[serde(try_from = "__StatGroupMaybeInline")]
pub enum StatGroup {
	Hitting,
	Pitching,
	Fielding,
	Catching,
	Running,
	Game,
	Team,
	Streak,
}

#[derive(Deserialize)]
#[serde(untagged)]
#[doc(hidden)]
enum __StatGroupMaybeInline {
	Wrapped {
		#[serde(rename = "displayName")]
		display_name: String,
	},
	Inline(String),
}

impl __StatGroupMaybeInline {
	#[must_use]
	pub fn into_string(self) -> String {
		match self {
			Self::Wrapped { display_name } => display_name,
			Self::Inline(name) => name,
		}
	}
}

impl TryFrom<__StatGroupMaybeInline> for StatGroup {
	type Error = derive_more::FromStrError;

	fn try_from(value: __StatGroupMaybeInline) -> Result<Self, Self::Error> {
		value.into_string().parse::<Self>()
	}
}

meta_kind_impl!("statGroups" => StatGroup);
static_request_entry_cache_impl!(StatGroup);
test_impl!(StatGroup);
