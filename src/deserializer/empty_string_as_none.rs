use std::str::FromStr;

use serde::{Deserialize, Deserializer};
use serde_json::Value;

pub fn empty_string_as_none<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let opt = Option::<String>::deserialize(deserializer)?;
    Ok(opt.filter(|s| !s.is_empty()))
}

pub fn empty_str_as_none<'de, D>(deserializer: D) -> Result<Option<&'de str>, D::Error>
where
    D: Deserializer<'de>,
{
    let opt = Option::<&str>::deserialize(deserializer)?;
    Ok(opt.filter(|s| !s.is_empty()))
}
