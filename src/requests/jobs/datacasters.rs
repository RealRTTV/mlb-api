use crate::types::MLB_API_DATE_FORMAT;
use bon::Builder;
use chrono::NaiveDate;
use std::fmt::{Display, Formatter};
use crate::sports::SportId;

#[derive(Builder)]
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
