use std::ops::{Deref, DerefMut};
use derive_more::{Deref, DerefMut, From};
use serde::Deserialize;
use crate::endpoints::meta::MetaKind;

#[derive(Debug, Deserialize, Deref, DerefMut, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HydratedPitchCode {
    pub description: String,
    #[serde(rename = "swingStatus")] pub has_swing: bool,
    #[serde(rename = "swingMissStatus")] pub is_whiff: bool,
    #[serde(rename = "swingContactStatus")] pub swing_made_contact: bool,
    #[serde(rename = "strikeStatus")] pub is_strike: bool,
    #[serde(rename = "ballStatus")] pub is_ball: bool,
    #[serde(rename = "pitchStatus")] pub is_pitch: bool,
    pub pitch_result_text: String,
    #[serde(rename = "buntAttemptStatus")] pub is_bunt_attempt: bool,
    #[serde(rename = "contactStatus")] pub made_contact: bool,

    #[deref]
    #[deref_mut]
    #[serde(flatten)]
    inner: IdentifiablePitchCode,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
pub struct IdentifiablePitchCode {
    pub code: String,
}

#[derive(Debug, Deserialize, Eq, Clone, From)]
#[serde(untagged)]
pub enum PitcherCode {
    Hydrated(HydratedPitchCode),
    Identifiable(IdentifiablePitchCode),
}

impl PartialEq for PitcherCode {
    fn eq(&self, other: &Self) -> bool {
        self.code == other.code
    }
}

impl Deref for PitcherCode {
    type Target = IdentifiablePitchCode;

    fn deref(&self) -> &Self::Target {
        match self {
            Self::Hydrated(inner) => inner,
            Self::Identifiable(inner) => inner,
        }
    }
}

impl DerefMut for PitcherCode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            Self::Hydrated(inner) => inner,
            Self::Identifiable(inner) => inner,
        }
    }
}

impl MetaKind for PitcherCode {
    const ENDPOINT_NAME: &'static str = "pitchCodes";
}
