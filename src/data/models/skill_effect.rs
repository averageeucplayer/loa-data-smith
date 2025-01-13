use crate::deserializer::*;
use chrono::Duration;
use serde::{Deserialize, Serialize};
use crate::deserializer::*;

#[derive(Debug, Default, Deserialize, Clone)]
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
    #[serde(deserialize_with = "to_duration")]
    pub duration: Duration,
    pub effect_type: SkillEffectType,
    pub directional_mask: Option<i32>,
    pub item_type: SkillEffectItemType,
    pub icon: Option<&'a str>,
    #[serde(deserialize_with = "null_as_empty_u32_vec")]
    pub source_skills: Vec<u32>,
}

#[derive(Debug, Default, PartialEq, Eq, Hash, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum SkillEffectType {
    #[default]
    None,
    #[serde(rename = "approach")]
    Approach,
    #[serde(rename = "add_status_effect")]
    AddStatusEffect,
    #[serde(rename = "remove_status_effect")]
    RemoveStatusEffect,
    #[serde(rename = "abnormal_move")]
    AbnormalMove,
    #[serde(rename = "attach_pc")]
    AttachPc,
    #[serde(rename = "detach_pc")]
    DetachPc,
    #[serde(rename = "suicide")]
    Suicide,
    #[serde(rename = "teleport")]
    Teleport,
    #[serde(rename = "heal")]
    Heal,
    #[serde(rename = "reset_cooldown")]
    ResetCooldown,
    #[serde(rename = "fill_identity_gauge")]
    FillIdentityGauge,
    #[serde(rename = "change_contents_gauge")]
    ChangeContentsGauge,
    #[serde(rename = "add_skill_stack")]
    AddSkillStack,
    #[serde(rename = "throw_projectile")]
    ThrowProjectile,
    #[serde(rename = "throw_projectile_position")]
    ThrowProjectilePosition,
    #[serde(rename = "physical_damage")]
    PhysicalDamage,
    #[serde(rename = "magical_damage")]
    MagicalDamage,
    #[serde(rename = "summon_npc")]
    SummonNpc,
    #[serde(rename = "summon_trap")]
    SummonTrap,
    #[serde(rename = "replace_npc")]
    ReplaceNpc,
    #[serde(rename = "percent_damage")]
    PercentDamage,
    #[serde(rename = "replenish_hp")]
    ReplenishHp,
    #[serde(rename = "replenish_mp")]
    ReplenishMp,
    #[serde(rename = "kill")]
    Kill,
    #[serde(other)]
    Unknown
}

#[derive(Debug, Default, PartialEq, Eq, Hash, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum SkillEffectItemType {
    #[default]
    None,
    #[serde(rename = "na")]
    NotApplicable,
    #[serde(rename = "useup")]
    UseUp
}