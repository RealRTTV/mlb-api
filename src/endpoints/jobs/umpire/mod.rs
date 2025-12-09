use crate::gen_params;
use crate::jobs::JobsResponse;
use crate::sports::SportId;
use crate::types::MLB_API_DATE_FORMAT;
use crate::StatsAPIEndpointUrl;
use bon::Builder;
use chrono::NaiveDate;
use std::fmt::{Display, Formatter};
// pub mod games; // needs private mlb-only api key -- absolutely not going to implement this.

#[derive(Builder)]
pub struct JobsUmpiresEndpoint {
    #[builder(into)]
    sport_id: Option<SportId>,
    date: Option<NaiveDate>,
}

impl Display for JobsUmpiresEndpoint {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "http://statsapi.mlb.com/api/v1/jobs/umpires{}", gen_params! { "sportId"?: self.sport_id, "date"?: self.date.as_ref().map(|date| date.format(MLB_API_DATE_FORMAT)) })
    }
}

impl StatsAPIEndpointUrl for JobsUmpiresEndpoint {
	type Response = JobsResponse;
}
