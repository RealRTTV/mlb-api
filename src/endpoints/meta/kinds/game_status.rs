use crate::endpoints::meta::MetaKind;
use derive_more::{Deref, DerefMut, From};
use serde::Deserialize;
use std::ops::{Deref, DerefMut};

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IdentifiableGameStatus {
	pub detailed_state: String,
}

#[derive(Debug, Deserialize, Deref, DerefMut, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HydratedGameStatus {
	pub abstract_game_state: String,
	pub coded_game_state: String,
	pub status_code: String,
	pub reason: Option<String>,
	pub abstract_game_code: String,

	#[deref]
	#[deref_mut]
	#[serde(flatten)]
	inner: IdentifiableGameStatus,
}

#[derive(Debug, Deserialize, Eq, Clone, From)]
#[serde(untagged)]
pub enum GameStatus {
	Hydrated(HydratedGameStatus),
	Identifiable(IdentifiableGameStatus),
}

impl Deref for GameStatus {
	type Target = IdentifiableGameStatus;

	fn deref(&self) -> &Self::Target {
		match self {
			Self::Hydrated(inner) => inner,
			Self::Identifiable(inner) => inner,
		}
	}
}

impl DerefMut for GameStatus {
	fn deref_mut(&mut self) -> &mut Self::Target {
		match self {
			Self::Hydrated(inner) => inner,
			Self::Identifiable(inner) => inner,
		}
	}
}

impl PartialEq for GameStatus {
	fn eq(&self, other: &Self) -> bool {
		self.detailed_state == other.detailed_state
	}
}

impl MetaKind for GameStatus {
	const ENDPOINT_NAME: &'static str = "gameStatus";
}

#[cfg(test)]
mod tests {
	use crate::endpoints::meta::MetaEndpointUrl;
	use crate::endpoints::StatsAPIUrl;

	#[tokio::test]
	async fn parse_meta() {
		let _response = MetaEndpointUrl::<super::GameStatus>::new().get().await.unwrap();
	}
}
