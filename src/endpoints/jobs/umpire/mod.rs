use crate::endpoints::jobs::JobsResponse;
use crate::endpoints::sports::SportId;
use crate::endpoints::StatsAPIUrl;
use crate::gen_params;
use crate::types::MLB_API_DATE_FORMAT;
use chrono::NaiveDate;
use std::fmt::{Display, Formatter};

// pub mod games; // needs private mlb-only api key -- absolutely not going to implement this.

pub struct JobsUmpiresEndpointUrl {
    pub sport_id: Option<SportId>,
    pub date: Option<NaiveDate>,
}

impl Display for JobsUmpiresEndpointUrl {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "http://statsapi.mlb.com/api/v1/jobs/umpires{}", gen_params! { "sportId"?: self.sport_id, "date"?: self.date.as_ref().map(|date| date.format(MLB_API_DATE_FORMAT)) })
    }
}

impl StatsAPIUrl for JobsUmpiresEndpointUrl {
	type Response = JobsResponse;
}
