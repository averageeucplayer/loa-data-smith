use std::collections::HashSet;

use once_cell::sync::Lazy;
use rustc_hash::FxHashMap;

use crate::{models::class::{Class, Specialisation}, RawEsther, RawNpc};

use super::models::{engraving::RawEngraving, skill::RawSkill, skill_buff::RawSkillBuff, skill_descriptor::SkillDescriptor, skill_effect::RawSkillEffect};

pub static RAW_NPC_MAP: Lazy<FxHashMap<u32, RawNpc>> = Lazy::new(|| {
    let json_bytes = include_bytes!("./Npc.json");
    serde_json::from_slice(json_bytes).unwrap()
});

pub static ESTHERS: Lazy<Vec<RawEsther>> = Lazy::new(|| {
    let json_bytes = include_bytes!("./Esther.json");
    serde_json::from_slice(json_bytes).unwrap()
});

pub static RAW_SKILL_MAP: Lazy<FxHashMap<u32, RawSkill>> = Lazy::new(|| {
    let json_bytes = include_bytes!("./Skill.json");
    serde_json::from_slice(json_bytes).unwrap()
});

pub static HYPER_AWAKENING_SKILL_IDS: Lazy<HashSet<u32>> = Lazy::new(|| {
    let json_bytes = include_bytes!("./HyperAwakeningSkillIds.json");
    serde_json::from_slice(json_bytes).unwrap()
});

pub static RAW_CLASS_BY_ID_MAP: Lazy<FxHashMap<u32, Class>> = Lazy::new(|| {
    let json_bytes = include_bytes!("./Class.json");
    serde_json::from_slice(json_bytes).unwrap()
});

pub static RAW_CLASS_BY_NAME_MAP: Lazy<FxHashMap<String, Class>> = Lazy::new(|| {
    let json_bytes = include_bytes!("./Class.json");
    let map: FxHashMap<u32, Class> = serde_json::from_slice(json_bytes).unwrap();
    map.into_iter().map(|(_, class)| (class.name.to_string(), class)).collect()
});

pub static RAW_SKILL_BUFF_MAP: Lazy<FxHashMap<u32, RawSkillBuff>> = Lazy::new(|| {
    let json_bytes = include_bytes!("./SkillBuff.json");
    serde_json::from_slice(json_bytes).unwrap()
});

pub static RAW_SKILL_EFFECT_MAP: Lazy<FxHashMap<u32, RawSkillEffect>> = Lazy::new(|| {
    let json_bytes = include_bytes!("./SkillEffect.json");
    serde_json::from_slice(json_bytes).unwrap()
});

pub static RAW_SPECIALISATION_MAP: Lazy<FxHashMap<u32, Specialisation>> = Lazy::new(|| {
    let json_bytes = include_bytes!("./Specialisation.json");
    serde_json::from_slice(json_bytes).unwrap()
});

pub static RAW_ENGRAVING_MAP: Lazy<FxHashMap<u32, RawEngraving>> = Lazy::new(|| {
    let json_bytes = include_bytes!("./Ability.json");
    serde_json::from_slice(json_bytes).unwrap()
});

pub static RAW_SKILL_TO_SKILL_BUFF_MAP: Lazy<FxHashMap<u32, Vec<u32>>> = Lazy::new(|| {
    let json_bytes = include_bytes!("./SkillToSkillBuff.json");
    serde_json::from_slice(json_bytes).unwrap()
});

pub static RAW_SKILL_BUFF_TO_SKILL_MAP: Lazy<FxHashMap<u32, u32>> = Lazy::new(|| {
    let json_bytes = include_bytes!("./SkillBuffToSkill.json");
    serde_json::from_slice(json_bytes).unwrap()
});

pub static SKILL_DESCRIPTOR_MAP: Lazy<FxHashMap<u32, SkillDescriptor>> = Lazy::new(|| {
    let json_bytes = include_bytes!("./SkillDescriptor.json");
    serde_json::from_slice(json_bytes).unwrap()
});
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_maps() {
        assert_eq!(RAW_NPC_MAP.len(), 31749);
        assert_eq!(ESTHERS.len(), 14);
        assert_eq!(RAW_SKILL_MAP.len(), 20953);
        assert_eq!(RAW_CLASS_BY_ID_MAP.len(), 36);
        assert_eq!(RAW_CLASS_BY_NAME_MAP.len(), 36);
        assert_eq!(RAW_SKILL_BUFF_MAP.len(), 21137);
        assert_eq!(RAW_SKILL_EFFECT_MAP.len(), 84715);
        assert_eq!(RAW_ENGRAVING_MAP.len(), 249);
        assert_eq!(RAW_SPECIALISATION_MAP.len(), 52);
    }

}