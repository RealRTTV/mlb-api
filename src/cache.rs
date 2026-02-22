//! The caching feature of `mlb-api`
//!
//! Some requests (especially [`MetaRequest`]s) are great candidates for a cache since their contents do not change.
//!
//! Because of this, many types implement [`Requestable`] and [`RequestableEntrypoint`] such that they can be accessed easily from within the code.
//!
//! Types like [`NamedPosition`](crate::NamedPosition) can benefit even more than [`Person`] due to their ability to be cached more aggressively.
//! By enabling the `aggressive_cache` feature, and or calling [`precache`] at the start of your `main` fn. You can cache these values in advance to make their lookups extremely fast.
//! 
//! Note that even without the `cache` feature, some of this module is still accessible, making requests just... not cache, and instead act as another lookup.
//! 
//! # Examples
//! ```
//! use mlb_api::person::PersonId;
//!
//! let person: PersonId = 660_271.into();
//! // dbg!(&person.full_name); // person.full_name does not exist
//!
//! let person: Arc<Person> = person.as_complete_or_request().await.unwrap();
//! dbg!(&person.full_name);
//! ```
//! 
//! ```
//! use mlb_api::meta::NamedPosition;
//!
//! let position: NamedPosition = NamedPosition { ..Default::default() }; // very common type to see
//!
//! let position: Arc<Position> = position.as_complete_or_request().await.unwrap();
//! dbg!(&position.short_name);
//! ```

use crate::RwLock;
use crate::meta::MetaRequest;
use crate::request::{RequestURL, RequestURLBuilderExt};
use fxhash::FxBuildHasher;
use serde::de::DeserializeOwned;
use std::collections::HashMap;
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::sync::Arc;
use thiserror::Error;
use crate::person::Person;
use crate::person::players::PlayersRequest;
use crate::sport::SportId;

/// A type that can be requested via a URL, such as a [`Position`], [`Award`], or [`Team`].
///
/// [`Position`]: crate::meta::Position
/// [`Award`]: crate::awards::Award
/// [`Team`]: crate::team::Team
pub trait Requestable: 'static + Send + Sync + DeserializeOwned + Debug + Clone + Eq {
    type Identifier: Clone + Eq + Hash + Display + Sync + Debug;
    type URL: RequestURL;

    fn id(&self) -> &Self::Identifier;

    fn url_for_id(id: &Self::Identifier) -> Self::URL;

    fn get_entries(response: <Self::URL as RequestURL>::Response) -> impl IntoIterator<Item = Self> where Self: Sized;

    #[cfg(feature = "cache")]
    fn get_cache_table() -> &'static RwLock<CacheTable<Self>> where Self: Sized;
}

/// A type in which it can be [`as_complete_or_request`](RequestableEntrypoint::as_complete_or_request)ed into it's [`Complete`](RequestableEntrypoint::Complete) type.
pub trait RequestableEntrypoint {
    type Complete: Requestable;

    fn id(&self) -> &<<Self as RequestableEntrypoint>::Complete as Requestable>::Identifier;

    #[cfg(feature = "reqwest")]
    #[cfg(feature = "cache")]
    fn as_complete_or_request(&self) -> impl Future<Output = Result<Arc<<Self as RequestableEntrypoint>::Complete>, Error<Self>>>
    where
        Self: Sized,
    { async {
        let cache_lock = <<Self as RequestableEntrypoint>::Complete as Requestable>::get_cache_table();
        let id = self.id();
        let cache = cache_lock.read().await;
        if let Some(complete_entry) = cache.get(id).cloned() {
            return Ok(complete_entry);
        }
        drop(cache);

        let mut cache = cache_lock.write().await;
        cache.request_and_add(id).await?;
        cache.get(id).cloned().ok_or_else(|| Error::NoMatchingVariant(id.clone()))
    } }

    #[cfg(feature = "ureq")]
    #[cfg(feature = "cache")]
    fn as_complete_or_request(&self) -> Result<Arc<<Self as RequestableEntrypoint>::Complete>, Error<Self>>
    where
        Self: Sized
    {
        let cache_lock = <<Self as RequestableEntrypoint>::Complete as Requestable>::get_cache_table();
        let id = self.id();
        let cache = cache_lock.read();
        if let Some(complete_entry) = cache.get(id).cloned() {
            return Ok(complete_entry);
        }

        let mut cache = cache_lock.write();
        cache.request_and_add(id)?;
        cache.get(id).cloned().ok_or_else(|| Error::NoMatchingVariant(id.clone()))
    }

