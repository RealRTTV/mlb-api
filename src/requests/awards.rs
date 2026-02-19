//! An award, such as the Cy Young, MVP, etc.
//! 
//! There are over a thousand awards for pretty much every organized league you can think of.

use crate::league::LeagueId;
use crate::request::RequestURL;
use crate::season::SeasonId;
use crate::sport::SportId;
use crate::Copyright;
use bon::Builder;
use serde::Deserialize;
use std::fmt::{Display, Formatter};
use crate::cache::Requestable;

#[cfg(feature = "cache")]
use crate::{rwlock_const_new, RwLock, cache::CacheTable};

/// Response from the `awards` endpoint.
/// Returns a [`Vec`] of [`Award`]s
///
/// Example: <http://statsapi.mlb.com/api/v1/awards>
#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
pub struct AwardsResponse {
	pub copyright: Copyright,
	pub awards: Vec<Award>,
}

/// An award, such as the Cy Young or MVP.
#[derive(Debug, Deserialize, Clone)]
pub struct Award {
	/// Name of the awward
	pub name: String,
	/// Description of the award
	pub description: Option<String>,
	/// Sport associated with the award
	pub sport: Option<SportId>,
	/// League associated with the award
	pub league: Option<LeagueId>,
	/// Notes (if necessary)
	pub notes: Option<String>,
	#[serde(flatten)]
	pub id: AwardId,
}

id_only_eq_impl!(Award, id);
id!(AwardId { id: String });

/// Returns [`AwardResponse`]
#[derive(Builder)]
#[builder(derive(Into))]
pub struct AwardRequest {
	#[builder(into)]
	award_id: Option<AwardId>,
	#[builder(into)]
	sport_id: Option<SportId>,
	#[builder(into)]
	league_id: Option<LeagueId>,
	#[builder(into)]
	season: Option<SeasonId>,
}

impl<S: award_request_builder::State + award_request_builder::IsComplete> crate::request::RequestURLBuilderExt for AwardRequestBuilder<S> {
    type Built = AwardRequest;
}

impl Display for AwardRequest {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"http://statsapi.mlb.com/api/v1/awards{}",
			gen_params! { "awardId"?: self.award_id.as_ref(), "sportId"?: self.sport_id, "leagueId"?: self.league_id, "season"?: self.season }
		)
	}
}

impl RequestURL for AwardRequest {
	type Response = AwardsResponse;
}

#[cfg(feature = "cache")]
static CACHE: RwLock<CacheTable<Award>> = rwlock_const_new(CacheTable::new());

impl Requestable for Award {
	type Identifier = AwardId;
	type URL = AwardRequest;

	fn id(&self) -> &Self::Identifier {
		&self.id
	}

	#[cfg(feature = "aggressive_cache")]
	fn url_for_id(_id: &Self::Identifier) -> Self::URL {
		AwardRequest::builder().build()
	}

	#[cfg(not(feature = "aggressive_cache"))]
	fn url_for_id(id: &Self::Identifier) -> Self::URL {
		AwardRequest::builder().award_id(id.clone()).build()
	}

	fn get_entries(response: <Self::URL as RequestURL>::Response) -> impl IntoIterator<Item=Self>
	where
		Self: Sized
	{
		response.awards
	}

	#[cfg(feature = "cache")]
	fn get_cache_table() -> &'static RwLock<CacheTable<Self>>
	where
		Self: Sized
	{
		&CACHE
	}
}

entrypoint!(AwardId => Award);
entrypoint!(Award.id => Award);

#[cfg(test)]
mod tests {
	use crate::awards::AwardRequest;
	use crate::request::RequestURLBuilderExt;

	#[tokio::test]
	async fn parse_this_season() {
		let _response = AwardRequest::builder().build_and_get().await.unwrap();
	}
}
