use std::{fmt, u8};

use chrono::Duration;
use serde::{Deserialize, Serialize};
use crate::deserializer::*;
use strum_macros::{Display, EnumString};

use super::passive_option::PassiveOption;

#[derive(Debug, Default, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawSkillBuff<'a> {
    pub id: u32,
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
    // pub status_effect_values: Option<Vec<i32>>,
    pub buff_category: SkillBuffCategory,
    pub target: SkillBuffTarget,
    #[serde(deserialize_with = "to_skill_buff_unique_group")]
    pub unique_group: SkillBuffUniqueGroup,
    #[serde(rename(deserialize = "overlap"))]
    pub overlap_flag: u8,
    pub passive_options: Vec<PassiveOption>,
    #[serde(deserialize_with = "null_as_empty_u32_vec")]
    pub source_skills: Vec<u32>,
    #[serde(deserialize_with = "to_skill_buff_set_name")]
    pub set_name: SkillBuffSetName
}

impl<'a> RawSkillBuff<'a> {
    pub fn has_valid_skill_for_raid(&self) -> bool {
        matches!(self.buff_category, SkillBuffCategory::ClassSkill 
            | SkillBuffCategory::ArkPassive 
            | SkillBuffCategory::Identity
            | SkillBuffCategory::Ability
        ) && self.unique_group != SkillBuffUniqueGroup::None
    }
}

impl<'a> fmt::Display for RawSkillBuff<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Name: {:?} Duration: {:?}", self.name.unwrap_or("Unknown"), self.duration)
    }
}

#[derive(Debug, Default, PartialEq, Eq, Hash, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum SkillBuffUniqueGroup {
    #[default]
    None,
    Unknown(u32),
    Food,
    DropsOfEther,
    SupportMarking,
    SupportEvolutionBuff,
    PaladinAttackPowerBuff,
    ArtistAttackPowerBuff,
    BardAttackPowerBuff,
    PaladinIdentityBuff,
    SupportHyperAwakeningTechniqueBuff
}

impl SkillBuffUniqueGroup {
    pub fn from(value: u32) -> SkillBuffUniqueGroup {
        match value {
            0 => SkillBuffUniqueGroup::None,
            80000 | 80001 | 80002 => SkillBuffUniqueGroup::Food,
            210230 => SkillBuffUniqueGroup::SupportMarking,
            101105 => SkillBuffUniqueGroup::PaladinAttackPowerBuff,
            314004 => SkillBuffUniqueGroup::ArtistAttackPowerBuff,
            101204 => SkillBuffUniqueGroup::BardAttackPowerBuff,
            368000 => SkillBuffUniqueGroup::PaladinIdentityBuff,
            212305 => SkillBuffUniqueGroup::SupportHyperAwakeningTechniqueBuff,
            2000260 | 2000360 => SkillBuffUniqueGroup::SupportEvolutionBuff,
            501 | 502 | 503 | 504 | 505 => SkillBuffUniqueGroup::DropsOfEther,
            id => SkillBuffUniqueGroup::Unknown(id),
        }
    }
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
    #[serde(rename = "weaken_defense")]
    WeakenDefense,
    #[serde(rename = "weaken_resistance")]
    WeakenResistance,
    #[serde(rename = "skill_damage_amplify")]
    SkillDamageAmplify,
    #[serde(rename = "beattacked_damage_amplify")]
    BeattackedDamageAmplify,
    #[serde(rename = "skill_damage_amplify_attack")]
    SkillDamageAmplifyAttack,
    #[serde(rename = "directional_attack_amplify")]
    DirectionalAttackAmplify,
    #[serde(rename = "instant_stat_amplify")]
    InstantStatAmplify,
    #[serde(rename = "attack_power_amplify")]
    AttackPowerAmplify,
    #[serde(rename = "instant_stat_amplify_by_contents")]
    InstantStatAmplifyByContents,
    #[serde(rename = "evolution_type_damage")]
    EvolutionTypeDamage,
    #[serde(rename = "move_speed_down")]
    MoveSpeedDown,
    #[serde(rename = "reset_cooldown")]
    ResetCooldown,
    #[serde(rename = "change_ai_point")]
    ChangeAiPoint,
    #[serde(rename = "ai_point_amplify")]
    AiPointAmplify,
    #[serde(rename = "increase_identity_gauge")]
    IncreaseIdentityGauge,
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
    #[serde(rename = "Dazzling Twilight")]
    DazzlingTwilight,
    Charm
}
