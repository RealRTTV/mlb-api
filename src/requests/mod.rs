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
pub mod conference;
pub mod division;
pub mod draft;
pub mod game;
// pub mod high_low; // i think this is part of game
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
// pub mod device_properties;
