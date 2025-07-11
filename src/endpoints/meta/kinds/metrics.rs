use std::ops::{Deref, DerefMut};
use derive_more::{Deref, DerefMut, Display, From};
use serde::Deserialize;
use uom::si::f64 as si;
use crate::endpoints::meta::MetaKind;
use crate::endpoints::meta::stat_groups::StatGroup;

macro_rules! units {
    ($($name:ident($ty:ty)),+ $(,)?) => {
        #[derive(Debug, ::serde::Deserialize, PartialEq, Clone)]
        #[serde(try_from = "__UnitStruct")]
        pub enum Unit {
            $($name ( $ty ),)+
            Unknown(String),
        }

        impl Eq for Unit {}

        #[derive(::serde::Deserialize)]
        struct __UnitStruct {
            inner: String,
        }

        impl TryFrom<__UnitStruct> for Unit {
            type Error = ::uom::str::ParseQuantityError;

            fn try_from(value: __UnitStruct) -> Result<Self, Self::Error> {
                let __UnitStruct { inner } = value;

                $(match ::core::str::FromStr::from_str(&*inner) {
                    Ok(result) => return Ok(Self::$name(result)),
                    Err(::uom::str::ParseQuantityError::UnknownUnit) => {},
                    Err(e) => return Err(e),
                })+

                Ok(Self::Unknown(inner))
            }
        }
    };
}

units! {
    AngularVelocity(si::AngularVelocity),
    Length(si::Length),
    Velocity(si::Velocity),
    Angle(si::Angle),
    Time(si::Time),
}

#[derive(Debug, Deserialize, Deref, DerefMut, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HydratedMetric {
    #[serde(deserialize_with = "crate::types::deserialize_comma_seperated_vec")]
    pub group: Vec<StatGroup>,
    pub unit: Unit,

    #[deref]
    #[deref_mut]
    #[serde(flatten)]
    inner: NamedMetric,
}

#[derive(Debug, Deserialize, Deref, DerefMut, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NamedMetric {
    pub name: String,

    #[deref]
    #[deref_mut]
    #[serde(flatten)]
    inner: IdentifiableMetric,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IdentifiableMetric {
    pub id: MetricId,
}

#[repr(transparent)]
#[derive(Debug, Deserialize, Deref, Display, PartialEq, Eq, Copy, Clone)]
pub struct MetricId(u32);

#[derive(Debug, Deserialize, Eq, Clone, From)]
#[serde(untagged)]
pub enum Metric {
    Hydrated(HydratedMetric),
    Named(NamedMetric),
    Identifiable(IdentifiableMetric),
}

impl PartialEq for Metric {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Deref for Metric {
    type Target = IdentifiableMetric;

    fn deref(&self) -> &Self::Target {
        match self {
            Self::Hydrated(inner) => inner,
            Self::Named(inner) => inner,
            Self::Identifiable(inner) => inner,
        }
    }
}

impl DerefMut for Metric {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            Self::Hydrated(inner) => inner,
            Self::Named(inner) => inner,
            Self::Identifiable(inner) => inner,
        }
    }
}

impl MetaKind for Metric {
    const ENDPOINT_NAME: &'static str = "metrics";
}
