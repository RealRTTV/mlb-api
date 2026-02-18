use crate::sport::SportId;
use crate::request::{RequestURL, RequestURLBuilderExt};
use bon::Builder;
use derive_more::{Deref, DerefMut};
use itertools::Itertools;
use serde::Deserialize;
use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};
use crate::cache::Requestable;
use crate::season::{Season, SeasonState};

#[cfg(feature = "cache")]
use crate::{rwlock_const_new, RwLock, cache::CacheTable};

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LeagueResponse {
	pub copyright: String,
	pub leagues: Vec<League>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NamedLeague {
	pub name: String,
	#[serde(flatten)]
	pub id: LeagueId,
}

impl Hash for NamedLeague {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.id.hash(state);
	}
}

#[allow(clippy::struct_excessive_bools, reason = "false positive")]
#[derive(Debug, Deserialize, Deref, DerefMut, Clone)]
#[serde(rename_all = "camelCase")]
pub struct League {
	pub abbreviation: String,
	#[serde(rename = "nameShort")]
	pub short_name: Option<String>,
	#[serde(rename = "orgCode")]
	pub code: String,
	pub season_state: SeasonState,
	#[serde(flatten, deserialize_with = "bad_league_season_schema_deserializer")]
	#[serde(rename = "seasonDateInfo")]
	pub season: Season,
	#[serde(default)]
	pub has_split_season: bool,
	pub num_games: u8,
	pub has_playoff_points: Option<bool>,
	pub num_teams: u8,
	pub num_wildcard_teams: Option<u8>,
	#[serde(rename = "conferencesInUse")]
	pub has_conferences: bool,
	#[serde(rename = "divisionsInUse")]
	pub has_divisions: bool,
	pub sport: Option<SportId>,
	pub active: bool,

	#[deref]
	#[deref_mut]
	#[serde(flatten)]
	inner: NamedLeague,
}

// this is annoying me that it exists
#[derive(Deserialize)]
struct BadLeagueSeasonSchema {
	#[serde(rename = "hasWildCard")]
	has_wildcard: bool,
	#[serde(rename = "seasonDateInfo")]
	rest: Season,
}

fn bad_league_season_schema_deserializer<'de, D: serde::Deserializer<'de>>(deserializer: D) -> Result<Season, D::Error> {
	let BadLeagueSeasonSchema { has_wildcard, mut rest } = BadLeagueSeasonSchema::deserialize(deserializer)?;
	rest.has_wildcard = has_wildcard;
	Ok(rest)
}

id!(LeagueId { id: u32 });
id_only_eq_impl!(League, id);
id_only_eq_impl!(NamedLeague, id);

impl NamedLeague {
	#[must_use]
	pub(crate) fn unknown_league() -> Self {
		Self {
			name: "null".to_owned(),
			id: LeagueId::new(0),
		}
	}

	#[must_use]
	pub fn is_unknown(&self) -> bool {
		*self.id == 0
	}
}

#[derive(Builder)]
#[builder(derive(Into))]
pub struct LeaguesRequest {
	#[builder(into)]
	sport_id: Option<SportId>,
	#[builder(setters(vis = "", name = league_ids_internal))]
	league_ids: Option<Vec<LeagueId>>,
}

impl<S: leagues_request_builder::State + leagues_request_builder::IsComplete> RequestURLBuilderExt for LeaguesRequestBuilder<S> {
	type Built = LeaguesRequest;
}

impl<S: leagues_request_builder::State> LeaguesRequestBuilder<S> {
	#[allow(dead_code, reason = "could be used by the end user")]
	pub fn league_ids<T: Into<LeagueId>>(self, league_ids: Vec<T>) -> LeaguesRequestBuilder<leagues_request_builder::SetLeagueIds<S>> where S::LeagueIds: leagues_request_builder::IsUnset {
		self.league_ids_internal(league_ids.into_iter().map(T::into).collect::<Vec<_>>())
	}
}

impl Display for LeaguesRequest {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "http://statsapi.mlb.com/api/v1/leagues{}", gen_params! {
			"sportId"?: self.sport_id,
			"leagueIds"?: self.league_ids.as_ref().map(|ids| ids.iter().copied().join(",")),
		})
	}
}

impl RequestURL for LeaguesRequest {
	type Response = LeagueResponse;
}

#[cfg(feature = "cache")]
static CACHE: RwLock<CacheTable<League>> = rwlock_const_new(CacheTable::new());

impl Requestable for League {
	type Identifier = LeagueId;
	type URL = LeaguesRequest;

	fn id(&self) -> &Self::Identifier {
		&self.id
	}

	#[cfg(feature = "aggressive_cache")]
	fn url_for_id(_id: &Self::Identifier) -> Self::URL {
		LeaguesRequest::builder().build()
	}

	#[cfg(not(feature = "aggressive_cache"))]
	fn url_for_id(id: &Self::Identifier) -> Self::URL {
		LeaguesRequest::builder().league_ids_internal(vec![*id]).build()
	}

	fn get_entries(response: <Self::URL as RequestURL>::Response) -> impl IntoIterator<Item=Self>
	where
		Self: Sized
	{
		response.leagues
	}

	#[cfg(feature = "cache")]
	fn get_cache_table() -> &'static RwLock<CacheTable<Self>>
	where
		Self: Sized
	{
		&CACHE
	}
}

entrypoint!(LeagueId => League);
entrypoint!(NamedLeague.id => League);
entrypoint!(League.id => League);
