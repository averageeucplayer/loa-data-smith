use rustc_hash::FxHashMap;

use crate::{create_struct_with_npcs, Npc};

create_struct_with_npcs!(
    SwordTalk,
    possessed_soldier => "Possessed Soldier"
);

create_struct_with_npcs!(
    SilentRemnant,
    varto => "Varto",
    sol_grande => "Sol Grande",
    jederico => "Jederico",
    evolved_mari => "Evolved Mari"
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
    NamelessElemental,
    cube_bewitched_mage => "Cube Bewitched Mage"
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

create_struct_with_npcs!(
    Plague271,
    akkan_plague_gargoyle => "Akkan Plague Gargoyle",
    harzal => "Harzal"
);

#[derive(Debug, Default, Clone)]
pub struct Stub<const N: usize>{}

#[derive(Debug, Default, Clone)]
pub struct Cube<'a>(
    pub SwordTalk<'a>,
    pub SilentRemnant<'a>,
    pub DisruptiveDissident<'a>,
    pub WorldOfMonsters<'a>,
    pub FoulOrder<'a>,
    pub PermaFrost<'a>,
    pub DemonsStrength<'a>,
    pub PossessedMask<'a>,
    pub SelfishExecutioner<'a>,
    pub NamelessElemental<'a>,
    pub ShackledAgent<'a>,
    pub CracklingFire<'a>,
    pub PhantomFragment<'a>,
    pub ShardOfDarkness<'a>,
    pub Plague271<'a>
);

// 41 Marionette

impl<'a> Cube<'a> {
    pub fn from_npc_map(npc_map: &'a FxHashMap<String, Npc>) -> Self {
        Self {
            0: SwordTalk::from_npc_map(npc_map), // 15
            1: SilentRemnant::from_npc_map(npc_map), // 18
            2: DisruptiveDissident::from_npc_map(npc_map), // 19
            3: WorldOfMonsters::from_npc_map(npc_map), // 47
            4: FoulOrder::from_npc_map(npc_map), // 54
            5: PermaFrost::from_npc_map(npc_map), // 57
            6: DemonsStrength::from_npc_map(npc_map), // 86
            7: PossessedMask::from_npc_map(npc_map), // 93 
            8: SelfishExecutioner::from_npc_map(npc_map), // 150
            9: NamelessElemental::from_npc_map(npc_map), // 173
            10: ShackledAgent::from_npc_map(npc_map), // 224
            11: CracklingFire::from_npc_map(npc_map), // 244
            12: PhantomFragment::from_npc_map(npc_map), // 256
            13: ShardOfDarkness::from_npc_map(npc_map), // 263
            14: Plague271::from_npc_map(npc_map), // 263
            ..Default::default()
        }
    }   
}