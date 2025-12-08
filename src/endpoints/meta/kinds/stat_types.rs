#[cfg(not(feature = "static_stat_types"))]
use derive_more::Display;
#[cfg(not(feature = "static_stat_types"))]
use serde::Deserializer;
use crate::meta::{MetaEndpoint, MetaKind};
use serde::Deserialize;
use crate::cache::{EndpointEntryCache, HydratedCacheTable};
use crate::{rwlock_const_new, RwLock};
use crate::StatsAPIEndpointUrl;

#[cfg(feature = "static_stat_types")]
use derive_more::{Display, FromStr};

#[cfg(feature = "static_stat_types")]
macro_rules! create_stat_type {
    ($($variant:ident),* $(,)?) => {
	    #[derive(Debug, Deserialize, PartialEq, Eq, Copy, Clone, FromStr, Display, Hash)]
	    #[non_exhaustive]
	    #[serde(try_from = "__StatTypeMaybeInline")]
	    pub enum StatType {
		    $($variant,)*
	    }

	    impl StatType {
		    #[must_use]
		    pub fn as_str(&self) -> &'static str {
			    match self {
				    $(
				    Self::$variant => stringify!($variant),
				    )*
			    }
		    }
	    }
    };
}
#[cfg(feature = "static_stat_types")]
create_stat_type! {
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

#[doc(hidden)]
#[derive(Deserialize)]
#[serde(untagged)]
enum __StatTypeMaybeInline {
	Wrapped {
		#[serde(rename = "displayName")]
		display_name: String,
	},
	Inline(String),
}

impl __StatTypeMaybeInline {
	#[must_use]
	pub fn into_string(self) -> String {
		match self {
			__StatTypeMaybeInline::Wrapped { display_name } => display_name,
			__StatTypeMaybeInline::Inline(name) => name,
		}
	}
}

#[cfg(feature = "static_stat_types")]
impl TryFrom<__StatTypeMaybeInline> for StatType {
	type Error = derive_more::FromStrError;

	fn try_from(value: __StatTypeMaybeInline) -> Result<Self, Self::Error> {
		value.into_string().parse::<Self>()
	}
}

#[cfg(not(feature = "static_stat_types"))]
#[derive(Debug, PartialEq, Eq, Clone, Display, Hash)]
#[display("{name}")]
pub struct StatType {
	pub name: String,
}

#[cfg(not(feature = "static_stat_types"))]
impl<'de> Deserialize<'de> for StatType {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>
	{
		Ok(StatType { name: __StatTypeMaybeInline::deserialize(deserializer)?.into_string() })
	}
}

#[cfg(not(feature = "static_stat_types"))]
impl StatType {
	pub fn as_str(&self) -> &str {
		self.name.as_str()
	}
}

impl MetaKind for StatType {
	const ENDPOINT_NAME: &'static str = "statTypes";
}

static CACHE: RwLock<HydratedCacheTable<StatType>> = rwlock_const_new(HydratedCacheTable::new());

impl EndpointEntryCache for StatType {
	type HydratedVariant = StatType;
	type Identifier = StatType;
	type URL = MetaEndpoint<Self>;

	fn into_hydrated_variant(self) -> Option<Self::HydratedVariant> {
		Some(self)
	}

	fn id(&self) -> &Self::Identifier {
		self
	}

	fn url_for_id(_id: &Self::Identifier) -> Self::URL {
		MetaEndpoint::new()
	}

	fn get_entries(response: <Self::URL as StatsAPIEndpointUrl>::Response) -> impl IntoIterator<Item=Self>
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
	use crate::StatsAPIEndpointUrl;
	use crate::meta::MetaEndpoint;

	#[cfg(feature = "static_stat_types")]
	#[tokio::test]
	async fn is_still_up_to_date() {
		use crate::meta::MetaEndpoint;
		use crate::meta::kinds::stat_types::StatType;
		use serde::Deserialize;

		#[derive(Deserialize)]
		#[serde(rename_all = "camelCase")]
		struct StatTypeStruct {
			display_name: String,
		}

		let json = reqwest::get(MetaEndpoint::<StatType>::new().to_string()).await.unwrap().bytes().await.unwrap();
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
		let _response = MetaEndpoint::<super::StatType>::new().get().await.unwrap();
	}
}
