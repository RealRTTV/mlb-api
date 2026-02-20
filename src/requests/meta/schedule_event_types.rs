use serde::Deserialize;

id!(#[doc = "A [`String`] code representing a [`ScheduleEventType`]"] ScheduleEventTypeId { code: String });

/// A detailed `struct` representing an event in the schedule.
/// ```
/// ScheduleEventType {
///     name: "All-Star Weekend Event".into(),
///     id: "A".into(),
/// }
/// ```
#[derive(Debug, Deserialize, Clone)]
pub struct ScheduleEventType {
	pub name: String,
	#[serde(flatten)]
	pub id: ScheduleEventTypeId,
}

id_only_eq_impl!(ScheduleEventType, id);
meta_kind_impl!("scheduleEventTypes" => ScheduleEventType);
tiered_request_entry_cache_impl!(ScheduleEventType.id: ScheduleEventTypeId);
test_impl!(ScheduleEventType);