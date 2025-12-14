macro_rules! integer_id {
	($name:ident) => {
		integer_id!(#[derive(::core::fmt::Debug, ::core::default::Default, ::serde::Deserialize, ::derive_more::Deref, ::derive_more::Display, ::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::hash::Hash, ::derive_more::From)] $name);
	};
    ($(#[$meta:meta])+ $name:ident) => {
	    $(#[$meta])*
	    #[repr(transparent)]
		pub struct $name(u32);

		impl $name {
			#[must_use]
			pub const fn new(id: u32) -> Self {
				Self(id)
			}
		}
    };
}

macro_rules! string_id {
    ($name:ident) => {
	    string_id!(#[derive(::core::fmt::Debug, ::serde::Deserialize, ::derive_more::Deref, ::derive_more::Display, ::core::cmp::PartialEq, ::core::cmp::Eq, ::core::clone::Clone, ::core::hash::Hash, ::derive_more::From)] $name);
    };
	($(#[$meta:meta])+ $name:ident) => {
		$(#[$meta])*
		#[repr(transparent)]
		pub struct $name(String);

		impl $name {
			#[must_use]
			pub fn new(id: impl ::core::convert::Into<String>) -> Self {
				Self(id.into())
			}
		}
	};
}

macro_rules! gen_params {
    (@ $builder:ident $key:literal: $value:expr $(, $($rest:tt)*)?) => {
        let is_empty = $builder.is_empty();
        let _ = write!(&mut $builder, "{prefix}{key}={value}", key = $key, value = $value, prefix = if is_empty { '?' } else { '&' });
        gen_params!(@ $builder $($($rest)*)? );
    };
    (@ $builder:ident $key:literal?: $value:expr $(, $($rest:tt)*)?) => {
        if let Option::Some(value) = $value {
            let is_empty = $builder.is_empty();
            let _ = write!(&mut $builder, "{prefix}{key}={value}", key = $key, prefix = if is_empty { '?' } else { '&' });
        }
        gen_params!(@ $builder $($($rest)*)? );
    };
    (@ $builder:ident $value:expr $(, $($rest:tt)*)?) => {
        let is_empty = $builder.is_empty();
        let _ = write!(&mut $builder, "{prefix}{value}", value = $value, prefix = if is_empty { '?' } else { '&' });
        gen_params!(@ $builder $($($rest)*)?);
    };
    (@ $builder:ident $($args:tt)*) => {};
    ($($args:tt)*) => {{
        use ::core::fmt::Write;

        let mut builder = String::new();
        gen_params! { @ builder $($args)* }
        builder
    }};
}

macro_rules! id_only_eq_impl {
    ($name:ty, $id_field:ident) => {
		impl ::core::cmp::PartialEq for $name {
			fn eq(&self, other: &Self) -> bool {
				self.$id_field == other.$id_field
			}
		}
	};
}

macro_rules! tiered_request_entry_cache_impl {
	($name:ty => $hydrated_name:ty; $id_field:ident: $id:ty) => {
		static CACHE: $crate::RwLock<$crate::cache::HydratedCacheTable<$name>> = $crate::rwlock_const_new($crate::cache::HydratedCacheTable::new());

		impl $crate::cache::RequestEntryCache for $name {
			type HydratedVariant = $hydrated_name;
			type Identifier = $id;
			type URL = $crate::requests::meta::MetaRequest<Self>;

			fn into_hydrated_variant(self) -> Option<Self::HydratedVariant> {
				self.try_into_hydrated()
			}

			fn id(&self) -> &Self::Identifier {
				&self.$id_field
			}

			fn url_for_id(_id: &Self::Identifier) -> Self::URL {
				$crate::requests::meta::MetaRequest::new()
			}

			fn get_entries(response: <Self::URL as $crate::request::StatsAPIRequestUrl>::Response) -> impl IntoIterator<Item=Self>
			where
				Self: Sized
			{
				response.entries
			}

			fn get_hydrated_cache_table() -> &'static $crate::RwLock<$crate::cache::HydratedCacheTable<Self>>
			where
				Self: Sized
			{
				&CACHE
			}
		}
	};
    ($url:ty => |$id_field:ident: $id:ty| $expr:block . $entries:ident => $name:ty => $hydrated_name:ty) => {
		static CACHE: $crate::RwLock<$crate::cache::HydratedCacheTable<$name>> = $crate::rwlock_const_new($crate::cache::HydratedCacheTable::new());

		impl $crate::cache::RequestEntryCache for $name {
			type HydratedVariant = $hydrated_name;
			type Identifier = $id;
			type URL = $url;

			fn into_hydrated_variant(self) -> Option<Self::HydratedVariant> {
				self.try_into_hydrated()
			}

			fn id(&self) -> &Self::Identifier {
				&self.$id_field
			}

			fn url_for_id(id: &Self::Identifier) -> Self::URL {
				let $id_field = id;
				$expr
			}

			fn get_entries(response: <Self::URL as $crate::request::StatsAPIRequestUrl>::Response) -> impl IntoIterator<Item=Self>
			where
				Self: Sized
			{
				response.$entries
			}

			fn get_hydrated_cache_table() -> &'static $crate::RwLock<$crate::cache::HydratedCacheTable<Self>>
			where
				Self: Sized
			{
				&CACHE
			}
		}
	};
}

macro_rules! static_request_entry_cache_impl {
    ($name:ty) => {
		static CACHE: $crate::RwLock<$crate::cache::HydratedCacheTable<$name>> = $crate::rwlock_const_new($crate::cache::HydratedCacheTable::new());

		impl $crate::cache::RequestEntryCache for $name {
			type HydratedVariant = Self;
			type Identifier = Self;
			type URL = $crate::requests::meta::MetaRequest<Self>;

			fn into_hydrated_variant(self) -> Option<Self::HydratedVariant> {
				Some(self)
			}

			fn id(&self) -> &Self::Identifier {
				self
			}

			fn url_for_id(_id: &Self::Identifier) -> Self::URL {
				$crate::requests::meta::MetaRequest::new()
			}

			fn get_entries(response: <Self::URL as $crate::request::StatsAPIRequestUrl>::Response) -> impl IntoIterator<Item=Self>
			where
				Self: Sized
			{
				response.entries
			}

			fn get_hydrated_cache_table() -> &'static $crate::RwLock<$crate::cache::HydratedCacheTable<Self>>
			where
				Self: Sized
			{
				&CACHE
			}
		}
	};
}

pub mod all_star;
pub mod attendance; // done
pub mod awards; // done
pub mod conferences; // done
pub mod divisions; // done
pub mod draft; // done
pub mod game;
pub mod high_low;
pub mod home_run_derby;
pub mod jobs; // done
pub mod league; // done
pub mod meta;  // done
pub mod person; // done
pub mod schedule; // done
pub mod season; // done
pub mod sports; // done
pub mod standings; // done
pub mod stats;
// done
pub mod transactions; // done
pub mod venue;
pub mod team;
// done

pub use meta::*;
