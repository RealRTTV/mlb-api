use crate::endpoints::meta::MetaKind;
use derive_more::{Deref, DerefMut, From};
use serde::Deserialize;
use std::ops::{Deref, DerefMut};

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
pub struct IdentifiableReviewReason {
	pub code: String,
}

#[derive(Debug, Deserialize, Deref, DerefMut, PartialEq, Eq, Clone)]
pub struct HydratedReviewReason {
	pub description: String,

	#[deref]
	#[deref_mut]
	#[serde(flatten)]
	inner: IdentifiableReviewReason,
}

#[derive(Debug, Deserialize, Eq, Clone, From)]
#[serde(untagged)]
pub enum ReviewReason {
	Hydrated(HydratedReviewReason),
	Identifiable(IdentifiableReviewReason),
}

impl PartialEq for ReviewReason {
	fn eq(&self, other: &Self) -> bool {
		self.code == other.code
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

#[cfg(test)]
mod tests {
	use crate::endpoints::meta::MetaEndpointUrl;
	use crate::endpoints::StatsAPIUrl;

	#[tokio::test]
	async fn parse_meta() {
		let _response = MetaEndpointUrl::<super::ReviewReason>::new().get().await.unwrap();
	}
}
