use crate::deserializer::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Clone)]
pub struct RawCombatEffect {
    pub effects: Vec<CombatEffectDetail>,
}

#[derive(Debug, Default, Deserialize, Clone)]
pub struct CombatEffectDetail {
    pub ratio: u32,
    pub cooldown: u32,
    pub conditions: Vec<CombatEffectCondition>,
    pub actions: Vec<CombatEffectAction>,
}

#[derive(Debug, Default, Deserialize, Clone)]
#[serde(rename_all = "camelCase", default)]
pub struct CombatEffectCondition {
    #[serde(rename(deserialize = "type"))]
    pub condition_type: String,
    pub actor_type: String,
    pub arg: i32,
}

#[derive(Debug, Default, Deserialize, Clone)]
#[serde(rename_all = "camelCase", default)]
pub struct CombatEffectAction {
    pub action_type: CombatEffectActionType,
    pub actor_type: CombatEffectActorType,
    pub args: Vec<i32>,
}

#[derive(Debug, Default, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum CombatEffectActionType {
    #[default]
    None,
    ExecActiveEffectWhenDamage,
    ModifyDamage,
    ModifyReactiveDamage,
    ModifyDamageShieldMultiplier,
    ModifyPenetration,
    ExecAfterEffect,
    ModifyCriticalMultiplier,
    ModifyFinalDamage,
    ModifyCriticalRatio,
    ModifyPenetrationWhenCritical,
    ExecActiveEffectWhenKill,
    ExecWhenBeKilled,
    ExecReactiveEffectWhenDamage,
    ExecStartSkill,
    ExecAfterSkill,
    ExecReactiveEffectWhenCritical,
    ExecActiveEffectWhenCritical,
    ModifyPenetrationAddend,
    ModifyPenetrationAddendWhenCritical,
    ModifyEvolutionDamageMultiplier,
    ApplyHeal,
    ModifyParalyzationPoint,
    ModifyDamageWhenCritical,
    ExecWhenCounter
}

#[derive(Debug, Default, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum CombatEffectActorType {
    #[default]
    None,
    #[serde(rename = "self")]
    ActorTypeSelf,
    Target,
    Caster
}