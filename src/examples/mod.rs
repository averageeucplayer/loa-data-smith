use crate::{data::models::skill_buff::{SkillBuffCategory, SkillBuffSetName}, models::{class::{ClassWithSkills, Classes}, raid::{abyssal_dungeon::AbyssalDungeon, guardian_raid::GuardianRaid, kazeros_raid::KazerosRaid, legion_raid::LegionRaid}, skill::Skills}, Npcs, RAW_CLASS_BY_NAME_MAP, RAW_SKILL_BUFF_MAP};


pub fn guardian_raid() {
   let npcs = Npcs::new();

    let guardian_raid = GuardianRaid::from_npc_map(&npcs.by_name);
    let gargadeth = guardian_raid.chaos_guardian_purification.one.gargadeth;
    println!("{}", gargadeth);
}

pub fn kazeros_raid() {
    let npcs = Npcs::new();

    let kazeros_raid = KazerosRaid::from_npc_map(&npcs.by_name);
    let boss = kazeros_raid.overture.gate_one.red_doom_narkiel;
    println!("{}", boss);
}

pub fn legion_raid() {
    let npcs = Npcs::new();

    let legion_raid = LegionRaid::from_npc_map(&npcs.by_name);
    let boss = legion_raid.thaemine.hard.gate_four.darkness_legion_commander_thaemine;
    println!("{}", boss);
}

pub fn abyssal_dungeon() {
    let npcs = Npcs::new();

    let abyssal_dungeon = AbyssalDungeon::from_npc_map(&npcs.by_name);
    let boss = abyssal_dungeon.ivory_tower_of_chaos.gate_three.lazaramthe_trailblazer;
    println!("{}", boss);
}

pub fn all_classses_skills() {
    let classes = Classes::new();

    let skill = classes.berserker.skills_by_name.get("Red Dust");
    println!("{:?}", skill);
}

pub fn class_skills() {
    let skills = Skills::new();
    
    let class = ClassWithSkills::new("Artist",  &RAW_CLASS_BY_NAME_MAP, &skills.by_class_id);
    let skills: Vec<_> = class.skills.iter().filter_map(|skill| skill.name).collect();
    
    println!("{:?}", skills);
}

pub fn skill_buffs() {
    let yearning_buffs: Vec<_> = RAW_SKILL_BUFF_MAP.iter()
        .filter(|buff| buff.1.set_name == SkillBuffSetName::Yearning)
        .map(|buff| buff.1.name)
        .collect();

    println!("{:?}", yearning_buffs);

    let cook_buffs: Vec<_> = RAW_SKILL_BUFF_MAP.iter()
        .filter(|buff| buff.1.buff_category == SkillBuffCategory::Cook)
        .map(|buff| buff.1.name)
        .collect();

    println!("{:?}", cook_buffs);
}