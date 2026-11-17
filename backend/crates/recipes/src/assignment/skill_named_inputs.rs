use std::{collections::BTreeMap, fmt};

use serde::{
    Deserialize, Deserializer,
    de::{Error, MapAccess, Visitor},
};
use serde_json::value::RawValue;

pub(super) struct SkillNamedInputs<'a>(pub(super) BTreeMap<String, &'a RawValue>);

impl<'de> Deserialize<'de> for SkillNamedInputs<'de> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct Fields;
        impl<'de> Visitor<'de> for Fields {
            type Value = SkillNamedInputs<'de>;

            fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str("two named arrays without duplicate fields")
            }

            fn visit_map<M: MapAccess<'de>>(self, mut input: M) -> Result<Self::Value, M::Error> {
                let mut result = BTreeMap::new();
                while let Some(key) = input.next_key::<String>()? {
                    if key.len() > 64 || result.len() == 2 || result.contains_key(&key) {
                        return Err(M::Error::custom("invalid or repeated input key"));
                    }
                    result.insert(key, input.next_value::<&'de RawValue>()?);
                }
                Ok(SkillNamedInputs(result))
            }
        }
        deserializer.deserialize_map(Fields)
    }
}
