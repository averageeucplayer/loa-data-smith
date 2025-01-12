use std::{rc::Rc, str::FromStr};

use data::models::skill::RawSkill;
use models::{class::Class, skill::*};
use rustc_hash::FxHashMap;

use crate::*;

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

pub fn populate_name_maps(
    raw_npc_map: &FxHashMap<u32, RawNpc>,
    esther_by_npc_id_map: &FxHashMap<u32, Rc<RawEsther>>,
    npc_by_name_map: &mut FxHashMap<String, Vec<RawNpc>>,
    boss_by_name_map: &mut FxHashMap<String, Vec<RawNpc>>,
    npc_by_grade_type_species_map: &mut FxHashMap<NpcKey, Vec<RawNpc>>
) {
    for (id, npc) in raw_npc_map.iter() {

        if esther_by_npc_id_map.get(id).is_some() {
            continue;
        }

        if let Some(name) = &npc.name {
            match &npc.grade {
                NpcGrade::Boss | NpcGrade::Commander |
                NpcGrade::EpicRaid | NpcGrade::Raid => {
                    let entry = boss_by_name_map
                        .entry(name.to_string())
                        .or_insert_with(|| Vec::new());

                    entry.push(npc.clone());
                  
                }
                NpcGrade::Elite | NpcGrade::Lucky |
                NpcGrade::Normal | NpcGrade::Named |
                NpcGrade::Underling | NpcGrade::Seed |
                NpcGrade::None => {
                    let entry = npc_by_name_map
                        .entry(name.to_string())
                        .or_insert_with(|| Vec::new());

                    entry.push(npc.clone());
                },
            }

            continue;
        }

        let npc_key = NpcKey {
            grade: npc.grade.clone(),
            npc_type: npc.npc_type.clone(),
            species: npc.species.clone(),
        };

        let entry = npc_by_grade_type_species_map
            .entry(npc_key)
            .or_insert_with(|| Vec::new());

        entry.push(npc.clone());
    }
}

