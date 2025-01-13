use serde::Deserialize;

#[derive(Debug, Deserialize, Default)]
pub struct SkillDescriptor {
    pub is_awakening: bool,
    pub is_hyper_awakening_technique: bool,
    pub is_hyper_awakening: bool,
    pub has_marking_debuff: bool,
    pub has_attack_power_buff: bool,
}