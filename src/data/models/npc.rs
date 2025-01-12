use serde::{Deserialize, Serialize};
use crate::deserializer::u16_zero_as_none::u16_zero_as_none;

#[derive(Debug, Default, Deserialize, Clone)]
pub struct RawNpc {
    pub id: u32,
    pub name: Option<String>,
    pub grade: NpcGrade,
    #[serde(rename = "type")]
    pub npc_type: NpcType,
    pub species: NpcSpecies,
    #[serde(rename = "hpBars")]
    #[serde(deserialize_with = "u16_zero_as_none")]
    pub hp_bars: Option<u16>
}

#[derive(Debug, Default, Deserialize, Clone)]
pub struct RawEsther<'a> {
    pub name: &'a str,
    pub icon: &'a str,
    pub skills: Vec<u32>,
    #[serde(alias = "npcs")]
    pub npc_ids: Vec<u32>,
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Eq, Hash, Copy, Clone)]
#[serde(rename_all = "lowercase")]
pub enum NpcGrade {
    #[default]
    None,
    Normal,
    Boss,
    Elite,
    Commander,
    Lucky,
    Raid,
    #[serde(rename = "epic_raid")]
    EpicRaid,
    Named,
    Underling,
    Seed
}


#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Eq, Hash, Copy, Clone)]
#[serde(rename_all = "lowercase")]
pub enum NpcType {
    #[default]
    None,
    Townsfolk,
    Monster,
    Pet,
    #[serde(rename = "monster_pc_form")]
    MonsterPcForm,
    Summoned,
    Totem
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Eq, Hash, Copy, Clone)]
#[serde(rename_all = "lowercase")]
pub enum NpcSpecies {
    #[default]
    Unknown,
    Structure,
    Spirit,
    Substance,
    #[serde(rename = "voyage_hunting")]
    VoyageHunting,
    None,
    Undead,
    #[serde(rename = "voyage_ghost")]
    VoyageGhost,
    Insect,
    Archefiend,
    Devil,
    Ancient,
    Humanoid,
    Mechanic,
    Plant,
    #[serde(rename = "wild_beast")]
    WildBeast
}