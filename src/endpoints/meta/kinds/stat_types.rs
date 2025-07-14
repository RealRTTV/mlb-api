use serde::Deserialize;
use crate::endpoints::meta::MetaKind;

#[cfg(feature = "static_stat_types")]
use r#static::*;

#[cfg(feature = "static_stat_types")]
mod r#static {
	use derive_more::FromStr;
	use serde::Deserialize;

	#[derive(Debug, Deserialize, PartialEq, Eq, Copy, Clone, FromStr)]
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
#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
pub struct StatType {
	#[serde(rename = "displayName")]
	pub name: String,
}

impl MetaKind for StatType {
	const ENDPOINT_NAME: &'static str = "statTypes";
}

#[cfg(test)]
mod tests {
	use crate::endpoints::meta::MetaEndpointUrl;
	use crate::endpoints::StatsAPIUrl;
	
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
