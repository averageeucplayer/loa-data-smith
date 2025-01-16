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
    pub params: Vec<i32>,
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
    ChangeBehitMoveInfo,
    ChangeSummonNpcLifeTime,
    ChangeTargeting,
    EnableStageBuff,
    ChangeSkillBonus,
    ChangeProjection,
    ChangeStageSpeed,
    ChangePushPvpInfo,
    ChangeProjectileBankDataAddend,
    RemoveChainSkillEffect,
    ChangeSummonNpcSkillUsableTick,
    ChangeAreaAngle,
    ChangePushInfo,
    EnableDirChange,
    ChangeSkillStartStage,
    RecoverUsedCost,
    ChangeMaxRange,
    ChangeSkillNormalInfo,
    ChangeProjectileDist,
    ChangeSummonNpcSkillId,
    ChangeProjectileSpeed,
    ChangeSkillSlotVisibleEffect,
    ChangeSkillView,
    EnableIdentityEvent,
    ChangeDamAddend,
    AddStageBuff,
    ChangeForcedCritical,
    ChangeStageDirRate,
    ChangeProjectionSkillEffectId,
    ChangeSkillEffectDirTarget,
    ChangeSkillInvisibility,
    ChangeSummonTrapCount,
    ChangeChargeScale,
    ChangeAttackMask,
    AddBuffStat,
    ChangeSkillEffectIdentityProcReplaceInfo,
    ChangeSkillEffectIdentityProcReplacePvpInfo,
    ChangeSummonNpcSpawnBuffId,
    ChangeSkillConstraint,
    ChangeSummonNpcSkillUseOrder,
    ReduceActiveCooldown,
    ChangeAimTargetMaxRange
}