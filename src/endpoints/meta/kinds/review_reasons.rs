use crate::endpoints::meta::{MetaEndpoint, MetaKind};
use derive_more::{Deref, DerefMut, Display, From};
use serde::Deserialize;
use std::ops::{Deref, DerefMut};
use strum::EnumTryAs;
use crate::cache::{EndpointEntryCache, HydratedCacheTable};
use crate::{rwlock_const_new, RwLock};
use crate::endpoints::StatsAPIEndpointUrl;

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
pub struct IdentifiableReviewReason {
	#[serde(rename = "code")] pub id: ReviewReasonId,
}

#[repr(transparent)]
#[derive(Debug, Deserialize, Deref, Display, PartialEq, Eq, Clone, Hash, From)]
pub struct ReviewReasonId(String);

#[derive(Debug, Deserialize, Deref, DerefMut, PartialEq, Eq, Clone)]
pub struct HydratedReviewReason {
	pub description: String,

	#[deref]
	#[deref_mut]
	#[serde(flatten)]
	inner: IdentifiableReviewReason,
}

#[derive(Debug, Deserialize, Eq, Clone, From, EnumTryAs)]
#[serde(untagged)]
pub enum ReviewReason {
	Hydrated(HydratedReviewReason),
	Identifiable(IdentifiableReviewReason),
}

impl PartialEq for ReviewReason {
	fn eq(&self, other: &Self) -> bool {
		self.id == other.id
	}
}

impl Deref for ReviewReason {
	type Target = IdentifiableReviewReason;

	fn deref(&self) -> &Self::Target {
		match self {
			Self::Hydrated(inner) => inner,
			Self::Identifiable(inner) => inner,
		}
	}
}

impl DerefMut for ReviewReason {
	fn deref_mut(&mut self) -> &mut Self::Target {
		match self {
			Self::Hydrated(inner) => inner,
			Self::Identifiable(inner) => inner,
		}
	}
}

impl MetaKind for ReviewReason {
	const ENDPOINT_NAME: &'static str = "reviewReasons";
}

static CACHE: RwLock<HydratedCacheTable<ReviewReason>> = rwlock_const_new(HydratedCacheTable::new());

impl EndpointEntryCache for ReviewReason {
	type HydratedVariant = HydratedReviewReason;
	type Identifier = ReviewReasonId;
	type URL = MetaEndpoint<Self>;

	fn into_hydrated_variant(self) -> Option<Self::HydratedVariant> {
		self.try_as_hydrated()
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
		let _response = MetaEndpoint::<super::ReviewReason>::new().get().await.unwrap();
	}
}
