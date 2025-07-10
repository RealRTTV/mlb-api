use derive_more::Display;
use serde::Deserialize;
use crate::endpoints::meta::kinds::MetaKind;

#[derive(Debug, Deserialize, PartialEq, Eq, Copy, Clone, Display)]
#[serde(try_from = "HitTrajectoryStruct")]
pub enum HitTrajectory {
    #[display("Bunt - Ground Ball")]
    BuntGrounder,

    #[display("Bunt - Popup")]
    BuntPopup,

    #[display("Bunt - Line Drive")]
    BuntLineDrive,

    #[display("Line Drive")]
    LineDrive,

    #[display("Ground Ball")]
    GroundBall,

    #[display("Fly Ball")]
    FlyBall,

    #[display("Popup")]
    Popup,
}

#[derive(Deserialize)]
struct HitTrajectoryStruct {
    code: String,
}

impl TryFrom<HitTrajectoryStruct> for HitTrajectory {
    type Error = &'static str;

    fn try_from(value: HitTrajectoryStruct) -> Result<Self, Self::Error> {
        Ok(match &*value.code {
            "bunt_grounder" => HitTrajectory::BuntGrounder,
            "bunt_popup" => HitTrajectory::BuntPopup,
            "bunt_line_drive" => HitTrajectory::BuntLineDrive,
            "line_drive" => HitTrajectory::LineDrive,
            "ground_ball" => HitTrajectory::GroundBall,
            "fly_ball" => HitTrajectory::FlyBall,
            "popup" => HitTrajectory::Popup,
            _ => return Err("unknown hit trajectory"),
        })
    }
}

impl MetaKind for HitTrajectory {
    const ENDPOINT_NAME: &'static str = "hitTrajectories";
}
