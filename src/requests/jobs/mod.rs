//! Jobs data.
//! 
//! Lists of different people with different positions

use crate::person::{JerseyNumber, NamedPerson};
use crate::{Copyright, MLB_API_DATE_FORMAT};
use bon::Builder;
use chrono::NaiveDate;
use serde::Deserialize;
use serde_with::{serde_as, DefaultOnError};
use std::fmt::{Display, Formatter};
use crate::meta::JobTypeId;
use crate::request::RequestURL;
use crate::sport::SportId;

mod datacasters;
mod official_scorers;
mod umpire;

pub use datacasters::*;
pub use official_scorers::*;
pub use umpire::*;

/// Response from the `jobs` endpoints.
/// Returns a [`Vec`] of [`EmployedPerson`]
///
/// Example: <http://statsapi.mlb.com/api/v1/jobs?jobType=UMPR>
///
/// This type represents the response for:
/// - [`JobsRequest`]
/// - [`JobsDatacastersRequest`]
/// - [`JobsOfficialScorersRequest`]
/// - [`JobsUmpiresRequest`]
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct JobsResponse {
    pub copyright: Copyright,
    #[serde(default)]
    pub roster: Vec<EmployedPerson>,
}

/// Person with a job
///
/// Wrapper of [`NamedPerson`] used in the [`JobsRequest`] endpoints that contains extra fields about their job.
#[serde_as]
#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EmployedPerson {
    #[serde(default = "NamedPerson::unknown_person")]
    pub person: NamedPerson,
	#[serde(default)]
	#[serde_as(deserialize_as = "DefaultOnError")]
    pub jersey_number: Option<JerseyNumber>,
    #[serde(rename = "job")] pub job_name: String,
    pub job_id: JobTypeId,
    #[serde(rename = "title")] pub job_title: String,
}

/// Returns [`JobsResponse`].
///
/// Example: <http://statsapi.mlb.com/api/v1/jobs?jobType=UMPR>
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
    use crate::meta::JobType;
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
