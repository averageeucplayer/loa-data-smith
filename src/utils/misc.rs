use std::{error::Error, rc::Rc, str::FromStr};

use data::models::skill::RawSkill;
use models::{class::Class, skill::*};
use rustc_hash::FxHashMap;
use serde::Serialize;

use crate::*;

pub fn create_empty_npc_skill_map() {
    let mut npc_skill_map: HashMap<u32, Vec<u32>> = HashMap::new();

    for npc_id in RAW_NPC_MAP.keys() {
        npc_skill_map.entry(*npc_id).or_default();
    }

    save_json(&npc_skill_map, "src/data/json/NpcSkills.json").unwrap();
}

pub fn extract_distinct_values_and_copy_to_clipboard<T, F, I>(
    map: I, 
    extract_fn: F
) -> Result<(), Box<dyn std::error::Error>> 
where
    I: IntoIterator<Item = T>,
    F: Fn(&T) -> String,
{
    let distinct: HashSet<String> = map.into_iter().map(|pr| extract_fn(&pr)).collect();
    let combined = distinct.into_iter()
        .map(|str| format!("{},", capitalize_first(str)))
        .collect::<Vec<String>>()
        .join("\n");
    
    copy_to_clipboard(combined)?;

    Ok(())
}

pub fn create_shared_class_map<'a>(class_map: &FxHashMap<u32, Class<'a>>) -> FxHashMap<u32, Rc<Class<'a>>> {
    class_map.iter()
        .map(|(&key, class)| (key, Rc::new(class.clone())))
        .collect()
}

pub fn vec_to_option<T>(vec: &Vec<T>) -> Option<&Vec<T>> {
    Some(vec).filter(|v| !v.is_empty())
}

pub fn create_esther_by_npc_id_map<'a>(esthers: &'a Vec<RawEsther<'a>>) -> FxHashMap<u32, Rc<RawEsther<'a>>> {
    esthers
        .clone()
        .into_iter()
        .flat_map(|esther| {
            let npc_ids = esther.npc_ids.clone();
            let shared_esther = Rc::new(esther);

            npc_ids.into_iter().map(move |npc_id| (npc_id.clone(), shared_esther.clone()))
        })
        .collect()
}

pub fn capitalize_first(str: String) -> String {
    let mut char = str.chars();
    match char.next() {
        Some(first) => first.to_uppercase().chain(char).collect(),
        None => String::new(),
    }
}

pub fn copy_to_clipboard(text: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut ctx: ClipboardContext = ClipboardProvider::new()?;
    ctx.set_contents(text)?;
    Ok(())
}

pub fn print_skill_name_duplicate_summary() {
    let mut name_count: HashMap<String, usize> = HashMap::new();

    for skill in RAW_SKILL_MAP.values() {
        let name = skill.name.unwrap_or("Unknown").to_string();
        *name_count.entry(name).or_insert(0) += 1;
    }

    let mut sorted_name_count: Vec<(String, usize)> = name_count.into_iter().collect();
    sorted_name_count.sort_by(|a, b| b.1.cmp(&a.1)); 

    for (name, count) in sorted_name_count.iter().filter(|&(_, count)| *count > 1) {
        println!("{}: {}", name, count);
    }
}

pub fn generate_skill_buff_to_skill_map() -> HashMap<u32, u32> {
    let mut skill_buff_id_to_skill_id_map  = HashMap::new();

    for (&skill_buff_id, skill_buff) in RAW_SKILL_BUFF_MAP.iter() {

        if let Some(skill_id) = skill_buff.source_skills.first() {

            if let Some(skill) = RAW_SKILL_MAP.get(skill_id) {
                if let Some(first_skill_id) = skill.summon_source_skills.first() {
                    if RAW_SKILL_MAP.get(first_skill_id).is_some() {
                        skill_buff_id_to_skill_id_map.insert(skill_buff_id, *first_skill_id);
                    }
                }
                else {
                    skill_buff_id_to_skill_id_map.insert(skill_buff_id, *skill_id);
                }
            }

            continue;
        }

        let mut skill_id = skill_buff_id / 10;

        if RAW_SKILL_MAP.get(&skill_id).is_some() {
            skill_buff_id_to_skill_id_map.insert(skill_buff_id, skill_id);

            continue;
        }

        skill_id = (skill_buff_id / 100) * 10;

        if RAW_SKILL_MAP.get(&skill_id).is_some() {
            skill_buff_id_to_skill_id_map.insert(skill_buff_id, skill_id);

            continue;
        }

        match skill_buff.unique_group {
            SkillBuffUniqueGroup::Unknown(id) => {
                skill_id = id / 10;

                if RAW_SKILL_MAP.get(&skill_id).is_some() {
                    skill_buff_id_to_skill_id_map.insert(skill_buff_id, skill_id);
                }
            },
            _ => {
                continue;
            }
        }
   }

   skill_buff_id_to_skill_id_map
}

pub fn generate_skill_buff_to_skill_json(output: &str) -> Result<(), Box<dyn Error>> {
    let map = generate_skill_buff_to_skill_map();

    let json_string = serde_json::to_string_pretty(&map)?;
        
    std::fs::write(output, &json_string)?;

    Ok(())
}

pub fn save_json<T>(data: &T, output: &str) -> Result<(), Box<dyn Error>>
    where T: Serialize {
    let json_string = serde_json::to_string_pretty(&data)?;
        
    std::fs::write(output, &json_string)?;

    Ok(())
}