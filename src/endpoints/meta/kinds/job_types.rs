use derive_more::{Deref, DerefMut};
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IdentifiableJobType {
    pub code: String,
}

impl From<IdentifiableJobType> for JobType {
    fn from(value: IdentifiableJobType) -> Self {
        #[cfg(feature = "static_job_types")] {
            if let Ok(inner) = StaticJobType::try_from(&*value.code) {
                return JobType::Static(inner)
            }
        }
        
        JobType::Identifiable(value)
    }
}

#[derive(Debug, Deserialize, Deref, DerefMut, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HydratedJobType {
    pub job: String,
    
    #[deref]
    #[deref_mut]
    #[serde(flatten)]
    inenr: IdentifiableJobType,
}

impl From<HydratedJobType> for JobType {
    fn from(value: HydratedJobType) -> Self {
        JobType::Hydrated(value)
    }
}

#[cfg(feature = "static_job_types")]
use r#static::*;
use crate::endpoints::meta::kinds::MetaKind;

#[cfg(feature = "static_job_types")]
mod r#static {
    use serde::Deserialize;
    use mlb_api_proc::HttpCache;
    use crate::endpoints::meta::job_types::{IdentifiableJobType, JobType};
    use crate::endpoints::StaticParseError;

    macro_rules! generate {
        ({
            "code": $code:literal,
            "job": $job:literal
            $(, "sortOrder": $_0:literal)?
        }) => {
            StaticJobType {
                code: $code,
                job: $job,
            }
        };
    }

    #[derive(Debug, Deserialize, PartialEq, Eq, Copy, Clone, HttpCache)]
    #[serde(try_from = "IdentifiableJobType")]
    #[request(type = "GET", url = "http://statsapi.mlb.com/api/v1/jobTypes")]
    #[parse(macro = generate, variant_name = "job", type = StaticJobType)]
    #[try_from(lifetimes = ['a], type = &'a str, field = "code")]
    pub struct StaticJobType {
        pub code: &'static str,
        pub job: &'static str,
    }
    
    impl TryFrom<IdentifiableJobType> for StaticJobType {
        type Error = StaticParseError<String>;

        fn try_from(value: IdentifiableJobType) -> Result<Self, Self::Error> {
            Self::try_from(&*value.code).map_err(|_| StaticParseError::InvalidId(value.code))
        }
    }
    
    impl From<StaticJobType> for JobType {
        fn from(value: StaticJobType) -> Self {
            JobType::Static(value)
        }
    }
}

#[derive(Debug, Deserialize, Eq, Clone)]
#[serde(untagged)]
pub enum JobType {
    #[cfg(feature = "static_job_types")]
    Static(StaticJobType),
    Hydrated(HydratedJobType),
    Identifiable(IdentifiableJobType),
}

impl JobType {
    #[must_use]
    pub fn code(&self) -> &str {
        match self {
            #[cfg(feature = "static_job_types")]
            Self::Static(inner) => inner.code,
            Self::Hydrated(inner) => &inner.code,
            Self::Identifiable(inner) => &inner.code,
        }
    }
}

impl PartialEq for JobType {
    fn eq(&self, other: &Self) -> bool {
        self.code() == other.code()
    }
}

impl MetaKind for JobType {
    const ENDPOINT_NAME: &'static str = "jobTypes";
}
