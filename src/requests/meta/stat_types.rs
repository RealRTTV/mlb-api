use derive_more::Display;
use serde::Deserialize;
use serde::Deserializer;

#[doc(hidden)]
#[derive(Deserialize)]
#[serde(untagged)]
enum __StatTypeMaybeInline {
	Wrapped {
		#[serde(rename = "displayName")]
		display_name: String,
	},
	Inline(String),
}

impl __StatTypeMaybeInline {
	#[must_use]
	pub fn into_string(self) -> String {
		match self {
			Self::Wrapped { display_name } => display_name,
			Self::Inline(name) => name,
		}
	}
}

#[derive(Debug, PartialEq, Eq, Clone, Display, Hash)]
#[display("{name}")]
pub struct StatType {
	pub name: String,
}

impl<'de> Deserialize<'de> for StatType {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>
	{
		Ok(Self { name: __StatTypeMaybeInline::deserialize(deserializer)?.into_string() })
	}
}

impl StatType {
	#[must_use]
	pub const fn as_str(&self) -> &str {
		self.name.as_str()
	}
}

meta_kind_impl!("statTypes" => StatType);

static_request_entry_cache_impl!(StatType);

#[cfg(test)]
mod tests {
	use crate::meta::MetaRequest;
	use crate::request::StatsAPIRequestUrl;

	#[tokio::test]
	async fn parse_meta() {
		let _response = MetaRequest::<super::StatType>::new().get().await.unwrap();
	}
}
