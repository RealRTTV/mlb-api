//! Different "sports"; MLB, AAA, AA, A+, A, Rookieball, etc.

use crate::Copyright;
use crate::request::RequestURL;
use bon::Builder;
use serde::Deserialize;
use std::fmt::{Display, Formatter};
use crate::cache::{Requestable};
#[cfg(feature = "cache")]
use crate::{rwlock_const_new, RwLock, cache::CacheTable};

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SportsResponse {
	pub copyright: Copyright,
	pub sports: Vec<Sport>,
}

id!(SportId { id: u32 });

impl SportId {
	/// This is only here because we can rest assured that it won't ever go away.
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

impl<S: sports_request_builder::State + sports_request_builder::IsComplete> crate::request::RequestURLBuilderExt for SportsRequestBuilder<S> {
	type Built = SportsRequest;
}

impl Display for SportsRequest {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "http://statsapi.mlb.com/api/v1/sports{}", gen_params! { "sportId"?: self.id })
	}
}

impl RequestURL for SportsRequest {
	type Response = SportsResponse;
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Sport {
	pub code: String,
	pub name: String,
	pub abbreviation: String,
	#[serde(rename = "activeStatus")]
	pub active: bool,
	#[serde(flatten)]
	pub id: SportId,
}

id_only_eq_impl!(Sport, id);

#[cfg(feature = "cache")]
static CACHE: RwLock<CacheTable<Sport>> = rwlock_const_new(CacheTable::new());

impl Requestable for Sport {
	type Identifier = SportId;
	type URL = SportsRequest;

	fn id(&self) -> &Self::Identifier {
		&self.id
	}

	#[cfg(feature = "aggressive_cache")]
	fn url_for_id(_id: &Self::Identifier) -> Self::URL {
		SportsRequest::builder().build()
	}

	#[cfg(not(feature = "aggressive_cache"))]
	fn url_for_id(id: &Self::Identifier) -> Self::URL {
		SportsRequest::builder().id(*id).build()
	}

	fn get_entries(response: <Self::URL as RequestURL>::Response) -> impl IntoIterator<Item=Self>
	where
		Self: Sized
	{
		response.sports
	}

	#[cfg(feature = "cache")]
	fn get_cache_table() -> &'static RwLock<CacheTable<Self>>
	where
		Self: Sized
	{
		&CACHE
	}
}

entrypoint!(SportId => Sport);
entrypoint!(Sport.id => Sport);

#[cfg(test)]
mod tests {
	use super::*;
	use crate::request::RequestURLBuilderExt;

	#[tokio::test]
	async fn parse_all_sports() {
		let _result = SportsRequest::builder().build_and_get().await.unwrap();
	}
}
