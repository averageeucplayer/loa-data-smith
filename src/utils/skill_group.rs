use std::{error::Error, rc::Rc, str::FromStr};

use data::models::{skill::RawSkill, skill_descriptor::{self, SkillDescriptor}, skill_effect::SkillEffectType, skill_group::SkillGroup};
use models::{class::Class, skill::*};
use rustc_hash::FxHashMap;
use serde::Serialize;

use crate::*;

pub fn create_skill_descriptor<'a>() {
    let mut skill_descriptor_map: FxHashMap<u32, SkillDescriptor> = FxHashMap::default();

    for (id, skill) in RAW_SKILL_MAP.iter() {
        let mut skill_descriptor = SkillDescriptor::default();

        for group_id in skill.groups.iter() {
            match group_id {
                SkillGroup::AllClassAwakeningSkills => {
                    skill_descriptor.is_awakening = true;
                },
                SkillGroup::ArtistIdentitySkills => {
                    skill_descriptor.is_identity = true;
                },
                SkillGroup::BardIdentitySkills1 => {
                    skill_descriptor.is_identity = true;
                },
                SkillGroup::PaladinAwakeningAndHyperAwakeningTechniqueSkills => {

                },
                _ => {
                    
                },
            }
        }

        let skill_buff_ids = skill.name.map(|name| extract_skill_buffs_from_description(name)).unwrap_or_default();

        for skill_buff_id in skill_buff_ids {

            let skill_effect = RAW_SKILL_EFFECT_MAP.get(&skill_buff_id);

            if let Some(skill_effect) = skill_effect {
                if skill_effect.effect_type == SkillEffectType::Heal {
                    skill_descriptor.is_heal = true;
                }
            }
        }

        let skill_buff_ids = MANUAL_SKILL_TO_SKILL_BUFF_MAP.get(id).cloned().unwrap_or_default();

        for skill_buff_id in skill_buff_ids {

            let skill_effect = RAW_SKILL_EFFECT_MAP.get(&skill_buff_id);

            if let Some(skill_effect) = skill_effect {
                if skill_effect.effect_type == SkillEffectType::Heal {
                    skill_descriptor.is_heal = true;
                }
            }
        }

        skill_descriptor_map.insert(*id, skill_descriptor);
    }

    save_json(&skill_descriptor_map, "src/data/json/SkillDescriptor1.json").unwrap();
    // let skill_buffs = RAW_SKILL_TO_SKILL_BUFF_MAP.get(id);
}