use crate::sports::SportId;
use crate::gen_params;
use crate::types::MLB_API_DATE_FORMAT;
use chrono::NaiveDate;
use std::fmt::{Display, Formatter};

pub struct JobsDatacastersEndpoint {
    pub sport_id: Option<SportId>,
    pub date: Option<NaiveDate>,
}

impl Display for JobsDatacastersEndpoint {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "http://statsapi.mlb.com/api/v1/jobs/datacasters{}", gen_params! { "sportId"?: self.sport_id, "date"?: self.date.as_ref().map(|date| date.format(MLB_API_DATE_FORMAT)) })
    }
}
