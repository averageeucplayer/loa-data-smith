use crate::{create_struct_with_npcs, wrap_npc_struct};

create_struct_with_npcs!(
    GateOne,
    echoing_phantom_wardog => "Echoing Phantom Wardog",
    despairing_statue => "Despairing Statue",
    nightmare_gehenna => "Nightmare Gehenna",
    nightmare_helkasirs => "Nightmare Helkasirs",
    gehenna_helkasirs => "Gehenna Helkasirs"
);

create_struct_with_npcs!(
    GateTwo,
    phantom_apostle => "Phantom Apostle",
    ashtarot => "Ashtarot"
);

create_struct_with_npcs!(
    GateThree,
    primordial_nightmare => "Primordial Nightmare"
);

create_struct_with_npcs!(
    GateFour,
    phantom_legion_commander_brelshaza => "Phantom Legion Commander Brelshaza",
    the_last_stop => "The Last Stop"
);

create_struct_with_npcs!(
    InfernoGateOne,
    brelshaza_monarch_of_nightmares => "Brelshaza, Monarch of Nightmares",
    imagined_primordial_nightmare => "Imagined Primordial Nightmare",
    pseudospace_primordial_nightmare => "Pseudospace Primordial Nightmare"
);

create_struct_with_npcs!(
    InfernoGateTwo,
    phantom_legion_commander_brelshaza => "Phantom Legion Commander Brelshaza",
    the_last_stop => "The Last Stop"
);

wrap_npc_struct!(Brelshaza,
{
    normal: BrelshazaNormalOrHard<'a>,
    hard: BrelshazaNormalOrHard<'a>,
    inferno: BrelshazaInferno<'a>
});

wrap_npc_struct!(BrelshazaNormalOrHard,
{
    gate_one: GateOne<'a>,
    gate_two: GateTwo<'a>,
    gate_three: GateThree<'a>,
    gate_four: GateFour<'a>,
});

wrap_npc_struct!(BrelshazaInferno,
{
    gate_one: InfernoGateOne<'a>,
    gate_two: InfernoGateTwo<'a>,
});