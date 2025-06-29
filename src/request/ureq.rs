use serde::de::DeserializeOwned;
use crate::endpoints::Url;

pub type Result<T, E = ureq::Error> = std::result::Result<T, E>;

pub fn get<T: DeserializeOwned>(url: &impl Url<T>) -> Result<T> {
    ureq::get(url.to_string()).call()?.into_body().read_json::<T>()
}
