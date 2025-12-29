use std::borrow::Cow;
use std::fmt::Debug;
use serde::de::DeserializeOwned;

pub trait Hydrations: 'static + Debug + DeserializeOwned + Eq + Clone + HydrationText {}

pub trait HydrationText {
	fn hydration_text() -> Cow<'static, str>;
}

impl Hydrations for () {}

impl HydrationText for () {
	fn hydration_text() -> Cow<'static, str> {
		Cow::Borrowed("")
	}
}
