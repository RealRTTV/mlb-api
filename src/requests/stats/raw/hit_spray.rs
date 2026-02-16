use serde::Deserialize;
use crate::stats::RawStat;
use crate::stats::units::PercentageStat;

#[allow(clippy::struct_field_names, reason = "is a piece")]
#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename = "camelCase")]
pub struct HitSpray {
	left_field: PercentageStat,
	left_center_field: PercentageStat,
	center_field: PercentageStat,
	right_center_field: PercentageStat,
	right_field: PercentageStat,
}

impl Default for HitSpray {
	fn default() -> Self {
		Self {
			left_field: PercentageStat::new(0.0),
			left_center_field: PercentageStat::new(0.0),
			center_field: PercentageStat::new(0.0),
			right_center_field: PercentageStat::new(0.0),
			right_field: PercentageStat::new(0.0),
		}
	}
}

impl RawStat for HitSpray {}