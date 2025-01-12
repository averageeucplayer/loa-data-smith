use std::{fmt, ptr::addr_of, rc::Rc};
use rustc_hash::FxHashMap;
use strum_macros::{Display, EnumString};

use crate::{data::json::{ESTHERS, RAW_NPC_MAP}, utils::create_struct_with_npcs, NpcGrade, NpcSpecies, NpcType};

pub struct Npcs {
    pub by_id: FxHashMap<u32, Npc>,
    pub by_name: FxHashMap<String, Npc>
}

impl Npcs {
    pub fn new() -> Self {
        create_struct_with_npcs(
            &RAW_NPC_MAP,
            &ESTHERS
        )
    }
}

#[derive(Debug, Default)]
pub enum Npc {
    #[default]
    None,
    Unknown(Rc<UnknownNpc>),
    UnknownGroup(Rc<UnknownNpcGroup>),
    Npc(Rc<NpcStruct>),
    NpcGroup(Rc<NpcGroup>),
    EstherGroup(Rc<EstherGroup>),
    Boss(Rc<Boss>),
    BossGroup(Rc<BossGroup>),
}

static mut DEFAULT_NPC: Npc = Npc::None;

impl Default for &Npc {
    fn default() -> Self {
        unsafe { &*addr_of!(DEFAULT_NPC) }
    }
}

impl fmt::Display for Npc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Npc::None => write!(f, "None"),
            Npc::Unknown(npc) => write!(f, "({:?})", npc),
            Npc::UnknownGroup(group) => write!(f, "({:?})", group),
            Npc::Npc(npc) => write!(f, "({:?})", npc),
            Npc::NpcGroup(group) => write!(f, "({:?})", group),
            Npc::EstherGroup(group) => write!(f, "({:?})", group),
            Npc::Boss(boss) => write!(f, "({:?})", boss),
            Npc::BossGroup(group) => write!(f, "({:?})", group),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Default)]
pub struct NpcKey {
    pub grade: NpcGrade,
    pub npc_type: NpcType,
    pub species: NpcSpecies,
}

#[derive(Debug, Default)]
pub struct UnknownNpc {
    pub grade: NpcGrade,
    pub npc_type: NpcType,
    pub species: NpcSpecies,
}

#[derive(Debug, Default)]
pub struct UnknownNpcGroup {
    pub ids: Vec<u32>,
    pub grade: NpcGrade,
    pub npc_type: NpcType,
    pub species: NpcSpecies,
}

#[derive(Debug, Default, Clone)]
pub struct NpcStruct {
    pub id: u32,
    pub name: String,
    pub grade: NpcGrade,
    pub npc_type: NpcType,
    pub species: NpcSpecies,
    pub hp_bars: Option<u16>
}

#[derive(Debug, Default, Clone)]
pub struct NpcGroup {
    pub ids: Vec<u32>,
    pub name: String,
    pub grade: NpcGrade,
    pub npc_type: NpcType,
    pub species: NpcSpecies,
    pub hp_bars: Option<u16>
}

#[derive(Debug, Default, Clone)]
pub struct EstherGroup {
    pub ids: Vec<u32>,
    pub name: EstherName,
    pub grade: NpcGrade,
    pub npc_type: NpcType,
    pub species: NpcSpecies,
}

#[derive(Debug, EnumString, Default, Display, Clone)]
pub enum EstherName {
    #[default]
    Unknown,
    Avele,
    Azena,
    Ealyn,
    Ephernia,
    Balthorr,
    Bastian,
    #[strum(serialize = "Kadan Attack")]
    KadanAttack,
    #[strum(serialize = "Kadan Defense")]
    KadanDefense,
    Inanna,
    Nineveh,
    Shandi,
    Thar,
    Thirain,
    Wei,
}

#[derive(Debug, Default, Clone)]
pub struct Boss {
    pub id: u32,
    pub name: String,
    pub grade: NpcGrade,
    pub npc_type: NpcType,
    pub species: NpcSpecies,
    pub hp_bars: Option<u16>
}

#[derive(Debug, Default, Clone)]
pub struct BossGroup {
    pub ids: Vec<u32>,
    pub name: String,
    pub grade: NpcGrade,
    pub npc_type: NpcType,
    pub species: NpcSpecies,
    pub hp_bars: Option<u16>
}