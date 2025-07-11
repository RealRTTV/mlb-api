use crate::endpoints::meta::kinds::MetaKind;
use derive_more::Display;
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Eq, Copy, Clone, Display)]
#[serde(try_from = "__HitTrajectoryStruct")]
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
struct __HitTrajectoryStruct {
	code: String,
}

impl TryFrom<__HitTrajectoryStruct> for HitTrajectory {
	type Error = &'static str;

	fn try_from(value: __HitTrajectoryStruct) -> Result<Self, Self::Error> {
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
