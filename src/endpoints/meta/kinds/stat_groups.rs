use crate::endpoints::{MetaKind, StatsAPIUrl};
use derive_more::{Display, FromStr};
use serde::Deserialize;
use crate::cache::{EndpointEntryCache, HydratedCacheTable};
use crate::{rwlock_const_new, RwLock};
use crate::endpoints::meta::MetaEndpointUrl;

#[derive(Debug, Deserialize, PartialEq, Eq, Copy, Clone, FromStr, Hash, Display)]
#[serde(try_from = "__StatGroupStruct")]
pub enum StatGroup {
	Hitting,
	Pitching,
	Fielding,
	Catching,
	Running,
	Game,
	Team,
	Streak,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct __StatGroupStruct {
	display_name: String,
}

impl TryFrom<__StatGroupStruct> for StatGroup {
	type Error = derive_more::FromStrError;

	fn try_from(value: __StatGroupStruct) -> Result<Self, Self::Error> {
		value.display_name.parse::<Self>()
	}
}

impl MetaKind for StatGroup {
	const ENDPOINT_NAME: &'static str = "statGroups";
}

static CACHE: RwLock<HydratedCacheTable<StatGroup>> = rwlock_const_new(HydratedCacheTable::new());

impl EndpointEntryCache for StatGroup {
	type HydratedVariant = StatGroup;
	type Identifier = StatGroup;
	type URL = MetaEndpointUrl<Self>;

	fn into_hydrated_variant(self) -> Option<Self::HydratedVariant> {
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
		let _response = MetaEndpointUrl::<super::StatGroup>::new().get().await.unwrap();
	}
}
