use crate::endpoints::meta::MetaKind;
use derive_more::{Deref, DerefMut, From};
use serde::Deserialize;
use std::ops::{Deref, DerefMut};

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
pub struct IdentifiableSituationCode {
	pub code: String,
}

#[derive(Debug, Deserialize, Deref, DerefMut, PartialEq, Eq, Clone)]
pub struct HydratedSituationCode {
	#[serde(rename = "navigationMenu")]
	pub navigation_menu_kind: String,
	pub description: String,
	#[serde(rename = "team")]
	pub is_team_active: bool,
	#[serde(rename = "batting")]
	pub is_batting_active: bool,
	#[serde(rename = "fielding")]
	pub is_fielding_active: bool,
	#[serde(rename = "pitching")]
	pub is_pitching_active: bool,

	#[deref]
	#[deref_mut]
	#[serde(flatten)]
	inner: IdentifiableSituationCode,
}

#[derive(Debug, Deserialize, Eq, Clone, From)]
#[serde(untagged)]
pub enum SituationCode {
	Hydrated(HydratedSituationCode),
	Identifiable(IdentifiableSituationCode),
}

impl PartialEq for SituationCode {
	fn eq(&self, other: &Self) -> bool {
		self.code == other.code
	}
}

impl Deref for SituationCode {
	type Target = IdentifiableSituationCode;

	fn deref(&self) -> &Self::Target {
		match self {
			Self::Hydrated(inner) => inner,
			Self::Identifiable(inner) => inner,
		}
	}
}

impl DerefMut for SituationCode {
	fn deref_mut(&mut self) -> &mut Self::Target {
		match self {
			Self::Hydrated(inner) => inner,
			Self::Identifiable(inner) => inner,
		}
	}
}

impl MetaKind for SituationCode {
	const ENDPOINT_NAME: &'static str = "situationCodes";
}

#[cfg(test)]
mod tests {
	use crate::endpoints::meta::MetaEndpointUrl;
	use crate::endpoints::StatsAPIUrl;

	#[tokio::test]
	async fn parse_meta() {
		let _response = MetaEndpointUrl::<super::SituationCode>::new().get().await.unwrap();
	}
}
