use crate::gen_params;
use crate::person::Person;
use crate::sports::SportId;
use crate::types::{Copyright, MLB_API_DATE_FORMAT};
use crate::{JobTypeId, StatsAPIEndpointUrl};
use bon::Builder;
use chrono::NaiveDate;
use serde::Deserialize;
use std::fmt::{Display, Formatter};

pub mod datacasters; // done
pub mod official_scorers; // done
pub mod umpire; // done

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
pub struct JobsResponse {
    pub copyright: Copyright,
    #[serde(default)]
    pub roster: Vec<EmployedPerson>,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EmployedPerson {
    pub person: Person,
    #[serde(deserialize_with = "crate::types::try_from_str")]
    pub jersey_number: Option<u8>,
    #[serde(rename = "job")] pub job_name: String,
    pub job_id: JobTypeId,
    #[serde(rename = "title")] pub job_title: String,
}

#[derive(Builder)]
#[builder(derive(Into))]
pub struct JobsEndpoint {
    #[builder(into)]
    job_type: JobTypeId,
    #[builder(into)]
    sport_id: Option<SportId>,
    date: Option<NaiveDate>,
}

impl<S: jobs_endpoint_builder::State> crate::endpoints::links::StatsAPIEndpointUrlBuilderExt for JobsEndpointBuilder<S> where S: jobs_endpoint_builder::IsComplete {
    type Built = JobsEndpoint;
}

impl Display for JobsEndpoint {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "http://statsapi.mlb.com/api/v1/jobs{}", gen_params! {
            "jobType": self.job_type,
            "sportId"?: self.sport_id,
            "date"?: self.date.as_ref().map(|date| date.format(MLB_API_DATE_FORMAT))
        })
    }
}

impl StatsAPIEndpointUrl for JobsEndpoint {
	type Response = JobsResponse;
}

#[cfg(test)]
mod tests {
    use crate::jobs::JobsEndpoint;
    use crate::meta::MetaEndpoint;
    use crate::{JobType, StatsAPIEndpointUrl, StatsAPIEndpointUrlBuilderExt};

    #[tokio::test]
    async fn parse_all_job_types() {
        let job_types = MetaEndpoint::<JobType>::new().get().await.unwrap().entries;
        for job_type in job_types.into_iter() {
            let _response = JobsEndpoint::builder().job_type(job_type.id.clone()).build_and_get().await.unwrap();
        }
    }
}
