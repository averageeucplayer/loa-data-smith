use std::any::Any;

use rustc_hash::FxHashMap;

use crate::{create_struct_with_npcs, Npc};

create_struct_with_npcs!(
    HumanHunter,
    adventurer => "Adventurer",
    brigand => "Brigand",
    raider => "Raider",
    whitesand_outlaw => "Whitesand Outlaw"
);

create_struct_with_npcs!(
    TrialRemnant,
    kohinorr => "Kohinorr",
    agherun => "Agherun",
    giant_worm => "Giant Worm",
    phantom_legion_knight => "Phantom Legion Knight",
    dr_bergstrom => "Dr. Bergstrom",
    darkness_legion_amaus =>  "Darkness Legion Amaus",
    plaguebringer => "Plaguebringer"
);

create_struct_with_npcs!(
    SwordTalk,
    possessed_soldier => "Possessed Soldier",
    dagger_assassin => "Dagger Assassin",
    dual_sword_assassin => "Dual Sword Assassin",
    mysterious_assassin => "Mysterious Assassin"
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
    aide_seto => "Aide Seto",
    tarsila => "Tarsila",
    paromensa => "Paromensa",
    ark_guardian_cccel => "Ark Guardian Occel"
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
    GooeyCovetusLump,
    demonic_lump => "Demonic Lump"
);

create_struct_with_npcs!(
    AncestorsFury,
    eikerr => "Eikerr",
    naber => "Naber"
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
    stella => "Stella",
    liru => "Liru",
    shana => "Shana"
);

create_struct_with_npcs!(
    ReplicatedMachine,
    scientist_s => "Scientist S"
);

create_struct_with_npcs!(
    QueensKnight,
    avele => "Avele",
    thar => "Thar",
    hartem => "Hartem",
    kiessa => "Kiessa"
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
    FourLeggedWeapon,
    steel_dobermech => "Steel Dobermech"
);

create_struct_with_npcs!(
    UndergroundKnights,
    telpa_captain => "Telpa Captain"
);

create_struct_with_npcs!(
    WorldOfMonsters,
    zakel_warrior => "Zakel Warrior",
    zakel_soldier => "Zakel Soldier"
);

create_struct_with_npcs!(
    NastyNecromancy,
    veloran => "Veloran"
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
    RebelliousRenegade,
    azakiel => "Azakiel",
    diogenes => "Diogenes"
);

create_struct_with_npcs!(
    ARollingStone,
    pebbling => "Pebbling",
    locarok => "Locarok"
);

create_struct_with_npcs!(
    FireElemental,
    ancient_wildfire_tyrant => "Ancient Wildfire Tyrant",
    cereon => "Cereon"
);

create_struct_with_npcs!(
    HoneyComb,
    hybee_scout => "Hybee Scout",
    hybee_guard => "Hybee Guard"
);

create_struct_with_npcs!(
    TowerSpirit,
    corrupting_light_leshar => "Corrupting Light Leshar"
);

create_struct_with_npcs!(
    ShinyTail,
    young_mera_snake => "Young Mera Snake",
    plague_mucus_monster => "Plague Mucus Monster"
);

create_struct_with_npcs!(
    SwordPossessedLady,
    kyzra => "Kyzra"
);

create_struct_with_npcs!(
    SteelTongs,
    saldone => "Saldone"
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
    agherun => "Agherun",
    kyzra => "Kyzra",
    velkan => "Velkan",
    sigmund => "Sigmund",
    varto => "Varto",
    corrupted_ogre => "Corrupted Ogre"
);

create_struct_with_npcs!(
    TemperatureOfDespair,
    nakshun => "Nakshun"
);

create_struct_with_npcs!(
    AncientHomunculus,
    abandoned_giant_homunculus => "Abandoned Giant Homunculus"
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
    CubesGiantFortune,
    light_shining_treasure_chest =>  "Light Shining Treasure Chest"
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
    MadGiggle,
    seto => "Seto"
);

create_struct_with_npcs!(
    LightKnight,
    light_archon => "Light Archon"
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
    Knightfall,
    luterran_swordfighter => "Luterran Swordfighter"
);

create_struct_with_npcs!(
    BladeOfFaith,
    archbishop_wilhelm => "Archbishop Wilhelm"  
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
    LostSoul,
    cube_bewitched_martial_artist => "Cube Bewitched Martial Artist"
);

create_struct_with_npcs!(
    VengefulMayhem,
    wolf_of_vengeance => "Wolf of Vengeance"
);

create_struct_with_npcs!(
    WickedAdventurer,
    redeye_swordfighter => "Redeye Swordfighter"
);

create_struct_with_npcs!(
    NamelessElemental,
    cube_bewitched_mage => "Cube Bewitched Mage"
);

create_struct_with_npcs!(
    ExecutionerOfRebelliousLight,
    elgacia_soldier => "Elgacia Soldier"
);

create_struct_with_npcs!(
    PoisonousFlower,
    rovlen => "Rovlen",
    wooden_gazer => "Wooden Gazer"
);

create_struct_with_npcs!(
    IllegallyModifiedHuman,
    redeye_gunner => "Redeye Gunner",
    personal_guard_soldier => "Personal Guard Soldier"
);

create_struct_with_npcs!(
    SteelHeart,
    signatus => "Signatus"
);

create_struct_with_npcs!(
    SanguinarySorcery,
    red_shaman => "Red Shaman",
    red_robed_mage => "Red Robed Mage"
);

create_struct_with_npcs!(
    SorrowfulWitch,
    tarsila => "Tarsila"
);

create_struct_with_npcs!(
    FrozenMace,
    maneth => "Maneth"
);

