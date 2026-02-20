//! # Stat Type
//! Describes different "types" (effectively splits) of statistics, seasonal, career, etc.
//!
//! The full list can be found via:
//! ```
//! mlb_api::meta::MetaRequest::<mlb_api::stat_types::StatType>::new()
//!     .get()
//!     .await?
//!     .entries
//! ```

id!(#[doc = "A [`String`] representing a specific type of stat collection"] StatType { displayName: String });

meta_kind_impl!("statTypes" => StatType);
static_request_entry_cache_impl!(StatType);
test_impl!(StatType);
