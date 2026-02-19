use crate::jobs::JobsResponse;
use crate::request::RequestURL;
use bon::Builder;
use std::fmt::{Display, Formatter};

/// Returns [`JobsResponse`]
///
/// This request can be replicated with [`JobsRequest`](super::JobsRequest) and a datacaster job type.
///
/// Example: <http://statsapi.mlb.com/api/v1/jobs/officialScorers>
#[derive(Builder)]
#[builder(derive(Into))]
pub struct JobsOfficialScorersRequest {}

impl Display for JobsOfficialScorersRequest {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "http://statsapi.mlb.com/api/v1/jobs/officialScorers")
    }
}

impl RequestURL for JobsOfficialScorersRequest {
	type Response = JobsResponse;
}

impl<S: jobs_official_scorers_request_builder::State + jobs_official_scorers_request_builder::IsComplete> crate::request::RequestURLBuilderExt for JobsOfficialScorersRequestBuilder<S> {
    type Built = JobsOfficialScorersRequest;
}