create_struct_with_npcs!(
    ForbiddenMagickCreature,
    maneth => "Magmadon"
);

create_struct_with_npcs!(
    ExquisiteHomunculus,
    homunculus => "Homunculus"
);

create_struct_with_npcs!(
    ShackledDemon,
    cube_bewitched_assassin => "Cube Bewitched Assassin"
);

create_struct_with_npcs!(
    ShackledAgent,
    cube_bewitched_assassin => "Cube Bewitched Assassin"
);

create_struct_with_npcs!(
    FloweringBeast,
    flower_chameleon => "Flower Chameleon"
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
    MayhemShard,
    seal_stone => "Seal Stone"
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
        HumanHunter::from_npc_map(npc_map); // 10
        TrialRemnant::from_npc_map(npc_map); // 14
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
        GooeyCovetusLump::from_npc_map(npc_map); // 35
        AncestorsFury::from_npc_map(npc_map); // 36
        SpiritOfGold::from_npc_map(npc_map); // 37
        ShackledCovetusScale::from_npc_map(npc_map); // 39
        Marionette::from_npc_map(npc_map); // 41
        ReplicatedMachine::from_npc_map(npc_map); // 45
        QueensKnight::from_npc_map(npc_map); // 46
        WorldOfMonsters::from_npc_map(npc_map); // 47
        NastyNecromancy::from_npc_map(npc_map); // 53
        FoulOrder::from_npc_map(npc_map); // 54
        NaturalOrder::from_npc_map(npc_map); // 55
        PermaFrost::from_npc_map(npc_map); // 57
        TwistedVengeance::from_npc_map(npc_map); // 58
        StrongBeak::from_npc_map(npc_map); // 59
        RebelliousRenegade::from_npc_map(npc_map); // 60
        ARollingStone::from_npc_map(npc_map); // 61
        FireElemental::from_npc_map(npc_map); // 64
        HoneyComb::from_npc_map(npc_map); // 66
        TowerSpirit::from_npc_map(npc_map); // 69
        ShinyTail::from_npc_map(npc_map); // 70
        SwordPossessedLady::from_npc_map(npc_map); // 73
        Cobwebs::from_npc_map(npc_map); // 75
        SteelTongs::from_npc_map(npc_map); // 77
        AshenRemnants::from_npc_map(npc_map); // 78
        TemperatureOfDespair::from_npc_map(npc_map); // 80
        AncientHomunculus::from_npc_map(npc_map); // 83
        DemonsStrength::from_npc_map(npc_map); // 86
        TaintedGold::from_npc_map(npc_map); // 87
        FourLeggedWeapon::from_npc_map(npc_map); // 88
        UndergroundKnights::from_npc_map(npc_map); // 89
        PossessedMask::from_npc_map(npc_map); // 93 
        OutragedElemental::from_npc_map(npc_map); // 96 
        CubesGiantFortune::from_npc_map(npc_map); // 101
        CubesFortune::from_npc_map(npc_map); // 102
        CubesFortune::from_npc_map(npc_map); // 105
        SinfulExpressions::from_npc_map(npc_map); // 111
        WrigglingMalice::from_npc_map(npc_map); // 115
        MadGiggle::from_npc_map(npc_map); // 116
        LightKnight::from_npc_map(npc_map); // 124
        FerociousDogs::from_npc_map(npc_map); // 126
        UnpopularCarnival::from_npc_map(npc_map); // 127
        FaithfulExecution::from_npc_map(npc_map); // 128
        Knightfall::from_npc_map(npc_map); // 137
        BladeOfFaith::from_npc_map(npc_map); // 138
        LostIntegrity::from_npc_map(npc_map); // 145
        SelfishExecutioner::from_npc_map(npc_map); // 150
        LostSoul::from_npc_map(npc_map); // 153
        VengefulMayhem::from_npc_map(npc_map); // 161
        WickedAdventurer::from_npc_map(npc_map); // 166
        NamelessElemental::from_npc_map(npc_map); // 173
        ExecutionerOfRebelliousLight::from_npc_map(npc_map); // 175
        PoisonousFlower::from_npc_map(npc_map); // 176
        IllegallyModifiedHuman::from_npc_map(npc_map); // 184
        SteelHeart::from_npc_map(npc_map); // 195
        SanguinarySorcery::from_npc_map(npc_map); // 198
        SorrowfulWitch::from_npc_map(npc_map); // 199
        CubesFortune::from_npc_map(npc_map); // 204
        FrozenMace::from_npc_map(npc_map); // 211
        ForbiddenMagickCreature::from_npc_map(npc_map); // 216
        ExquisiteHomunculus::from_npc_map(npc_map); // 221
        ShackledAgent::from_npc_map(npc_map); // 224
        ShackledDemon::from_npc_map(npc_map); // 230
        FloweringBeast::from_npc_map(npc_map); // 234
        SweetVoice::from_npc_map(npc_map); // 237
        FalseFog::from_npc_map(npc_map); // 239
        CracklingFire::from_npc_map(npc_map); // 244
        GlideStrikeForces::from_npc_map(npc_map); // 245
        ViciousMiasma::from_npc_map(npc_map); // 247
        LightDevourer::from_npc_map(npc_map); // 248
        MayhemShard::from_npc_map(npc_map); // 251
        PhantomFragment::from_npc_map(npc_map); // 256
        ShardOfDarkness::from_npc_map(npc_map); // 263
        PlagueShard::from_npc_map(npc_map); // 263
        CovetousShard::from_npc_map(npc_map); // 267
        CubesFortune::from_npc_map(npc_map); // 302
        CubesFortune::from_npc_map(npc_map); // 303
        Self {
            sword_talk: SwordTalk::from_npc_map(npc_map)
        }
    }   
}