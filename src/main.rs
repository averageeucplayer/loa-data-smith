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
use models::{class::{ClassWithSkills, Classes}, cube::Cube, field_bosses::FieldBosses, npc::*, raid::{abyssal_dungeon::AbyssalDungeon, guardian_raid::{chaos_guardian_purification::PurificationLevelOne, GuardianRaid}, kazeros_raid::{self, KazerosRaid}, legion_raid::LegionRaid}, skill::{Skill, Skills}, skill_buff::{SkillBuff, SkillBuffs}};
use serde::Serialize;
use utils::*;

fn main() {

    for (skill_id, feature) in SKILL_FEATURE_MAP.iter() {

        if feature.tripods.is_empty() {

        }
    }

    // let map = generate_skill_buff_to_skill_map();
    // let mut skill_to_skill_buff_map: HashMap<u32, HashSet<u32>> = HashMap::new();

    // for (skill_buff_id, skill_ids) in map {

    //     for skill_id in skill_ids {
    //         skill_to_skill_buff_map.entry(skill_id).or_default().insert(skill_buff_id);
    //     }
    // }

    // save_json(&skill_to_skill_buff_map, "src/data/json/SkillToSkillBuff.json").unwrap();

    // for aa in RAW_COMBAT_EFFECT_MAP.iter() {

    // }

    // let mut set = HashSet::new();

    // for (id, skill_effect) in RAW_SKILL_EFFECT_MAP.iter() {

    //     if let Some(skill_buff) = RAW_SKILL_BUFF_MAP.get(id) {
    //         set.insert(id);
    //     }

    // }

    // print!("{:#?}", set.len());

    // let skills = Skills::new();

    // let skill_buffs = SkillBuffs::new();

    // let skill_buff = skill_buffs.by_name.get("Moonfall").unwrap();

    // let filtered: Vec<_> = skill_buffs.by_id.iter().filter_map(|(id, sb)| match sb.as_ref() {
    //     SkillBuff::Unknown(buff) => buff.name,
    //     _ => None
    // }).collect();

    // print!("{:#?}", filtered);

    // create_skill_group_map()

    // generate_skill_buff_to_skill_json("src/data/json/SkillBuffToSkill.json").unwrap;
    

    // party_skill_buffs_from_skill();
}
