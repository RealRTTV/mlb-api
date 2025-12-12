use crate::cache::{HydratedCacheTable, RequestEntryCache};
use crate::meta::{MetaKind, MetaRequest};
use crate::StatsAPIRequestUrl;
use crate::{rwlock_const_new, RwLock};
use derive_more::Display;
use serde::Deserialize;
use std::ops::Deref;

#[derive(Debug, Deserialize, Display, PartialEq, Eq, Copy, Clone, Hash)]
#[serde(try_from = "__RosterTypeStruct")]
pub enum RosterType {
	#[display("40Man")]
	FourtyMan,
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
			"40Man" => Self::FourtyMan,
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

impl MetaKind for RosterType {
	const ENDPOINT_NAME: &'static str = "rosterTypes";
}

static CACHE: RwLock<HydratedCacheTable<RosterType>> = rwlock_const_new(HydratedCacheTable::new());

impl RequestEntryCache for RosterType {
	type HydratedVariant = Self;
	type Identifier = Self;
	type URL = MetaRequest<Self>;

	fn into_hydrated_variant(self) -> Option<Self::HydratedVariant> {
		Some(self)
	}

	fn id(&self) -> &Self::Identifier {
		self
	}

	fn url_for_id(_id: &Self::Identifier) -> Self::URL {
		MetaRequest::new()
	}

	fn get_entries(response: <Self::URL as StatsAPIRequestUrl>::Response) -> impl IntoIterator<Item=Self>
	where
		Self: Sized
	{
		response.entries
	}

	fn get_hydrated_cache_table() -> &'static RwLock<HydratedCacheTable<Self>>
	where
		Self: Sized
	{
		&CACHE
	}
}

#[cfg(test)]
mod tests {
	use crate::meta::MetaRequest;
	use crate::StatsAPIRequestUrl;

	#[tokio::test]
	async fn parse_meta() {
		let _response = MetaRequest::<super::RosterType>::new().get().await.unwrap();
	}
}