    #[cfg(feature = "reqwest")]
    #[cfg(not(feature = "cache"))]
    fn as_complete_or_request(&self) -> impl Future<Output = Result<<Self as RequestableEntrypoint>::Complete, Error<Self>>>
    where
        Self: Sized,
    { async {
        let id = self.id();
        let url = <Self::Complete as Requestable>::url_for_id(id).to_string();
        let response: <<Self::Complete as Requestable>::URL as RequestURL>::Response = crate::request::get::<<<<Self as RequestableEntrypoint>::Complete as Requestable>::URL as RequestURL>::Response>(url).await?;
        let entries = <Self::Complete as Requestable>::get_entries(response);
        entries.into_iter().next().ok_or_else(|| Error::<Self>::NoMatchingVariant(id.clone()))
    } }

    #[cfg(feature = "ureq")]
    #[cfg(not(feature = "cache"))]
    fn as_complete_or_request(&self) -> Result<Arc<<Self as RequestableEntrypoint>::Complete>, Error<Self>> {
        let id = self.id();
        let url = <Self::Complete as Requestable>::url_for_id(id).to_string();
        let response = crate::request::get::<<<Self::Complete as Requestable>::URL as RequestURL>::Response>(&url)?;
        let entries = <Self::Complete as Requestable>::get_entries(response);
        entries.into_iter().next().ok_or_else(|| Error::<Self>::NoMatchingVariant(id.clone()))
    }
}

/// Type representing the cached values of `T`; stored as `static` using [`Arc<RwLock<_>>`]
///
/// underlying structure is an [`FxHashMap`](fxhash::FxHashMap).
#[cfg(feature = "cache")]
pub struct CacheTable<T: Requestable> {
    cached_values: HashMap<T::Identifier, Arc<T>, FxBuildHasher>,
}

/// Errors for [`as_complete_or_request`](RequestableEntrypoint::as_complete_or_request) calls.
#[derive(Debug, Error)]
pub enum Error<T: RequestableEntrypoint> {
    #[error(transparent)]
    Url(#[from] crate::request::Error),
    #[error("No matching entry was found for id {0}")]
    NoMatchingVariant(<T::Complete as Requestable>::Identifier),
}

#[cfg(feature = "cache")]
impl<T: Requestable> CacheTable<T> {
    #[allow(clippy::new_without_default, reason = "needs to be const")]
    #[must_use]
    pub const fn new() -> Self {
        Self {
            cached_values: HashMap::with_hasher(FxBuildHasher::new()),
        }
    }

    #[must_use]
    pub fn get(&self, id: &T::Identifier) -> Option<&Arc<T>> {
        self.cached_values.get(id)
    }

    pub fn insert(&mut self, value: T) {
        self.cached_values.insert(value.id().clone(), Arc::new(value));
    }
    
    pub fn clear(&mut self) {
        self.cached_values.clear();
    }
    
    pub fn add_entries(&mut self, entries: impl IntoIterator<Item = T>) {
        for entry in entries {
            self.insert(entry);
        }
    }

    /// # Errors
    /// See variants of [`crate::request::Error`]
    #[cfg(feature = "reqwest")]
    pub async fn request_and_add(&mut self, id: &T::Identifier) -> Result<(), crate::request::Error> {
        let url = <T as Requestable>::url_for_id(id).to_string();
        let response = crate::request::get::<<<T as Requestable>::URL as RequestURL>::Response>(url).await?;
        self.add_entries(<T as Requestable>::get_entries(response));
        Ok(())
    }

