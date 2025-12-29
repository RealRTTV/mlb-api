use serde::Deserialize;

id!(EventTypeId { code: String });

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EventType {
	pub plate_appearance: bool,
	pub hit: bool,
	pub base_running_event: bool,
	pub description: String,
	#[serde(flatten)]
	pub id: EventTypeId,
}

id_only_eq_impl!(EventType, id);
meta_kind_impl!("eventTypes" => EventType);
tiered_request_entry_cache_impl!(EventType.id: EventTypeId);
test_impl!(EventType);
