use chrono::Duration;
use serde::{Deserialize, Serialize};
use crate::deserializer::*;
use strum_macros::{Display, EnumString};

#[derive(Debug, Default, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawSkillBuff<'a> {
    pub id: i32,
    #[serde(deserialize_with = "empty_str_as_none")]
    pub name: Option<&'a str>,
    #[serde(deserialize_with = "empty_string_as_none")]
    pub desc: Option<String>,
    pub icon: Option<&'a str>,
    pub icon_show_type: SkillBuffIconShowType,
    #[serde(deserialize_with = "to_duration")]
    pub duration: Duration,
    pub category: SkillBuffOrDebuff,
    #[serde(rename(deserialize = "type"))]
    #[serde(deserialize_with = "to_buff_type")]
    pub buff_type: SkillBuffType,
    pub status_effect_values: Option<Vec<i32>>,
    pub buff_category: SkillBuffCategory,
    pub target: SkillBuffTarget,
    #[serde(deserialize_with = "to_skill_buff_unique_group")]
    pub unique_group: SkillBuffUniqueGroup,
    #[serde(rename(deserialize = "overlap"))]
    pub overlap_flag: i32,
    pub passive_options: Vec<PassiveOption>,
    #[serde(deserialize_with = "null_as_empty_u32_vec")]
    pub source_skills: Vec<u32>,
    #[serde(deserialize_with = "to_skill_buff_set_name")]
    pub set_name: SkillBuffSetName
}

#[derive(Debug, Default, PartialEq, Eq, Hash, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum SkillBuffUniqueGroup {
    #[default]
    Unknown,
    DropsOfEther,
    SupportMarking,
    SupportBuff,
    PaladinAttackPowerBuff,
    ArtistAttackPowerBuff,
    BardAttackPowerBuff,
    PaladinIdentityBuff,
    SupportHyperAwakeningTechniqueBuff
}

#[derive(Debug, Default, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum SkillBuffOrDebuff {
    #[default]
    Buff,
    Debuff
}

#[derive(Debug, Display, EnumString, Default, PartialEq, Eq, Hash, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")] 
pub enum SkillBuffType {
    #[default]
    None,
    Shield,
    Heal,
    Fear,
    Burn,
    Aura,
    Freeze,
    Darkness,
    #[serde(other)]
    Unknown
}

#[derive(Debug, Default, PartialEq, Eq, Hash, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum SkillBuffCategory {
    #[default]
    None,
    ArkPassive,
    Set,
    Pet,
    ClassSkill,
    Elixir,
    Etc,
    Ability,
    Identity,
    Cook,
    Bracelet,
    BattleItem
}

#[derive(Debug, Default, PartialEq, Eq, Hash, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum SkillBuffTarget {
    #[default]
    None,
    #[serde(rename = "self")]
    TargetSelf,
    #[serde(rename = "self_party")]
    SelfParty,
    Party
}

#[derive(Debug, Default, PartialEq, Eq, Hash, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum SkillBuffIconShowType {
    #[default]
    None,
    #[serde(rename = "all_without_self")]
    AllWithoutSelf,
    #[serde(rename = "caster_or_target")]
    CasterOrTarget,
    #[serde(rename = "caster_or_party")]
    CasterOrParty,
    #[serde(rename = "all")]
    All
}

#[derive(Debug, Display, EnumString, Default, PartialEq, Eq, Hash, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub enum SkillBuffSetName {
    #[default]
    NotApplicable,
    MuteGuardian,
    Sunset,
    Entropy,
    Hallucination,
    Nightmare,
    Destruction,
    Yearning,
    Salvation,
    Charm
}

#[derive(Debug, Default, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PassiveOption {
    #[serde(rename(deserialize = "type"))]
    pub option_type: String,
    pub key_stat: String,
    pub key_index: i32,
    pub value: i32,
}