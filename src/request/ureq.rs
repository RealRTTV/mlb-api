use serde::de::DeserializeOwned;

pub type Result<T, E = ureq::Error> = std::result::Result<T, E>;

pub fn get<T: DeserializeOwned>(url: &impl ToString) -> Result<T> {
    ureq::get(url.to_string()).call()?.into_body().read_json::<T>()
}
