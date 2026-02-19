//! General hydrations types for the hydrations system
//! 
//! Everything here is rather internal, not much will be of interest

#![allow(missing_docs, reason = "unnecessary")]

use std::borrow::Cow;
use std::fmt::Debug;
use serde::de::DeserializeOwned;

pub trait Hydrations: 'static + Debug + DeserializeOwned + Eq + Clone + HydrationText {}

pub trait HydrationText {
	type RequestData;

	fn hydration_text(data: &Self::RequestData) -> Cow<'static, str>;
}

impl Hydrations for () {}

impl HydrationText for () {
	type RequestData = ();

	fn hydration_text((): &()) -> Cow<'static, str> {
		Cow::Borrowed("")
	}
}
