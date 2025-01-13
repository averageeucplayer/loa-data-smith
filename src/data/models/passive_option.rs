use std::u8;

use chrono::Duration;
use serde::{Deserialize, Serialize};
use crate::deserializer::*;
use strum_macros::{Display, EnumString};

#[derive(Debug, Default, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PassiveOption {
    #[serde(rename(deserialize = "type"))]
    pub option_type: PassiveOptionType,
    pub key_stat: PassiveOptionKeyStat,
    pub key_index: u32,
    pub value: i32,
}

#[derive(Debug, Default, PartialEq, Eq, Hash, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum PassiveOptionType {
    #[default]
    None,
    Stat,
    #[serde(rename = "combat_effect")]
    CombatEffect,
    #[serde(rename = "skill_damage")]
    SkillDamage,
    #[serde(rename = "skill_group_damage")]
    SkillGroupDamage,
    #[serde(rename = "skill_cooldown_reduction")]
    SkillCooldownReduction,
    #[serde(rename = "skill_group_cooldown_reduction")]
    SkillGroupCooldownReduction,
    #[serde(rename = "mana_reduction")]
    ManaReduction,
    #[serde(rename = "ability_point_passive")]
    AbilityPointPassive,
    #[serde(rename = "attack_power_amplify_multiplier")]
    AttackPowerAmplifyMultiplier,
    #[serde(rename = "class_option")]
    ClassOption,
    #[serde(rename = "life_bonus_type_success")]
    LifeBonusTypeCuccess,
    #[serde(rename = "life_casting_speed")]
    LifeCastingSpeed,
    #[serde(rename = "life_durability_bonus")]
    LifeDurabilityBonus,
    #[serde(rename = "ability_point")]
    AbilityPoint
}

