use rustc_hash::FxHashMap;

use crate::{create_struct_with_npcs, wrap_npc_struct, Npc};

create_struct_with_npcs!(
    Sonavel,
    sonavel => "Sonavel",
    sonavel_core => "Sonavel Core"
);

create_struct_with_npcs!(
    Argeos,
    argeos => "Argeos",
    chaos_lightning_drago_jade => "Chaos Lightning Dragon Jade"
);

pub struct PurificationLevelOne<'a> {
    pub sonavel: Sonavel<'a>,
    pub gargadeth: &'a Npc,
    pub veskal: &'a Npc,
}

impl<'a> PurificationLevelOne<'a> {
    pub fn from_npc_map(npc_map: &'a FxHashMap<String, Npc>) -> Self {
        Self {
            sonavel: Sonavel::from_npc_map(npc_map),
            gargadeth: npc_map.get("Gargadeth").unwrap(),
            veskal: npc_map.get("Veskal").unwrap(),
        }
    }   
}

wrap_npc_struct!(PurificationLevelTwo,
{
    argeos: Argeos<'a>,
});

wrap_npc_struct!(ChaosGuardianPurification,
{
    one: PurificationLevelOne<'a>,
    two: PurificationLevelTwo<'a>,
});