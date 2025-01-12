use std::ptr::addr_of;

use serde::{Deserialize, Serialize};
use crate::deserializer::{null_as_empty_u32_vec, u16_zero_as_none, u32_zero_as_none};

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawSkill<'a> {
    pub id: u32,
    pub name: Option<&'a str>,
    pub desc: Option<String>,
    #[serde(deserialize_with = "u32_zero_as_none")]
    pub class_id: Option<u32>,
    pub icon: Option<&'a str>,
    #[serde(deserialize_with = "u16_zero_as_none")]
    pub cooldown: Option<u16>,
    #[serde(rename = "type")]
    pub skill_type: SkillType,
    pub grade: SkillGrade,
    #[serde(alias = "groups")]
    #[serde(deserialize_with = "null_as_empty_u32_vec")]
    pub groups: Vec<u32>,
    #[serde(deserialize_with = "null_as_empty_u32_vec")]
    pub summon_source_skills: Vec<u32>,
    #[serde(deserialize_with = "null_as_empty_u32_vec")]
    pub source_skills: Vec<u32>,
}


#[derive(Debug, Default, Deserialize, Copy, Clone)]
#[serde(rename_all = "lowercase")]
pub enum SkillType {
    #[default]
    Unknown,
    None,
    Casting,
    Normal,
    Combo,
    Getup,
    Chain,
    Holding,
    Nomotion,
    Charge,
    Moving,
}

#[derive(Debug, Default, Deserialize, Copy, Clone)]
#[serde(rename_all = "lowercase")]
pub enum SkillGrade {
    #[default]
    Unknown,
    Normal,
    Super,
    Awakening,
}