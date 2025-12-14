use crate::types::Copyright;
use crate::request::StatsAPIRequestUrl;
use bon::Builder;
use derive_more::{Deref, DerefMut, Display, From};
use mlb_api_proc::{EnumDeref, EnumDerefMut, EnumTryAs, EnumTryAsMut, EnumTryInto};
use serde::Deserialize;
use std::fmt::{Display, Formatter};

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SportsResponse {
	pub copyright: Copyright,
	pub sports: Vec<Sport>,
}

integer_id!(#[derive(Debug, Deserialize, Deref, Display, PartialEq, Eq, Copy, Clone, Hash, From)] SportId);

impl SportId {
	/// This is here because we can rest assured that it won't ever go away.
	pub const MLB: Self = Self::new(1);
}

impl Default for SportId {
	fn default() -> Self {
		Self::MLB
	}
}

#[derive(Builder)]
#[builder(derive(Into))]
pub struct SportsRequest {
	#[builder(into)]
	id: Option<SportId>,
}

impl<S: sports_request_builder::State + sports_request_builder::IsComplete> crate::request::StatsAPIRequestUrlBuilderExt for SportsRequestBuilder<S> {
	type Built = SportsRequest;
}

impl Display for SportsRequest {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "http://statsapi.mlb.com/api/v1/sports{}", gen_params! { "sportId"?: self.id })
	}
}

impl StatsAPIRequestUrl for SportsRequest {
	type Response = SportsResponse;
}

#[derive(Debug, Deserialize, PartialEq, Eq, Copy, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IdentifiableSport {
	pub id: SportId,
}

#[derive(Debug, Deserialize, Deref, DerefMut, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NamedSport {
	pub name: String,

	#[deref]
	#[deref_mut]
	#[serde(flatten)]
	pub(super) inner: IdentifiableSport,
}

#[derive(Debug, Deserialize, Deref, DerefMut, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HydratedSport {
	pub code: String,
	pub abbreviation: String,
	#[serde(rename = "activeStatus")]
	pub active: bool,

	#[deref]
	#[deref_mut]
	#[serde(flatten)]
	pub(super) inner: NamedSport,
}

#[derive(Debug, Deserialize, Eq, Clone, From, EnumTryAs, EnumTryAsMut, EnumTryInto, EnumDeref, EnumDerefMut)]
#[serde(untagged)]
pub enum Sport {
	Hydrated(HydratedSport),
	Named(NamedSport),
	Identifiable(IdentifiableSport),
}

id_only_eq_impl!(Sport, id);
tiered_request_entry_cache_impl!(SportsRequest => |id: SportId| { SportsRequest::builder().id(*id).build() }.sports => Sport => HydratedSport);

#[cfg(test)]
mod tests {
	use super::*;
	use crate::request::StatsAPIRequestUrlBuilderExt;

	#[tokio::test]
	async fn parse_all_sports() {
		let _result = SportsRequest::builder().build_and_get().await.unwrap();
	}
}
