use serde::{Deserialize, de::IgnoredAny};
use derive_more::{Deref, DerefMut};
use serde_with::{serde_as, DefaultOnError};
use uuid::Uuid;

use crate::{Handedness, game::SituationCount, meta::{EventType, PitchCodeId, PitchTypeId}, stats::RawStat};

#[derive(Debug, Deserialize, PartialEq, Clone, Deref, DerefMut)]
pub struct PlayStat {
	pub play: PitchStatData,
}

impl RawStat for PlayStat {}

#[derive(Debug, Deserialize, PartialEq, Clone, Deref, DerefMut)]
pub struct PitchStat {
	pub play: PitchStatData,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PitchStatData {
	pub details: PitchStatDetails,
	pub count: SituationCount,
	pub play_id: Option<Uuid>,
	/// Ordinal of pitch in the AB; starts at 1.
	#[serde(rename = "pitchNumber")]
	pub pitch_ordinal: usize,
	/// Ordinal of the AB in a game; starts at 1.
    #[serde(rename = "atBatNumber")]
	pub at_bat_ordinal: usize,

	#[doc(hidden)]
	#[serde(rename = "isPitch", default)]
	pub __is_pitch: IgnoredAny,
}

#[serde_as]
#[allow(clippy::struct_excessive_bools, reason = "incorrect")]
#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PitchStatDetails {
	pub call: PitchCodeId,
	#[serde_as(deserialize_as = "DefaultOnError")]
	#[serde(rename = "eventType")]
	pub event: Option<EventType>,
	pub is_in_play: bool,
	pub is_strike: bool,
	pub is_ball: bool,
	pub is_base_hit: bool,
	pub is_at_bat: bool,
	pub is_plate_appearance: bool,
	#[serde(rename = "type")]
	pub pitch_type: Option<PitchTypeId>,
	pub pitch_hand: Handedness,
	pub bat_side: Handedness,

	#[doc(hidden)]
	#[serde(rename = "event", default)]
	pub __event: IgnoredAny,
}

impl RawStat for PitchStat {}

#[cfg(test)]
mod tests {
    use crate::single_stat;

    #[tokio::test]
    async fn parse_pitch_log() {
        let _ = single_stat!(PitchLog + Hitting for 660_271; with |builder| builder.season(2025)).await.unwrap();
        let _ = single_stat!(PitchLog + Pitching for 660_271; with |builder| builder.season(2025)).await.unwrap();
    }
    
    #[tokio::test]
    async fn parse_play_log() {
        let _ = single_stat!(PlayLog + Hitting for 660_271; with |builder| builder.season(2025)).await.unwrap();
        let _ = single_stat!(PlayLog + Pitching for 660_271; with |builder| builder.season(2025)).await.unwrap();
    }
}

