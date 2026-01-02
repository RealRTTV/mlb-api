use crate::{MetaRequest, RwLock};
use crate::request::{StatsAPIRequestUrl, StatsAPIRequestUrlBuilderExt};
use fxhash::FxBuildHasher;
use serde::de::DeserializeOwned;
use std::collections::HashMap;
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::sync::Arc;
use thiserror::Error;
use crate::person::Person;
use crate::person::players::PlayersRequest;

pub trait RequestEntryCache: 'static + Send + Sync + DeserializeOwned + Debug + Clone + Eq {
    type Identifier: Clone + Eq + Hash + Display + Sync + Debug;
    type URL: StatsAPIRequestUrl;

    fn id(&self) -> &Self::Identifier;

    fn url_for_id(id: &Self::Identifier) -> Self::URL;

    fn get_entries(response: <Self::URL as StatsAPIRequestUrl>::Response) -> impl IntoIterator<Item = Self> where Self: Sized;

    #[cfg(feature = "cache")]
    fn get_cache_table() -> &'static RwLock<CacheTable<Self>> where Self: Sized;
}

pub trait RequestEntryCacheEntrypoint {
    type Complete: RequestEntryCache;

    fn id(&self) -> &<<Self as RequestEntryCacheEntrypoint>::Complete as RequestEntryCache>::Identifier;

