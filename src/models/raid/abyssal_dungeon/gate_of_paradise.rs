use crate::{create_struct_with_npcs, wrap_npc_struct};

create_struct_with_npcs!(
    SeaOfIndolence,
    deep_sea_orboros => "Deep Sea Orboros",
    indolence_sentinel_akam => "Indolence Sentinel Akam"
);

create_struct_with_npcs!(
    TranquilKarkosa,
    kallivan_of_the_eroded_storm => "Kallivan of the Eroded Storm",
    karkosas_punisher => "Karkosa's Punisher",
    karkosa_monarch_draikhan => "Karkosa Monarch Draikhan"
);

create_struct_with_npcs!(
    AlaricsSanctuary,
    karkosa_light => "Karkosa's Light",
    witch_of_enchantment_sarahiel => "Witch of Enchantment Sarahiel",
    nightmare_cncarnate_belloc => "Nightmare Incarnate Belloc",
    alaric => "Alaric"
);

wrap_npc_struct!(GateOfParadise,
{
    sea_of_indolence: SeaOfIndolence<'a>,
    tranquil_karkosa: TranquilKarkosa<'a>,
    alarics_sanctuary: AlaricsSanctuary<'a>,
});