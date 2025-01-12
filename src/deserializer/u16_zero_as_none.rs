use serde::{de::{self, Visitor}, Deserializer};
use std::fmt;

pub fn u16_zero_as_none<'de, D>(deserializer: D) -> Result<Option<u16>, D::Error>
where
    D: Deserializer<'de>,
{
    struct ZeroAsNone;

    impl<'de> Visitor<'de> for ZeroAsNone {
        type Value = Option<u16>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a number (0 to be interpreted as None)")
        }

        fn visit_u32<E>(self, value: u32) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            if value == 0 {
                Ok(None)
            } else {
                Ok(Some(value as u16))
            }
        }

        fn visit_i32<E>(self, value: i32) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            if value == 0 {
                Ok(None)
            } else {
                Ok(Some(value as u16))
            }
        }

        fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            if value == 0 {
                Ok(None)
            } else {
                Ok(Some(value as u16))
            }
        }

        fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            if value == 0 {
                Ok(None)
            } else {
                Ok(Some(value as u16))
            }
        }
    }

    deserializer.deserialize_any(ZeroAsNone)
}
