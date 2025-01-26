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
    mayhem_legion_hecate => "Mayhem Legion Hecate"
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
    plague_legion_madrigos => "Plague Legion Madrigos"
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