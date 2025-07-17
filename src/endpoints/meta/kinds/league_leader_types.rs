use crate::endpoints::meta::MetaKind;
use derive_more::From;
use serde::Deserialize;
use std::ops::{Deref, DerefMut};
use strum::EnumTryAs;

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
pub struct IdentifiableLeagueLeaderType {
	#[serde(rename = "displayName")]
	pub name: String,
}

#[derive(Debug, Deserialize, Eq, Clone, From, EnumTryAs)]
#[serde(untagged)]
pub enum LeagueLeaderType {
	Identifiable(IdentifiableLeagueLeaderType),
}

impl PartialEq for LeagueLeaderType {
	fn eq(&self, other: &Self) -> bool {
		self.name == other.name
	}
}

impl Deref for LeagueLeaderType {
	type Target = IdentifiableLeagueLeaderType;

	fn deref(&self) -> &Self::Target {
		match self {
			Self::Identifiable(inner) => inner,
		}
	}
}

impl DerefMut for LeagueLeaderType {
	fn deref_mut(&mut self) -> &mut Self::Target {
		match self {
			Self::Identifiable(inner) => inner,
		}
	}
}

impl MetaKind for LeagueLeaderType {
	const ENDPOINT_NAME: &'static str = "leagueLeaderTypes";
}

#[cfg(test)]
mod tests {
	use crate::endpoints::StatsAPIUrl;
	use crate::endpoints::meta::MetaEndpointUrl;

	#[tokio::test]
	async fn parse_meta() {
		let _response = MetaEndpointUrl::<super::LeagueLeaderType>::new().get().await.unwrap();
	}
}
