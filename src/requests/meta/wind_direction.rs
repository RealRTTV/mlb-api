use serde::Deserialize;

id!(#[doc = "A [`String`] representing a direction the wind is going"] WindDirectionId { code: String });

/// A detailed `struct` representing the direction wind is going.
///
/// Note these are not cardinal directions, you'd have to find out the venue's coordinates to calculate from these, but it's likely easier to use another weather API for wind.
///
/// ## Examples
/// ```
/// WindDirection {
///     id: "Out to CF".into(),
///     description: /* same as id */
/// }
/// ```
#[derive(Debug, Deserialize, Clone)]
pub struct WindDirection {
	pub description: String,
	#[serde(flatten)]
	pub id: WindDirectionId,
}

id_only_eq_impl!(WindDirection, id);
meta_kind_impl!("windDirection" => WindDirection);
tiered_request_entry_cache_impl!(WindDirection.id: WindDirectionId);
test_impl!(WindDirection);
