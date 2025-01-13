use std::{fmt, rc::Rc};

use rustc_hash::FxHashMap;

use crate::{data::{json::*, models::{skill::{SkillGrade, SkillType}, skill_buff::*, skill_descriptor::SkillDescriptor}}, misc::*, skill::create_skill_map};

use super::class::Class;

pub struct Skills<'a> {
    pub by_name: FxHashMap<&'a str, Vec<Skill<'a>>>,
    pub by_id: FxHashMap<u32, Skill<'a>>,
    pub by_class_id: FxHashMap<u32, Vec<Skill<'a>>>,
    pub by_group_id: FxHashMap<u32, Vec<Skill<'a>>>
}

impl<'a> Skills<'a> {
    pub fn new() -> Self {
        let class_map = create_shared_class_map(&RAW_CLASS_BY_ID_MAP);
        
        create_skill_map(
            &RAW_SKILL_MAP,
            &HYPER_AWAKENING_SKILL_IDS,
            &class_map)
    }

    pub fn get_class_skill_by_name(&self, name: &str) -> Option<Rc<ClassSkill<'a>>> {

        if let Some(skills) = self.by_name.get(name) {

            if let Some(skill) = skills.first() {

                if let Skill::ClassSkill(skill) = skill {
                    return Some(skill.clone());
                }

                return None
            }

            return None
        }

        None
    }
}

#[derive(Debug, Default, Clone)]
pub enum Skill<'a> {
    #[default]
    None,
    UnknownSkill(Rc<UnknownSkill<'a>>),
    UnspecifiedSkill(Rc<UnspecifiedSkill<'a>>),
    ClassSkill(Rc<ClassSkill<'a>>),
    SkillWithSource(Rc<SkillWithSource<'a>>),
}

impl fmt::Display for Skill<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Skill::None => write!(f, "None"),
            Skill::UnspecifiedSkill(skill) => write!(f, "({:?})", skill),
            Skill::ClassSkill(skill) => write!(f, "({:?})", skill),
            Skill::SkillWithSource(skill) => write!(f, "({:?})", skill),
            _ => write!(f, "TO-DO")
        }
    }
}

#[derive(Debug, Default)]
pub struct UnknownSkill<'a> {
    pub id: u32,
    pub cooldown: Option<u16>,
    pub skill_type: SkillType,
    pub grade: SkillGrade,
    pub icon: Option<&'a str>,
}

#[derive(Debug, Default)]
pub struct UnspecifiedSkill<'a> {
    pub id: u32,
    pub name: Option<&'a str>,
    pub desc: Option<String>,
    pub icon: Option<&'a str>,
    pub cooldown: Option<u16>,
    pub skill_type: SkillType,
    pub grade: SkillGrade,
}

#[derive(Debug, Default)]
pub struct ClassSkill<'a> {
    pub id: u32,
    pub descriptor: Option<&'a SkillDescriptor>,
    pub name: Option<&'a str>,
    pub desc: Option<String>,
    pub icon: Option<&'a str>,
    pub class: Rc<Class<'a>>,
    pub cooldown: Option<u16>,
    pub skill_type: SkillType,
    pub grade: SkillGrade,
}

impl<'a> ClassSkill<'a> {
    pub fn get_party_buffs(&self) -> Vec<RawSkillBuff> {
        let skill_buff_ids = RAW_SKILL_TO_SKILL_BUFF_MAP.get(&self.id).cloned().unwrap_or_default();
        let mut skill_buffs = vec![];

        for skill_buff_id in skill_buff_ids {
            let skill_buff = RAW_SKILL_BUFF_MAP.get(&skill_buff_id).cloned().unwrap();

            if skill_buff.target != SkillBuffTarget::SelfParty {
                continue;
            }

            skill_buffs.push(skill_buff);
        }

        skill_buffs
    }

    pub fn get_skill_buffs(&self) -> Vec<RawSkillBuff> {
        let skill_buff_ids = RAW_SKILL_TO_SKILL_BUFF_MAP.get(&self.id).cloned().unwrap_or_default();
        let mut skill_buffs = vec![];

        for skill_buff_id in skill_buff_ids {
            let skill_buff = RAW_SKILL_BUFF_MAP.get(&skill_buff_id).cloned().unwrap();
            skill_buffs.push(skill_buff);
        }

        skill_buffs
    }
}
#[derive(Debug, Default)]
pub struct SkillWithSource<'a> {
    pub skill: Skill<'a>,
    pub has_summon_source: bool,
    pub source_skills: Vec<Skill<'a>>,
}