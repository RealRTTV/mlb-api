use crate::jobs::JobsResponse;
use crate::types::MLB_API_DATE_FORMAT;
use crate::request::StatsAPIRequestUrl;
use bon::Builder;
use chrono::NaiveDate;
use std::fmt::{Display, Formatter};
use crate::sport::SportId;

// pub mod games; // needs private mlb-only api key -- absolutely not going to implement this.

#[derive(Builder)]
pub struct JobsUmpiresRequest {
    #[builder(into, default)]
    sport_id: SportId,
    date: Option<NaiveDate>,
}

impl Display for JobsUmpiresRequest {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "http://statsapi.mlb.com/api/v1/jobs/umpires{}", gen_params! { "sportId": self.sport_id, "date"?: self.date.as_ref().map(|date| date.format(MLB_API_DATE_FORMAT)) })
    }
}

impl StatsAPIRequestUrl for JobsUmpiresRequest {
	type Response = JobsResponse;
}
