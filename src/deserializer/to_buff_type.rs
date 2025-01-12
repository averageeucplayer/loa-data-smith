use std::str::FromStr;

use serde::{Deserialize, Deserializer};
use serde_json::Value;

use crate::data::models::skill_buff::SkillBuffType;

pub fn to_buff_type<'de, D>(deserializer: D) -> Result<SkillBuffType, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Value::deserialize(deserializer)?;
    match value {
        Value::String(str) => {
            let value = SkillBuffType::from_str(&str.as_str()).unwrap_or_default();
            Ok(value)
        },
        Value::Number(n) => Ok(SkillBuffType::None),
        _ => Err(serde::de::Error::custom("Expected an enum")),
    }
}