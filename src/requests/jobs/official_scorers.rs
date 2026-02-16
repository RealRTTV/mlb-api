use crate::jobs::JobsResponse;
use crate::request::RequestURL;
use bon::Builder;
use std::fmt::{Display, Formatter};

#[derive(Builder)]
pub struct JobsOfficialScorersRequest {}

impl Display for JobsOfficialScorersRequest {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "http://statsapi.mlb.com/api/v1/jobs/officialScorers")
    }
}

impl RequestURL for JobsOfficialScorersRequest {
	type Response = JobsResponse;
}
