use std::collections::HashMap;
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::sync::Arc;
use fxhash::FxBuildHasher;
use serde::de::DeserializeOwned;
use thiserror::Error;
use crate::endpoints::StatsAPIUrl;
use crate::RwLock;

pub trait EndpointEntryCache: 'static+ Debug + DeserializeOwned + Eq + Clone {
    type HydratedVariant;
    type Identifier: Clone + Eq + Hash + Display;
    type URL: StatsAPIUrl;
    
    fn into_hydrated_entry(self) -> Option<Self::HydratedVariant>;
    
    fn id(&self) -> &Self::Identifier;

    fn url_for_id(id: &Self::Identifier) -> Self::URL;

    fn get_entries(response: <Self::URL as StatsAPIUrl>::Response) -> impl IntoIterator<Item = Self> where Self: Sized;

    fn get_hydrated_cache_table() -> &'static RwLock<HydratedCacheTable<Self>> where Self: Sized;

    #[cfg(feature = "reqwest")]
    async fn as_hydrated_or_request(&self) -> Result<Arc<Self::HydratedVariant>, Error<Self>> {
        let cache_lock = Self::get_hydrated_cache_table();
        let id = self.id();
        let cache = cache_lock.read().await;
        if let Some(hydrated_entry) = cache.get(id).cloned() {
            return Ok(hydrated_entry);
        }

        let mut cache = cache_lock.write().await;
        cache.request_and_add(id).await?;
        cache.get(id).cloned().ok_or_else(|| Error::NoMatchingVariant(id.clone()))
    }

    #[cfg(feature = "ureq")]
    fn as_hydrated_or_request(&self) -> Result<Arc<Self::HydratedVariant>, Error<Self>> {
        let cache_lock = Self::get_hydrated_cache_table();
        let id = self.id();
        let cache = cache_lock.read();
        if let Some(hydrated_entry) = cache.get(id).cloned() {
            return Ok(hydrated_entry);
        }

        let mut cache = cache_lock.write();
        cache.request_and_add(id)?;
        cache.get(id).cloned().ok_or_else(|| Error::NoMatchingVariant(id.clone()))
    }
}

pub struct HydratedCacheTable<T: EndpointEntryCache> {
    cached_values: HashMap<T::Identifier, Arc<T::HydratedVariant>, FxBuildHasher>,
}

#[derive(Debug, Error)]
pub enum Error<T: EndpointEntryCache> {
    #[error(transparent)]
    Url(#[from] crate::request::Error),
    #[error("No matching entry was found for id {0}")]
    NoMatchingVariant(T::Identifier),
}

impl<T: EndpointEntryCache> HydratedCacheTable<T> {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            cached_values: HashMap::with_hasher(FxBuildHasher::new()),
        }
    }

    #[must_use]
    pub fn get(&self, id: &T::Identifier) -> Option<&Arc<T::HydratedVariant>> {
        self.cached_values.get(id)
    }

    // make this unionize hydrations when those are eventually implemented
    pub fn insert(&mut self, id: T::Identifier, value: T::HydratedVariant) {
        self.cached_values.insert(id.clone(), Arc::new(value));
    }
    
    pub fn clear(&mut self) {
        self.cached_values.clear();
    }
    
    pub fn try_add_entries(&mut self, entries: impl IntoIterator<Item = T>) {
        for (id, entry) in entries.into_iter().filter_map(|entry| {
            let id = entry.id().clone();
            entry.into_hydrated_entry().map(|entry| (id, entry))
        }) {
            self.insert(id, entry);
        }
    }

    #[cfg(feature = "reqwest")]
    pub async fn request_and_add(&mut self, id: &T::Identifier) -> Result<(), crate::request::Error> {
        let url = <T as EndpointEntryCache>::url_for_id(&id);
        let response = url.get().await?;
        self.try_add_entries(<T as EndpointEntryCache>::get_entries(response));
        Ok(())
    }

    #[cfg(feature = "ureq")]
    pub fn request_and_add(&mut self, id: &T::Identifier) -> Result<(), crate::request::Error> {
        let url = <T as EndpointEntryCache>::url_for_id(&id);
        let response = url.get()?;
        self.try_add_entries(<T as EndpointEntryCache>::get_entries(response));
        Ok(())
    }
}