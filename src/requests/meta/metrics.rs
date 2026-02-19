use crate::meta::stat_groups::StatGroup;
use derive_more::{Deref, DerefMut};
use serde::Deserialize;

id!(MetricId { metricId: u32 });

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

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NamedMetric {
	pub name: String,
	#[serde(flatten)]
	pub id: MetricId,
}

#[derive(Debug, Deserialize, Deref, DerefMut, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Metric {
	#[serde(default)]
	#[serde(rename = "group", deserialize_with = "crate::deserialize_comma_separated_vec")]
	pub groups: Vec<StatGroup>,
	pub unit: Option<Unit>,

	#[deref]
	#[deref_mut]
	#[serde(flatten)]
	inner: NamedMetric,
}

id_only_eq_impl!(NamedMetric, id);
id_only_eq_impl!(Metric, id);
meta_kind_impl!("metrics" => Metric);
tiered_request_entry_cache_impl!([Metric, NamedMetric].id: MetricId);
test_impl!(Metric);
