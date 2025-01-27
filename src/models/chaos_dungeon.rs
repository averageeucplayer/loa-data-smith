use std::any::Any;

use rustc_hash::FxHashMap;

use crate::{create_struct_with_npcs, Npc};

create_struct_with_npcs!(
    DarknessLegion,
    darkness_legion_dark_knight => "Darkness Legion Dark Knight",
    darkness_legion_elemental_knight => "Darkness Legion Elemental Knight",
    darkness_legion_knight => "Darkness Legion Knight",
    darkness_legion_soldier => "Darkness Legion Soldier",
    darkness_legion_jamael => "Darkness Legion Jamael",
    darkness_legion_alcador => "Darkness Legion Alcador",
    darkness_legion_corax => "Darkness Legion Corax",
    darkness_legion_drakwald => "Darkness Legion Drakwald",
    darkness_legion_tarcos => "Darkness Legion Tarcos",
    darkness_legion_skeleton_soldier => "Darkness Legion Skeleton Soldier"
);

create_struct_with_npcs!(
    PhantomLegion,
    phantom_legion_pawn => "Phantom Legion Pawn",
    phantom_legion_soldier => "Phantom Legion Soldier",
    phantom_legion_demon_dog => "Phantom Legion Demon Dog",
    phantom_legion_visilia => "Phantom Legion Visilia",
    phantom_legion_summoner => "Phantom Legion Summoner",
    phantom_legion_knight => "Phantom Legion Knight",
    phantom_legion_king => "Phantom Legion King",
    phantom_legion_rook => "Phantom Legion Rook",
    phantom_legion_bishop => "Phantom Legion Bishop"
);

create_struct_with_npcs!(
    MayhemLegion,
    mayhem_legion_gorak => "Mayhem Legion Gorak",
    mayhem_legion_geunyo => "Mayhem Legion Geunyo",
    mayhem_legion_belled =>  "Mayhem Legion Belled",
    mayhem_legion_kiriko =>  "Mayhem Legion Kiriko",
    mayhem_legion_brook => "Mayhem Legion Brook",
    mayhem_legion_big_toad => "Mayhem Legion Big Toad",
    mayhem_legion_laakir => "Mayhem Legion Laakir",
    mayhem_legion_cursed_doll => "Mayhem Legion Cursed Doll",
    mayhem_legion_warrior => "Mayhem Legion Warrior",
    mayhem_legion_corpse_tailor => "Mayhem Legion Corpse Tailor",
    mayhem_legion_hecate => "Mayhem Legion Hecate",
    mayhem_legion_mayhem_pest => "Mayhem Legion Mayhem Pest",
    mayhem_legion_crackcrick => "Mayhem Legion Crackcrick",
    mayhem_legion_shazlik => "Mayhem Legion Shazlik"
);

create_struct_with_npcs!(
    PlagueLegion,
    plague_legion_plague_keeper => "Plague Legion Plague Keeper",
    plague_legion_reanimated_sylvain => "Plague Legion Reanimated Sylvain",
    plague_legion_sakuul => "Plague Legion Sakuul",
    plague_legion_bamku => "Plague Legion Bamku",
    plague_legion_demon_dog => "Plague Legion Demon Dog",
    plague_legion_mutant => "Plague Legion Mutant",
    plague_legion_tazkul => "Plague Legion Tazkul",
    plague_legion_madrigos => "Plague Legion Madrigos",
    plague_legion_magdoff => "Plague Legion Magdoff",
    plague_legion_bartuk => "Plague Legion Bartuk",
    plague_legion_gelok => "Plague Legion Gelok",
    plague_legion_worm_fist => "Plague Legion Worm Fist",
    plague_legion_nimble_flesh => "Plague Legion Nimble Flesh"
);

create_struct_with_npcs!(
    Sylmael,
    mutated_vinus_delain => "Mutated Vinus Delain",
    mutated_sylmael_demon_dog => "Mutated Sylmael Demon Dog",
    mutated_sylmael_giant_demon_dog => "Mutated Sylmael Giant Demon Dog",
    skolakia_eroded_by_miasma => "Skolakia Eroded by Miasma",
    argeos_eroded_by_miasma => "Argeos Eroded by Miasma",
    enkalitus_eroded_by_miasma => "Enkalitus Eroded by Miasma"
);


create_struct_with_npcs!(
    Telpa,
    telpa_captain => "Telpa Captain",
    sawtooth_rat => "Sawtooth Rat",
    telpa_worker => "Telpa Worker",
    telpa_explosive_worker => "Telpa Explosive Expert",
    corrupted_tunnel_rat =>  "Corrupted Tunnel Rat",
    telpa_shield_lancer =>  "Telpa Shield Lancer",
    shade_rictus => "Shade Rictus"
);

create_struct_with_npcs!(
    Goblin,
    goblin => "Goblin",
    goblin_shielder => "Goblin Shielder",
    goblin_peltast => "Goblin Peltast",
    axe_ielding_nurt => "Axe-Wielding Nurt",
    fire_dokkaebi => "Fire Dokkaebi"
);

create_struct_with_npcs!(
    Elgacia,
    foul_swordfighter => "Foul Swordfighter",
    foul_soldier => "Foul Soldier",
    foul_knight => "Foul Knight",
    foul_archer => "Foul Archer",
    foul_construct => "Foul Construct",
    foul_bayul => "Foul Bayul",
    foul_elver => "Foul Elver",
    foul_skyshadow => "Foul Skyshadow",
    foul_ryucrokota => "Foul Ryucrokota"
);

create_struct_with_npcs!(
    Punika,
    ant => "Ant",
    desert_ant => "Desert Ant",
    giant_soldier_ant => "Giant Soldier Ant",
    desert_ant_queen => "Desert Ant Queen"
);

create_struct_with_npcs!(
    Forest,
    mutant_mushroom => "Mutant Mushroom",
    root_talker => "Root Stalker",
    blue_spore => "Blue Spore",
    ominous_poison_thorn_mandrake => "Ominous Poison Thorn Mandrake"
);


create_struct_with_npcs!(
    ElzowinShade,
    large_cave_slime => "Large Cave Slime",
    mandrake => "Mandrake",
    poison_thorn_mandrake => "Poison Thorn Mandrake",
    spore => "Spore",
    root_stalker => "Root Stalker",
    treant => "Treant",
    forest_avizagura => "Forest Avizagura",
    scarlet_bubble_slime => "Scarlet Bubble Slime"
);

create_struct_with_npcs!(
    Demon,
    demon_dog => "Demon Dog",
    evolved_demon_dog => "Evolved Demon Dog",
    gargoyle => "Gargoyle",
    dark_warrior => "Dark Warrior",
    admos => "Admos",
    kalmaris => "Kalmaris"
);

create_struct_with_npcs!(
    SouthVern,
    black_knights_soldier => "Black Knights Soldier",
    black_knights_hound => "Black Knights Hound",
    black_knights_insect_knight => "Black Knights Insect Knight",
    black_knights_insect_soldier => "Black Knights Insect Soldier"
);