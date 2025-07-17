pub mod players;

use crate::endpoints::StatsAPIUrl;
use crate::gen_params;
use crate::types::Copyright;
use derive_more::{Deref, DerefMut, Display, From};
use serde::Deserialize;
use std::fmt::{Display, Formatter};
use std::ops::{Deref, DerefMut};
use strum::EnumTryAs;

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SportsResponse {
	pub copyright: Copyright,
	pub sports: Vec<Sport>,
}

#[repr(transparent)]
#[derive(Debug, Deserialize, Deref, Display, PartialEq, Eq, Copy, Clone)]
pub struct SportId(pub(super) u32);

impl SportId {
	#[must_use]
	pub const fn new(id: u32) -> Self {
		Self(id)
	}

	pub const MLB: Self = Self::new(1);
}

impl Default for SportId {
	fn default() -> Self {
		Self(1)
	}
}

pub struct SportsEndpointUrl {
	pub id: Option<SportId>,
}

impl Display for SportsEndpointUrl {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "http://statsapi.mlb.com/api/v1/sports{}", gen_params! { "sportId"?: self.id })
	}
}

impl StatsAPIUrl for SportsEndpointUrl {
	type Response = SportsResponse;
}

#[derive(Debug, Deserialize, PartialEq, Eq, Copy, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IdentifiableSport {
	pub id: SportId,
}

#[derive(Debug, Deserialize, Deref, DerefMut, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NamedSport {
	pub name: String,

	#[deref]
	#[deref_mut]
	#[serde(flatten)]
	pub(super) inner: IdentifiableSport,
}

#[derive(Debug, Deserialize, Deref, DerefMut, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HydratedSport {
	pub code: String,
	pub abbreviation: String,
	#[serde(rename = "activeStatus")]
	pub active: bool,

	#[deref]
	#[deref_mut]
	#[serde(flatten)]
	pub(super) inner: NamedSport,
}

#[derive(Debug, Deserialize, Eq, Clone, From, EnumTryAs)]
#[serde(untagged)]
pub enum Sport {
	Hydrated(HydratedSport),
	Named(NamedSport),
	Identifiable(IdentifiableSport),
}

impl PartialEq for Sport {
	fn eq(&self, other: &Self) -> bool {
		self.id == other.id
	}
}

impl Deref for Sport {
	type Target = IdentifiableSport;

	fn deref(&self) -> &Self::Target {
		match self {
			Self::Hydrated(inner) => inner,
			Self::Named(inner) => inner,
			Self::Identifiable(inner) => inner,
		}
	}
}

impl DerefMut for Sport {
	fn deref_mut(&mut self) -> &mut Self::Target {
		match self {
			Self::Hydrated(inner) => inner,
			Self::Named(inner) => inner,
			Self::Identifiable(inner) => inner,
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::endpoints::StatsAPIUrl;

	#[tokio::test]
	async fn parse_all_sports() {
		let _result = SportsEndpointUrl { id: None }.get().await.unwrap();
	}
}
