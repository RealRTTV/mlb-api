use crate::endpoints::meta::MetaKind;
use derive_more::{Deref, DerefMut, From};
use serde::Deserialize;
use std::ops::{Deref, DerefMut};
use strum::EnumTryAs;

#[derive(Debug, Deserialize, Deref, DerefMut, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HydratedPitchCode {
	pub description: String,
	#[serde(rename = "swingStatus")]
	pub has_swing: bool,
	#[serde(rename = "swingMissStatus")]
	pub is_whiff: bool,
	#[serde(rename = "swingContactStatus")]
	pub swing_made_contact: bool,
	#[serde(rename = "strikeStatus")]
	pub is_strike: bool,
	#[serde(rename = "ballStatus")]
	pub is_ball: bool,
	#[serde(rename = "pitchStatus")]
	pub is_pitch: bool,
	pub pitch_result_text: String,
	#[serde(rename = "buntAttemptStatus")]
	pub is_bunt_attempt: bool,
	#[serde(rename = "contactStatus")]
	pub made_contact: bool,

	#[deref]
	#[deref_mut]
	#[serde(flatten)]
	inner: IdentifiablePitchCode,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
pub struct IdentifiablePitchCode {
	pub code: String,
}

#[derive(Debug, Deserialize, Eq, Clone, From, EnumTryAs)]
#[serde(untagged)]
pub enum PitchCode {
	Hydrated(HydratedPitchCode),
	Identifiable(IdentifiablePitchCode),
}

impl PartialEq for PitchCode {
	fn eq(&self, other: &Self) -> bool {
		self.code == other.code
	}
}

impl Deref for PitchCode {
	type Target = IdentifiablePitchCode;

	fn deref(&self) -> &Self::Target {
		match self {
			Self::Hydrated(inner) => inner,
			Self::Identifiable(inner) => inner,
		}
	}
}

impl DerefMut for PitchCode {
	fn deref_mut(&mut self) -> &mut Self::Target {
		match self {
			Self::Hydrated(inner) => inner,
			Self::Identifiable(inner) => inner,
		}
	}
}

impl MetaKind for PitchCode {
	const ENDPOINT_NAME: &'static str = "pitchCodes";
}

#[cfg(test)]
mod tests {
	use crate::endpoints::StatsAPIUrl;
	use crate::endpoints::meta::MetaEndpointUrl;

	#[tokio::test]
	async fn parse_meta() {
		let _response = MetaEndpointUrl::<super::PitchCode>::new().get().await.unwrap();
	}
}
