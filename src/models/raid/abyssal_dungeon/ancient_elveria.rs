use crate::{create_struct_with_npcs, wrap_npc_struct};

create_struct_with_npcs!(
    DemonBeastCanyon,
    corrupted_vazuela => "Corrupted Vazuela",
    vilec_scarkrill => "Vile Scarkrill"
);

create_struct_with_npcs!(
    NecromancersOrigin,
    phantom_legion_rook => "Reanimated Garum",
    phantom_legion_bishop => "Sigmund the Immortal"
);

wrap_npc_struct!(AncientElveria,
{
    demon_beast_canyon: DemonBeastCanyon<'a>,
    necromancers_origin: NecromancersOrigin<'a>,
});