pub fn create_struct_with_npcs(
    raw_npc_map: &FxHashMap<u32, RawNpc>,
    esthers: &Vec<RawEsther>,
) -> Npcs {
    let mut npc_by_grade_type_species_map: FxHashMap<NpcKey, Vec<RawNpc>> = FxHashMap::default();
    let mut npc_by_name_map: FxHashMap<String, Vec<RawNpc>> = FxHashMap::default();
    let mut boss_by_name_map: FxHashMap<String, Vec<RawNpc>> = FxHashMap::default();
    let mut npc_by_id_map: FxHashMap<u32, Npc> = FxHashMap::default();
    let mut new_npc_by_name_map: FxHashMap<String, Npc> = FxHashMap::default();
    let esther_by_npc_id_map = create_esther_by_npc_id_map(&esthers);

    populate_name_maps(
        &raw_npc_map,
        &esther_by_npc_id_map,
        &mut npc_by_name_map,
        &mut boss_by_name_map,
        &mut npc_by_grade_type_species_map
    );

    for esther in esthers {
        let esther_name = EstherName::from_str(&esther.name).unwrap();

        let first = esther.npc_ids.first().and_then(|id| raw_npc_map.get(id)).unwrap();
        
        let entry = EstherGroup {
            name: esther_name,
            grade: first.grade,
            npc_type: first.npc_type,
            species: first.species,
            ..Default::default()
        };

        let shared = Rc::new(entry);

        for npc_id in &esther.npc_ids {
            npc_by_id_map.insert(*npc_id, Npc::EstherGroup(shared.clone()));
            new_npc_by_name_map.insert(esther.name.to_string(), Npc::EstherGroup(shared.clone()));
        }
    }

    for (name, npcs) in npc_by_name_map.iter_mut() {
        npcs.sort_by(|a, b| {
            let hp_a = a.hp_bars.unwrap_or(0);
            let hp_b = b.hp_bars.unwrap_or(0);
            hp_b.cmp(&hp_a)
        });

        if npcs.len() == 1 {
            
            let npc = npcs.first().unwrap();

            let npc_struct = NpcStruct {
                id: npc.id,
                name: name.clone(),
                grade: npc.grade,
                npc_type: npc.npc_type,
                species: npc.species,
                hp_bars: npc.hp_bars
            };

            let shared = Rc::new(npc_struct);
            npc_by_id_map.insert(npc.id, Npc::Npc(shared.clone()));
            new_npc_by_name_map.insert(name.clone(), Npc::Npc(shared.clone()));
            
            continue;
        }

        let first = npcs.first().unwrap();

        let mut group = NpcGroup {
            name: name.clone(),
            hp_bars: first.hp_bars,
            npc_type: first.npc_type,
            species: first.species,
            grade: first.grade,
            ..Default::default()
        };

        for npc in npcs.iter() {
            group.ids.push(npc.id);
        }

        let group = Rc::new(group);
        for npc in npcs.iter() {
            npc_by_id_map.insert(npc.id, Npc::NpcGroup(group.clone()));
        }
        
        new_npc_by_name_map.insert(name.clone(), Npc::NpcGroup(group.clone()));
    }

    for (name, npcs) in boss_by_name_map.iter_mut() {

        npcs.sort_by(|a, b| {
            let hp_a = a.hp_bars.unwrap_or(0);
            let hp_b = b.hp_bars.unwrap_or(0);
            hp_b.cmp(&hp_a)
        });

        if npcs.len() == 1 {
            
            let npc = npcs.first().unwrap();

            let entry = Boss {
                id: npc.id,
                name: name.clone(),
                npc_type: npc.npc_type,
                species: npc.species,
                grade: npc.grade,
                hp_bars: npc.hp_bars
            };

            let shared = Rc::new(entry);
            npc_by_id_map.insert(npc.id, Npc::Boss(shared.clone()));
            new_npc_by_name_map.insert(name.clone(), Npc::Boss(shared.clone()));

            continue;
        }

        let first = npcs.first().unwrap();

        let mut group = BossGroup {
            name: name.clone(),
            hp_bars: first.hp_bars,
            npc_type: first.npc_type,
            species: first.species,
            grade: first.grade,
            ..Default::default()
        };

        for npc in npcs.iter() {
            group.ids.push(npc.id);
        }

        let shared = Rc::new(group);
        for npc in npcs.iter() {
            npc_by_id_map.insert(npc.id, Npc::BossGroup(shared.clone()));
        }
        
        new_npc_by_name_map.insert(name.clone(), Npc::BossGroup(shared));
    }

    for (key, npcs) in npc_by_grade_type_species_map {
     
        if npcs.len() == 1 {
            let npc = npcs.first().unwrap();

            let entry = UnknownNpc {
                grade: key.grade,
                npc_type: key.npc_type,
                species: key.species,
            };       
        
            npc_by_id_map.insert(npc.id, Npc::Unknown(Rc::new(entry)));
            continue;
        }

        let first = npcs.first().unwrap();

        let mut group = UnknownNpcGroup {
            grade: first.grade,
            npc_type: first.npc_type,
            species: first.species,
            ..Default::default()
        };

        for npc in npcs.iter() {
            group.ids.push(npc.id);
        }

        let shared = Rc::new(group);
        for npc in npcs.iter() {
            npc_by_id_map.insert(npc.id, Npc::UnknownGroup(shared.clone()));
        }
    }

    Npcs {
        by_id: npc_by_id_map,
        by_name: new_npc_by_name_map,
    }
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
            let is_awakening = false;
            let is_hyper_awakening = hyper_awakening_ids.contains(&raw_skill.id);
            let is_hyper_awakening_technique = false;
            let class = class_map.get(&class_id).cloned().unwrap();
            let class_skill = ClassSkill {
                id: raw_skill.id,
                name: raw_skill.name,
                desc: raw_skill.desc.clone(),
                icon: raw_skill.icon,
                class: class.clone(),
                is_awakening,
                is_hyper_awakening,
                is_hyper_awakening_technique,
                cooldown: raw_skill.cooldown,
                grade: raw_skill.grade,
                skill_type: raw_skill.skill_type
            };

            Skill::ClassSkill(Rc::new(class_skill))
        },
        (Some(class_id), Some(_)) => {
            let is_awakening = false;
            let is_hyper_awakening = hyper_awakening_ids.contains(&raw_skill.id);
            let is_hyper_awakening_technique = false;
            let class = class_map.get(&class_id).cloned().unwrap();
            let class_skill = ClassSkill {
                id: raw_skill.id,
                name: raw_skill.name,
                desc: raw_skill.desc.clone(),
                icon: raw_skill.icon,
                class: class.clone(),
                is_awakening,
                is_hyper_awakening,
                is_hyper_awakening_technique,
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