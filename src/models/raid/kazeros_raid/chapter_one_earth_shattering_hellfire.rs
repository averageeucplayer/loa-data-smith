use rustc_hash::FxHashMap;

use crate::{create_struct_with_npcs, wrap_npc_struct, Npc};

create_struct_with_npcs!(
    GateOneEsthers,
    wei => "Wei",
    bastian => "Bastian",
    thar => "Thar"
);

pub struct GateOne<'a> {
    pub akkan_lord_of_death: &'a Npc,
    pub abyss_monarch_aegir: &'a Npc,
    pub esthers: GateOneEsthers<'a>
}

impl <'a> GateOne<'a> {
    pub fn from_npc_map(npc_map: &'a FxHashMap<String, Npc>) -> Self {
        Self {
            akkan_lord_of_death: npc_map.get("Akkan, Lord of Death").unwrap(),
            abyss_monarch_aegir: npc_map.get("Abyss Monarch Aegir").unwrap(),
            esthers: GateOneEsthers::from_npc_map(npc_map),
        }
    }   
}

create_struct_with_npcs!(
    GateTwoEsthers,
    wei => "Wei",
    ealyn => "Ealyn",
    avele => "Avele"
);

pub struct GateTwo<'a> {
    pub aegir_the_oppressor: &'a Npc,
    pub pulsating_giants_heart: &'a Npc,
    pub esthers: GateTwoEsthers<'a>
}

impl <'a> GateTwo<'a> {
    pub fn from_npc_map(npc_map: &'a FxHashMap<String, Npc>) -> Self {
        Self {
            aegir_the_oppressor: npc_map.get("Aegir, the Oppressor").unwrap(),
            pulsating_giants_heart: npc_map.get("Pulsating Giant's Heart").unwrap(),
            esthers: GateTwoEsthers::from_npc_map(npc_map),
        }
    }   
}

wrap_npc_struct!(ChapterOneEarthShatteringHellfire,
{
    gate_one: GateOne<'a>,
    gate_two: GateTwo<'a>,
});    