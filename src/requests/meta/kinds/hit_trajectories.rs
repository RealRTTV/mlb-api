use crate::cache::{HydratedCacheTable, RequestEntryCache};
use crate::meta::kinds::MetaKind;
use crate::meta::MetaRequest;
use crate::StatsAPIRequestUrl;
use crate::{rwlock_const_new, RwLock};
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

impl MetaKind for HitTrajectory {
	const ENDPOINT_NAME: &'static str = "hitTrajectories";
}

static CACHE: RwLock<HydratedCacheTable<HitTrajectory>> = rwlock_const_new(HydratedCacheTable::new());

impl RequestEntryCache for HitTrajectory {
	type HydratedVariant = Self;
	type Identifier = Self;
	type URL = MetaRequest<Self>;

	fn into_hydrated_variant(self) -> Option<Self::HydratedVariant> {
		Some(self)
	}

	fn id(&self) -> &Self::Identifier {
		self
	}

	fn url_for_id(_id: &Self::Identifier) -> Self::URL {
		MetaRequest::new()
	}

	fn get_entries(response: <Self::URL as StatsAPIRequestUrl>::Response) -> impl IntoIterator<Item=Self>
	where
		Self: Sized
	{
		response.entries
	}

	fn get_hydrated_cache_table() -> &'static RwLock<HydratedCacheTable<Self>>
	where
		Self: Sized
	{
		&CACHE
	}
}

#[cfg(test)]
mod tests {
	use crate::meta::MetaRequest;
	use crate::StatsAPIRequestUrl;

	#[tokio::test]
	async fn parse_meta() {
		let _response = MetaRequest::<super::HitTrajectory>::new().get().await.unwrap();
	}
}
