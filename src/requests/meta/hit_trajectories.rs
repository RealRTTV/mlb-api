use derive_more::Display;
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Eq, Copy, Clone, Display, Hash)]
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
#[doc(hidden)]
struct __HitTrajectoryStruct {
	code: String,
}

impl TryFrom<__HitTrajectoryStruct> for HitTrajectory {
	type Error = &'static str;

	fn try_from(value: __HitTrajectoryStruct) -> Result<Self, Self::Error> {
		Ok(match &*value.code {
			"bunt_grounder" => Self::BuntGrounder,
			"bunt_popup" => Self::BuntPopup,
			"bunt_line_drive" => Self::BuntLineDrive,
			"line_drive" => Self::LineDrive,
			"ground_ball" => Self::GroundBall,
			"fly_ball" => Self::FlyBall,
			"popup" => Self::Popup,
			_ => return Err("unknown hit trajectory"),
		})
	}
}

meta_kind_impl!("hitTrajectories" => HitTrajectory);
static_request_entry_cache_impl!(HitTrajectory);
test_impl!(HitTrajectory);
