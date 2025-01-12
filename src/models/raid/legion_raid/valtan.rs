use crate::{create_struct_with_npcs, wrap_npc_struct};

create_struct_with_npcs!(
    GateOne,
    dark_mountain_predator => "Dark Mountain Predator",
    destroyer_lcucas => "Destroyer Lucas",
    leader_lugaru => "Leader Lugaru"
);

create_struct_with_npcs!(
    GateTwo,
    demon_beast_commander_valtan => "Demon Beast Commander Valtan",
    ravaged_tyrant_of_beasts => "Ravaged Tyrant of Beasts"
);

wrap_npc_struct!(Valtan,
{
    gate_one: GateOne<'a>,
    gate_two: GateTwo<'a>,
});