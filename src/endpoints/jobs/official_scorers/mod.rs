use crate::endpoints::jobs::JobsResponse;
use crate::endpoints::StatsAPIUrl;
use std::fmt::{Display, Formatter};

pub struct JobsOfficialScorersEndpoint {}

impl Display for JobsOfficialScorersEndpoint {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "http://statsapi.mlb.com/api/v1/jobs/officialScorers")
    }
}

impl StatsAPIUrl for JobsOfficialScorersEndpoint {
	type Response = JobsResponse;
}
