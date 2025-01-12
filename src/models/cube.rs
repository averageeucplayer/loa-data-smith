use rustc_hash::FxHashMap;

use crate::{create_struct_with_npcs, Npc};

create_struct_with_npcs!(
    PermaFrost,
    avesta_assassin => "Frozen Fighter"
);

create_struct_with_npcs!(
    SelfishExecutioner,
    avesta_assassin => "Avesta Assassin"
);

create_struct_with_npcs!(
    PossessedMask,
    ren => "Ren",
    pehta => "Pehta",
    nefer => "Nefer",
    gemet => "Gemet",
    tamut => "Tamut"
);

create_struct_with_npcs!(
    ShardOfDarkness,
    seal_stone => "Seal Stone"  
);

#[derive(Debug, Default, Clone)]
pub struct Stub<const N: usize>{}

#[derive(Debug, Default, Clone)]
pub struct Cube<'a>(
    pub PermaFrost<'a>,
    pub PossessedMask<'a>,
    pub SelfishExecutioner<'a>,
    pub ShardOfDarkness<'a>);

impl<'a> Cube<'a> {
    pub fn from_npc_map(npc_map: &'a FxHashMap<String, Npc>) -> Self {
        Self {
            0: PermaFrost::from_npc_map(npc_map), // 57
            1: PossessedMask::from_npc_map(npc_map), // 93 
            2: SelfishExecutioner::from_npc_map(npc_map), // 150
            3: ShardOfDarkness::from_npc_map(npc_map), // 263
            ..Default::default()
        }
    }   
}