use serde::{de::{self, Visitor}, Deserializer};
use std::fmt;

use crate::data::models::skill_group::SkillGroup;

pub fn to_skill_group<'de, D>(deserializer: D) -> Result<Vec<SkillGroup>, D::Error>
where
    D: Deserializer<'de>,
{
    struct NullAsEmptyVec;

    impl<'de> Visitor<'de> for NullAsEmptyVec {
        type Value = Vec<SkillGroup>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a vector (null to be interpreted as empty vec)")
        }

        fn visit_unit<E>(self) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(Vec::new())
        }

        fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where
            A: de::SeqAccess<'de>,
        {
            let mut vec = Vec::new();
            while let Some(value) = seq.next_element()? {
                vec.push(value);
            }
            Ok(vec)
        }
    }

    deserializer.deserialize_any(NullAsEmptyVec)
}