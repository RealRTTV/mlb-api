use crate::endpoints::meta::MetaKind;
use derive_more::{Deref, DerefMut, Display, From};
use serde::Deserialize;
use std::ops::{Deref, DerefMut};
use strum::EnumTryAs;

#[repr(transparent)]
#[derive(Debug, Deserialize, Deref, Display, PartialEq, Eq, Clone)]
pub struct RosterTypeId(String);

impl RosterTypeId {
	#[must_use]
	pub const fn new(id: String) -> Self {
		Self(id)
	}
}

impl Default for RosterTypeId {
	fn default() -> Self {
		Self::new("active".to_owned())
	}
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
pub struct IdentifiableRosterType {
	#[serde(rename = "lookupName")]
	pub id: RosterTypeId,
}

#[derive(Debug, Deserialize, Deref, DerefMut, PartialEq, Eq, Clone)]
pub struct HydratedRosterType {
	pub parameter: String,
	pub description: String,

	#[deref]
	#[deref_mut]
	#[serde(flatten)]
	inner: IdentifiableRosterType,
}

#[derive(Debug, Deserialize, Eq, Clone, From, EnumTryAs)]
#[serde(untagged)]
pub enum RosterType {
	Hydrated(HydratedRosterType),
	Identifiable(IdentifiableRosterType),
}

impl PartialEq for RosterType {
	fn eq(&self, other: &Self) -> bool {
		self.id == other.id
	}
}

impl Deref for RosterType {
	type Target = IdentifiableRosterType;

	fn deref(&self) -> &Self::Target {
		match self {
			Self::Hydrated(inner) => inner,
			Self::Identifiable(inner) => inner,
		}
	}
}

impl DerefMut for RosterType {
	fn deref_mut(&mut self) -> &mut Self::Target {
		match self {
			Self::Hydrated(inner) => inner,
			Self::Identifiable(inner) => inner,
		}
	}
}

impl MetaKind for RosterType {
	const ENDPOINT_NAME: &'static str = "rosterTypes";
}

#[cfg(test)]
mod tests {
	use crate::endpoints::StatsAPIUrl;
	use crate::endpoints::meta::MetaEndpointUrl;

	#[tokio::test]
	async fn parse_meta() {
		let _response = MetaEndpointUrl::<super::RosterType>::new().get().await.unwrap();
	}
}
