use crate::league::League;
use crate::season::SeasonId;
use crate::sports::Sport;
use crate::types::Copyright;
use crate::request::StatsAPIRequestUrl;
use bon::Builder;
use derive_more::{Deref, DerefMut, From};
use mlb_api_proc::{EnumDeref, EnumDerefMut, EnumTryAs, EnumTryAsMut, EnumTryInto};
use serde::Deserialize;
use std::fmt::{Display, Formatter};

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ConferencesResponse {
	pub copyright: Copyright,
	pub conferences: Vec<Conference>,
}

#[derive(Debug, Deserialize, Deref, DerefMut, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HydratedConference {
	pub abbreviation: String,
	#[serde(rename = "nameShort")]
	pub short_name: String,
	pub has_wildcard: bool,
	pub league: League,
	pub sport: Sport,

	#[deref]
	#[deref_mut]
	#[serde(flatten)]
	inner: NamedConference,
}

#[derive(Debug, Deserialize, Deref, DerefMut, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NamedConference {
	pub name: String,

	#[deref]
	#[deref_mut]
	#[serde(flatten)]
	inner: IdentifiableConference,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
pub struct IdentifiableConference {
	pub id: ConferenceId,
}

integer_id!(ConferenceId);

#[derive(Debug, Deserialize, Eq, Clone, From, EnumTryAs, EnumTryAsMut, EnumTryInto, EnumDeref, EnumDerefMut)]
#[serde(untagged)]
pub enum Conference {
	Hydrated(Box<HydratedConference>),
	Named(NamedConference),
	Identifiable(IdentifiableConference),
}

id_only_eq_impl!(Conference, id);

#[derive(Builder)]
#[builder(derive(Into))]
pub struct ConferencesRequest {
	#[builder(into)]
	conference_id: Option<ConferenceId>,
	#[builder(into)]
	season: Option<SeasonId>,
}

impl<S: conferences_request_builder::State + conferences_request_builder::IsComplete> crate::request::StatsAPIRequestUrlBuilderExt for ConferencesRequestBuilder<S> {
    type Built = ConferencesRequest;
}

impl Display for ConferencesRequest {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "http://statsapi.mlb.com/api/v1/conferences{}", gen_params! { "conferenceId"?: self.conference_id, "season"?: self.season })
	}
}

impl StatsAPIRequestUrl for ConferencesRequest {
	type Response = ConferencesResponse;
}

tiered_request_entry_cache_impl!(ConferencesRequest => |id: ConferenceId| { ConferencesRequest::builder().conference_id(*id).build() }.conferences => Conference => Box<HydratedConference>);

#[cfg(test)]
mod tests {
	use crate::conferences::ConferencesRequest;
	use crate::request::StatsAPIRequestUrlBuilderExt;

	#[tokio::test]
	async fn parse_all_conferences() {
		let _response = ConferencesRequest::builder().build_and_get().await.unwrap();
	}
}