    #[cfg(feature = "ureq")]
    pub fn request_and_add(&mut self, id: &T::Identifier) -> Result<(), crate::request::Error> {
        let url = <T as Requestable>::url_for_id(id).to_string();
        let response = crate::request::get::<<<T as Requestable>::URL as RequestURL>::Response>(url)?;
        self.add_entries(<T as Requestable>::get_entries(response));
        Ok(())
    }
}

/// Caches popular types for [`Requestable`] use.
///
/// # Errors
/// See variants of [`crate::request::Error`]
#[cfg(feature = "cache")]
#[cfg(feature = "reqwest")]
pub async fn precache() -> Result<(), crate::request::Error> {
    let people_response = PlayersRequest::for_sport(SportId::MLB).build_and_get();
    
    let award_response = crate::awards::AwardRequest::builder().build_and_get();
    let division_response = crate::divisions::DivisionsRequest::builder().build_and_get();
    let conference_response = crate::conferences::ConferencesRequest::builder().build_and_get();
    let venue_response = crate::venue::VenuesRequest::builder().build_and_get();
    let league_response = crate::league::LeaguesRequest::builder().build_and_get();
    let sport_response = crate::sport::SportsRequest::builder().build_and_get();
    <crate::awards::Award as Requestable>::get_cache_table().write().await.add_entries(award_response.await?.awards);
    <crate::divisions::Division as Requestable>::get_cache_table().write().await.add_entries(division_response.await?.divisions);
    <crate::conferences::Conference as Requestable>::get_cache_table().write().await.add_entries(conference_response.await?.conferences);
    <crate::venue::Venue as Requestable>::get_cache_table().write().await.add_entries(venue_response.await?.venues);
    <crate::league::League as Requestable>::get_cache_table().write().await.add_entries(league_response.await?.leagues);
    <crate::sport::Sport as Requestable>::get_cache_table().write().await.add_entries(sport_response.await?.sports);
    
    <crate::meta::BaseballStat as Requestable>::get_cache_table().write().await.add_entries(MetaRequest::<crate::meta::BaseballStat>::new().get().await?.entries);
    <crate::meta::JobType as Requestable>::get_cache_table().write().await.add_entries(MetaRequest::<crate::meta::JobType>::new().get().await?.entries);
    <crate::meta::EventType as Requestable>::get_cache_table().write().await.add_entries(MetaRequest::<crate::meta::EventType>::new().get().await?.entries);
    <crate::meta::GameStatus as Requestable>::get_cache_table().write().await.add_entries(MetaRequest::<crate::meta::GameStatus>::new().get().await?.entries);
    <crate::meta::Metric as Requestable>::get_cache_table().write().await.add_entries(MetaRequest::<crate::meta::Metric>::new().get().await?.entries);
    <crate::meta::PitchCode as Requestable>::get_cache_table().write().await.add_entries(MetaRequest::<crate::meta::PitchCode>::new().get().await?.entries);
    <crate::meta::PitchType as Requestable>::get_cache_table().write().await.add_entries(MetaRequest::<crate::meta::PitchType>::new().get().await?.entries);
    <crate::meta::Platform as Requestable>::get_cache_table().write().await.add_entries(MetaRequest::<crate::meta::Platform>::new().get().await?.entries);
    <crate::meta::Position as Requestable>::get_cache_table().write().await.add_entries(MetaRequest::<crate::meta::Position>::new().get().await?.entries);
    <crate::meta::ReviewReason as Requestable>::get_cache_table().write().await.add_entries(MetaRequest::<crate::meta::ReviewReason>::new().get().await?.entries);
    <crate::meta::ScheduleEventType as Requestable>::get_cache_table().write().await.add_entries(MetaRequest::<crate::meta::ScheduleEventType>::new().get().await?.entries);
    <crate::meta::SituationCode as Requestable>::get_cache_table().write().await.add_entries(MetaRequest::<crate::meta::SituationCode>::new().get().await?.entries);
    <crate::meta::SkyDescription as Requestable>::get_cache_table().write().await.add_entries(MetaRequest::<crate::meta::SkyDescription>::new().get().await?.entries);
    <crate::meta::GameType as Requestable>::get_cache_table().write().await.add_entries(MetaRequest::<crate::meta::GameType>::new().get().await?.entries);
    <crate::meta::GameType as Requestable>::get_cache_table().write().await.add_entries(MetaRequest::<crate::meta::GameType>::new().get().await?.entries);
    <crate::meta::WindDirection as Requestable>::get_cache_table().write().await.add_entries(MetaRequest::<crate::meta::WindDirection>::new().get().await?.entries);

    <Person as Requestable>::get_cache_table().write().await.add_entries(people_response.await?.people.into_iter().map(Person::Ballplayer));

    Ok(())
}

/// Caches popular types for [`Requestable`] use.
///
/// # Errors
/// See variants of [`crate::request::Error`]
#[cfg(feature = "cache")]
#[cfg(feature = "ureq")]
pub fn precache() -> Result<(), crate::request::Error> {
    <crate::awards::Award as Requestable>::get_cache_table().write().add_entries(crate::awards::AwardRequest::builder().build_and_get()?.awards);
    <crate::divisions::Division as Requestable>::get_cache_table().write().add_entries(crate::divisions::DivisionsRequest::builder().build_and_get()?.divisions);
    <crate::conferences::Conference as Requestable>::get_cache_table().write().add_entries(crate::conferences::ConferencesRequest::builder().build_and_get()?.conferences);
    <crate::venue::Venue as Requestable>::get_cache_table().write().add_entries(crate::venue::VenuesRequest::builder().build_and_get()?.venues);
    <crate::league::League as Requestable>::get_cache_table().write().add_entries(crate::league::LeaguesRequest::builder().build_and_get()?.leagues);
    <crate::sport::Sport as Requestable>::get_cache_table().write().add_entries(crate::sport::SportsRequest::builder().build_and_get()?.sports);

    <crate::meta::BaseballStat as Requestable>::get_cache_table().write().add_entries(MetaRequest::<crate::meta::BaseballStat>::new().get()?.entries);
    <crate::meta::JobType as Requestable>::get_cache_table().write().add_entries(MetaRequest::<crate::meta::JobType>::new().get()?.entries);
    <crate::meta::EventType as Requestable>::get_cache_table().write().add_entries(MetaRequest::<crate::meta::EventType>::new().get()?.entries);
    <crate::meta::GameStatus as Requestable>::get_cache_table().write().add_entries(MetaRequest::<crate::meta::GameStatus>::new().get()?.entries);
    <crate::meta::Metric as Requestable>::get_cache_table().write().add_entries(MetaRequest::<crate::meta::Metric>::new().get()?.entries);
    <crate::meta::PitchCode as Requestable>::get_cache_table().write().add_entries(MetaRequest::<crate::meta::PitchCode>::new().get()?.entries);
    <crate::meta::PitchType as Requestable>::get_cache_table().write().add_entries(MetaRequest::<crate::meta::PitchType>::new().get()?.entries);
    <crate::meta::Platform as Requestable>::get_cache_table().write().add_entries(MetaRequest::<crate::meta::Platform>::new().get()?.entries);
    <crate::meta::Position as Requestable>::get_cache_table().write().add_entries(MetaRequest::<crate::meta::Position>::new().get()?.entries);
    <crate::meta::ReviewReason as Requestable>::get_cache_table().write().add_entries(MetaRequest::<crate::meta::ReviewReason>::new().get()?.entries);
    <crate::meta::ScheduleEventType as Requestable>::get_cache_table().write().add_entries(MetaRequest::<crate::meta::ScheduleEventType>::new().get()?.entries);
    <crate::meta::SituationCode as Requestable>::get_cache_table().write().add_entries(MetaRequest::<crate::meta::SituationCode>::new().get()?.entries);
    <crate::meta::SkyDescription as Requestable>::get_cache_table().write().add_entries(MetaRequest::<crate::meta::SkyDescription>::new().get()?.entries);
    <crate::meta::GameType as Requestable>::get_cache_table().write().add_entries(MetaRequest::<crate::meta::GameType>::new().get()?.entries);
    <crate::meta::GameType as Requestable>::get_cache_table().write().await.add_entries(MetaRequest::<crate::meta::GameType>::new().get().await?.entries);
    <crate::meta::WindDirection as Requestable>::get_cache_table().write().add_entries(MetaRequest::<crate::meta::WindDirection>::new().get()?.entries);

    <crate::person::Person as Requestable>::get_cache_table().write().add_entries(PlayersRequest::for_sport(SportId::MLB).build_and_get()?.people);

    Ok(())
}
