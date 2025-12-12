use crate::cache::{HydratedCacheTable, RequestEntryCache};
use crate::meta::MetaRequest;
use crate::{rwlock_const_new, RwLock};
use crate::{MetaKind, StatsAPIRequestUrl};
use derive_more::Display;
use serde::Deserialize;
use std::fmt::{Debug, Formatter};
use std::ops::Deref;

#[derive(Deserialize, Default, PartialEq, Eq, Copy, Clone, Display, Hash)]
#[serde(try_from = "__GameTypeStruct")]
pub enum GameType {
	#[display("Spring Training")]
	SpringTraining,

	#[display("Intrasquad")]
	Intrasquad,

	#[display("Exhibition")]
	Exhibition,

	#[display("Nineteenth Century Series")]
	NineteenthCenturySeries,

	#[default]
	#[display("Regular Season")]
	RegularSeason,

	#[display("All Star Game")]
	AllStarGame,

	#[display("Divisional Series")]
	DivisionalSeries,

	#[display("Wild Card Series")]
	WildCardSeries,

	#[display("Championship Series")]
	ChampionshipSeries,

	#[display("World Series")]
	WorldSeries,

	#[display("Playoffs")]
	Playoffs,

	#[display("Championship")]
	Championship,
}

impl Debug for GameType {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"{}",
			match self {
				Self::SpringTraining => 'S',
				Self::Intrasquad => 'I',
				Self::Exhibition => 'E',
				Self::NineteenthCenturySeries => 'N',
				Self::RegularSeason => 'R',
				Self::AllStarGame => 'A',
				Self::DivisionalSeries => 'D',
				Self::WildCardSeries => 'F',
				Self::ChampionshipSeries => 'L',
				Self::WorldSeries => 'W',
				Self::Playoffs => 'P',
				Self::Championship => 'C',
			}
		)
	}
}

#[derive(Deserialize)]
#[doc(hidden)]
#[serde(untagged)]
enum __GameTypeStruct {
	Wrapped {
		id: String,
	},
	Inline(String),
}

impl Deref for __GameTypeStruct {
	type Target = String;

	fn deref(&self) -> &Self::Target {
		let (Self::Wrapped { id, .. } | Self::Inline(id)) = self;
		id
	}
}

impl TryFrom<__GameTypeStruct> for GameType {
	type Error = &'static str;

	fn try_from(value: __GameTypeStruct) -> Result<Self, Self::Error> {
		Ok(match &**value {
			"S" => Self::SpringTraining,
			"I" => Self::Intrasquad,
			"E" => Self::Exhibition,
			"N" => Self::NineteenthCenturySeries,
			"R" => Self::RegularSeason,
			"A" => Self::AllStarGame,
			"D" => Self::DivisionalSeries,
			"F" => Self::WildCardSeries,
			"L" => Self::ChampionshipSeries,
			"W" => Self::WorldSeries,
			"P" => Self::Playoffs,
			"C" => Self::Championship,
			_ => return Err("unknown game type"),
		})
	}
}

impl MetaKind for GameType {
	const ENDPOINT_NAME: &'static str = "gameTypes";
}

static CACHE: RwLock<HydratedCacheTable<GameType>> = rwlock_const_new(HydratedCacheTable::new());

impl RequestEntryCache for GameType {
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
		let _response = MetaRequest::<super::GameType>::new().get().await.unwrap();
	}
}
