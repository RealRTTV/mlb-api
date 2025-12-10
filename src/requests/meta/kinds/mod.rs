pub mod baseball_stats;
pub use baseball_stats::*;

pub mod event_types;
pub use event_types::*;

pub mod game_status;
pub use game_status::*;

pub mod game_types;
pub use game_types::*;

pub mod hit_trajectories;
pub use hit_trajectories::*;

pub mod job_types;
pub use job_types::*;

pub mod languages;
pub use languages::*;

pub mod league_leader_types;
pub use league_leader_types::*;

pub mod logical_events;
pub use logical_events::*;

pub mod metrics;
pub use metrics::*;

pub mod pitch_codes;
pub use pitch_codes::*;

pub mod pitch_types;
pub use pitch_types::*;

pub mod platforms;
pub use platforms::*;

pub mod positions;
pub use positions::*;

pub mod review_reasons;
pub use review_reasons::*;

pub mod roster_types;
pub use roster_types::*;

pub mod schedule_event_types;
pub use schedule_event_types::*;

pub mod situation_codes;
pub use situation_codes::*;

pub mod sky;
pub use sky::*;

pub mod standings_types;
pub use standings_types::*;

pub mod stat_groups;
pub use stat_groups::*;

pub mod stat_types;
pub use stat_types::*;

pub mod wind_direction;
pub use wind_direction::*;

use crate::cache::RequestEntryCache;

pub trait MetaKind: RequestEntryCache {
	const ENDPOINT_NAME: &'static str;
}
