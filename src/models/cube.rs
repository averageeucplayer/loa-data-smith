use std::any::Any;

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
    evolved_mari => "Evolved Mari",
    fragile_targatus => "Fragile Targatus",
    berhar => "Berhart"
);

create_struct_with_npcs!(
    DisruptiveDissident,
    violent_pirate_brawler => "Violent Pirate Brawler",
    violetblade_pirates => "Violetblade Pirates"
);

create_struct_with_npcs!(
    MercenaryOfDarkness,
    blood_swordfighter => "Blood Swordfighter"
);

create_struct_with_npcs!(
    SpiritOfGold,
    redeye_swordfighter => "Redeye Swordfighter",
    elite_personal_guard_soldier => "Elite Personal Guard Soldier"
);

create_struct_with_npcs!(
    Marionette,
    stella => "Stella"
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
    TwistedVengeance,
    hanun => "Hanun",
    marinna => "Marinna",
    myun_hidaka => "Myun Hidaka",
    danika => "Danika"
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
    OutragedElemental,
    avesta_assassin => "Outraged Wing Elemental"
);

create_struct_with_npcs!(
    FaithfulExecution,
    sacrian_priest_spearman => "Sacrian Priest Spearman"
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
    PoisonousFlower,
    rovlen => "Rovlen"
);

create_struct_with_npcs!(
    ExquisiteHomunculus,
    homunculus => "Homunculus"
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
    ViciousMiasma,
    miasmic_mutated_giant_demon_dog => "Miasmic Mutated Giant Demon Dog"
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
    PlagueShard,
    seal_stone => "Seal Stone",
    akkan_plague_gargoyle => "Akkan Plague Gargoyle",
    harzal => "Harzal"
);

create_struct_with_npcs!(
    CovetousShard,
    seal_stone => "Seal Stone"
);

#[derive(Debug, Default)]
pub struct Cube<'a> {
    sword_talk: SwordTalk<'a>
}

impl<'a> Cube<'a> {
    pub fn from_npc_map(npc_map: &'a FxHashMap<String, Npc>) -> Self {
        SwordTalk::from_npc_map(npc_map); // 15
        SilentRemnant::from_npc_map(npc_map); // 18
        DisruptiveDissident::from_npc_map(npc_map); // 19
        MercenaryOfDarkness::from_npc_map(npc_map); // 24
        SpiritOfGold::from_npc_map(npc_map); // 37
        Marionette::from_npc_map(npc_map); // 41
        WorldOfMonsters::from_npc_map(npc_map); // 47
        FoulOrder::from_npc_map(npc_map); // 54
        PermaFrost::from_npc_map(npc_map); // 57
        TwistedVengeance::from_npc_map(npc_map); // 58
        DemonsStrength::from_npc_map(npc_map); // 86
        PossessedMask::from_npc_map(npc_map); // 93 
        OutragedElemental::from_npc_map(npc_map); // 96 
        FaithfulExecution::from_npc_map(npc_map); // 128
        SelfishExecutioner::from_npc_map(npc_map); // 150
        NamelessElemental::from_npc_map(npc_map); // 173
        PoisonousFlower::from_npc_map(npc_map); // 176
        ExquisiteHomunculus::from_npc_map(npc_map); // 221
        ShackledAgent::from_npc_map(npc_map); // 224
        CracklingFire::from_npc_map(npc_map); // 244
        ViciousMiasma::from_npc_map(npc_map); // 247
        PhantomFragment::from_npc_map(npc_map); // 256
        ShardOfDarkness::from_npc_map(npc_map); // 263
        PlagueShard::from_npc_map(npc_map); // 263
        CovetousShard::from_npc_map(npc_map); // 267

        Self {
            sword_talk: SwordTalk::from_npc_map(npc_map)
        }
    }   
}