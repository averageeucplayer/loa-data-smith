use rustc_hash::FxHashMap;

use crate::{create_struct_with_npcs, wrap_npc_struct, Npc};

create_struct_with_npcs!(
    RaidLevelOne,
    ur_nil => "Ur'nil",
    lumerus => "Lumerus",
    icy_legoros => "Icy Legoros",
    vertus => "Vertus"
);

create_struct_with_npcs!(
    RaidLevelTwo,
    chromanium => "Chromanium",
    nacrasena => "Nacrasena",
    flame_fox_yoho => "Flame Fox Yoho",
    tytalos => "Tytalos"
);

create_struct_with_npcs!(
    RaidLevelThree,
    dark_legoros => "Dark Legoros",
    helgaia => "Helgaia",
    calventus => "Calventus",
    achates => "Achates"
);

create_struct_with_npcs!(
    RaidLevelFive,
    armored_nacrasena => "Dark Legoros",
    igrexion => "Igrexion",
    night_fox_yoho => "Night Fox Yoho",
    velganos => "Velganos"
);

create_struct_with_npcs!(
    RaidLevelSix,
    deskaluda => "Deskaluda",
    kungelanium => "Kungelanium",
    calligos => "Caliligos",
    hanumatan => "Hanumatan"
);

create_struct_with_npcs!(
    Levanos,
    levanos => "Levanos",
    levanos_core => "Levanos Core"
);

wrap_npc_struct!(VaigrysTest,
{
    one: RaidLevelOne<'a>,
    two: RaidLevelTwo<'a>,
    three: RaidLevelThree<'a>,
    four: RaidLevelFour<'a>,
    five: RaidLevelFive<'a>,
    six: RaidLevelSix<'a>,
});

pub struct RaidLevelFour<'a> {
    pub frost_helgaia: &'a Npc,
    pub lava_chromanium: &'a Npc,
    pub levanos: Levanos<'a>,
    pub alberhastic: &'a Npc,
}

impl <'a> RaidLevelFour<'a> {
    pub fn from_npc_map(npc_map: &'a FxHashMap<String, Npc>) -> Self {
        Self {
            frost_helgaia: npc_map.get("Frost Helgaia").unwrap(),
            lava_chromanium: npc_map.get("Lava Chromanium").unwrap(),
            levanos: Levanos::from_npc_map(npc_map),
            alberhastic: npc_map.get("Alberhastic").unwrap(),
        }
    }   
}