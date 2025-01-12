use crate::{create_struct_with_npcs, wrap_npc_struct};

create_struct_with_npcs!(
    GateOne,
    tienis => "Tienis"
);

create_struct_with_npcs!(
    GateTwo,
    prunya => "Prunya"
);

create_struct_with_npcs!(
    GateThree,
    lauriel => "Kaltaya, the Blooming Chaos"
);

wrap_npc_struct!(Kayangel,
{
    gate_one: GateOne<'a>,
    gate_two: GateTwo<'a>,
    gate_three: GateThree<'a>,
});