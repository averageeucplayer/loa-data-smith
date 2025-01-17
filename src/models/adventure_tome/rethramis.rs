use rustc_hash::FxHashMap;

use crate::{create_struct_with_npcs, wrap_npc_struct, Npc};

create_struct_with_npcs!(
    Monsters,
    lead_white_redbeak => "Lead White Redbeak",
    lead_croclaw => "Lead Croclaw",
    toxiclaw_explorer => "Toxiclaw Explorer",
    lead_moss_wolf => "Lead Moss Wolf",
    giant_skehal => "Giant Skehal",
    boss_gravedigger => "Boss Gravedigger",
    old_mera_snake => "Old Mera Snake",
    boss_brook => "Boss Brook",
    flesh_eating_spider_boss => "Flesh-eating Spider Boss",
    toxiclaw_elite_archer => "Toxiclaw Elite Archer",
    decomposed_infected => "Decomposed Infected",
    pungent_gravedigger => "Pungent Gravedigger",
    giant_thornwalker => "Giant Thornwalker",
    wandering_infected => "Wandering Infected",
    boss_mystical_snake => "Boss Mystical Snake"
);

create_struct_with_npcs!(
    Rapport,
    neria => "Neria",
    siera => "Siera"
);

create_struct_with_npcs!(
    Bosses,
    rudric => "Rudric"
);

wrap_npc_struct!(Rethramis,
{
    monsters: Monsters<'a>,
    rapport: Rapport<'a>,
    bosses: Bosses<'a>,
});