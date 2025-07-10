use std::fmt::Debug;
use serde::Deserialize;

pub mod awards;
pub mod baseball_stats;
pub mod event_types;
pub mod game_status;
pub mod game_types;
pub mod hit_trajectories;
pub mod job_types;
pub mod languages;
pub mod league_leader_types;
// pub mod logical_events;
// pub mod metrics;
// pub mod pitch_codes;
// pub mod pitch_types;
// pub mod platforms;
// pub mod positions;
// pub mod review_reasons;
// pub mod roster_types;
// pub mod schedule_event_types;
// pub mod situation_codes;
// pub mod sky;
// pub mod standings_types;
pub mod stat_groups;
// pub mod stat_types;
// pub mod wind_direction;

pub trait MetaKind: Debug + for<'de> Deserialize<'de> + Eq + Clone {
    const ENDPOINT_NAME: &'static str;
}
