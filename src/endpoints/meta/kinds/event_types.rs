use crate::endpoints::meta::MetaKind;
use derive_more::{Deref, DerefMut, From};
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IdentifiableEventType {
	pub code: String,
}

#[derive(Debug, Deserialize, Deref, DerefMut, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HydratedEventType {
	plate_appearance: bool,
	hit: bool,
	base_running_event: bool,
	description: String,

	#[deref]
	#[deref_mut]
	#[serde(flatten)]
	inner: IdentifiableEventType,
}

#[derive(Debug, Deserialize, Eq, Clone, From)]
#[serde(untagged)]
pub enum EventType {
	Hydrated(HydratedEventType),
	Identifiable(IdentifiableEventType),
}

impl PartialEq for EventType {
	fn eq(&self, other: &Self) -> bool {
		self.code() == other.code()
	}
}

impl EventType {
	#[must_use]
	pub fn code(&self) -> &str {
		match self {
			Self::Hydrated(inner) => &inner.code,
			Self::Identifiable(inner) => &inner.code,
		}
	}
}

impl MetaKind for EventType {
	const ENDPOINT_NAME: &'static str = "eventTypes";
}

#[cfg(test)]
mod tests {
	use crate::endpoints::meta::MetaEndpointUrl;
	use crate::endpoints::StatsAPIUrl;

	#[tokio::test]
	async fn parse_meta() {
		let _response = MetaEndpointUrl::<super::EventType>::new().get().await.unwrap();
	}
}
