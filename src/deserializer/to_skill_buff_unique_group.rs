use serde::{Deserialize, Deserializer};

use crate::data::models::skill_buff::SkillBuffUniqueGroup;

pub fn to_skill_buff_unique_group<'de, D>(deserializer: D) -> Result<SkillBuffUniqueGroup, D::Error>
where
    D: Deserializer<'de>,
{
    let value = u32::deserialize(deserializer)?;
    Ok(SkillBuffUniqueGroup::from(value))
}