use crate::person::NamedPerson;
use crate::types::{Copyright, MLB_API_DATE_FORMAT};
use bon::Builder;
use chrono::NaiveDate;
use serde::Deserialize;
use std::fmt::{Display, Formatter};
use crate::job_types::JobTypeId;
use crate::request::RequestURL;
use crate::sport::SportId;

pub mod datacasters;
pub mod official_scorers;
pub mod umpire;

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
pub struct JobsResponse {
    pub copyright: Copyright,
    #[serde(default)]
    pub roster: Vec<EmployedPerson>,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EmployedPerson {
    #[serde(default = "NamedPerson::unknown_person")]
    pub person: NamedPerson,
    #[serde(deserialize_with = "crate::types::try_from_str")]
    pub jersey_number: Option<u8>,
    #[serde(rename = "job")] pub job_name: String,
    pub job_id: JobTypeId,
    #[serde(rename = "title")] pub job_title: String,
}

#[derive(Builder)]
#[builder(derive(Into))]
pub struct JobsRequest {
    #[builder(into)]
    job_type: JobTypeId,
    #[builder(into, default)]
    sport_id: SportId,
    date: Option<NaiveDate>,
}

impl<S: jobs_request_builder::State + jobs_request_builder::IsComplete> crate::request::RequestURLBuilderExt for JobsRequestBuilder<S> {
    type Built = JobsRequest;
}

impl Display for JobsRequest {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "http://statsapi.mlb.com/api/v1/jobs{}", gen_params! {
            "jobType": self.job_type,
            "sportId": self.sport_id,
            "date"?: self.date.as_ref().map(|date| date.format(MLB_API_DATE_FORMAT))
        })
    }
}

impl RequestURL for JobsRequest {
	type Response = JobsResponse;
}

#[cfg(test)]
mod tests {
    use crate::job_types::JobType;
    use crate::jobs::JobsRequest;
    use crate::meta::MetaRequest;
    use crate::request::{RequestURL, RequestURLBuilderExt};

    #[tokio::test]
    async fn parse_all_job_types() {
        let job_types = MetaRequest::<JobType>::new().get().await.unwrap().entries;
        for job_type in job_types {
            let _response = JobsRequest::builder().job_type(job_type.id.clone()).build_and_get().await.unwrap();
        }
    }
}
