use crate::jobs::JobsResponse;
use crate::StatsAPIEndpointUrl;
use bon::Builder;
use std::fmt::{Display, Formatter};

#[derive(Builder)]
pub struct JobsOfficialScorersEndpoint {}

impl Display for JobsOfficialScorersEndpoint {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "http://statsapi.mlb.com/api/v1/jobs/officialScorers")
    }
}

impl StatsAPIEndpointUrl for JobsOfficialScorersEndpoint {
	type Response = JobsResponse;
}
