#![allow(unused_imports)] 

pub mod models;
pub mod deserializer;
pub mod utils;
pub mod data;
pub mod examples;

#[macro_use]
pub mod macros;

use std::collections::{HashMap, HashSet};
use clipboard::{ClipboardProvider, ClipboardContext};

use data::{json::*, models::{npc::*, skill_buff::{SkillBuffCategory, SkillBuffSetName}}};
use models::{class::{ClassWithSkills, Classes}, cube::Cube, field_bosses::FieldBosses, npc::*, raid::{abyssal_dungeon::AbyssalDungeon, guardian_raid::{chaos_guardian_purification::PurificationLevelOne, GuardianRaid}, kazeros_raid::{self, KazerosRaid}, legion_raid::LegionRaid}, skill::{Skill, Skills}};
use utils::*;

fn main() {
    let skills = Skills::new();

    if let Some(skill) = skills.get_class_skill_by_name("Divine Justice") {
        let skill_buff_id = RAW_SKILL_TO_SKILL_BUFF_MAP.get(&skill.id).unwrap().first().unwrap();
        let skill_buff = RAW_SKILL_BUFF_MAP.get(&skill_buff_id);

        println!("{:?}", skill)
    }

}
