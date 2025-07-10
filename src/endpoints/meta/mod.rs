use std::fmt::{Debug, Display, Formatter};
use std::marker::PhantomData;
use derive_more::{Deref, DerefMut};
use serde::{de, Deserialize, Deserializer};
use serde::de::{Error, MapAccess, SeqAccess};
use crate::endpoints::Url;

mod kinds;

pub use kinds::*;

#[derive(Debug, Deref, DerefMut, PartialEq, Eq, Clone)]
pub struct MetaResponse<T: MetaKind> {
    pub entries: Vec<T>,
}

impl<'de, T: MetaKind> Deserialize<'de> for MetaResponse<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>
    {
        struct Visitior<T>(PhantomData<T>);

        impl<'de, T: MetaKind> de::Visitor<'de> for Visitior<T> {
            type Value = MetaResponse<T>;

            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                formatter.write_str("either copyright and other entry, or just raw list")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let mut entries = vec![];
                while let Some(element) = seq.next_element::<T>()? {
                    entries.push(element);
                }
                Ok(MetaResponse { entries })
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                while let Some(key) = map.next_key::<String>()? {
                    if key != "copyright" {
                        let entries = map.next_value::<Vec<T>>()?;
                        return Ok(MetaResponse { entries });
                    }
                }
                Err(Error::custom("Could not find a field that deserializes to the entries"))
            }
        }

        deserializer.deserialize_any(Visitior(PhantomData))
    }
}

pub struct MetaEndpointUrl<T: MetaKind> {
    _marker: PhantomData<T>,
}

impl<T: MetaKind> Display for MetaEndpointUrl<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "http://statsapi.mlb.com/api/v1/{}", T::ENDPOINT_NAME)
    }
}

impl<T: MetaKind> Url<MetaResponse<T>> for MetaEndpointUrl<T> {}
