use rustc_hash::FxHashMap;

use crate::{create_struct_with_npcs, wrap_npc_struct, Npc};

create_struct_with_npcs!(
    GateOneEsthers,
    azena => "Azena",
    avele => "Avele",
    thar => "Thar"
);

pub struct GateOne<'a> {
    pub red_doom_narkiel: &'a Npc, 
    pub agris: &'a Npc,
    pub esthers: GateOneEsthers<'a>
}
 
impl <'a> GateOne<'a> {
    pub fn from_npc_map(npc_map: &'a FxHashMap<String, Npc>) -> Self {
        Self {
            red_doom_narkiel: npc_map.get("Red Doom Narkiel").unwrap(),
            agris: npc_map.get("Agris").unwrap(),
            esthers: GateOneEsthers::from_npc_map(npc_map)
        }
    }   
}

create_struct_with_npcs!(
    GateTwoEsthers,
    azena => "Azena",
    ephernia => "Ephernia",
    thar => "Thar"
);

pub struct GateTwo<'a> {
    pub echidna: &'a Npc,
    pub desire_in_full_bloom_echidna: &'a Npc,
    pub covetus_master_echidna: &'a Npc,
    pub alcaone_the_twisted_venos: &'a Npc,
    pub agris_the_devouring_bog: &'a Npc,
    pub esthers: GateTwoEsthers<'a>
}

impl <'a> GateTwo<'a> {
    pub fn from_npc_map(npc_map: &'a FxHashMap<String, Npc>) -> Self {
        Self {
            echidna: npc_map.get("Echidna").unwrap(),
            desire_in_full_bloom_echidna: npc_map.get("Desire in Full Bloom, Echidna").unwrap(),
            covetus_master_echidna: npc_map.get("Covetous Master Echidna").unwrap(),
            agris_the_devouring_bog: npc_map.get("Agris, the Devouring Bog").unwrap(),
            alcaone_the_twisted_venos: npc_map.get("Alcaone, the Twisted Venom").unwrap(),
            esthers: GateTwoEsthers::from_npc_map(npc_map)
        }
    }   
}

wrap_npc_struct!(OvertureSpiralOfTheCrimsonMidnightSun,
{
    gate_one: GateOne<'a>,
    gate_two: GateTwo<'a>,
});
    