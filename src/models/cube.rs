use std::any::Any;

use rustc_hash::FxHashMap;

use crate::{create_struct_with_npcs, Npc};

create_struct_with_npcs!(
    SwordTalk,
    possessed_soldier => "Possessed Soldier"
);

create_struct_with_npcs!(
    KingsShadow,
    thanatos => "Thanatos",
    morai_clay_giant => "Morai Clay Giant"
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
    SolarRemnant,
    celedan => "Celedan",
    repletio => "Repletio",
    aide_seto => "Aide Seto"
);

create_struct_with_npcs!(
    MercenaryDownfall,
    ironblood_mercenaries => "Ironblood Mercenaries"
);

create_struct_with_npcs!(
    MercenaryOfDarkness,
    blood_swordfighter => "Blood Swordfighter"
);

create_struct_with_npcs!(
    EternalRemnant,
    jederico => "Jederico",
    phantom_legion_rook => "Phantom Legion Rook",
    harzal => "Harzal"
);

create_struct_with_npcs!(
    BorderSenitel,
    demons_wheel => "Demons's Wheel"
);

// create_struct_with_npcs!(
//     ShardOfDarkness,
//     jederico => "Jederico",
//     chaos_zaika => "Chaos Zaika"
// );

create_struct_with_npcs!(
    BackdoorDeal,
    smuggler => "Smuggler",
    elite_personal_guard_soldier => "Elite Personal Guard Soldier"
);

create_struct_with_npcs!(
    SpiritOfGold,
    redeye_swordfighter => "Redeye Swordfighter",
    elite_personal_guard_soldier => "Elite Personal Guard Soldier"
);

create_struct_with_npcs!(
    ShackledCovetusScale,
    moonshadow => "Moonshadow"
);

create_struct_with_npcs!(
    Marionette,
    stella => "Stella"
);

create_struct_with_npcs!(
    ReplicatedMachine,
    scientist_s => "Scientist S"
);

create_struct_with_npcs!(
    DemonsStrength,
    evolved_demon_dog => "Evolved Demon Dog"
);

create_struct_with_npcs!(
    TaintedGold,
    zeherade => "Zeherade",
    mariu => "Mariu"
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
    NaturalOrder,
    spotted_bat => "Spotted Bat",
    egg_robber_bat => "Egg Robber Bat"
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
    StrongBeak,
    prairie_alifer => "Prairie Alifer",
    red_eared_mythical_lizard => "Red-Eared Mythical Lizard"
);

create_struct_with_npcs!(
    TowerSpirit,
    corrupting_light_leshar => "Corrupting Light Leshar"
);

create_struct_with_npcs!(
    Cobwebs,
    pod_spider => "Pod Spider",
    forest_spider => "Forest Spider"
);

create_struct_with_npcs!(
    AshenRemnants,
    celestial_senitel => "Celestial Senitel",
    labyrinth_guardian => "Labyrinth Guardian",
    mucus_demon_mukus => "Mucus Demon Mukus",
    big_mouth_anglerfish => "Big Mouth Anglerfish",
    rudric => "Rudric",
    agherun => "Agherun"
);

create_struct_with_npcs!(
    TemperatureOfDespair,
    nakshun => "Nakshun"
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
    CubesFortune,
    greedy_tuturi => "Greedy Tuturi",
    lost_tooki_king => "Lost Tooki King"
);

create_struct_with_npcs!(
    SinfulExpressions,
    fragile_targatus => "Fragile Targatus",
    evolved_demon_dog => "Evolved Demon Dog"
);

create_struct_with_npcs!(
    WrigglingMalice,
    salt_worm => "Salt Worm"
);

create_struct_with_npcs!(
    FerociousDogs,
    rabid_demon_dog => "Rabid Demon Dog",
    evolved_demon_dog => "Evolved Demon Dog"
);

create_struct_with_npcs!(
    UnpopularCarnival,
    broken_stella => "Broken Stella"
);

create_struct_with_npcs!(
    FaithfulExecution,
    sacrian_priest_spearman => "Sacrian Priest Spearman"
);

create_struct_with_npcs!(
    LostIntegrity,
    cube_bewitched_martial_artist => "Cube Bewitched Martial Artist"
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
    rovlen => "Rovlen",
    wooden_gazer => "Wooden Gazer"
);

