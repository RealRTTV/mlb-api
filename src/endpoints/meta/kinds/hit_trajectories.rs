use crate::endpoints::meta::kinds::MetaKind;
use derive_more::Display;
use serde::Deserialize;
use crate::cache::{EndpointEntryCache, HydratedCacheTable};
use crate::{rwlock_const_new, RwLock};
use crate::endpoints::meta::MetaEndpointUrl;
use crate::endpoints::StatsAPIUrl;

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

static CACHE: RwLock<HydratedCacheTable<HitTrajectory>> = rwlock_const_new(HydratedCacheTable::new());

impl EndpointEntryCache for HitTrajectory {
	type HydratedVariant = HitTrajectory;
	type Identifier = HitTrajectory;
	type URL = MetaEndpointUrl<Self>;

	fn into_hydrated_entry(self) -> Option<Self::HydratedVariant> {
		Some(self)
	}

	fn id(&self) -> &Self::Identifier {
		self
	}

	fn url_for_id(_id: &Self::Identifier) -> Self::URL {
		MetaEndpointUrl::new()
	}

	fn get_entries(response: <Self::URL as StatsAPIUrl>::Response) -> impl IntoIterator<Item=Self>
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
	use crate::endpoints::StatsAPIUrl;
	use crate::endpoints::meta::MetaEndpointUrl;

	#[tokio::test]
	async fn parse_meta() {
		let _response = MetaEndpointUrl::<super::HitTrajectory>::new().get().await.unwrap();
	}
}
