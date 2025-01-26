use rustc_hash::FxHashMap;

use crate::{create_struct_with_npcs, wrap_npc_struct, Npc};

create_struct_with_npcs!(
    GateOneEsthers,
    wei => "Jederico",
    azena => "Azena",
    ephernia => "Ephernia"
);

pub struct GateOne<'a> {
    pub narok_the_butcher: &'a Npc,
    pub esthers: GateOneEsthers<'a>
}

impl <'a> GateOne<'a> {
    pub fn from_npc_map(npc_map: &'a FxHashMap<String, Npc>) -> Self {
        Self {
            narok_the_butcher: npc_map.get("Narok the Butcher",).unwrap(),
            esthers: GateOneEsthers::from_npc_map(npc_map),
        }
    }   
}

create_struct_with_npcs!(
    GateTwoEsthers,
    gustaven => "Gustaven",
    nineveh => "Nineveh",
    azena => "Azena"
);

pub struct GateTwo<'a> {
    pub phantom_legion_commander_brelshaza: &'a Npc,
    pub esthers: GateTwoEsthers<'a>
}

impl <'a> GateTwo<'a> {
    pub fn from_npc_map(npc_map: &'a FxHashMap<String, Npc>) -> Self {
        Self {
            phantom_legion_commander_brelshaza: npc_map.get("Phantom Legion Commander Brelshaza").unwrap(),
            esthers: GateTwoEsthers::from_npc_map(npc_map),
        }
    }   
}

wrap_npc_struct!(ActTwoRequiemOfTheFloatingNightmare,
{
    gate_one: GateOne<'a>,
    gate_two: GateTwo<'a>,
});    