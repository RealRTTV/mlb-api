//! Tied games (?).

use crate::meta::GameType;
use crate::request::RequestURL;
use crate::schedule::{ScheduleHydrations, ScheduleResponse};
use crate::season::SeasonId;
use bon::Builder;
use itertools::Itertools;
use std::fmt::{Display, Formatter};
use std::marker::PhantomData;

#[derive(Builder)]
#[builder(derive(Into))]
pub struct ScheduleTiedRequest<H: ScheduleHydrations> {
	#[builder(into)]
	season: SeasonId,
	game_types: Option<Vec<GameType>>,
	#[builder(skip)]
	_marker: PhantomData<H>,
}

impl<H: ScheduleHydrations, S: schedule_tied_request_builder::State + schedule_tied_request_builder::IsComplete> crate::request::RequestURLBuilderExt for ScheduleTiedRequestBuilder<H, S> {
	type Built = ScheduleTiedRequest<H>;
}

impl<H: ScheduleHydrations> Display for ScheduleTiedRequest<H> {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		let hydrations = Some(H::hydration_text(&())).filter(|s| !s.is_empty());

		write!(
			f,
			"http://statsapi.mlb.com/api/v1/schedule/games/tied{params}",
			params = gen_params! {
				"season": self.season,
				"gameTypes"?: self.game_types.as_ref().map(|x| x.iter().map(|g| format!("{g:?}")).join(",")),
				"hydrate"?: hydrations,
			}
		)
	}
}

impl<H: ScheduleHydrations> RequestURL for ScheduleTiedRequest<H> {
	type Response = ScheduleResponse<H>;
}

#[cfg(test)]
mod tests {
	use crate::TEST_YEAR;
	use crate::request::RequestURLBuilderExt;
	use crate::schedule::tied::ScheduleTiedRequest;

	#[tokio::test]
	async fn test_one_season() {
		let _ = ScheduleTiedRequest::<()>::builder().season(TEST_YEAR).build_and_get().await.unwrap();
	}

	#[tokio::test]
	#[cfg_attr(not(feature = "_heavy_tests"), ignore)]
	async fn test_all_seasons() {
		for season in 1876..=TEST_YEAR {
			let _ = ScheduleTiedRequest::<()>::builder().season(season).build_and_get().await.unwrap();
		}
	}
}
