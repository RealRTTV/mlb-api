use serde::de::DeserializeOwned;
use std::borrow::Cow;

pub trait Hydrations: DeserializeOwned {
	fn request_text() -> Option<Cow<'static, str>>;
}

impl Hydrations for () {
	fn request_text() -> Option<Cow<'static, str>> {
		None
	}
}
