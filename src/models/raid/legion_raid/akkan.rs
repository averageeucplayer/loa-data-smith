use crate::{create_struct_with_npcs, wrap_npc_struct};

create_struct_with_npcs!(
    GateOne,
    griefbringer_maurug => "Griefbringer Maurug",
    evolved_maurug => "Evolved Maurug"
);

create_struct_with_npcs!(
    GateTwo,
    lord_of_degradation_akkan => "Lord of Degradation Akkan"
);

create_struct_with_npcs!(
    GateThree,
    plague_legion_commander_akkan =>  "Plague Legion Commander Akkan"
);

create_struct_with_npcs!(
    SecretPhase,
    lord_of_kartheon_akkan =>  "Lord of Kartheon Akkan"
);

wrap_npc_struct!(Akkan,
{
    normal: AkkanNormal<'a>,
    hard: AkkanHard<'a>
});

wrap_npc_struct!(AkkanNormal,
{
    gate_one: GateOne<'a>,
    gate_two: GateOne<'a>,
    gate_three: GateOne<'a>,
});

wrap_npc_struct!(AkkanHard,
{
    gate_one: GateOne<'a>,
    gate_two: GateOne<'a>,
    gate_three: GateOne<'a>,
    secret_phase: SecretPhase<'a>,
});