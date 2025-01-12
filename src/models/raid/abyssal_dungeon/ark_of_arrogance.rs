use crate::{create_struct_with_npcs, wrap_npc_struct};

create_struct_with_npcs!(
    RoadOfLament,
    nazan => "Nazan",
    kyzra => "Kyzra"
);

create_struct_with_npcs!(
    ForgeOfFallenPride,
    amaus => "Amaus",
    kaishur => "Kaishur"
);

wrap_npc_struct!(ArkOfArrogance,
{
    road_of_lament: RoadOfLament<'a>,
    forge_of_fallen_pride: ForgeOfFallenPride<'a>,
});