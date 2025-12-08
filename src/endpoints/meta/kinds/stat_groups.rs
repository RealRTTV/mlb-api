use crate::{MetaKind, StatsAPIEndpointUrl};
use derive_more::{Display, FromStr};
use serde::Deserialize;
use crate::cache::{EndpointEntryCache, HydratedCacheTable};
use crate::{rwlock_const_new, RwLock};
use crate::meta::MetaEndpoint;

#[derive(Debug, Deserialize, PartialEq, Eq, Copy, Clone, FromStr, Hash, Display)]
#[serde(try_from = "__StatGroupMaybeInline")]
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
#[serde(untagged)]
#[doc(hidden)]
enum __StatGroupMaybeInline {
	Wrapped {
		#[serde(rename = "displayName")]
		display_name: String,
	},
	Inline(String),
}

impl __StatGroupMaybeInline {
	#[must_use]
	pub fn into_string(self) -> String {
		match self {
			__StatGroupMaybeInline::Wrapped { display_name } => display_name,
			__StatGroupMaybeInline::Inline(name) => name,
		}
	}
}

impl TryFrom<__StatGroupMaybeInline> for StatGroup {
	type Error = derive_more::FromStrError;

	fn try_from(value: __StatGroupMaybeInline) -> Result<Self, Self::Error> {
		value.into_string().parse::<Self>()
	}
}

impl MetaKind for StatGroup {
	const ENDPOINT_NAME: &'static str = "statGroups";
}

static CACHE: RwLock<HydratedCacheTable<StatGroup>> = rwlock_const_new(HydratedCacheTable::new());

impl EndpointEntryCache for StatGroup {
	type HydratedVariant = StatGroup;
	type Identifier = StatGroup;
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

	#[tokio::test]
	async fn parse_meta() {
		let _response = MetaEndpoint::<super::StatGroup>::new().get().await.unwrap();
	}
}
