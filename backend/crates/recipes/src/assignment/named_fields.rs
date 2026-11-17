use std::{collections::BTreeMap, fmt};

use serde::{
    Deserialize, Deserializer,
    de::{Error, MapAccess, Visitor},
};
use serde_json::Value;

pub(super) struct NamedFields(pub BTreeMap<String, Value>);

impl<'de> Deserialize<'de> for NamedFields {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct Fields;
        impl<'de> Visitor<'de> for Fields {
            type Value = NamedFields;

            fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str("four named input fields without duplicates")
            }

            fn visit_map<M: MapAccess<'de>>(self, mut input: M) -> Result<Self::Value, M::Error> {
                let mut result = BTreeMap::new();
                while let Some(key) = input.next_key::<String>()? {
                    if key.len() > 64 || result.len() == 4 || result.contains_key(&key) {
                        return Err(M::Error::custom("invalid or repeated input key"));
                    }
                    result.insert(key, input.next_value::<Value>()?);
                }
                Ok(NamedFields(result))
            }
        }
        deserializer.deserialize_map(Fields)
    }
}
