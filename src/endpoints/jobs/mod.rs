use std::fmt::{Display, Formatter};
use chrono::NaiveDate;
use serde::Deserialize;
use crate::endpoints::{JobTypeId, StatsAPIUrl};
use crate::endpoints::person::Person;
use crate::endpoints::sports::SportId;
use crate::gen_params;
use crate::types::{Copyright, MLB_API_DATE_FORMAT};

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

pub struct JobsEndpointUrl {
    pub job_type: JobTypeId,
    pub sport_id: Option<SportId>,
    pub date: Option<NaiveDate>,
}

impl Display for JobsEndpointUrl {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "http://statsapi.mlb.com/api/v1/jobs{}", gen_params! {
            "jobType": self.job_type,
            "sportId"?: self.sport_id,
            "date"?: self.date.as_ref().map(|date| date.format(MLB_API_DATE_FORMAT))
        })
    }
}

impl StatsAPIUrl for JobsEndpointUrl {
	type Response = JobsResponse;
}

#[cfg(test)]
mod tests {
    use crate::endpoints::{JobType, StatsAPIUrl};
    use crate::endpoints::jobs::JobsEndpointUrl;
    use crate::endpoints::meta::MetaEndpointUrl;

    #[tokio::test]
    async fn parse_all_job_types() {
        let job_types = MetaEndpointUrl::<JobType>::new().get().await.unwrap().entries;
        for job_type in job_types.into_iter() {
            let _response = JobsEndpointUrl { job_type: job_type.id.clone(), sport_id: None, date: None }.get().await.unwrap();
        }
    }
}
