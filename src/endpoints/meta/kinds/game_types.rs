use crate::endpoints::{MetaKind, StatsAPIUrl};
use derive_more::Display;
use serde::Deserialize;
use std::fmt::{Debug, Formatter};
use crate::cache::{EndpointEntryCache, HydratedCacheTable};
use crate::{rwlock_const_new, RwLock};
use crate::endpoints::meta::MetaEndpointUrl;

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
struct __GameTypeStruct {
	id: String,
}

impl TryFrom<__GameTypeStruct> for GameType {
	type Error = &'static str;

	fn try_from(value: __GameTypeStruct) -> Result<Self, Self::Error> {
		Ok(match &*value.id {
			"S" => GameType::SpringTraining,
			"I" => GameType::Intrasquad,
			"E" => GameType::Exhibition,
			"N" => GameType::NineteenthCenturySeries,
			"R" => GameType::RegularSeason,
			"A" => GameType::AllStarGame,
			"D" => GameType::DivisionalSeries,
			"F" => GameType::WildCardSeries,
			"L" => GameType::ChampionshipSeries,
			"W" => GameType::WorldSeries,
			"P" => GameType::Playoffs,
			"C" => GameType::Championship,
			_ => return Err("unknown game type"),
		})
	}
}

impl MetaKind for GameType {
	const ENDPOINT_NAME: &'static str = "gameTypes";
}

static CACHE: RwLock<HydratedCacheTable<GameType>> = rwlock_const_new(HydratedCacheTable::new());

impl EndpointEntryCache for GameType {
	type HydratedVariant = GameType;
	type Identifier = GameType;
	type URL = MetaEndpointUrl<Self>;

	fn into_hydrated_entry(self) -> Option<Self::HydratedVariant> {
		Some(self)
	}

	fn id(&self) -> &Self::Identifier {
		self
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
		let _response = MetaEndpointUrl::<super::GameType>::new().get().await.unwrap();
	}
}
