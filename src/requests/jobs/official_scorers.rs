use crate::jobs::JobsResponse;
use crate::request::StatsAPIRequestUrl;
use bon::Builder;
use std::fmt::{Display, Formatter};

#[derive(Builder)]
pub struct JobsOfficialScorersRequest {}

impl Display for JobsOfficialScorersRequest {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "http://statsapi.mlb.com/api/v1/jobs/officialScorers")
    }
}

impl StatsAPIRequestUrl for JobsOfficialScorersRequest {
	type Response = JobsResponse;
}
