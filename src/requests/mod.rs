macro_rules! id {
    ($name:ident { $id_field:ident: String }) => {
		#[derive(::core::fmt::Debug, ::derive_more::Deref, ::derive_more::Display, ::core::cmp::PartialEq, ::core::cmp::Eq, ::core::clone::Clone, ::core::hash::Hash, ::derive_more::From)]
		#[repr(transparent)]
		pub struct $name(String);

		impl<'de> ::serde::Deserialize<'de> for $name {
			#[allow(non_snake_case, reason = "is camel case because serde deserializes that from the API")]
			fn deserialize<D: ::serde::de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
				#[derive(::serde::Deserialize)]
				#[serde(untagged)]
				enum Repr {
					Wrapped { $id_field: String },
					Inline(String),
				}

				let (Repr::Wrapped { $id_field } | Repr::Inline($id_field)) = Repr::deserialize(deserializer)?;
				Ok($name($id_field))
			}
		}

		impl $name {
			#[must_use]
			pub fn new(id: impl Into<String>) -> Self {
				Self(id.into())
			}
		}
	};
    ($name:ident { $id_field:ident: u32 }) => {
		#[derive(::core::fmt::Debug, ::derive_more::Deref, ::derive_more::Display, ::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::hash::Hash, ::derive_more::From)]
		#[repr(transparent)]
		pub struct $name(u32);

		impl<'de> ::serde::Deserialize<'de> for $name {
			#[allow(non_snake_case, reason = "is camel case because serde deserializes that from the API")]
			fn deserialize<D: ::serde::de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
				#[derive(::serde::Deserialize)]
				#[serde(untagged)]
				enum Repr {
					Wrapped { $id_field: u32 },
					Inline(u32),
				}

				let (Repr::Wrapped { $id_field } | Repr::Inline($id_field)) = Repr::deserialize(deserializer)?;
				Ok($name($id_field))
			}
		}

		impl $name {
			#[must_use]
			pub const fn new(id: u32) -> Self {
				Self(id)
			}
		}
	};
}

macro_rules! gen_params {
    (@ $builder:ident $key:literal: $value:expr $(, $($rest:tt)*)?) => {
        let is_empty = $builder.is_empty();
        let _ = ::core::write!(&mut $builder, "{prefix}{key}={value}", key = $key, value = $value, prefix = if is_empty { '?' } else { '&' });
        gen_params!(@ $builder $($($rest)*)? );
    };
    (@ $builder:ident $key:literal?: $value:expr $(, $($rest:tt)*)?) => {
        if let ::core::option::Option::Some(value) = $value {
            let is_empty = str::is_empty(&$builder);
            let _ = ::core::write!(&mut $builder, "{prefix}{key}={value}", key = $key, prefix = if is_empty { '?' } else { '&' });
        }
        gen_params!(@ $builder $($($rest)*)? );
    };
    (@ $builder:ident $value:expr $(, $($rest:tt)*)?) => {
        let is_empty = $builder.is_empty();
        let _ = ::core::write!(&mut $builder, "{prefix}{value}", value = $value, prefix = if is_empty { '?' } else { '&' });
        gen_params!(@ $builder $($($rest)*)?);
    };
    (@ $builder:ident $($args:tt)*) => {};
    ($($args:tt)*) => {{
        use ::core::fmt::Write;

        let mut builder = ::std::string::String::new();
        gen_params! { @ builder $($args)* }
        builder
    }};
}

macro_rules! id_only_eq_impl {
    ($name:ident, $id_field:ident) => {
		impl ::core::cmp::PartialEq for $name {
			fn eq(&self, other: &Self) -> bool {
				self.$id_field == other.$id_field
			}
		}

		impl ::core::cmp::Eq for $name {}
	};
}

macro_rules! entrypoint {
	(for<$($generic:ident)+> $name:ident <$($ty_generic_use:ident)*> $(. $field:ident)? => $complete:ident <$($complete_generic_use:ident)*> where $($t:tt)*) => {
		impl<$($generic)+> $crate::cache::RequestableEntrypoint for $name <$($ty_generic_use)+> where $($t)* {
			type Complete = $complete <$($complete_generic_use)*>;
			
			fn id(&self) -> &<<Self as $crate::cache::RequestableEntrypoint>::Complete as $crate::cache::Requestable>::Identifier {
				&self$(.$field)?
			}
		}
	};
    ($name:ident $(. $field:ident)? => $complete:ident) => {
		impl $crate::cache::RequestableEntrypoint for $name {
			type Complete = $complete;
			
			fn id(&self) -> &<<Self as $crate::cache::RequestableEntrypoint>::Complete as $crate::cache::Requestable>::Identifier {
				&self$(.$field)?
			}
		}
	};
}

pub mod all_star;
pub mod attendance;
pub mod awards;
pub mod conferences;
pub mod divisions;
pub mod draft;
pub mod game;
pub mod high_low;
pub mod home_run_derby;
pub mod jobs;
pub mod league;
pub mod meta;
pub mod person;
pub mod schedule;
pub mod season;
pub mod sport;
pub mod standings;
pub mod stats;
pub mod transactions;
pub mod venue;
pub mod team;

pub use meta::*;
