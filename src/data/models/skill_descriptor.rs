use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SkillDescriptor {
    pub name: Option<String>,
    pub class_name: Option<String>,
    pub is_identity: bool,
    pub is_awakening: bool,
    pub is_hyper_awakening_technique: bool,
    pub is_hyper_awakening: bool,
    pub has_marking_debuff: bool,
    pub has_attack_power_buff: bool,
    pub has_synergy_buff: bool,
    pub has_shield_buff: bool,
    pub has_damage_reduction_buff: bool,
    pub is_heal: bool
}