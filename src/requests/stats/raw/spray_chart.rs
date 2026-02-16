use serde::Deserialize;
use crate::person::NamedPerson;
use crate::stats::RawStat;
use crate::stats::raw::HitSpray;

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename = "camelCase")]
pub struct SprayChart {
	#[serde(rename = "stat")]
	spray: HitSpray,
	batter: NamedPerson,
}

impl Default for SprayChart {
	fn default() -> Self {
		Self {
			spray: HitSpray::default(),
			batter: NamedPerson::unknown_person(),
		}
	}
}

impl RawStat for SprayChart {}
