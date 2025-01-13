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

use data::{json::*, models::{npc::*, skill_buff::{self, SkillBuffCategory, SkillBuffSetName, SkillBuffUniqueGroup}}};
use examples::party_skill_buffs_from_skill;
use misc::{*};
use models::{class::{ClassWithSkills, Classes}, cube::Cube, field_bosses::FieldBosses, npc::*, raid::{abyssal_dungeon::AbyssalDungeon, guardian_raid::{chaos_guardian_purification::PurificationLevelOne, GuardianRaid}, kazeros_raid::{self, KazerosRaid}, legion_raid::LegionRaid}, skill::{Skill, Skills}};
use utils::*;

fn main() {

    // generate_skill_buff_to_skill_json("src/data/json/SkillBuffToSkill.json").unwrap;
    // let map = generate_skill_buff_to_skill_map();
    // let mut skill_to_skill_buff_map: HashMap<u32, HashSet<u32>> = HashMap::new();

    // for (skill_buff_id, skill_id) in map {
    //     skill_to_skill_buff_map.entry(skill_id).or_default().insert(skill_buff_id);
    // }

    // save_json(&skill_to_skill_buff_map, "src/data/json/SkillToSkillBuff.json").unwrap();

    party_skill_buffs_from_skill();
}
