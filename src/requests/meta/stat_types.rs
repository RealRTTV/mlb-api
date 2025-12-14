use crate::cache::{HydratedCacheTable, RequestEntryCache};
use crate::meta::{MetaKind, MetaRequest};
use crate::request::StatsAPIRequestUrl;
use crate::{rwlock_const_new, RwLock};
#[cfg(not(feature = "static_stat_types"))]
use derive_more::Display;
use serde::Deserialize;
#[cfg(not(feature = "static_stat_types"))]
use serde::Deserializer;

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
			Self::Wrapped { display_name } => display_name,
			Self::Inline(name) => name,
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
		Ok(Self { name: __StatTypeMaybeInline::deserialize(deserializer)?.into_string() })
	}
}

#[cfg(not(feature = "static_stat_types"))]
impl StatType {
	#[must_use]
	pub const fn as_str(&self) -> &str {
		self.name.as_str()
	}
}

impl MetaKind for StatType {
	const ENDPOINT_NAME: &'static str = "statTypes";
}

static CACHE: RwLock<HydratedCacheTable<StatType>> = rwlock_const_new(HydratedCacheTable::new());

impl RequestEntryCache for StatType {
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
	use crate::request::StatsAPIRequestUrl;

	#[cfg(feature = "static_stat_types")]
	#[tokio::test]
	async fn is_still_up_to_date() {
		use crate::meta::MetaRequest;
		use crate::requests::meta::stat_types::StatType;
		use serde::Deserialize;

		#[derive(Deserialize)]
		#[serde(rename_all = "camelCase")]
		struct StatTypeStruct {
			display_name: String,
		}

		let json = reqwest::get(MetaRequest::<StatType>::new().to_string()).await.unwrap().bytes().await.unwrap();
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
		let _response = MetaRequest::<super::StatType>::new().get().await.unwrap();
	}
}
