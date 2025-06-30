use std::fmt::Display;
use serde::de::DeserializeOwned;
use crate::request;

pub trait Url<T: DeserializeOwned>: Display {
    #[cfg(feature = "ureq")]
    fn get(&self) -> request::Result<T> where Self: Sized {
        request::get(self)
    }

    #[cfg(feature = "reqwest")]
    fn get(&self) -> impl Future<Output = request::Result<T>> where Self: Sized {
        request::get(self)
    }
}

#[macro_export]
macro_rules! gen_params {
    (@ $builder:ident $key:literal: $value:expr $(, $($rest:tt)*)?) => {{
        let is_empty = $builder.is_empty();
        let _ = write!(&mut $builder, "{prefix}{key}={value}", key = $key, value = $value, prefix = if is_empty { '?' } else { '&' });
        gen_params!(@ $builder $($($rest)*)? );
    }};
    (@ $builder:ident $key:literal?: $value:expr $(, $($rest:tt)*)?) => {{
        if let Option::Some(value) = $value {
            let is_empty = $builder.is_empty();
            let _ = write!(&mut $builder, "{prefix}{key}={value}", key = $key, prefix = if is_empty { '?' } else { '&' });
        }
        gen_params!(@ $builder $($($rest)*)? );
    }};
    (@ $builder:ident $(,)?) => {};
    ($($args:tt)*) => {{
        use ::core::fmt::Write;
        
        let mut builder = String::new();
        gen_params! { @ builder $($args)* }
        builder
    }};
}
