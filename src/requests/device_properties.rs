//! Represents CSS & HTML data for rendering teams on sites.
//!
//! This information is left lightly parsed as general usage of this endpoint isn't clear yet and design decisions regarding it have not been made.
//!
//! There is no known `deviceProperties` endpoint, however through some [`Hydrations`](crate::hydrations::Hydrations), this can be accessed per team or sport.

use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
pub struct DeviceProperties {

}