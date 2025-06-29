use serde::de::DeserializeOwned;
use crate::endpoints::Url;

pub type Result<T, E = reqwest::Error> = std::result::Result<T, E>;

pub async fn get<T: DeserializeOwned>(url: &impl Url<T>) -> Result<T> {
    reqwest::get(url.to_string()).await?.json::<T>().await
}