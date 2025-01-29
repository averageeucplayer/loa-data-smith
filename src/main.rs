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

use data::{json::*, models::{npc::*, skill::{RawSkill, SkillGrade}, skill_buff::{self, SkillBuffCategory, SkillBuffSetName, SkillBuffUniqueGroup}, zone::Zone}};
use examples::party_skill_buffs_from_skill;
use misc::{*};
use models::{class::{ClassWithSkills, Classes}, cube::Cube, field_bosses::FieldBosses, npc::*, raid::{abyssal_dungeon::AbyssalDungeon, guardian_raid::{chaos_guardian_purification::PurificationLevelOne, GuardianRaid}, kazeros_raid::{self, KazerosRaid}, legion_raid::LegionRaid}, skill::{Skill, Skills}, skill_buff::{SkillBuff, SkillBuffs}, skill_effect::SkillEffects};
use rustc_hash::FxHashMap;
use serde::Serialize;
use utils::{skill_group::save_skill_descriptor, *};
use web_scrapper::{DefaultLostArkCodexWebScrapper, LostArkCodexWebScrapper};

fn main() {
    // examples::battle_items();

    save_skill_descriptor();

    // println!("{:#?}", hyper_awakening_technique_skills);

    // let mut awakening_skills: Vec<_> = RAW_SKILL_MAP
    //     .iter()
    //     .filter(|(id, sk)|  sk.grade == SkillGrade::Awakening && sk.class_id.filter(|id| *id == 105).is_some())
    //     .filter_map(|(id, pr)| pr.name.map(|name| (id, name)))
    //     .collect();

    // awakening_skills.sort_unstable_by(|a, b| a.0.cmp(b.0));
    // println!("{:#?}", awakening_skills);

    // let web_scrapper = DefaultLostArkCodexWebScrapper::new();

    // // let zone_name = web_scrapper.get_zone_name(37511).unwrap();
    // let mut new_zones: FxHashMap<u32, Zone> = FxHashMap::default();
    
    // for (id, zone) in ZONES_MAP.iter() {
    //     let zone_name = web_scrapper.get_zone_name(*id).unwrap();

    //     if let Some(zone_name) = zone_name.as_ref() {
    //         println!("id: {} name: {}", id, zone_name);
    //     }

    //     new_zones.entry(*id).or_insert_with(move || {
    //         Zone {
    //             id: *id,
    //             name: zone_name.map(|zone| zone),
    //             raid_difficulty: zone.raid_difficulty.clone()
    //         }
    //     });
    // }

    // let skill_feature = SKILL_FEATURE_MAP.get(&31200).unwrap();

    // let tripod = skill_feature.tripods
    //     .values()
    //     .find(|tripod| tripod.name == "Ink Brand").unwrap();

    // let entry = tripod.entries.first().unwrap();
    // let skill_buff_id = entry.params.get(3).unwrap();

    // println!("{}", skill_buff_id);

    

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
