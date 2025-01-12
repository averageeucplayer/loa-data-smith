use std::{fmt, str::FromStr};
use serde::{Deserialize, Deserializer};
use serde_json::Value;

use crate::data::models::skill_buff::SkillBuffSetName;

pub fn to_skill_buff_set_name<'de, D>(deserializer: D) -> Result<SkillBuffSetName, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Value::deserialize(deserializer)?;

    match value {
        Value::String(str) => {
            let value = SkillBuffSetName::from_str(&str.as_str()).unwrap_or_default();
            Ok(value)
        },
        Value::Null => Ok(SkillBuffSetName::NotApplicable),
        _ => Err(serde::de::Error::custom("Expected an enum")),
    }
}