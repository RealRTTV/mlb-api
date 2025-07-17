use crate::endpoints::meta::MetaKind;
use derive_more::{Deref, DerefMut, Display, From};
use serde::Deserialize;
use std::ops::{Deref, DerefMut};
use strum::EnumTryAs;

#[repr(transparent)]
#[derive(Debug, Deserialize, Deref, Display, PartialEq, Eq, Clone)]
pub struct JobTypeId(String);

impl JobTypeId {
	#[must_use]
	pub const fn new(id: String) -> Self {
		Self(id)
	}
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IdentifiableJobType {
	#[serde(rename = "code")] pub id: JobTypeId,
}

#[derive(Debug, Deserialize, Deref, DerefMut, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HydratedJobType {
	pub job: String,

	#[deref]
	#[deref_mut]
	#[serde(flatten)]
	inenr: IdentifiableJobType,
}

#[derive(Debug, Deserialize, Eq, Clone, From, EnumTryAs)]
#[serde(untagged)]
pub enum JobType {
	Hydrated(HydratedJobType),
	Identifiable(IdentifiableJobType),
}

impl PartialEq for JobType {
	fn eq(&self, other: &Self) -> bool {
		self.id == other.id
	}
}

impl Deref for JobType {
	type Target = IdentifiableJobType;

	fn deref(&self) -> &Self::Target {
		match self {
			Self::Hydrated(inner) => inner,
			Self::Identifiable(inner) => inner,
		}
	}
}

impl DerefMut for JobType {
	fn deref_mut(&mut self) -> &mut Self::Target {
		match self {
			Self::Hydrated(inner) => inner,
			Self::Identifiable(inner) => inner,
		}
	}
}

impl MetaKind for JobType {
	const ENDPOINT_NAME: &'static str = "jobTypes";
}

#[cfg(test)]
mod tests {
	use crate::endpoints::StatsAPIUrl;
	use crate::endpoints::meta::MetaEndpointUrl;

	#[tokio::test]
	async fn parse_meta() {
		let _response = MetaEndpointUrl::<super::JobType>::new().get().await.unwrap();
	}
}
