use std::rc::Rc;

use rustc_hash::FxHashMap;
use serde::de;

use crate::{data::models::skill_effect::{self, SkillEffectType}, RAW_SKILL_EFFECT_MAP};

pub struct SkillEffects<'a> {
    pub by_id: FxHashMap<u32, Rc<SkillEffect<'a>>>,
    pub by_name: FxHashMap<&'a str, Rc<SkillEffect<'a>>>,
}

impl<'a> SkillEffects<'a> {
    pub fn new() -> Self {
        let mut skill_effect_by_id_map = FxHashMap::default();
        let mut skill_effect_by_name_map = FxHashMap::default();

        for (&id, raw_skill_effect) in RAW_SKILL_EFFECT_MAP.iter() {
            let skill_effect = match raw_skill_effect.effect_type {
                SkillEffectType::Heal => {
                    SkillEffect::Heal(id)
                },
                SkillEffectType::AddStatusEffect => {
                    let mut info = None;

                    if let Some(name) = raw_skill_effect.name {
                        info = Some(SkillEffectInfo {
                            name: name,
                            description: raw_skill_effect.description.clone(),
                            icon: raw_skill_effect.icon.unwrap()
                        })
                    }

                    SkillEffect::AddStatusEffect(id, info)
                },
                SkillEffectType::ThrowProjectile => {
                    let mut info = None;

                    if let Some(name) = raw_skill_effect.name {
                        info = Some(SkillEffectInfo {
                            name: name,
                            description: raw_skill_effect.description.clone(),
                            icon: raw_skill_effect.icon.unwrap()
                        })
                    }

                    SkillEffect::ThrowProjectile(id, info)
                },
                _ => SkillEffect::Unknown(id)
            };

            let shared = Rc::new(skill_effect);
            skill_effect_by_id_map.insert(id, shared.clone());

            if let Some(name) = raw_skill_effect.name {
                skill_effect_by_name_map.entry(name).or_insert_with(|| shared);
            }
        }
        
        Self {
            by_id: skill_effect_by_id_map,
            by_name: skill_effect_by_name_map
        }
    }
}

#[derive(Debug, Default)]
pub enum SkillEffect<'a> {
    #[default]
    None,
    Unknown(u32),
    Heal(u32),
    AddStatusEffect(u32, Option<SkillEffectInfo<'a>>),
    ThrowProjectile(u32, Option<SkillEffectInfo<'a>>),
}

#[derive(Debug, Default)]
pub struct SkillEffectInfo<'a> {
    pub name: &'a str,
    pub description: Option<String>,
    pub icon: &'a str,
}