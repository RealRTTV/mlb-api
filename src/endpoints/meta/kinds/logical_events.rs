use crate::endpoints::meta::MetaKind;
use derive_more::From;
use serde::Deserialize;
use std::ops::{Deref, DerefMut};
use strum::EnumTryAs;

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
pub struct IdentifiableLogicalEvent {
	pub code: String,
}

#[derive(Debug, Deserialize, Eq, Clone, From, EnumTryAs)]
#[serde(untagged)]
pub enum LogicalEvent {
	Identifiable(IdentifiableLogicalEvent),
}

impl PartialEq for LogicalEvent {
	fn eq(&self, other: &Self) -> bool {
		self.code == other.code
	}
}

impl Deref for LogicalEvent {
	type Target = IdentifiableLogicalEvent;

	fn deref(&self) -> &Self::Target {
		match self {
			Self::Identifiable(inner) => inner,
		}
	}
}

impl DerefMut for LogicalEvent {
	fn deref_mut(&mut self) -> &mut Self::Target {
		match self {
			Self::Identifiable(inner) => inner,
		}
	}
}

impl MetaKind for LogicalEvent {
	const ENDPOINT_NAME: &'static str = "logicalEvents";
}

#[cfg(test)]
mod tests {
	use crate::endpoints::StatsAPIUrl;
	use crate::endpoints::meta::MetaEndpointUrl;

	#[tokio::test]
	async fn parse_meta() {
		let _response = MetaEndpointUrl::<super::LogicalEvent>::new().get().await.unwrap();
	}
}
