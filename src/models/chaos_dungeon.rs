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
    Sylmael,
    mutated_vinus_delain => "Mutated Vinus Delain",
    mutated_sylmael_demon_dog => "Mutated Sylmael Demon Dog",
    mutated_sylmael_giant_demon_dog => "Mutated Sylmael Giant Demon Dog",
    skolakia_eroded_by_miasma => "Skolakia Eroded by Miasma",
    argeos_eroded_by_miasma => "Argeos Eroded by Miasma"
);