    #[cfg(feature = "reqwest")]
    #[cfg(feature = "cache")]
    fn as_complete_or_request(&self) -> impl Future<Output = Result<Arc<<Self as RequestEntryCacheEntrypoint>::Complete>, Error<Self>>>
    where
        Self: Sized,
    { async {
        let cache_lock = <<Self as RequestEntryCacheEntrypoint>::Complete as RequestEntryCache>::get_cache_table();
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
    fn as_complete_or_request(&self) -> Result<Arc<<Self as RequestEntryCacheEntrypoint>::Complete>, Error<Self>>
    where
        Self: Sized
    {
        let cache_lock = <<Self as RequestEntryCacheEntrypoint>::Complete as RequestEntryCache>::get_cache_table();
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
    fn as_complete_or_request(&self) -> impl Future<Output = Result<<Self as RequestEntryCacheEntrypoint>::Complete, Error<Self>>>
    where
        Self: Sized,
    { async {
        let id = self.id();
        let url = <Self::Complete as RequestEntryCache>::url_for_id(id).to_string();
        let response = crate::request::get::<<<Self::Complete as RequestEntryCache>::URL as StatsAPIRequestUrl>::Response>(url).await?;
        response
    } }

    #[cfg(feature = "ureq")]
    #[cfg(not(feature = "cache"))]
    fn as_complete_or_request(&self) -> Result<Arc<<Self as RequestEntryCacheEntrypoint>::Complete>, Error<Self>> {
        let id = self.id();
        let url = <Self::Complete as RequestEntryCache>::url_for_id(id).to_string();
        let response = crate::request::get::<<<Self::Complete as RequestEntryCache>::URL as StatsAPIRequestUrl>::Response>(url)?;
        response
    }
}

#[cfg(feature = "cache")]
pub struct CacheTable<T: RequestEntryCache> {
    cached_values: HashMap<T::Identifier, Arc<T>, FxBuildHasher>,
}

#[derive(Debug, Error)]
pub enum Error<T: RequestEntryCacheEntrypoint> {
    #[error(transparent)]
    Url(#[from] crate::request::Error),
    #[cfg(feature = "cache")]
    #[error("No matching entry was found for id {0}")]
    NoMatchingVariant(<T::Complete as RequestEntryCache>::Identifier),
}

#[cfg(feature = "cache")]
impl<T: RequestEntryCache> CacheTable<T> {
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
        let url = <T as RequestEntryCache>::url_for_id(id).to_string();
        let response = crate::request::get::<<<T as RequestEntryCache>::URL as StatsAPIRequestUrl>::Response>(url).await?;
        self.add_entries(<T as RequestEntryCache>::get_entries(response));
        Ok(())
    }

    #[cfg(feature = "ureq")]
    pub fn request_and_add(&mut self, id: &T::Identifier) -> Result<(), crate::request::Error> {
        let url = <T as RequestEntryCache>::url_for_id(id).to_string();
        let response = crate::request::get::<<<T as RequestEntryCache>::URL as StatsAPIRequestUrl>::Response>(url)?;
        self.add_entries(<T as RequestEntryCache>::get_entries(response));
        Ok(())
    }
}

/// # Errors
/// See variants of [`crate::request::Error`]
#[cfg(feature = "cache")]
#[cfg(feature = "reqwest")]
pub async fn precache() -> Result<(), crate::request::Error> {
    let people_response = PlayersRequest::builder().build_and_get();
    
    let award_response = crate::awards::AwardRequest::builder().build_and_get();
    let division_response = crate::divisions::DivisionsRequest::builder().build_and_get();
    let conference_response = crate::conferences::ConferencesRequest::builder().build_and_get();
    let venue_response = crate::venue::VenuesRequest::builder().build_and_get();
    let league_response = crate::league::LeaguesRequest::builder().build_and_get();
    let sport_response = crate::sport::SportsRequest::builder().build_and_get();
    <crate::awards::Award as RequestEntryCache>::get_cache_table().write().await.add_entries(award_response.await?.awards);
    <crate::divisions::Division as RequestEntryCache>::get_cache_table().write().await.add_entries(division_response.await?.divisions);
    <crate::conferences::Conference as RequestEntryCache>::get_cache_table().write().await.add_entries(conference_response.await?.conferences);
    <crate::venue::Venue as RequestEntryCache>::get_cache_table().write().await.add_entries(venue_response.await?.venues);
    <crate::league::League as RequestEntryCache>::get_cache_table().write().await.add_entries(league_response.await?.leagues);
    <crate::sport::Sport as RequestEntryCache>::get_cache_table().write().await.add_entries(sport_response.await?.sports);
    
    <crate::baseball_stats::BaseballStat as RequestEntryCache>::get_cache_table().write().await.add_entries(MetaRequest::<crate::baseball_stats::BaseballStat>::new().get().await?.entries);
    <crate::job_types::JobType as RequestEntryCache>::get_cache_table().write().await.add_entries(MetaRequest::<crate::job_types::JobType>::new().get().await?.entries);
    <crate::event_types::EventType as RequestEntryCache>::get_cache_table().write().await.add_entries(MetaRequest::<crate::event_types::EventType>::new().get().await?.entries);
    <crate::game_status::GameStatus as RequestEntryCache>::get_cache_table().write().await.add_entries(MetaRequest::<crate::game_status::GameStatus>::new().get().await?.entries);
    <crate::metrics::Metric as RequestEntryCache>::get_cache_table().write().await.add_entries(MetaRequest::<crate::metrics::Metric>::new().get().await?.entries);
    <crate::pitch_codes::PitchCode as RequestEntryCache>::get_cache_table().write().await.add_entries(MetaRequest::<crate::pitch_codes::PitchCode>::new().get().await?.entries);
    <crate::pitch_types::PitchType as RequestEntryCache>::get_cache_table().write().await.add_entries(MetaRequest::<crate::pitch_types::PitchType>::new().get().await?.entries);
    <crate::platforms::Platform as RequestEntryCache>::get_cache_table().write().await.add_entries(MetaRequest::<crate::platforms::Platform>::new().get().await?.entries);
    <crate::positions::Position as RequestEntryCache>::get_cache_table().write().await.add_entries(MetaRequest::<crate::positions::Position>::new().get().await?.entries);
    <crate::review_reasons::ReviewReason as RequestEntryCache>::get_cache_table().write().await.add_entries(MetaRequest::<crate::review_reasons::ReviewReason>::new().get().await?.entries);
    <crate::schedule_event_types::ScheduleEventType as RequestEntryCache>::get_cache_table().write().await.add_entries(MetaRequest::<crate::schedule_event_types::ScheduleEventType>::new().get().await?.entries);
    <crate::situations::SituationCode as RequestEntryCache>::get_cache_table().write().await.add_entries(MetaRequest::<crate::situations::SituationCode>::new().get().await?.entries);
    <crate::sky::SkyDescription as RequestEntryCache>::get_cache_table().write().await.add_entries(MetaRequest::<crate::sky::SkyDescription>::new().get().await?.entries);
    <crate::standings_types::StandingsType as RequestEntryCache>::get_cache_table().write().await.add_entries(MetaRequest::<crate::standings_types::StandingsType>::new().get().await?.entries);
    <crate::wind_direction::WindDirection as RequestEntryCache>::get_cache_table().write().await.add_entries(MetaRequest::<crate::wind_direction::WindDirection>::new().get().await?.entries);

    <crate::person::Person as RequestEntryCache>::get_cache_table().write().await.add_entries(people_response.await?.people.into_iter().map(Box::new).map(Person::Ballplayer));

    Ok(())
}

/// # Errors
/// See variants of [`crate::request::Error`]
#[cfg(feature = "cache")]
#[cfg(feature = "ureq")]
pub fn precache() -> Result<(), crate::request::Error> {
    <crate::awards::Award as RequestEntryCache>::get_cache_table().write().add_entries(crate::awards::AwardRequest::builder().build_and_get()?.awards);
    <crate::divisions::Division as RequestEntryCache>::get_cache_table().write().add_entries(crate::divisions::DivisionsRequest::builder().build_and_get()?.divisions);
    <crate::conferences::Conference as RequestEntryCache>::get_cache_table().write().add_entries(crate::conferences::ConferencesRequest::builder().build_and_get()?.conferences);
    <crate::venue::Venue as RequestEntryCache>::get_cache_table().write().add_entries(crate::venue::VenuesRequest::builder().build_and_get()?.venues);
    <crate::league::League as RequestEntryCache>::get_cache_table().write().add_entries(crate::league::LeaguesRequest::builder().build_and_get()?.leagues);
    <crate::sport::Sport as RequestEntryCache>::get_cache_table().write().add_entries(crate::sport::SportsRequest::builder().build_and_get()?.sports);

    <crate::baseball_stats::BaseballStat as RequestEntryCache>::get_cache_table().write().add_entries(MetaRequest::<crate::baseball_stats::BaseballStat>::new().get()?.entries);
    <crate::job_types::JobType as RequestEntryCache>::get_cache_table().write().add_entries(MetaRequest::<crate::job_types::JobType>::new().get()?.entries);
    <crate::event_types::EventType as RequestEntryCache>::get_cache_table().write().add_entries(MetaRequest::<crate::event_types::EventType>::new().get()?.entries);
    <crate::game_status::GameStatus as RequestEntryCache>::get_cache_table().write().add_entries(MetaRequest::<crate::game_status::GameStatus>::new().get()?.entries);
    <crate::metrics::Metric as RequestEntryCache>::get_cache_table().write().add_entries(MetaRequest::<crate::metrics::Metric>::new().get()?.entries);
    <crate::pitch_codes::PitchCode as RequestEntryCache>::get_cache_table().write().add_entries(MetaRequest::<crate::pitch_codes::PitchCode>::new().get()?.entries);
    <crate::pitch_types::PitchType as RequestEntryCache>::get_cache_table().write().add_entries(MetaRequest::<crate::pitch_types::PitchType>::new().get()?.entries);
    <crate::platforms::Platform as RequestEntryCache>::get_cache_table().write().add_entries(MetaRequest::<crate::platforms::Platform>::new().get()?.entries);
    <crate::positions::Position as RequestEntryCache>::get_cache_table().write().add_entries(MetaRequest::<crate::positions::Position>::new().get()?.entries);
    <crate::review_reasons::ReviewReason as RequestEntryCache>::get_cache_table().write().add_entries(MetaRequest::<crate::review_reasons::ReviewReason>::new().get()?.entries);
    <crate::schedule_event_types::ScheduleEventType as RequestEntryCache>::get_cache_table().write().add_entries(MetaRequest::<crate::schedule_event_types::ScheduleEventType>::new().get()?.entries);
    <crate::situations::SituationCode as RequestEntryCache>::get_cache_table().write().add_entries(MetaRequest::<crate::situations::SituationCode>::new().get()?.entries);
    <crate::sky::SkyDescription as RequestEntryCache>::get_cache_table().write().add_entries(MetaRequest::<crate::sky::SkyDescription>::new().get()?.entries);
    <crate::standings_types::StandingsType as RequestEntryCache>::get_cache_table().write().add_entries(MetaRequest::<crate::standings_types::StandingsType>::new().get()?.entries);
    <crate::wind_direction::WindDirection as RequestEntryCache>::get_cache_table().write().add_entries(MetaRequest::<crate::wind_direction::WindDirection>::new().get()?.entries);

    <crate::person::Person as RequestEntryCache>::get_cache_table().write().add_entries(PlayersRequest::builder().build_and_get()?.people);

    Ok(())
}
