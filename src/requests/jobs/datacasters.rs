use crate::MLB_API_DATE_FORMAT;
use bon::Builder;
use chrono::NaiveDate;
use std::fmt::{Display, Formatter};
use crate::jobs::JobsResponse;
use crate::request::{RequestURL, RequestURLBuilderExt};
use crate::sport::SportId;

/// Returns [`JobsResponse`]
///
/// This request can be replicated with [`JobsRequest`](super::JobsRequest) and a datacaster job type.
///
/// Example: <http://statsapi.mlb.com/api/v1/jobs/datacasters>
#[derive(Builder)]
#[builder(derive(Into))]
pub struct JobsDatacastersRequest {
    #[builder(into, default)]
    sport_id: SportId,
    date: Option<NaiveDate>,
}

impl Display for JobsDatacastersRequest {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "http://statsapi.mlb.com/api/v1/jobs/datacasters{}", gen_params! { "sportId": self.sport_id, "date"?: self.date.as_ref().map(|date| date.format(MLB_API_DATE_FORMAT)) })
    }
}

impl RequestURL for JobsDatacastersRequest {
    type Response = JobsResponse;
}

impl<S: jobs_datacasters_request_builder::State + jobs_datacasters_request_builder::IsComplete> RequestURLBuilderExt for JobsDatacastersRequestBuilder<S> {
    type Built = JobsDatacastersRequest;
}
