use crate::endpoints::league::League;
use crate::endpoints::meta::MetaKind;
use crate::endpoints::sports::Sport;
use derive_more::{Deref, DerefMut, From};
use serde::Deserialize;
use std::ops::Deref;

#[derive(Debug, Deserialize, Deref, DerefMut, PartialEq, Eq, Clone)]
pub struct HydratedAward {
	pub name: String,
	pub description: Option<String>,
	pub sport: Option<Sport>,
	pub league: Option<League>,
	pub notes: Option<String>,

	#[deref]
	#[deref_mut]
	#[serde(flatten)]
	inner: IdentifiableAward,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
pub struct IdentifiableAward {
	pub id: String,
}

#[derive(Debug, Deserialize, Eq, Clone, From)]
#[serde(untagged)]
pub enum Award {
	Hydrated(HydratedAward),
	Identifiable(IdentifiableAward),
}

impl PartialEq for Award {
	fn eq(&self, other: &Self) -> bool {
		self.id == other.id
	}
}

impl Deref for Award {
	type Target = IdentifiableAward;

	fn deref(&self) -> &Self::Target {
		match self {
			Self::Hydrated(inner) => inner,
			Self::Identifiable(inner) => inner,
		}
	}
}

impl MetaKind for Award {
	const ENDPOINT_NAME: &'static str = "awards";
}
