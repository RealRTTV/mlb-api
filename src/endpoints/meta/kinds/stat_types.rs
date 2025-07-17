use derive_more::Display;
use crate::endpoints::meta::{MetaEndpointUrl, MetaKind};
use serde::Deserialize;

#[cfg(feature = "static_stat_types")]
use r#static::*;
use crate::cache::{EndpointEntryCache, HydratedCacheTable};
use crate::{rwlock_const_new, RwLock};
use crate::endpoints::StatsAPIUrl;

#[cfg(feature = "static_stat_types")]
mod r#static {
	use derive_more::{Display, FromStr};
	use serde::Deserialize;

	#[derive(Debug, Deserialize, PartialEq, Eq, Copy, Clone, FromStr, Display, Hash)]
	#[non_exhaustive]
	#[serde(try_from = "__StatTypeStruct")]
	pub enum StatType {
		Projected,
		ProjectedRos,
		YearByYear,
		YearByYearAdvanced,
		YearByYearPlayoffs,
		Season,
		Standard,
		Advanced,
		Career,
		CareerRegularSeason,
		CareerAdvanced,
		SeasonAdvanced,
		CareerStatSplits,
		CareerPlayoffs,
		GameLog,
		PlayLog,
		PitchLog,
		MetricLog,
		MetricAverages,
		PitchArsenal,
		OutsAboveAverage,
		ExpectedStatistics,
		Sabermetrics,
		SprayChart,
		Tracking,
		VsPlayer,
		VsPlayerTotal,
		VsPlayer5Y,
		VsTeam,
		VsTeam5Y,
		VsTeamTotal,
		LastXGames,
		ByDateRange,
		ByDateRangeAdvanced,
		ByMonth,
		ByMonthPlayoffs,
		ByDayOfWeek,
		ByDayOfWeekPlayoffs,
		HomeAndAway,
		HomeAndAwayPlayoffs,
		WinLoss,
		WinLossPlayoffs,
		Rankings,
		RankingsByYear,
		StatsSingleSeason,
		StatsSingleSeasonAdvanced,
		HotColdZones,
		AvailableStats,
		OpponentsFaced,
		GameTypeStats,
		FirstYearStats,
		LastYearStats,
		StatSplits,
		StatSplitsAdvanced,
		AtGameStart,
		VsOpponents,
		SabermetricsMultiTeam,
	}

	#[derive(Deserialize)]
	#[serde(rename_all = "camelCase")]
	struct __StatTypeStruct {
		display_name: String,
	}

	impl TryFrom<__StatTypeStruct> for StatType {
		type Error = derive_more::FromStrError;

		fn try_from(value: __StatTypeStruct) -> Result<Self, Self::Error> {
			value.display_name.parse::<Self>()
		}
	}
}

#[cfg(not(feature = "static_stat_types"))]
#[derive(Debug, Deserialize, PartialEq, Eq, Clone, Display, Hash)]
#[display("{name}")]
pub struct StatType {
	#[serde(rename = "displayName")]
	pub name: String,
}

impl MetaKind for StatType {
	const ENDPOINT_NAME: &'static str = "statTypes";
}

static CACHE: RwLock<HydratedCacheTable<StatType>> = rwlock_const_new(HydratedCacheTable::new());

impl EndpointEntryCache for StatType {
	type HydratedVariant = StatType;
	type Identifier = StatType;
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

	#[cfg(feature = "static_stat_types")]
	#[tokio::test]
	async fn is_still_up_to_date() {
		use crate::endpoints::meta::MetaEndpointUrl;
		use crate::endpoints::meta::stat_types::StatType;
		use serde::Deserialize;

		#[derive(Deserialize)]
		#[serde(rename_all = "camelCase")]
		struct StatTypeStruct {
			display_name: String,
		}

		let json = reqwest::get(MetaEndpointUrl::<StatType>::new().to_string()).await.unwrap().bytes().await.unwrap();
		let first_kind: Vec<StatType> = {
			let mut de = serde_json::Deserializer::from_slice(&json);
			serde_path_to_error::deserialize(&mut de).unwrap()
		};
		let second_kind: Vec<StatTypeStruct> = {
			let mut de = serde_json::Deserializer::from_slice(&json);
			serde_path_to_error::deserialize(&mut de).unwrap()
		};
		assert_eq!(first_kind.len(), second_kind.len());
	}

	#[tokio::test]
	async fn parse_meta() {
		let _response = MetaEndpointUrl::<super::StatType>::new().get().await.unwrap();
	}
}
