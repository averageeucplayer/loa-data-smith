use std::{rc::Rc, str::FromStr};

use data::models::skill::RawSkill;
use misc::vec_to_option;
use models::{class::Class, skill::*};
use rustc_hash::FxHashMap;

use crate::*;

pub fn process_skill<'a>(
    raw_skill: &RawSkill<'a>,
    hyper_awakening_ids: &HashSet<u32>,
    class_map: &FxHashMap<u32, Rc<Class<'a>>>,
    has_summon_source: bool,
    source_skills: Vec<Skill<'a>>) -> Skill<'a> {

    if raw_skill.name.is_none()
        && raw_skill.desc.is_none()
        && raw_skill.class_id.is_none() {
        let skill = UnknownSkill {
            id: raw_skill.id,
            cooldown: raw_skill.cooldown,
            grade: raw_skill.grade,
            skill_type: raw_skill.skill_type,
            icon: raw_skill.icon,
        };

        return Skill::UnknownSkill(Rc::new(skill))
    }

    let skill = match (
        raw_skill.class_id,
        vec_to_option(&source_skills)) {
        (None, None) => {
            let skill = UnspecifiedSkill {
                id: raw_skill.id,
                name: raw_skill.name,
                desc: raw_skill.desc.clone(),
                icon: raw_skill.icon,
                cooldown: raw_skill.cooldown,
                grade: raw_skill.grade,
                skill_type: raw_skill.skill_type
            };

            Skill::UnspecifiedSkill(Rc::new(skill))
        },
        (None, Some(_)) => {
            let skill = UnspecifiedSkill {
                id: raw_skill.id,
                name: raw_skill.name,
                desc: raw_skill.desc.clone(),
                icon: raw_skill.icon,
                cooldown: raw_skill.cooldown,
                grade: raw_skill.grade,
                skill_type: raw_skill.skill_type
            };

            Skill::SkillWithSource(Rc::new(SkillWithSource {
                has_summon_source,
                skill: Skill::UnspecifiedSkill(Rc::new(skill)),
                source_skills,
            }))
        },
        (Some(class_id), None) => {
            let descriptor = SKILL_DESCRIPTOR_MAP.get(&raw_skill.id);

            let class = class_map.get(&class_id).cloned().unwrap();
            let class_skill = ClassSkill {
                id: raw_skill.id,
                name: raw_skill.name,
                desc: raw_skill.desc.clone(),
                icon: raw_skill.icon,
                class: class.clone(),
                descriptor,
                cooldown: raw_skill.cooldown,
                grade: raw_skill.grade,
                skill_type: raw_skill.skill_type
            };

            Skill::ClassSkill(Rc::new(class_skill))
        },
        (Some(class_id), Some(_)) => {
            let class = class_map.get(&class_id).cloned().unwrap();
            let descriptor = SKILL_DESCRIPTOR_MAP.get(&raw_skill.id);

            let class_skill = ClassSkill {
                id: raw_skill.id,
                name: raw_skill.name,
                desc: raw_skill.desc.clone(),
                icon: raw_skill.icon,
                class: class.clone(),
                descriptor,
                cooldown: raw_skill.cooldown,
                grade: raw_skill.grade,
                skill_type: raw_skill.skill_type
            };

            Skill::SkillWithSource(Rc::new(SkillWithSource {
                has_summon_source,
                skill: Skill::ClassSkill(Rc::new(class_skill)),
                source_skills,
            }))
        },
    };

    skill
}

pub fn create_skill_map<'a>(
    raw_skill_map: &FxHashMap<u32, RawSkill<'a>>,
    hyper_awakening_ids: &HashSet<u32>,
    class_map: &FxHashMap<u32, Rc<Class<'a>>>) -> Skills<'a> {
    let mut skill_by_name_map: FxHashMap<&'a str, Vec<Skill>> = FxHashMap::default();
    let mut skill_by_id_map: FxHashMap<u32, Skill> = FxHashMap::default();
    let mut skill_by_class_id_map: FxHashMap<u32, Vec<Skill>> = FxHashMap::default();
    let mut skill_by_group_id_map: FxHashMap<u32, Vec<Skill>> = FxHashMap::default();

    for (_, raw_skill) in raw_skill_map {
        let has_summon_source = !raw_skill.summon_source_skills.is_empty();
        let mut source_skill_ids = raw_skill.summon_source_skills.clone();
        source_skill_ids.extend(raw_skill.source_skills.clone());
        let mut source_skills = vec![];

        for skill_id in source_skill_ids {
            let raw_skill = raw_skill_map.get(&skill_id).unwrap();
            let skill = process_skill(
                raw_skill,
                hyper_awakening_ids,
                class_map,
                false,
                vec![]
            );
            skill_by_id_map.insert(raw_skill.id, skill.clone());
            source_skills.push(skill.clone());

            if let Some(name) = raw_skill.name {
                skill_by_name_map.entry(name).or_default().push(skill.clone());
            }

            if let Some(class_id) = raw_skill.class_id {
                skill_by_class_id_map.entry(class_id).or_default().push(skill.clone());
            }

            for group_id in raw_skill.groups.clone() {
                skill_by_group_id_map.entry(group_id).or_default().push(skill.clone());
            }
        }

        let skill = process_skill(raw_skill,
            hyper_awakening_ids,
            class_map,
            has_summon_source,
            source_skills);

        skill_by_id_map.insert(raw_skill.id, skill.clone());

        if let Some(name) = raw_skill.name {
            skill_by_name_map.entry(name).or_default().push(skill.clone());
        }

        if let Some(class_id) = raw_skill.class_id {
            skill_by_class_id_map.entry(class_id).or_default().push(skill.clone());
        }

        for group_id in raw_skill.groups.clone() {
            skill_by_group_id_map.entry(group_id).or_default().push(skill.clone());
        }
    }

    Skills {
        by_name: skill_by_name_map,
        by_id: skill_by_id_map,
        by_class_id: skill_by_class_id_map,
        by_group_id: skill_by_group_id_map
    }
}

