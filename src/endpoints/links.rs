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
