use crate::request;
use serde::de::DeserializeOwned;

#[cfg(all(feature = "reqwest", feature = "ureq"))]
compile_error!("Only one http backend is allowed!");

pub trait StatsAPIEndpointUrl: ToString {
    type Response: DeserializeOwned;

	#[cfg(feature = "ureq")]
	fn get(&self) -> request::Result<Self::Response>
	where
		Self: Sized,
	{
		request::get(self)
	}

	#[cfg(feature = "reqwest")]
	fn get(&self) -> impl Future<Output = request::Result<Self::Response>>
	where
		Self: Sized,
	{
		request::get(self)
	}
}

pub trait StatsAPIEndpointUrlBuilderExt where Self: Sized {
    type Built: StatsAPIEndpointUrl + From<Self>;
    
    #[cfg(feature = "ureq")]
    fn build_and_get(self) -> request::Result<<T as StatsAPIEndpointUrl>::Response> {
        let built = T::from(self);
        request::get(&built)
    }

    #[cfg(feature = "reqwest")]
    fn build_and_get(self) -> impl Future<Output = request::Result<<Self::Built as StatsAPIEndpointUrl>::Response>> {
        async {
            let built = Self::Built::from(self);
            let str = built.to_string();
            request::get::<<Self::Built as StatsAPIEndpointUrl>::Response>(&str).await
        }
    }
}

#[macro_export]
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
