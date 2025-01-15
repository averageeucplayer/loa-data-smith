use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Clone)]
pub struct RawSkillFeatureLevel<'a>
{
    #[serde(borrow)]
    pub tripods: HashMap<u8, RawSkillFeatureTripod<'a>>,
}

#[derive(Debug, Default, Deserialize, Clone)]
pub struct RawSkillFeatureTripod<'a> {
    pub name: &'a str,
    pub entries: Vec<RawSkillFeatureOption>,
}

#[derive(Debug, Default, Deserialize, Clone)]
#[serde(rename_all = "camelCase", default)]
pub struct RawSkillFeatureOption {
    #[serde(rename(deserialize = "type"))]
    pub effect_type: SkillFeatureEffectType,
    pub level: u16,
    #[serde(rename(deserialize = "paramtype"))]
    pub param_type: SkillFeatureParamType,
    pub param: Vec<i32>,
}

#[derive(Debug, Default, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum SkillFeatureParamType {
    #[default]
    None,
    Absolute,
    Relative
}

#[derive(Debug, Default, PartialEq, Eq, Hash, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum SkillFeatureEffectType {
    #[default]
    None,
    ChangeBuffParam,
    ReduceDefaultCooldown,
    ChangeCost,
    ChangeSkillEffectBonus,
    AddChainSkillEffect,
    AddChainCombatEffect,
    RemoveChainCombatEffect,
    ChangeSkillChainInfo,
    ChangeInstanceSkillEffectInfo,
    EnableNotify,
    ChangeDamAttr,
    ChangeDamValue,
    ChangeDamCritical,
    ChangeBuffDuration,
    ChangeBuffStat,
    ChangeLayer,
    ChangeProjectileMaxTargetHitCount,
    ChangeIdentityProcPvpValue,
    ChangeIdentityProcValue,
    AddChainCombatCffect,
    ChangeCombatEffectArg,
    ChangeSkillEffectIdentityProcPvpInfo,
    ChangeSkillEffectIdentityProcInfo,
    ChangeSkillBookType,
    ChangeSkillEffectAiPoint,
    ChangeDamCriticalRate,
    ChangeAbnormal,
    ChangeAttackStageSpeed,
    ChangeAreaRange,
    ChangePartsAttackAttr,
    ChangeHitted,
    ChangeSkillMoveSpeed,
    ChangeStageCollision,
    ChangeAccumulateDamRate,
    ChangeIdentityCategory,
    ChangeMoveDist,
    AddSkillBuff,
    ChangeStackMaxCount,
    ChangeStackChargeTime,
    change_behit_move_info,
    change_summon_npc_life_time,
    change_targeting,
    enable_stage_buff,
    change_skill_bonus,
    change_projection,
    change_stage_speed,
    change_push_pvp_info,
    change_projectile_bank_data_addend,
    remove_chain_skill_effect,
    change_summon_npc_skill_usable_tick,
    change_area_angle,
    change_push_info,
    enable_dir_change,
    change_skill_start_stage,
    recover_used_cost,
    change_max_range,
    change_skill_normal_info,
    change_projectile_dist,
    change_summon_npc_skill_id,
    change_projectile_speed,
    change_skill_slot_visible_effect,
    change_skill_view,
    enable_identity_event,
    change_dam_addend,
    add_stage_buff,
    change_forced_critical,
    change_stage_dir_rate,
    change_projection_skill_effect_id,
    change_skill_effect_dir_target,
    change_skill_invisibility,
    change_summon_trap_count,
    change_charge_scale,
    change_attack_mask,
    add_buff_stat,
    change_skill_effect_identity_proc_replace_info,
    change_skill_effect_identity_proc_replace_pvp_info,
    change_summon_npc_spawn_buff_id,
    change_skill_constraint,
    change_summon_npc_skill_use_order,
    reduce_active_cooldown,
    change_aim_target_max_range
}