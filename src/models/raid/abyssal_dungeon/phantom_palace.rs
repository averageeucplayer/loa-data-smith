use crate::{create_struct_with_npcs, wrap_npc_struct};

create_struct_with_npcs!(
    HallOfTheTwistedWarlord,
    phantom_legion_rook => "Phantom Legion Rook",
    phantom_legion_bishop => "Phantom Legion Bishop",
    phantom_legion_king => "Phantom Legion King"
);

create_struct_with_npcs!(
    HildebrandtPalace,
    phantom_legion_queen => "Phantom Legion Queen",
    brelshaza => "Brelshaza"
);

wrap_npc_struct!(PhantomPalace,
{
    hall_of_the_twisted_warlord: HallOfTheTwistedWarlord<'a>,
    hildebrandt_palace: HildebrandtPalace<'a>,
});