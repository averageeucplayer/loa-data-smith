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

use data::{json::*, models::{npc::*, skill::RawSkill, skill_buff::{self, SkillBuffCategory, SkillBuffSetName, SkillBuffUniqueGroup}}};
use examples::party_skill_buffs_from_skill;
use misc::{*};
use models::{class::{ClassWithSkills, Classes}, cube::Cube, field_bosses::FieldBosses, npc::*, raid::{abyssal_dungeon::AbyssalDungeon, guardian_raid::{chaos_guardian_purification::PurificationLevelOne, GuardianRaid}, kazeros_raid::{self, KazerosRaid}, legion_raid::LegionRaid}, skill::{Skill, Skills}, skill_buff::SkillBuffs};
use serde::Serialize;
use utils::*;

// pub fn create_skill_group_map() {

//     #[derive(Default, Debug, Serialize)]
//     struct Summary {
//         skill_count: u32,
//         class_count: u32,
//         classes: HashSet<String>,
//         skills: HashSet<String>
//     }

//     let mut skill_group_map: HashMap<u32, Summary> = HashMap::new();



//     for (id, skill) in RAW_SKILL_MAP.iter() {

//         for group_id in skill.groups.iter() {
//             let summary = skill_group_map.entry(*group_id).or_default();
//             let class = skill.class_id.and_then(|class_id| RAW_CLASS_BY_ID_MAP
//                 .get(&class_id)).map(|class| class.name).unwrap_or("");

//             if skill.class_id.is_some() {
//                 summary.classes.insert(class.to_string());
//             }

//             summary.skills.insert(skill.name.map(|name| format!("{} - {}", name.to_string(), class)).unwrap_or(id.to_string()));
//             summary.skill_count += 1;
//             summary.class_count = summary.classes.len() as u32;
//         }
        
//     }

//     let mut sorted_map: Vec<_> = skill_group_map.into_iter().collect();
//     sorted_map.sort_by_key(|(_, summary)| summary.skill_count);

//     let len = sorted_map.len();
//     let last_two = sorted_map.get(len.saturating_sub(3)..).unwrap();

//     let aaa = last_two.first().unwrap();
//     let bbb = last_two.last().unwrap();

//     let diff = aaa.1.skills.difference(&bbb.1.skills);

//     println!("{:#?}", diff);

//     // let sorted_dict: HashMap<u32, Summary> = sorted_map.into_iter().collect();

//     save_json(&sorted_map, "src/data/json/SkillGroups.json").unwrap();
// }

pub fn create_disguise_skills() {
    let mut skill_ids = HashSet::new();

    for (id, skill) in RAW_SKILL_MAP.iter() {

        if skill.class_id.is_none() && skill.name.is_some() {
            skill_ids.insert(id);
        }
        
    }

    save_json(&skill_ids, "src/data/json/DisguiseSkills.json").unwrap();
}

fn main() {

    // let skills = Skills::new();

    let skill_buffs = SkillBuffs::new();

    let skill_buff = skill_buffs.by_name.get("Moonfall").unwrap();

    print!("{:#?}", skill_buff);

    // create_skill_group_map()

    // generate_skill_buff_to_skill_json("src/data/json/SkillBuffToSkill.json").unwrap;
    // let map = generate_skill_buff_to_skill_map();
    // let mut skill_to_skill_buff_map: HashMap<u32, HashSet<u32>> = HashMap::new();

    // for (skill_buff_id, skill_id) in map {
    //     skill_to_skill_buff_map.entry(skill_id).or_default().insert(skill_buff_id);
    // }

    // save_json(&skill_to_skill_buff_map, "src/data/json/SkillToSkillBuff.json").unwrap();

    // party_skill_buffs_from_skill();
}
