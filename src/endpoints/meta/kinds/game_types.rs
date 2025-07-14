use std::fmt::{Debug, Formatter};
use derive_more::Display;
use serde::Deserialize;
use crate::endpoints::MetaKind;

#[derive(Deserialize, Default, PartialEq, Eq, Copy, Clone, Display)]
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
		write!(f, "{}", match self {
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
		})
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

#[cfg(test)]
mod tests {
	use crate::endpoints::meta::MetaEndpointUrl;
	use crate::endpoints::StatsAPIUrl;

	#[tokio::test]
	async fn parse_meta() {
		let _response = MetaEndpointUrl::<super::GameType>::new().get().await.unwrap();
	}
}
