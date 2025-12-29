use serde::Deserialize;

id!(ScheduleEventTypeId { code: String });

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