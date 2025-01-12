use crate::{create_struct_with_npcs, wrap_npc_struct};

create_struct_with_npcs!(
    GateOne,
    kaltaya_the_blooming_chaos => "Kaltaya, the Blooming Chaos"
);

create_struct_with_npcs!(
    GateTwo,
    rakathus_the_lurking_arrogance => "Rakathus, the Lurking Arrogance"
);

create_struct_with_npcs!(
    GateThree,
    lazaramthe_trailblazer => "Lazaram, the Trailblazer",
    subordinated_vertus => "Subordinated Vertus",
    subordinated_calventus => "Subordinated Calventus",
    subordinated_legoros => "Subordinated Legoros",
    brand_of_subordination => "Brand of Subordination"
);

wrap_npc_struct!(IvoryTowerOfChaos,
{
    gate_one: GateOne<'a>,
    gate_two: GateTwo<'a>,
    gate_three: GateThree<'a>,
});
