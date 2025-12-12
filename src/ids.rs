#[macro_export]
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

#[macro_export]
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
