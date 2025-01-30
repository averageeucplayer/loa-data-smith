use std::rc::Rc;

use rustc_hash::FxHashMap;
use serde::Deserialize;

use crate::RAW_CLASS_BY_NAME_MAP;

use super::skill::{ClassSkill, Skill, Skills};

#[derive(Debug, Default, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Class<'a> {
    pub id: u32,
    pub name: &'a str,
    pub is_generic: bool,
    pub is_support: bool
}

#[derive(Debug, Default, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Specialisation<'a> {
    pub id: u32,
    pub name: &'a str,
}

pub struct ClassWithSkills<'a> {
    pub id: u32,
    pub name: &'a str,
    pub skills: Vec<Rc<ClassSkill<'a>>>,
    pub skills_by_name: FxHashMap<&'a str, Rc<ClassSkill<'a>>>,
    pub weapon_attack: Option<Rc<ClassSkill<'a>>>,
    pub hand_attack: Option<Rc<ClassSkill<'a>>>,
}

impl<'a> ClassWithSkills<'a> {
    pub fn new(
        name: &'a str,
        class_by_name_map: &FxHashMap<String, Class<'a>>,
        skills_by_class_id: &FxHashMap<u32, Vec<Skill<'a>>>) -> Self {
        let class = class_by_name_map.get(name).unwrap();

        let skills: Vec<Rc<ClassSkill>> = skills_by_class_id
            .get(&class.id)
            .cloned()
            .unwrap_or_default()
            .into_iter()
            .filter_map(|skill| match skill {
                Skill::ClassSkill(class_skill) => Some(class_skill),
                _ => None,
            })
            .collect();
    
        let hand_attack = skills.iter()
            .find(|skill| skill.name.filter(|&name| name == "Hand Attack").is_some())
            .cloned();
    
        let weapon_attack = skills.iter()
            .find(|skill| skill.name.filter(|&name| name == "Weapon Attack").is_some())
            .cloned();
    
        let skills_by_name: FxHashMap<&str, Rc<ClassSkill>> = skills
            .clone()
            .into_iter()
            .filter_map(|rc_skill| {
                rc_skill.name.clone().map(|name| (name, rc_skill.clone()))
            })
            .collect();

        ClassWithSkills {
            id: class.id,
            name: class.name,
            skills,
            skills_by_name,
            hand_attack,
            weapon_attack
        }
    }
}

pub struct Classes<'a> {
    pub warrior_male: ClassWithSkills<'a>,
    pub berserker: ClassWithSkills<'a>,
    pub destroyer: ClassWithSkills<'a>,
    pub gunlancer: ClassWithSkills<'a>,
    pub paladin: ClassWithSkills<'a>,
    pub female_warrior: ClassWithSkills<'a>,
    pub slayer: ClassWithSkills<'a>,
    pub mage: ClassWithSkills<'a>,
    pub arcanist: ClassWithSkills<'a>,
    pub summoner: ClassWithSkills<'a>,
    pub bard: ClassWithSkills<'a>,
    pub sorceress: ClassWithSkills<'a>,
    pub martial_artist_female: ClassWithSkills<'a>,
    pub wardancer: ClassWithSkills<'a>,
    pub scrapper: ClassWithSkills<'a>,
    pub soulfist: ClassWithSkills<'a>,
    pub glaivier: ClassWithSkills<'a>,
    pub martial_artist_male: ClassWithSkills<'a>,
    pub striker: ClassWithSkills<'a>,
    pub breaker: ClassWithSkills<'a>,
    pub assassin: ClassWithSkills<'a>,
    pub deathblade: ClassWithSkills<'a>,
    pub shadowhunter: ClassWithSkills<'a>,
    pub reaper: ClassWithSkills<'a>,
    pub souleater: ClassWithSkills<'a>,
    pub gunner_male: ClassWithSkills<'a>,
    pub sharpshooter: ClassWithSkills<'a>,
    pub deadeye: ClassWithSkills<'a>,
    pub artillerist: ClassWithSkills<'a>,
    pub machinist: ClassWithSkills<'a>,
    pub gunner_female: ClassWithSkills<'a>,
    pub gunslinger: ClassWithSkills<'a>,
    pub specialist: ClassWithSkills<'a>,
    pub artist: ClassWithSkills<'a>,
    pub aeromancer: ClassWithSkills<'a>,
    pub alchemist: ClassWithSkills<'a>,
}

