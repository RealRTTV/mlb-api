//! Lists all players in the [sport](SportId) in a given season.

use crate::person::people::PeopleResponse;
use crate::season::SeasonId;
use crate::request::RequestURL;
use bon::Builder;
use std::fmt::{Display, Formatter};
use crate::hydrations::Hydrations;
use crate::person::PersonHydrations;
use crate::sport::SportId;

/// Returns a [`PeopleResponse`].
#[allow(unused)]
#[derive(Builder)]
#[builder(derive(Into))]
pub struct PlayersRequest<H: PersonHydrations> {
	#[builder(into)]
	#[builder(default)]
	sport_id: SportId,
	#[builder(into)]
	season: Option<SeasonId>,
	#[builder(into)]
	hydrations: H::RequestData,
}

impl<H: PersonHydrations, S: players_request_builder::State + players_request_builder::IsComplete> crate::request::RequestURLBuilderExt for PlayersRequestBuilder<H, S> {
	type Built = PlayersRequest<H>;
}

impl PlayersRequest<()> {
	pub fn for_sport(sport_id: impl Into<SportId>) -> PlayersRequestBuilder<(), players_request_builder::SetHydrations<players_request_builder::SetSportId>> {
		Self::builder().sport_id(sport_id).hydrations(<() as Hydrations>::RequestData::default())
	}
}

impl<H: PersonHydrations> Display for PlayersRequest<H> {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "http://statsapi.mlb.com/api/v1/sports/{}/players{}", self.sport_id, gen_params! { "sportId": self.sport_id, "season"?: self.season })
	}
}

impl<H: PersonHydrations> RequestURL for PlayersRequest<H> {
	type Response = PeopleResponse<H>;
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::TEST_YEAR;
	use crate::request::RequestURLBuilderExt;

	#[tokio::test]
	#[cfg_attr(not(feature = "_heavy_tests"), ignore)]
	async fn parse_all_players_all_seasons_mlb() {
		for season in 1876..=TEST_YEAR {
			let _response = PlayersRequest::for_sport(SportId::MLB).season(season).build_and_get().await.unwrap();
		}
	}

	#[tokio::test]
	async fn parse_all_players_mlb() {
		let _response = PlayersRequest::for_sport(SportId::MLB).build_and_get().await.unwrap();
	}
}
