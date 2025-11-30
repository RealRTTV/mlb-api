use crate::endpoints::meta::{MetaEndpointUrl, MetaKind};
use derive_more::{Deref, DerefMut, Display, From};
use serde::Deserialize;
use std::ops::{Deref, DerefMut};
use strum::EnumTryAs;
use crate::cache::{EndpointEntryCache, HydratedCacheTable};
use crate::{rwlock_const_new, RwLock};
use crate::endpoints::StatsAPIUrl;

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IdentifiableGameStatus {
	#[serde(rename = "detailedState")] pub id: GameStatusId,
}

#[repr(transparent)]
#[derive(Debug, Deserialize, Deref, Display, PartialEq, Eq, Clone, Hash)]
pub struct GameStatusId(String);

/// Detailed game status (use [`AbstractGameCode`] for simpler responses)
#[derive(Debug, Deserialize, PartialEq, Eq, Copy, Clone)]
pub enum CodedGameState {
	/// Game has not begun, but is scheduled to occur
	#[serde(rename = "S")]
	Scheduled,

	/// Game is currently undergoing pregame activities (such as warming up, or a slight delay before start)
	#[serde(rename = "P")]
	PreGame,

	/// Game is underway.
	#[serde(rename = "I")]
	InProgress,

	/// Manager is submitting a challenge
	#[serde(rename = "M")]
	ManagerChallenge,

	/// Umpires are reviewing a play
	#[serde(rename = "N")]
	UmpireReview,

	/// Game is postponed; has not begun but moved to a later date -- typically double-header.
	#[serde(rename = "D")]
	Postponed,

	/// Game is canceled and never began. Removed from total # of games played, no rescheduling.
	#[serde(rename = "C")]
	Cancelled,

	/// Game was finished.
	#[serde(rename = "F", alias = "O")] // unaware of the difference
	Finished,

	/// Game was suspended, will be played on a later date.
	#[serde(rename = "T", alias = "U")] // unaware of the difference
	Suspended,

	/// Game was forfeited.
	#[serde(rename = "Q", alias = "R")] // unaware of the difference
	Forfeit,

	/// Game is being written?? (Likely means that the official scorer is in the process of doing the finishing touches)
	#[serde(rename = "W")]
	Writing,

	/// Game state is unknown (typically assume the game was completed)
	#[serde(rename = "X")]
	Unknown,
}

/// Basic game status code, describes whether the game is in the past (finished), present (underway), or future (scheduled).
#[derive(Debug, Deserialize, PartialEq, Eq, Copy, Clone)]
pub enum AbstractGameCode {
	/// Game has not begun
	#[serde(rename = "P")]
	Preview,

	/// Game is underway
	#[serde(rename = "L")]
	Live,

	/// Game is finished
	#[serde(rename = "F")]
	Finished,

	/// Used for [`CodedGameStatus::Writing`] and [`CodedGameStatus::Unknown`], typically best to assume game is finished.
	#[serde(rename = "O")]
	Other,
}

impl AbstractGameCode {
	#[must_use]
	pub const fn has_begun(self) -> bool {
		matches!(self, Self::Live | Self::Finished | Self::Other)
	}

	#[must_use]
	pub const fn has_ended(self) -> bool {
		matches!(self, Self::Finished | Self::Other)
	}

	#[must_use]
	pub const fn is_preview(self) -> bool {
		matches!(self, Self::Preview)
	}

	#[must_use]
	pub const fn is_live(self) -> bool {
		matches!(self, Self::Live)
	}

	#[must_use]
	pub const fn is_finished(self) -> bool {
		matches!(self, Self::Finished)
	}

	#[must_use]
	pub const fn is_unknown(self) -> bool {
		matches!(self, Self::Other)
	}
}

#[derive(Debug, Deserialize, Deref, DerefMut, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HydratedGameStatus {
	pub abstract_game_state: String,
	pub coded_game_state: CodedGameState,
	pub status_code: String,
	pub reason: Option<String>,
	pub abstract_game_code: AbstractGameCode,

	#[deref]
	#[deref_mut]
	#[serde(flatten)]
	inner: IdentifiableGameStatus,
}

#[derive(Debug, Deserialize, Eq, Clone, From, EnumTryAs)]
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
		self.id == other.id
	}
}

impl MetaKind for GameStatus {
	const ENDPOINT_NAME: &'static str = "gameStatus";
}

static CACHE: RwLock<HydratedCacheTable<GameStatus>> = rwlock_const_new(HydratedCacheTable::new());

impl EndpointEntryCache for GameStatus {
	type HydratedVariant = HydratedGameStatus;
	type Identifier = GameStatusId;
	type URL = MetaEndpointUrl<Self>;

	fn into_hydrated_variant(self) -> Option<Self::HydratedVariant> {
		self.try_as_hydrated()
	}

	fn id(&self) -> &Self::Identifier {
		&self.id
	}

	fn url_for_id(_id: &Self::Identifier) -> Self::URL {
		MetaEndpointUrl::new()
	}

	fn get_entries(response: <Self::URL as StatsAPIUrl>::Response) -> impl IntoIterator<Item=Self>
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
	use crate::endpoints::StatsAPIUrl;
	use crate::endpoints::meta::MetaEndpointUrl;

	#[tokio::test]
	async fn parse_meta() {
		let _response = MetaEndpointUrl::<super::GameStatus>::new().get().await.unwrap();
	}
}
