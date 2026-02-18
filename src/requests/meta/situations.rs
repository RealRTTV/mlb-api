use serde::Deserialize;

#[derive(Copy, Clone, Default, PartialEq, Eq)]
pub enum SituationCodeFilter {
	/// Display results that match <u>all</u> the [`SituationCode`]s selected.
	All,

	/// Display results that match <u>any</u> the [`SituationCode`]s selected.
	#[default]
	Any,
}

id!(SituationCodeId { code: String });

#[allow(clippy::struct_excessive_bools, reason = "false positive")]
#[derive(Debug, Deserialize, Clone)]
pub struct SituationCode {
	#[serde(rename = "navigationMenu", default)]
	pub navigation_menu_kind: Option<String>,
	pub description: String,
	#[serde(rename = "team")]
	pub is_team_active: bool,
	#[serde(rename = "batting")]
	pub is_batting_active: bool,
	#[serde(rename = "fielding")]
	pub is_fielding_active: bool,
	#[serde(rename = "pitching")]
	pub is_pitching_active: bool,
	#[serde(flatten)]
	pub id: SituationCodeId,
}

id_only_eq_impl!(SituationCode, id);
meta_kind_impl!("situationCodes" => SituationCode);
tiered_request_entry_cache_impl!(SituationCode.id: SituationCodeId);
test_impl!(SituationCode);
