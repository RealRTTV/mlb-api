use derive_more::{Deref, DerefMut, From};
use mlb_api_proc::{EnumDeref, EnumDerefMut, EnumTryAs, EnumTryAsMut, EnumTryInto};
use serde::Deserialize;

string_id!(EventTypeId);

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IdentifiableEventType {
	#[serde(rename = "code")] pub id: EventTypeId,
}

#[derive(Debug, Deserialize, Deref, DerefMut, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HydratedEventType {
	plate_appearance: bool,
	hit: bool,
	base_running_event: bool,
	description: String,

	#[deref]
	#[deref_mut]
	#[serde(flatten)]
	inner: IdentifiableEventType,
}

#[derive(Debug, Deserialize, Eq, Clone, From, EnumTryAs, EnumTryAsMut, EnumTryInto, EnumDeref, EnumDerefMut)]
#[serde(untagged)]
pub enum EventType {
	Hydrated(HydratedEventType),
	Identifiable(IdentifiableEventType),
}

id_only_eq_impl!(EventType, id);
meta_kind_impl!("eventTypes" => EventType);
tiered_request_entry_cache_impl!(EventType => HydratedEventType; id: EventTypeId);
test_impl!(EventType);
