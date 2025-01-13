use std::{rc::Rc, str::FromStr};

use data::models::skill::RawSkill;
use misc::{create_esther_by_npc_id_map, vec_to_option};
use models::{class::Class, skill::*};
use rustc_hash::FxHashMap;

use crate::*;

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

pub fn create_npc_maps(
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

