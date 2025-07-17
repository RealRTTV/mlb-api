use crate::endpoints::meta::{MetaEndpointUrl, MetaKind};
use crate::endpoints::stat_groups::StatGroup;
use derive_more::{Deref, DerefMut, Display, From};
use serde::Deserialize;
use std::ops::{Deref, DerefMut};
use strum::EnumTryAs;
use crate::cache::{EndpointEntryCache, HydratedCacheTable};
use crate::{rwlock_const_new, RwLock};
use crate::endpoints::StatsAPIUrl;

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
	#[serde(rename = "metricId")]
	pub id: MetricId,
}

#[repr(transparent)]
#[derive(Debug, Deserialize, Deref, Display, PartialEq, Eq, Copy, Clone, Hash)]
pub struct MetricId(u32);

#[derive(Debug, Deserialize, Eq, Clone, From, EnumTryAs)]
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

static CACHE: RwLock<HydratedCacheTable<Metric>> = rwlock_const_new(HydratedCacheTable::new());

impl EndpointEntryCache for Metric {
	type HydratedVariant = HydratedMetric;
	type Identifier = MetricId;
	type URL = MetaEndpointUrl<Self>;

	fn into_hydrated_entry(self) -> Option<Self::HydratedVariant> {
		self.try_as_hydrated()
	}

	fn id(&self) -> &Self::Identifier {
		&self.id
	}

	fn url_for_id(_id: &Self::Identifier) -> Self::URL {
		MetaEndpointUrl::new()
	}

	fn get_entries(response: <Self::URL as StatsAPIUrl>::Response) -> impl IntoIterator<Item=Self>
	where
		Self: Sized
	{
		response.entries
	}

	fn get_hydrated_cache_table() -> &'static RwLock<HydratedCacheTable<Self>>
	where
		Self: Sized
	{
		&CACHE
	}
}

#[cfg(test)]
mod tests {
	use crate::endpoints::StatsAPIUrl;
	use crate::endpoints::meta::MetaEndpointUrl;

	#[tokio::test]
	async fn parse_meta() {
		let _response = MetaEndpointUrl::<super::Metric>::new().get().await.unwrap();
	}
}
