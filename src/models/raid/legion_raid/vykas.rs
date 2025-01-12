use crate::{create_struct_with_npcs, wrap_npc_struct};

create_struct_with_npcs!(
    GateOne,
    covetous_devourer_vykas =>  "Covetous Devourer Vykas"
);

create_struct_with_npcs!(
    GateTwo,
    covetous_legion_commander_vykas =>  "Covetous Legion Commander Vykas"
);

create_struct_with_npcs!(
    InfernoGateOne,
    incubus_morphe =>  "Incubus Morphe",
    nightmarish_morphe => "Nightmarish Morphe"
);

create_struct_with_npcs!(
    InfernoGateTwo,
    covetous_devourer_vykas =>  "Covetous Devourer Vykas"
);

create_struct_with_npcs!(
    InfernoGateThree,
    covetous_legion_commander_vykas =>  "Covetous Legion Commander Vykas"
);

wrap_npc_struct!(Vykas,
{
    normal: VykasNormalOrHard<'a>,
    hard: VykasNormalOrHard<'a>,
    inferno: VykasInferno<'a>
});

wrap_npc_struct!(VykasNormalOrHard,
{
    gate_one: GateOne<'a>,
    gate_two: GateTwo<'a>,
});

wrap_npc_struct!(VykasInferno,
{
    gate_one: InfernoGateOne<'a>,
    gate_two: InfernoGateTwo<'a>,
    gate_three: InfernoGateThree<'a>,
});
