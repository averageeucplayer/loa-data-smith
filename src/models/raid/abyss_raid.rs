use crate::{create_struct_with_npcs, wrap_npc_struct};

create_struct_with_npcs!(
    GateOne,
    argos => "Argos");

create_struct_with_npcs!(
    GateTwo,
    argos => "Argos",
    tarvos => "Tarvos",
    rift_of_tarvos => "Rift of Tarvos",
    veorix => "Veorix"
);

create_struct_with_npcs!(
    GateThree,
    argos => "Argos");

wrap_npc_struct!(Argos,
{
    gate_one: GateOne<'a>,
    gate_two: GateTwo<'a>,
    gate_three: GateThree<'a>,
});

pub struct AbyssRaid<'a> {
    pub argos: Argos<'a>
}