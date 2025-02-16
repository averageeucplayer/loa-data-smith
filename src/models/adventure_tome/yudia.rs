use rustc_hash::FxHashMap;

use crate::{create_struct_with_npcs, wrap_npc_struct, Npc};

create_struct_with_npcs!(
    Monsters,
    whitesand_bolchi_boss => "Whitesand Bolchi Boss",
    reanimated_chieftain => "Reanimated Chieftain",
    stinging_wallou_cactus => "Stinging Wallou Cactus",
    strong_worm_fist => "Strong Worm Fist",
    alluring_shaman => "Alluring Shaman",
    boss_pangolin => "Boss Pangolin",
    old_salt_rock_turtle => "Old Salt Rock Turtle",
    dirty_salt_worm => "Dirty Salt Worm",
    boss_paingolin => "	Boss Paingolin",
    hardshelled_saltbug => "Hardshelled Saltbug",
    salt_desert_bandit_fighter => "Salt Desert Bandit Fighter",
    boss_mungka => "Boss Mungka",
    dirty_mucus_lump => "Dirty Mucus Lump",
    corrupted_red_eyed_rock_demon => "Corrupted Red-Eyed Rock Demon",
    lonely_morai_clay_soldier => "Lonely Morai Clay Soldier",
    morai_clay_shaman_chief => "Morai Clay Shaman Chief"
);

create_struct_with_npcs!(
    Rapport,
    thunder => "Thunder",
    morina => "Morina"
);

create_struct_with_npcs!(
    Bosses,
    salt_giant => "Salt Giant"
);

wrap_npc_struct!(Yudia,
{
    monsters: Monsters<'a>,
    rapport: Rapport<'a>,
    bosses: Bosses<'a>,
});