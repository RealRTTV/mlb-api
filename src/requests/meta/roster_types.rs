use derive_more::Display;
use serde::Deserialize;
use std::ops::Deref;

#[derive(Debug, Deserialize, Display, PartialEq, Eq, Copy, Clone, Hash)]
#[serde(try_from = "__RosterTypeStruct")]
pub enum RosterType {
	#[display("40Man")]
	FortyMan,
	#[display("fullSeason")]
	FullSeason,
	#[display("fullRoster")]
	FullRoster,
	#[display("nonRosterInvitees")]
	NonRosterInvitees,
	#[display("active")]
	Active,
	#[display("allTime")]
	AllTime,
	#[display("depthChart")]
	DepthChart,
	#[display("gameday")]
	GameDay,
	#[display("coach")]
	Coach,
}

#[derive(Deserialize)]
#[doc(hidden)]
#[serde(untagged)]
enum __RosterTypeStruct {
	Wrapped {
		#[serde(rename = "parameter")]
		id: String,
	},
	Inline(String),
}

impl Deref for __RosterTypeStruct {
	type Target = str;

	fn deref(&self) -> &Self::Target {
		let (Self::Wrapped { id } | Self::Inline(id)) = self;
		id
	}
}

impl TryFrom<__RosterTypeStruct> for RosterType {
	type Error = String;

	fn try_from(value: __RosterTypeStruct) -> Result<Self, Self::Error> {
		Ok(match &*value {
			"40Man" => Self::FortyMan,
			"fullSeason" => Self::FullSeason,
			"fullRoster" => Self::FullRoster,
			"nonRosterInvitees" => Self::NonRosterInvitees,
			"active" => Self::Active,
			"allTime" => Self::AllTime,
			"depthChart" => Self::DepthChart,
			"gameday" => Self::GameDay,
			"coach" => Self::Coach,
			str => return Err(str.to_owned()),
		})
	}
}

meta_kind_impl!("rosterTypes" => RosterType);
static_request_entry_cache_impl!(RosterType);
test_impl!(RosterType);
