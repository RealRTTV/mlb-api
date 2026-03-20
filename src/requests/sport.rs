//! Different "sports"; MLB, AAA, AA, A+, A, Rookieball, etc.

use crate::Copyright;
use crate::request::RequestURL;
use bon::Builder;
use serde::Deserialize;
use std::fmt::{Display, Formatter};
use std::marker::PhantomData;
use crate::cache::{Requestable};
#[cfg(feature = "cache")]
use crate::{rwlock_const_new, RwLock, cache::CacheTable};
use crate::hydrations::Hydrations;

/// A [`Vec`] of [`Sport`]s.
#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase", bound = "H: SportsHydrations")]
pub struct SportsResponse<H: SportsHydrations> {
	pub copyright: Copyright,
	pub sports: Vec<Sport<H>>,
}

id!(#[doc = "A [`u32`] representing the ID of the sport; `SportId::MLB = 1`"] SportId { id: u32 });

impl SportId {
	pub const MLB: Self = Self::new(1);
}

impl Default for SportId {
	fn default() -> Self {
		Self::MLB
	}
}

/// Returns a [`SportsResponse`]
#[derive(Builder)]
#[builder(derive(Into))]
pub struct SportsRequest<H: SportsHydrations> {
	#[builder(into)]
	id: Option<SportId>,
	#[builder(skip)]
	_marker: PhantomData<H>,
}

impl<H: SportsHydrations, S: sports_request_builder::State + sports_request_builder::IsComplete> crate::request::RequestURLBuilderExt for SportsRequestBuilder<H, S> {
	type Built = SportsRequest<H>;
}

impl<H: SportsHydrations> Display for SportsRequest<H> {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		let hydrations = Some(H::hydration_text(&())).filter(|s| !s.is_empty());
		write!(f, "http://statsapi.mlb.com/api/v1/sports{}", gen_params! { "sportId"?: self.id, "hydrate"?: hydrations })
	}
}

impl<H: SportsHydrations> RequestURL for SportsRequest<H> {
	type Response = SportsResponse<H>;
}

/// A detailed `struct` representing information about a sport (or Organized Baseball Level)
///
/// ## Examples
/// ```no_run
/// Sport {
///     code: "mlb",
///     name: "Major League Baseball",
///     abbreviation: "MLB",
///     active: true,
///     id: 1,
/// }
/// ```
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase", bound = "H: SportsHydrations")]
pub struct Sport<H: SportsHydrations = ()> {
	pub code: String,
	pub name: String,
	pub abbreviation: String,
	#[serde(rename = "activeStatus")]
	pub active: bool,
	#[serde(flatten)]
	pub id: SportId,
	#[serde(flatten)]
	pub extras: H,
}

impl<H: SportsHydrations> PartialEq for Sport<H> {
	fn eq(&self, other: &Self) -> bool {
		self.id == other.id
	}
}

pub trait SportsHydrations: Hydrations<RequestData=()> {}

impl SportsHydrations for () {}

/// Creates hydrations for a sport
///
/// ## Examples
/// ```no_run
/// sports_hydrations! {
///     pub struct TestHydrations {
///         season,
///     }
/// }
///
/// let response = SportsRequest::<TestHydrations>::builder().build_and_get().await.unwrap();
/// for sport in response.sports {
///     dbg!(&sport.extras.season_date_info);
/// }
/// ```
///
/// ## Sport Hydrations
/// | Name                         | Type           |
/// |------------------------------|----------------|
/// | `season`                     | [`Season`]     |
///
/// [`Season`]: crate::season::Season
#[macro_export]
macro_rules! sports_hydrations {
	(@ inline_structs [$field:ident $(: $value:ty)? $(, $($t:tt)*)?] $vis:vis struct $name:ident { $($field_tt:tt)* }) => {
		$crate::sports_hydrations! { @ inline_structs [$($($t)*)?] $vis struct $name { $($field_tt)* $field $(: $value)?, } }
	};
	(@ inline_structs [$(,)?] $vis:vis struct $name:ident { $($t:tt)* }) => {
		$crate::sports_hydrations! { @ actual $vis struct $name { $($t)* } }
	};
	(@ actual $vis:vis struct $name:ident {
		$(season $season_comma:tt)?
	}) => {
		#[derive(::core::fmt::Debug, ::serde::Deserialize, ::core::cmp::PartialEq, ::core::clone::Clone)]
		#[serde(rename_all = "camelCase")]
		$vis struct $name {
			$(pub season_date_info: $crate::season::Season $season_comma)?
		}

		impl $crate::sport::SportsHydrations for $name {}

		impl $crate::hydrations::Hydrations for $name {
			type RequestData = ();

			fn hydration_text((): &Self::RequestData) -> ::std::borrow::Cow<'static, str> {
				::std::borrow::Cow::Borrowed(::std::concat!(
					$("season," $season_comma)?
				))
			}
		}
	};
    ($vis:vis struct $name:ident { $($t:tt)* }) => {
	    $crate::sports_hydrations! { @ inline_structs [$($t)*] $vis struct $name {} }
    };
}

#[cfg(feature = "cache")]
static CACHE: RwLock<CacheTable<Sport>> = rwlock_const_new(CacheTable::new());

impl Requestable for Sport {
	type Identifier = SportId;
	type URL = SportsRequest<()>;

	fn id(&self) -> &Self::Identifier {
		&self.id
	}

	#[cfg(feature = "aggressive_cache")]
	fn url_for_id(_id: &Self::Identifier) -> Self::URL {
		SportsRequest::builder().build()
	}

	#[cfg(not(feature = "aggressive_cache"))]
	fn url_for_id(id: &Self::Identifier) -> Self::URL {
		SportsRequest::builder().id(*id).build()
	}

	fn get_entries(response: <Self::URL as RequestURL>::Response) -> impl IntoIterator<Item=Self>
	where
		Self: Sized
	{
		response.sports
	}

	#[cfg(feature = "cache")]
	fn get_cache_table() -> &'static RwLock<CacheTable<Self>>
	where
		Self: Sized
	{
		&CACHE
	}
}

entrypoint!(SportId => Sport);
entrypoint!(Sport.id => Sport);

#[cfg(test)]
mod tests {
	use super::*;
	use crate::request::RequestURLBuilderExt;

	#[tokio::test]
	async fn parse_all_sports() {
		let _result = SportsRequest::<()>::builder().build_and_get().await.unwrap();
	}

	#[tokio::test]
	async fn parse_all_sports_with_hydrations() {
		sports_hydrations! {
			pub struct TestHydrations {
				season
			}
		}

		let _result = SportsRequest::<TestHydrations>::builder().build_and_get().await.unwrap();
	}
}
