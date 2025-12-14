use derive_more::From;
use mlb_api_proc::{EnumDeref, EnumDerefMut, EnumTryAs, EnumTryAsMut, EnumTryInto};
use serde::Deserialize;

string_id!(LogicalEventId);

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
pub struct IdentifiableLogicalEvent {
	#[serde(rename = "code")] pub id: LogicalEventId,
}

#[derive(Debug, Deserialize, Eq, Clone, From, EnumTryAs, EnumTryAsMut, EnumTryInto, EnumDeref, EnumDerefMut)]
#[serde(untagged)]
pub enum LogicalEvent {
	Identifiable(IdentifiableLogicalEvent),
}

impl LogicalEvent {
	#[must_use]
	pub fn try_into_hydrated(self) -> Option<IdentifiableLogicalEvent> {
		self.try_into_identifiable()
	}
}

id_only_eq_impl!(LogicalEvent, id);
meta_kind_impl!("logicalEvents" => LogicalEvent);
tiered_request_entry_cache_impl!(LogicalEvent => IdentifiableLogicalEvent; id: LogicalEventId);
test_impl!(LogicalEvent);
