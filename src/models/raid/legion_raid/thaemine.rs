use crate::{create_struct_with_npcs, wrap_npc_struct};

create_struct_with_npcs!(
    GateOne,
    killineza_the_dark_worshipper => "Killineza the Dark Worshipper"
);

create_struct_with_npcs!(
    GateTwo,
    valinak_knight_of_darkness => "Valinak, Knight of Darkness",
    valinak_taboo_usurper => "Valinak, Taboo Usurper",
    valinak_herald_of_the_end => "Valinak, Herald of the End"
);

create_struct_with_npcs!(
    GateThree,
    thaemine_the_lightqueller =>  "Thaemine the Lightqueller",
    dark_dreatswork =>  "Dark Greatsword"
);

create_struct_with_npcs!(
    GateFour,
    darkness_legion_commander_thaemine =>"Darkness Legion Commander Thaemine",
    thaemine_prokel => "Thaemine Prokel",
    thaemine_conqueror_of_stars => "Thaemine, Conqueror of Stars"
);

wrap_npc_struct!(ThaemineNormal,
{
    gate_one: GateOne<'a>,
    gate_two: GateTwo<'a>,
    gate_three: GateThree<'a>,
});

wrap_npc_struct!(ThaemineHard,
{
    gate_one: GateOne<'a>,
    gate_two: GateTwo<'a>,
    gate_three: GateThree<'a>,
    gate_four: GateFour<'a>,
});

wrap_npc_struct!(Thaemine,
{
    normal: ThaemineNormal<'a>,
    hard: ThaemineHard<'a>,
});