use crate::endpoints::meta::stat_groups::StatGroup;
use crate::endpoints::meta::MetaKind;
use derive_more::{Deref, DerefMut};
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
pub struct IdentifiableBaseballStat {
    #[serde(rename = "name")] pub id: String,
}

impl From<IdentifiableBaseballStat> for BaseballStat {
    fn from(value: IdentifiableBaseballStat) -> Self {
        #[cfg(feature = "static_baseball_stats")] {
            if let Ok(stat) = StaticBaseballStat::try_from(&*value.id) {
                return BaseballStat::Static(stat);
            }
        }

        BaseballStat::Identifiable(value)
    }
}

#[derive(Debug, Deserialize, Deref, DerefMut, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HydratedBaseballStat {
    lookup_param: Option<String>,
    is_counting: bool,
    label: Option<String>,
    stat_groups: Vec<StatGroup>,

    #[deref]
    #[deref_mut]
    #[serde(flatten)]
    inner: IdentifiableBaseballStat,
}

#[cfg(feature = "static_baseball_stats")]
use r#static::*;

#[cfg(feature = "static_baseball_stats")]
mod r#static {
    use serde::Deserialize;
    use crate::endpoints::meta::baseball_stats::IdentifiableBaseballStat;
    use crate::endpoints::meta::stat_groups::StatGroup;
    use crate::endpoints::StaticParseError;
    use mlb_api_proc::HttpCache;

    macro_rules! stat_group_from_literal {
        ("hitting") => {
            StatGroup::Hitting
        };
        ("pitching") => {
            StatGroup::Pitching
        };
        ("fielding") => {
            StatGroup::Fielding
        };
        ("catching") => {
            StatGroup::Catching
        };
        ("running") => {
            StatGroup::Running
        };
        ("game") => {
            StatGroup::Game
        };
        ("team") => {
            StatGroup::Team
        };
        ("streak") => {
            StatGroup::Streak
        };
    }

    macro_rules! generate {
       ({
            "name": $name:literal $(,)?
            $("lookupParam": $lookup_param:literal $(,)?)?
            "isCounting": $is_counting:ident $(,)?
            $("label": $label:literal $(,)?)?
            "statGroups": [
                $({
                    "displayName": $stat_group_name:tt
                } $(,)?)*
            ] $(,)?
            "orgTypes": [] $(,)?
            "highLowTypes": [] $(,)?
            "streakLevels": [] $(,)?
        }) => {
            StaticBaseballStat {
                id: $name,
                lookup_param: generate!(@ if $({ Some($lookup_param) })? else { None }),
                is_counting: $is_counting,
                label: generate!(@ if $({ Some($label) })? else { None }),
                stat_groups: &[$(stat_group_from_literal!($stat_group_name),)*]
            }
        };
        (@ if  else $default:block) => {
            $default
        };
        (@ if $value:block else $default:block) => {
            $value
        };
    }

    #[derive(Debug, Deserialize, PartialEq, Eq, Copy, Clone, HttpCache)]
    #[serde(try_from = "IdentifiableBaseballStat")]
    #[request(type = "GET", url = "http://statsapi.mlb.com/api/v1/baseballStats")]
    #[parse(macro = generate, variant_name = "name", type = StaticBaseballStat)]
    #[try_from(lifetimes = ['a], type = &'a str, field = "name")]
    pub struct StaticBaseballStat {
        pub id: &'static str,
        pub lookup_param: Option<&'static str>,
        pub is_counting: bool,
        pub label: Option<&'static str>,
        pub stat_groups: &'static [StatGroup],
    }
    
    impl TryFrom<IdentifiableBaseballStat> for StaticBaseballStat {
        type Error = StaticParseError<String>;

        fn try_from(value: IdentifiableBaseballStat) -> Result<Self, Self::Error> {
            Self::try_from(&*value.id).map_err(|_| StaticParseError::InvalidId(value.id))
        }
    }
}

#[derive(Debug, Deserialize, Eq, Clone)]
#[serde(untagged)]
pub enum BaseballStat {
    #[cfg(feature = "static_baseball_stats")]
    Static(StaticBaseballStat),
    Hydrated(HydratedBaseballStat),
    Identifiable(IdentifiableBaseballStat),
}

impl PartialEq for BaseballStat {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}

impl BaseballStat {
    #[must_use]
    pub fn id(&self) -> &str {
        match self {
            #[cfg(feature = "static_baseball_stats")]
            BaseballStat::Static(inner) => inner.id,
            BaseballStat::Hydrated(inner) => &inner.id,
            BaseballStat::Identifiable(inner) => &inner.id,
        }
    }
}

impl MetaKind for BaseballStat {
    const ENDPOINT_NAME: &'static str = "baseballStats";
}
