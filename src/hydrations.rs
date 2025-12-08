use std::borrow::Cow;
use serde::de::DeserializeOwned;

pub trait Hydrations: DeserializeOwned {
	fn request_text() -> Option<Cow<'static, str>>;
}

impl Hydrations for () {
	fn request_text() -> Option<Cow<'static, str>> {
		None
	}
}
