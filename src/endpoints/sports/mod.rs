pub mod players;

use crate::types::Copyright;
use derive_more::{Deref, DerefMut, From};
use serde::Deserialize;
use std::ops::{Deref, DerefMut};

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SportsResponse {
	pub copyright: Copyright,
	pub sports: Vec<Sport>,
}

pub use id::*;

mod id {
	use crate::endpoints::Url;
	use crate::endpoints::sports::SportsResponse;
	use crate::gen_params;
	use derive_more::{Deref, Display};
	use serde::Deserialize;
	use std::fmt::{Display, Formatter};

	#[repr(transparent)]
	#[derive(Debug, Deserialize, Deref, Display, PartialEq, Eq, Copy, Clone)]
	pub struct SportId(pub(super) u32);

	impl SportId {
		#[must_use]
		pub const fn new(id: u32) -> Self {
			Self(id)
		}
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
			write!(f, "http://statsapi.mlb.com/api/v1/sports{params}", params = gen_params! { "sportId"?: self.id })
		}
	}

	impl Url<SportsResponse> for SportsEndpointUrl {}
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

#[derive(Debug, Deserialize, Eq, Clone, From)]
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
	use crate::endpoints::Url;

	#[tokio::test]
	async fn check_updated() {
		let _result = SportsEndpointUrl { id: None }.get().await.unwrap();
	}
}
