use derive_more::{Deref, DerefMut, From};
use mlb_api_proc::{EnumDeref, EnumDerefMut, EnumTryAs, EnumTryAsMut, EnumTryInto};
use serde::Deserialize;

string_id!(SituationCodeId);

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
pub struct IdentifiableSituationCode {
	#[serde(rename = "code")] pub id: SituationCodeId,
}

#[allow(clippy::struct_excessive_bools, reason = "false positive")]
#[derive(Debug, Deserialize, Deref, DerefMut, PartialEq, Eq, Clone)]
pub struct HydratedSituationCode {
	#[serde(rename = "navigationMenu")]
	pub navigation_menu_kind: String,
	pub description: String,
	#[serde(rename = "team")]
	pub is_team_active: bool,
	#[serde(rename = "batting")]
	pub is_batting_active: bool,
	#[serde(rename = "fielding")]
	pub is_fielding_active: bool,
	#[serde(rename = "pitching")]
	pub is_pitching_active: bool,

	#[deref]
	#[deref_mut]
	#[serde(flatten)]
	inner: IdentifiableSituationCode,
}

#[derive(Debug, Deserialize, Eq, Clone, From, EnumTryAs, EnumTryAsMut, EnumTryInto, EnumDeref, EnumDerefMut)]
#[serde(untagged)]
pub enum SituationCode {
	Hydrated(HydratedSituationCode),
	Identifiable(IdentifiableSituationCode),
}

id_only_eq_impl!(SituationCode, id);
meta_kind_impl!("situationCodes" => SituationCode);
tiered_request_entry_cache_impl!(SituationCode => HydratedSituationCode; id: SituationCodeId);
test_impl!(SituationCode);
