use crate::cache::{RequestEntryCache, HydratedCacheTable};
use crate::meta::{MetaRequest, MetaKind};
use crate::StatsAPIRequestUrl;
use crate::{rwlock_const_new, RwLock};
use derive_more::{Deref, DerefMut, Display, From};
use mlb_api_proc::{EnumTryAs, EnumTryAsMut, EnumTryInto};
use serde::Deserialize;
use std::ops::{Deref, DerefMut};

#[allow(clippy::struct_excessive_bools, reason = "false positive")]
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
	#[serde(rename = "code")] pub id: PitchCodeId,
}

#[repr(transparent)]
#[derive(Debug, Deserialize, Deref, Display, PartialEq, Eq, Clone, Hash, From)]
pub struct PitchCodeId(String);

#[derive(Debug, Deserialize, Eq, Clone, From, EnumTryAs, EnumTryAsMut, EnumTryInto)]
#[serde(untagged)]
pub enum PitchCode {
	Hydrated(Box<HydratedPitchCode>),
	Identifiable(IdentifiablePitchCode),
}

impl PartialEq for PitchCode {
	fn eq(&self, other: &Self) -> bool {
		self.id == other.id
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

static CACHE: RwLock<HydratedCacheTable<PitchCode>> = rwlock_const_new(HydratedCacheTable::new());

impl RequestEntryCache for PitchCode {
	type HydratedVariant = Box<HydratedPitchCode>;
	type Identifier = PitchCodeId;
	type URL = MetaRequest<Self>;

	fn into_hydrated_variant(self) -> Option<Self::HydratedVariant> {
		self.try_into_hydrated()
	}

	fn id(&self) -> &Self::Identifier {
		&self.id
	}

	fn url_for_id(_id: &Self::Identifier) -> Self::URL {
		MetaRequest::new()
	}

	fn get_entries(response: <Self::URL as StatsAPIRequestUrl>::Response) -> impl IntoIterator<Item=Self>
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
    use crate::meta::MetaRequest;
    use crate::StatsAPIRequestUrl;

    #[tokio::test]
	async fn parse_meta() {
		let _response = MetaRequest::<super::PitchCode>::new().get().await.unwrap();
	}
}
