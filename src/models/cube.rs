use rustc_hash::FxHashMap;

use crate::{create_struct_with_npcs, Npc};

create_struct_with_npcs!(
    SilentRemnant,
    varto => "Varto",
    sol_grande => "Sol Grande",
    jederico => "Jederico"
);

create_struct_with_npcs!(
    DisruptiveDissident,
    violent_pirate_brawler => "Violent Pirate Brawler",
    violetblade_pirates => "Violetblade Pirates"
);

create_struct_with_npcs!(
    DemonsStrength,
    evolved_demon_dog => "Evolved Demon Dog"
);

create_struct_with_npcs!(
    WorldOfMonsters,
    zakel_warrior => "Zakel Warrior",
    zakel_soldier => "Zakel Soldier"
);

create_struct_with_npcs!(
    FoulOrder,
    wilhelm => "Wilhelm",
    revellos => "Revellos",
    osphere => "Osphere"
);

create_struct_with_npcs!(
    PermaFrost,
    avesta_assassin => "Frozen Fighter"
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
    SelfishExecutioner,
    avesta_assassin => "Avesta Assassin"
);

create_struct_with_npcs!(
    ShackledAgent,
    cube_bewitched_assassin => "Cube Bewitched Assassin"
);

create_struct_with_npcs!(
    CracklingFire,
    hermut => "Hermut"
);

create_struct_with_npcs!(
    PhantomFragment,
    seal_stone => "Seal Stone"  
);

create_struct_with_npcs!(
    ShardOfDarkness,
    seal_stone => "Seal Stone"  
);

#[derive(Debug, Default, Clone)]
pub struct Stub<const N: usize>{}

#[derive(Debug, Default, Clone)]
pub struct Cube<'a>(
    pub SilentRemnant<'a>,
    pub DisruptiveDissident<'a>,
    pub WorldOfMonsters<'a>,
    pub FoulOrder<'a>,
    pub PermaFrost<'a>,
    pub DemonsStrength<'a>,
    pub PossessedMask<'a>,
    pub SelfishExecutioner<'a>,
    pub ShackledAgent<'a>,
    pub CracklingFire<'a>,
    pub PhantomFragment<'a>,
    pub ShardOfDarkness<'a>);

impl<'a> Cube<'a> {
    pub fn from_npc_map(npc_map: &'a FxHashMap<String, Npc>) -> Self {
        Self {
            0: SilentRemnant::from_npc_map(npc_map), // 18
            1: DisruptiveDissident::from_npc_map(npc_map), // 19
            2: WorldOfMonsters::from_npc_map(npc_map), // 47
            3: FoulOrder::from_npc_map(npc_map), // 54
            4: PermaFrost::from_npc_map(npc_map), // 57
            5: DemonsStrength::from_npc_map(npc_map), // 86
            6: PossessedMask::from_npc_map(npc_map), // 93 
            7: SelfishExecutioner::from_npc_map(npc_map), // 150
            8: ShackledAgent::from_npc_map(npc_map), // 224
            9: CracklingFire::from_npc_map(npc_map), // 244
            10: PhantomFragment::from_npc_map(npc_map), // 256
            11: ShardOfDarkness::from_npc_map(npc_map), // 263
            ..Default::default()
        }
    }   
}