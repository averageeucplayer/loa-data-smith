use crate::{create_struct_with_npcs, wrap_npc_struct};

create_struct_with_npcs!(
    AirasOculus,
    frenzied_cicerra => "Frenzied Cicerra",
    lost_seto => "Lost Seto",
    mirror_ball => "Mirror Ball"
);

create_struct_with_npcs!(
    OrehaPreveza,
    angry_moguro_captain => "Angry Moguro Captain",
    corrupted_albion => "Corrupted Albion"
);

wrap_npc_struct!(OrehasWell,
{
    airas_oculus: AirasOculus<'a>,
    oreha_preveza: OrehaPreveza<'a>,
});
