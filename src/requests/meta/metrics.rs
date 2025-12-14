use crate::requests::meta::stat_groups::StatGroup;
use derive_more::{Deref, DerefMut, From};
use mlb_api_proc::{EnumDeref, EnumDerefMut, EnumTryAs, EnumTryAsMut, EnumTryInto};
use serde::Deserialize;

integer_id!(MetricId);

macro_rules! units {
    ($($name:ident($func:path => $units:ty)),+ $(,)?) => {
        #[derive(Debug, ::serde::Deserialize, Clone)]
        #[serde(try_from = "__UnitStruct")]
        pub enum Unit {
            $($name($units),)+
            Unknown(String),
        }

		impl PartialEq for Unit {
			fn eq(&self, other: &Self) -> bool {
				match (self, other) {
					// PartialEq trait doesn't exist yet, so we have to use the other stuff implemented here
					$((Self::$name(lhs), Self::$name(rhs)) => format!("{lhs:?}") == format!("{rhs:?}"),)+
					(Self::Unknown(lhs), Self::Unknown(rhs)) => lhs == rhs,
					_ => false,
				}
			}
		}

        impl Eq for Unit {}

        #[derive(::serde::Deserialize)]
        struct __UnitStruct(String);

        impl TryFrom<__UnitStruct> for Unit {
            type Error = ::uom::str::ParseQuantityError;

            fn try_from(value: __UnitStruct) -> Result<Self, Self::Error> {
                let __UnitStruct(inner) = value;

				$(
				for unit in $func() {
					let abbreviation = unit.abbreviation();
					if abbreviation.eq_ignore_ascii_case(&inner) {
						return Ok(Self::$name(unit));
					}
				}
				)+

                Ok(Self::Unknown(inner))
            }
        }
    };
}

units! {
	AngularVelocity(uom::si::angular_velocity::units => uom::si::angular_velocity::Units),
	Length(uom::si::length::units => uom::si::length::Units),
	Velocity(uom::si::velocity::units => uom::si::velocity::Units),
	Angle(uom::si::angle::units => uom::si::angle::Units),
	Time(uom::si::time::units => uom::si::time::Units),
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IdentifiableMetric {
	#[serde(rename = "metricId")]
	pub id: MetricId,
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

#[derive(Debug, Deserialize, Deref, DerefMut, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HydratedMetric {
	#[serde(deserialize_with = "crate::types::deserialize_comma_separated_vec")]
	pub group: Vec<StatGroup>,
	pub unit: Unit,

	#[deref]
	#[deref_mut]
	#[serde(flatten)]
	inner: NamedMetric,
}

#[derive(Debug, Deserialize, Eq, Clone, From, EnumTryAs, EnumTryAsMut, EnumTryInto, EnumDeref, EnumDerefMut)]
#[serde(untagged)]
pub enum Metric {
	Hydrated(HydratedMetric),
	Named(NamedMetric),
	Identifiable(IdentifiableMetric),
}

id_only_eq_impl!(Metric, id);
meta_kind_impl!("metrics" => Metric);
tiered_request_entry_cache_impl!(Metric => HydratedMetric; id: MetricId);
test_impl!(Metric);
