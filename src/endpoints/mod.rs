#![allow(dead_code)]

pub mod attendance;
pub mod awards;
pub mod conferences;
pub mod divisions;
pub mod draft;
pub mod game;
pub mod game_pace;
pub mod high_low;
pub mod home_run_derby;
pub mod league;
pub mod people;
pub mod person;
pub mod jobs;
pub mod schedule;
pub mod seasons;
pub mod sports; // COMPLETE!
pub mod standings;
pub mod stats;
pub mod teams;
pub mod transactions;
pub mod venue; // COMPLETE!
pub mod meta;

mod links;

pub use links::*;

#[cfg(feature = "static_lib")]
mod r#static {
    use std::fmt::Display;
    use thiserror::Error;

    #[derive(Debug, Error)]
    pub enum StaticParseError<T: Display> {
        #[error("Unknown ID ('{0}') for static")]
        InvalidId(T),
    }
}

#[cfg(feature = "static_lib")]
pub use r#static::*;
