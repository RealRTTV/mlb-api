use crate::endpoints::meta::MetaKind;
use derive_more::{Deref, DerefMut, From};
use serde::Deserialize;
use std::ops::{Deref, DerefMut};

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
pub struct IdentifiableScheduleEventType {
	pub code: String,
}

#[derive(Debug, Deserialize, Deref, DerefMut, PartialEq, Eq, Clone)]
pub struct HydratedScheduleEventType {
	pub name: String,

	#[deref]
	#[deref_mut]
	#[serde(flatten)]
	inner: IdentifiableScheduleEventType,
}

#[derive(Debug, Deserialize, Eq, Clone, From)]
#[serde(untagged)]
pub enum ScheduleEventType {
	Hydrated(HydratedScheduleEventType),
	Identifiable(IdentifiableScheduleEventType),
}

impl PartialEq for ScheduleEventType {
	fn eq(&self, other: &Self) -> bool {
		self.code == other.code
	}
}

impl Deref for ScheduleEventType {
	type Target = IdentifiableScheduleEventType;

	fn deref(&self) -> &Self::Target {
		match self {
			Self::Hydrated(inner) => inner,
			Self::Identifiable(inner) => inner,
		}
	}
}

impl DerefMut for ScheduleEventType {
	fn deref_mut(&mut self) -> &mut Self::Target {
		match self {
			Self::Hydrated(inner) => inner,
			Self::Identifiable(inner) => inner,
		}
	}
}

impl MetaKind for ScheduleEventType {
	const ENDPOINT_NAME: &'static str = "scheduleEventTypes";
}

#[cfg(test)]
mod tests {
	use crate::endpoints::meta::MetaEndpointUrl;
	use crate::endpoints::StatsAPIUrl;

	#[tokio::test]
	async fn parse_meta() {
		let _response = MetaEndpointUrl::<super::ScheduleEventType>::new().get().await.unwrap();
	}
}