#[derive(Debug, Default, PartialEq, Eq, Hash, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum PassiveOptionKeyStat {
    #[default]
    None,
    #[serde(rename = "attack_power_rate")]
    AttackPowerRate,
    #[serde(rename = "attack_power_rate_x")]
    AttackPowerRateX,
    #[serde(rename = "weapon_dam")]
    WeaponDamage,
    #[serde(rename = "def")]
    Defense,
    #[serde(rename = "def_x")]
    DefenseX,
    #[serde(rename = "attack_speed_rate")]
    AttackSpeedRate,
    #[serde(rename = "move_speed_rate")]
    MoveSpeedRate,
    #[serde(rename = "paralyzation_point_rate")]
    ParalyzationPointRate,
    #[serde(rename = "mastery_x")]
    MasteryX,
    #[serde(rename = "mastery")]
    Mastery,
    #[serde(rename = "cooldown_reduction")]
    CooldownReduction,
    #[serde(rename = "rapidity_x")]
    RapidityX,
    #[serde(rename = "rapidity")]
    Rapidity,
    #[serde(rename = "max_mp")]
    MaxMp,
    #[serde(rename = "max_mp_x")]
    MaxMpX,
    #[serde(rename = "normal_mp_recovery")]
    NormalMpRecovery,
    #[serde(rename = "combat_mp_recovery")]
    CombatMpRecovery,
    #[serde(rename = "normal_mp_recovery_rate")]
    NormalMpRecoveryRate,
    #[serde(rename = "combat_mp_recovery_rate")]
    CombatMpRecoveryRate,
    #[serde(rename = "resource_recovery_rate")]
    ResourceRecoveryRate,
    #[serde(rename = "con")]
    Con,
    #[serde(rename = "con_x")]
    ConX,
    #[serde(rename = "max_hp")]
    MaxHp,
    #[serde(rename = "max_hp_x")]
    MaxHpX,
    #[serde(rename = "max_hp_x_x")]
    MaxHpXX,
    #[serde(rename = "normal_hp_recovery")]
    NormalHpRecovery,
    #[serde(rename = "combat_hp_recovery")]
    CombatHpRecovery,
    #[serde(rename = "normal_hp_recovery_rate")]
    NormalHpRecoveryRate,
    #[serde(rename = "combat_hp_recovery_rate")]
    CombatHpRecoveryRate,
    #[serde(rename = "self_recovery_rate")]
    SelfRecoveryRate,
    #[serde(rename = "drain_hp_dam_rate")]
    DrainHpDamageRate,
    #[serde(rename = "vitality")]
    Vitality,
    #[serde(rename = "magical_inc_rate")]
    MagicalIncRate,
    #[serde(rename = "endurance")]
    Endurance,
    #[serde(rename = "endurance_x")]
    EnduranceX,
    #[serde(rename = "vehicle_move_speed_rate")]
    VehicleMoveSpeedRate,
    #[serde(rename = "move_speed")]
    MoveSpeed,
    #[serde(rename = "critical_hit_rate")]
    CriticalHitRate,
    #[serde(rename = "criticalhit")]
    CriticalHit,
    #[serde(rename = "attack_power_sub_rate_1")]
    AttackPowerSubRate1,
    #[serde(rename = "attack_power_sub_rate_2")]
    AttackPowerSubRate2,
    #[serde(rename = "skill_damage_sub_rate_1")]
    SkillDamageSubRate1,
    #[serde(rename = "skill_damage_sub_rate_2")]
    SkillDamageSubRate2,
    #[serde(rename = "elements_dam_rate")]
    ElementsDamageRate,
    #[serde(rename = "str")]
    Strength,
    #[serde(rename = "str_x")]
    StrX,
    #[serde(rename = "agi")]
    Agility,
    #[serde(rename = "agi_x")]
    AgilityX,
    #[serde(rename = "int")]
    Intelligence,
    #[serde(rename = "int_x")]
    IntelligenceX,
    #[serde(rename = "char_attack_dam")]
    CharAttackDamage,
    #[serde(rename = "skill_damage_rate")]
    SkillDamageRate,
    #[serde(rename = "skill_damage_rate_x")]
    SkillDamageRateX,
    #[serde(rename = "hit_rate")]
    HitRate,
    #[serde(rename = "dodge_rate")]
    DodgeRate,
    #[serde(rename = "critical_dam_rate")]
    CriticalDamageRate,
    #[serde(rename = "awakening_dam_rate")]
    AwakeningDamageRate,
    #[serde(rename = "class_option")]
    ClassOption,
    #[serde(rename = "res_x")]
    ResX,
    #[serde(rename = "physical_inc_rate")]
    PhysicalIncRate,
    #[serde(rename = "physical_inc_sub_rate_1")]
    PhysicalIncSubRate1,
    #[serde(rename = "physical_inc_sub_rate_2")]
    PhysicalIncSubRate2,
    #[serde(rename = "magical_inc_sub_rate_1")]
    MagicalIncSubRate1,
    #[serde(rename = "magical_inc_sub_rate_2")]
    MagicalIncSubRate2,
    #[serde(rename = "ice_res_rate")]
    IceResistanceRate,
    #[serde(rename = "fire_res_rate")]
    FireResistanceRate,
    #[serde(rename = "def_pen_rate")]
    DefPenetrationRate,
    #[serde(rename = "res_pen_rate")]
    ResistancePenetrationRate,
    #[serde(rename = "prop_move_speed_rate")]
    PropMoveSpeedRate,
    #[serde(rename = "npc_species_archfiend_dam_rate")]
    NpcSpeciesArchfiendDamageRate,
    #[serde(rename = "npc_species_undead_dam_rate")]
    NpcSpeciesUndeadDamageRate,
    #[serde(rename = "npc_species_devil_dam_rate")]
    NpcSpeciesDevilDamageRate,
    #[serde(rename = "npc_species_plant_dam_rate")]
    NpcSpeciesPlantDamageRate,
    #[serde(rename = "npc_species_insect_dam_rate")]
    NpcSpeciesInsectDamageRate,
    #[serde(rename = "npc_species_spirit_dam_rate")]
    NpcSpeciesSpiritDamageRate,
    #[serde(rename = "ship_move_speed_rate")]
    ShipMoveSpeedRate,
    #[serde(rename = "ship_move_speed")]
    ShipMoveSpeed,
    #[serde(rename = "ship_booter_speed")]
    ShipBooterSpeed,
    #[serde(rename = "gold_rate")]
    GoldRate,
    #[serde(rename = "stigma_power_rate")]
    StigmaPowerRate,
    #[serde(rename = "evolution_dam_rate")]
    EvolutionDamageRate,
    #[serde(rename = "specialty")]
    Specialty,
    #[serde(rename = "specialty_x")]
    SpecialtyX,
    #[serde(rename = "oppression")]
    Oppression,
    #[serde(rename = "oppression_x")]
    OppressionX,
    #[serde(rename = "res")]
    Resistance,
    #[serde(rename = "ultimate_awakening_dam_rate")]
    UltimateAwakeningDamageRate,
    #[serde(rename = "attack_power_addend")]
    AttackPowerAddend,
    #[serde(rename = "attack_power_addend_2")]
    AttackPowerAddend2,
    #[serde(rename = "dam_reflection_rate")]
    DamageReflectionRate,
    #[serde(rename = "identity_value1")]
    IdentityValue1,
    #[serde(rename = "identity_value2")]
    IdentityValue2,
    #[serde(rename = "identity_value3")]
    IdentityValue3,
    #[serde(rename = "item_drop_rate")]
    ItemDropRate,
    #[serde(rename = "holy_res_rate")]
    HolyResistanceRate,
    #[serde(rename = "self_shield_rate")]
    SelfShieldRate,
    #[serde(rename = "electricity_res_rate")]
    ElectricityResistanceRate,
    #[serde(rename = "self_cc_time_rate")]
    SelfCcTimeRate,
    #[serde(rename = "earth_res_rate")]
    EarthResistanceRate,
    #[serde(rename = "dark_res_rate")]
    DarkResistanceRate,
    #[serde(rename = "critical_res_rate")]
    CriticalResistanceRate
}