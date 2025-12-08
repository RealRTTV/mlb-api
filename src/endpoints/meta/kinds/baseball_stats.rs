use crate::endpoints::meta::{MetaEndpoint, MetaKind};
use crate::endpoints::stat_groups::StatGroup;
use derive_more::{Deref, DerefMut, Display, From};
use serde::Deserialize;
use std::ops::{Deref, DerefMut};
use mlb_api_proc::{EnumTryAs, EnumTryAsMut, EnumTryInto};
use crate::cache::{EndpointEntryCache, HydratedCacheTable};
use crate::{rwlock_const_new, RwLock};
use crate::endpoints::StatsAPIEndpointUrl;

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
pub struct IdentifiableBaseballStat {
	#[serde(rename = "name")]
	pub id: BaseballStatId,
}

#[repr(transparent)]
#[derive(Debug, Deserialize, Deref, Display, PartialEq, Eq, Clone, Hash, From)]
pub struct BaseballStatId(String);

impl BaseballStatId {
	#[must_use]
	pub const fn new(id: String) -> Self {
		Self(id)
	}
}

#[derive(Debug, Deserialize, Deref, DerefMut, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HydratedBaseballStat {
	lookup_param: Option<String>,
	is_counting: bool,
	label: Option<String>,
	stat_groups: Vec<StatGroup>,

	#[deref]
	#[deref_mut]
	#[serde(flatten)]
	inner: IdentifiableBaseballStat,
}

#[derive(Debug, Deserialize, Eq, Clone, From, EnumTryAs, EnumTryAsMut, EnumTryInto)]
#[serde(untagged)]
pub enum BaseballStat {
	Hydrated(HydratedBaseballStat),
	Identifiable(IdentifiableBaseballStat),
}

impl PartialEq for BaseballStat {
	fn eq(&self, other: &Self) -> bool {
		self.id == other.id
	}
}

impl Deref for BaseballStat {
	type Target = IdentifiableBaseballStat;

	fn deref(&self) -> &Self::Target {
		match self {
			Self::Hydrated(inner) => inner,
			Self::Identifiable(inner) => inner,
		}
	}
}

impl DerefMut for BaseballStat {
	fn deref_mut(&mut self) -> &mut Self::Target {
		match self {
			Self::Hydrated(inner) => inner,
			Self::Identifiable(inner) => inner,
		}
	}
}

impl MetaKind for BaseballStat {
	const ENDPOINT_NAME: &'static str = "baseballStats";
}

static CACHE: RwLock<HydratedCacheTable<BaseballStat>> = rwlock_const_new(HydratedCacheTable::new());

impl EndpointEntryCache for BaseballStat {
	type HydratedVariant = HydratedBaseballStat;
	type Identifier = BaseballStatId;
	type URL = MetaEndpoint<Self>;

	fn into_hydrated_variant(self) -> Option<Self::HydratedVariant> {
		self.try_into_hydrated()
	}

	fn id(&self) -> &Self::Identifier {
		&self.id
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
	use crate::endpoints::StatsAPIEndpointUrl;
	use crate::endpoints::meta::MetaEndpoint;

	#[tokio::test]
	async fn parse_meta() {
		let _response = MetaEndpoint::<super::BaseballStat>::new().get().await.unwrap();
	}
}
