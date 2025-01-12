use crate::{create_struct_with_npcs, wrap_npc_struct};

create_struct_with_npcs!(
    GateOne,
    saydon => "Saydon"
);

create_struct_with_npcs!(
    GateTwo,
    kakul => "Kakul"
);

create_struct_with_npcs!(
    GateThree,
    kakul_saydon => "Kakul-Saydon"
);

create_struct_with_npcs!(
    SecretPhase,
    encore_desiring_kakul_saydon => "Encore-Desiring Kakul-Saydon"
);

wrap_npc_struct!(KakulSaydon,
{
    gate_one: GateOne<'a>,
    gate_two: GateTwo<'a>,
    gate_three: GateThree<'a>,
    secret_phase: SecretPhase<'a>,
});