create_struct_with_npcs!(
    SanguinarySorcery,
    red_shaman => "Red Shaman",
    red_robed_mage => "Red Robed Mage"
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
    SweetVoice,
    assar_delain => "Assar Delain"
);

create_struct_with_npcs!(
    FalseFog,
    cube_bewitched_specialist => "Cube Bewitched Specialist"
);

create_struct_with_npcs!(
    CracklingFire,
    hermut => "Hermut"
);

create_struct_with_npcs!(
    GlideStrikeForces,
    wingsuit_mercenary => "Wingsuit Mercenary"
);

create_struct_with_npcs!(
    ViciousMiasma,
    miasmic_mutated_giant_demon_dog => "Miasmic Mutated Giant Demon Dog"
);

create_struct_with_npcs!(
    LightDevourer,
    executioner_mooduro => "Executioner Mooduro",
    celestial_knight => "Celestial Knight"
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
        KingsShadow::from_npc_map(npc_map); // 16
        SilentRemnant::from_npc_map(npc_map); // 18
        DisruptiveDissident::from_npc_map(npc_map); // 19
        SolarRemnant::from_npc_map(npc_map); // 20
        MercenaryDownfall::from_npc_map(npc_map); // 21
        MercenaryOfDarkness::from_npc_map(npc_map); // 24
        EternalRemnant::from_npc_map(npc_map); // 25
        BorderSenitel::from_npc_map(npc_map); // 30
        ShardOfDarkness::from_npc_map(npc_map); // 31
        BackdoorDeal::from_npc_map(npc_map); // 33
        SpiritOfGold::from_npc_map(npc_map); // 37
        ShackledCovetusScale::from_npc_map(npc_map); // 39
        Marionette::from_npc_map(npc_map); // 41
        ReplicatedMachine::from_npc_map(npc_map); // 45
        WorldOfMonsters::from_npc_map(npc_map); // 47
        FoulOrder::from_npc_map(npc_map); // 54
        NaturalOrder::from_npc_map(npc_map); // 55
        PermaFrost::from_npc_map(npc_map); // 57
        TwistedVengeance::from_npc_map(npc_map); // 58
        StrongBeak::from_npc_map(npc_map); // 59
        TowerSpirit::from_npc_map(npc_map); // 69
        Cobwebs::from_npc_map(npc_map); // 75
        AshenRemnants::from_npc_map(npc_map); // 78
        TemperatureOfDespair::from_npc_map(npc_map); // 80
        DemonsStrength::from_npc_map(npc_map); // 86
        TaintedGold::from_npc_map(npc_map); // 87
        PossessedMask::from_npc_map(npc_map); // 93 
        OutragedElemental::from_npc_map(npc_map); // 96 
        CubesFortune::from_npc_map(npc_map); // 105
        SinfulExpressions::from_npc_map(npc_map); // 111
        WrigglingMalice::from_npc_map(npc_map); // 115
        FerociousDogs::from_npc_map(npc_map); // 126
        UnpopularCarnival::from_npc_map(npc_map); // 127
        FaithfulExecution::from_npc_map(npc_map); // 128
        LostIntegrity::from_npc_map(npc_map); // 145
        SelfishExecutioner::from_npc_map(npc_map); // 150
        NamelessElemental::from_npc_map(npc_map); // 173
        PoisonousFlower::from_npc_map(npc_map); // 176
        SanguinarySorcery::from_npc_map(npc_map); // 198
        ExquisiteHomunculus::from_npc_map(npc_map); // 221
        ShackledAgent::from_npc_map(npc_map); // 224
        SweetVoice::from_npc_map(npc_map); // 237
        FalseFog::from_npc_map(npc_map); // 239
        CracklingFire::from_npc_map(npc_map); // 244
        GlideStrikeForces::from_npc_map(npc_map); // 245
        ViciousMiasma::from_npc_map(npc_map); // 247
        LightDevourer::from_npc_map(npc_map); // 248
        PhantomFragment::from_npc_map(npc_map); // 256
        ShardOfDarkness::from_npc_map(npc_map); // 263
        PlagueShard::from_npc_map(npc_map); // 263
        CovetousShard::from_npc_map(npc_map); // 267
        CubesFortune::from_npc_map(npc_map); // 302

        Self {
            sword_talk: SwordTalk::from_npc_map(npc_map)
        }
    }   
}