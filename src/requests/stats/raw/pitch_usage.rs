use serde::Deserialize;
use crate::pitch_types::PitchTypeId;
use crate::stats::SingletonSplitStat;
use crate::stats::units::PercentageStat;

#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(from = "__PitchUsageStruct")]
pub struct PitchUsage {
    pub count: u32,
    pub total_pitches: u32,
    pub average_speed: uom::si::f64::Velocity,
    pub pitch_type: PitchTypeId,
}

impl Default for PitchUsage {
    fn default() -> Self {
        Self {
            count: 0,
            total_pitches: 0,
            average_speed: uom::si::f64::Velocity::new::<uom::si::velocity::mile_per_hour>(0.0),
            pitch_type: PitchTypeId::new("4SFB"),
        }
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct __PitchUsageStruct {
    count: u32,
    total_pitches: u32,
    average_speed: f64,
    pitch_type: PitchTypeId,
}

impl From<__PitchUsageStruct> for PitchUsage {
    fn from(value: __PitchUsageStruct) -> Self {
        Self {
            count: value.count,
            total_pitches: value.total_pitches,
            average_speed: uom::si::f64::Velocity::new::<uom::si::velocity::mile_per_hour>(value.average_speed),
            pitch_type: value.pitch_type,
        }
    }
}

impl Eq for PitchUsage {}

impl SingletonSplitStat for PitchUsage {}

impl PitchUsage {
    /// Percentage of total pitches that are this pitch.
    #[must_use]
    pub fn pct(&self) -> PercentageStat {
        (self.count as f64 / self.total_pitches as f64).into()
    }
}
