use rustc_hash::FxHashMap;

use crate::{create_struct_with_npcs, wrap_npc_struct, Npc};

create_struct_with_npcs!(
    Monsters,
    a => ""
);

create_struct_with_npcs!(
    Rapport,
    a => ""
);

create_struct_with_npcs!(
    Bosses,
    a => ""
);

wrap_npc_struct!(Anikka,
{
    monsters: Monsters<'a>,
    rapport: Rapport<'a>,
    bosses: Bosses<'a>,
});