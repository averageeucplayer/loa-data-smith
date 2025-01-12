use crate::{create_struct_with_npcs, wrap_npc_struct};

create_struct_with_npcs!(
    GateOne,
    behemoth_the_storm_commander => "Behemoth, the Storm Commander",
    vicious_argeos => "Vicious Argeos",
    ruthless_lakadroff => "Ruthless Lakadroff",
    untrue_crimson_yoho => "Untrue Crimson Yoho",
    despicable_skolakia => "Despicable Skolakia"
);

create_struct_with_npcs!(
    GateTwo,
    behemoth_cruel_storm_slayer => "Behemoth, Cruel Storm Slayer"
);

wrap_npc_struct!(BehemothTheStormCommander,
{
    gate_one: GateOne<'a>,
    gate_two: GateTwo<'a>,
});

pub struct EpicRaid<'a> {
    pub behemoth_the_storm_commander: BehemothTheStormCommander<'a>
}