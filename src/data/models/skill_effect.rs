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
    #[serde(rename = "add_pickup_prop_gauge")]
    AddPickupPropGauge,
    #[serde(rename = "change_npc_faction")]
    ChangeNpcFaction,
    #[serde(rename = "add_status_effect_duration")]
    AddStatusEffectDuration,
    #[serde(rename = "change_prop_state")]
    ChangePropState,
    #[serde(rename = "decrease_status_effect")]
    DecreaseStatusEffect,
    #[serde(rename = "summon_prop")]
    SummonProp,
    #[serde(rename = "fixed_damage")]
    FixedDamage,
    #[serde(rename = "ai_point")]
    AiPoint,
    #[serde(rename = "exec_skill_effect")]
    ExecSkillEffect,
    #[serde(rename = "mass_teleport")]
    MassTeleport,
    Portal,
    #[serde(rename = "send_ai_signal")]
    SendAiSignal,
    #[serde(rename = "reset_life_skill")]
    ResetLifeSkill,
    #[serde(rename = "active_trap")]
    ActiveTrap,
    #[serde(rename = "command_summon_npc")]
    CommandSummonNpc,
    #[serde(rename = "add_target_marking")]
    AddTargetMarking,
    #[serde(rename = "remove_target_marking")]
    RemoveTargetMarking,
    #[serde(rename = "throw_projectile_target")]
    ThrowProjectileTarget,
    #[serde(rename = "detection_trap")]
    DetectionTrap,
    #[serde(rename = "remove_projectile")]
    RemoveProjectile,
    #[serde(rename = "drop_ether")]
    DropEther,
    #[serde(rename = "despawn_trap")]
    DespawnTrap,
    #[serde(rename = "condition_invoke_effect")]
    ConditionInvokeEffect,
    #[serde(rename = "fill_ultimate_point")]
    FillUltimatePoint,
    #[serde(rename = "parts_attack")]
    PartsAttack,
    #[serde(rename = "life_vessel")]
    LifeVessel,
    #[serde(rename = "despawn_prop")]
    DespawnProp,
    #[serde(rename = "remove_carrying_prop")]
    RemoveCarryingProp,
    #[serde(rename = "replace_pick_up_prop")]
    ReplacePickUpProp,
    #[serde(rename = "magic_damage_dist")]
    MagicDamageDist,
    #[serde(rename = "remove_status_effect_stack_greater")]
    RemoveStatusEffectStackGreater,
    #[serde(rename = "change_status_effect")]
    ChangeStatusEffect,
    #[serde(rename = "revive_part")]
    RevivePart,
    #[serde(rename = "create_part")]
    CreatePart,
    #[serde(rename = "destroy_parts")]
    DestroyParts,
    #[serde(rename = "add_battle_field_point")]
    AddBattleFieldPoint,
    #[serde(rename = "remove_status_effect_stack_less")]
    RemoveStatusEffectStackLess,
    #[serde(rename = "disable_prop")]
    DisableProp,
    #[serde(rename = "add_provoke_resist")]
    AddProvokeResist,
    #[serde(rename = "reverse_ruin_point")]
    ReverseRuinPoint,
    #[serde(rename = "add_status_effect_each")]
    AddStatusEffectEach,
    #[serde(rename = "mirror_npc_target")]
    MirrorNpcTarget,
    #[serde(rename = "mirror_npc")]
    MirrorNpc,
    #[serde(rename = "detach_summon_npc")]
    DetachSummonNpc,
    #[serde(rename = "attach_summon_npc")]
    AttachSummonNpc,
    #[serde(rename = "add_fury_time")]
    AddFuryTime,
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