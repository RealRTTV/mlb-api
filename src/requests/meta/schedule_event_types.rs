use derive_more::{Deref, DerefMut, From};
use mlb_api_proc::{EnumDeref, EnumDerefMut, EnumTryAs, EnumTryAsMut, EnumTryInto};
use serde::Deserialize;

string_id!(ScheduleEventTypeId);

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
pub struct IdentifiableScheduleEventType {
	#[serde(rename = "code")] pub id: ScheduleEventTypeId,
}

#[derive(Debug, Deserialize, Deref, DerefMut, PartialEq, Eq, Clone)]
pub struct HydratedScheduleEventType {
	pub name: String,

	#[deref]
	#[deref_mut]
	#[serde(flatten)]
	inner: IdentifiableScheduleEventType,
}

#[derive(Debug, Deserialize, Eq, Clone, From, EnumTryAs, EnumTryAsMut, EnumTryInto, EnumDeref, EnumDerefMut)]
#[serde(untagged)]
pub enum ScheduleEventType {
	Hydrated(HydratedScheduleEventType),
	Identifiable(IdentifiableScheduleEventType),
}

id_only_eq_impl!(ScheduleEventType, id);
meta_kind_impl!("scheduleEventTypes" => ScheduleEventType);
tiered_request_entry_cache_impl!(ScheduleEventType => HydratedScheduleEventType; id: ScheduleEventTypeId);
test_impl!(ScheduleEventType);