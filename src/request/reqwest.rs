use serde::de::DeserializeOwned;

pub type Result<T, E = reqwest::Error> = std::result::Result<T, E>;

pub async fn get<T: DeserializeOwned>(url: &impl ToString) -> Result<T> {
    reqwest::get(url.to_string()).await?.json::<T>().await
}