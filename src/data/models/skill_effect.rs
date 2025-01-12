use crate::deserializer::*;
use serde::{Deserialize, Serialize};
use crate::deserializer::*;

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawSkillEffect<'a> {
    pub id: u32,
    #[serde(deserialize_with = "empty_str_as_none")]
    pub comment: Option<&'a str>,
    #[serde(rename = "itemName")]
    pub name: Option<&'a str>,
    #[serde(rename = "itemDesc")]
    pub description: Option<String>,
    #[serde(rename = "type")]
    pub effect_type: SkillEffectType,
    pub directional_mask: Option<i32>,
    pub item_type: SkillEffectItemType,
    pub icon: Option<&'a str>,
    #[serde(deserialize_with = "null_as_empty_u32_vec")]
    pub source_skills: Vec<u32>,
}

#[derive(Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum SkillEffectType {
    #[default]
    None,
    #[serde(rename = "add_status_effect")]
    AddStatusEffect,
    #[serde(rename = "throw_projectile")]
    ThrowProjectile,
    #[serde(other)]
    Unknown
}

#[derive(Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum SkillEffectItemType {
    #[default]
    None,
    #[serde(rename = "na")]
    NotApplicable,
    #[serde(rename = "useup")]
    UseUp
}