impl<'a> Classes<'a> {
    pub fn new() -> Self {
        let skills = Skills::new();

        Self {
            warrior_male: ClassWithSkills::new("Warrior (Male)", &RAW_CLASS_BY_NAME_MAP, &skills.by_class_id),
            berserker: ClassWithSkills::new("Berserker", &RAW_CLASS_BY_NAME_MAP, &skills.by_class_id),
            destroyer: ClassWithSkills::new("Destroyer", &RAW_CLASS_BY_NAME_MAP, &skills.by_class_id),
            gunlancer: ClassWithSkills::new("Gunlancer", &RAW_CLASS_BY_NAME_MAP, &skills.by_class_id),
            paladin: ClassWithSkills::new("Paladin", &RAW_CLASS_BY_NAME_MAP, &skills.by_class_id),
            female_warrior: ClassWithSkills::new("Female Warrior", &RAW_CLASS_BY_NAME_MAP, &skills.by_class_id),
            slayer: ClassWithSkills::new("Slayer", &RAW_CLASS_BY_NAME_MAP, &skills.by_class_id),
            mage: ClassWithSkills::new("Mage", &RAW_CLASS_BY_NAME_MAP, &skills.by_class_id),
            arcanist: ClassWithSkills::new("Arcanist", &RAW_CLASS_BY_NAME_MAP, &skills.by_class_id),
            summoner: ClassWithSkills::new("Summoner", &RAW_CLASS_BY_NAME_MAP, &skills.by_class_id),
            bard: ClassWithSkills::new("Bard", &RAW_CLASS_BY_NAME_MAP, &skills.by_class_id),
            sorceress: ClassWithSkills::new("Sorceress", &RAW_CLASS_BY_NAME_MAP, &skills.by_class_id),
            martial_artist_female: ClassWithSkills::new("Martial Artist (Female)", &RAW_CLASS_BY_NAME_MAP, &skills.by_class_id),
            wardancer: ClassWithSkills::new("Wardancer", &RAW_CLASS_BY_NAME_MAP, &skills.by_class_id),
            scrapper: ClassWithSkills::new("Scrapper", &RAW_CLASS_BY_NAME_MAP, &skills.by_class_id),
            soulfist: ClassWithSkills::new("Soulfist", &RAW_CLASS_BY_NAME_MAP, &skills.by_class_id),
            glaivier: ClassWithSkills::new("Glaivier", &RAW_CLASS_BY_NAME_MAP, &skills.by_class_id),
            martial_artist_male: ClassWithSkills::new("Martial Artist (Male)", &RAW_CLASS_BY_NAME_MAP, &skills.by_class_id),
            striker: ClassWithSkills::new("Striker", &RAW_CLASS_BY_NAME_MAP, &skills.by_class_id),
            breaker: ClassWithSkills::new("Breaker", &RAW_CLASS_BY_NAME_MAP, &skills.by_class_id),
            assassin: ClassWithSkills::new("Assassin", &RAW_CLASS_BY_NAME_MAP, &skills.by_class_id),
            deathblade: ClassWithSkills::new("Deathblade", &RAW_CLASS_BY_NAME_MAP, &skills.by_class_id),
            shadowhunter: ClassWithSkills::new("Shadowhunter", &RAW_CLASS_BY_NAME_MAP, &skills.by_class_id),
            reaper: ClassWithSkills::new("Reaper", &RAW_CLASS_BY_NAME_MAP, &skills.by_class_id),
            souleater: ClassWithSkills::new("Souleater", &RAW_CLASS_BY_NAME_MAP, &skills.by_class_id),
            gunner_male: ClassWithSkills::new("Gunner (Male)", &RAW_CLASS_BY_NAME_MAP, &skills.by_class_id),
            sharpshooter: ClassWithSkills::new("Sharpshooter", &RAW_CLASS_BY_NAME_MAP, &skills.by_class_id),
            deadeye: ClassWithSkills::new("Deadeye", &RAW_CLASS_BY_NAME_MAP, &skills.by_class_id),
            artillerist: ClassWithSkills::new("Artillerist", &RAW_CLASS_BY_NAME_MAP, &skills.by_class_id),
            machinist: ClassWithSkills::new("Machinist", &RAW_CLASS_BY_NAME_MAP, &skills.by_class_id),
            gunner_female: ClassWithSkills::new("Gunner (Female)", &RAW_CLASS_BY_NAME_MAP, &skills.by_class_id),
            gunslinger: ClassWithSkills::new("Gunslinger", &RAW_CLASS_BY_NAME_MAP, &skills.by_class_id),
            specialist: ClassWithSkills::new("Specialist", &RAW_CLASS_BY_NAME_MAP, &skills.by_class_id),
            artist: ClassWithSkills::new("Artist", &RAW_CLASS_BY_NAME_MAP, &skills.by_class_id),
            aeromancer: ClassWithSkills::new("Aeromancer", &RAW_CLASS_BY_NAME_MAP, &skills.by_class_id),
            alchemist: ClassWithSkills::new("Alchemist", &RAW_CLASS_BY_NAME_MAP, &skills.by_class_id),
        }
    }
}