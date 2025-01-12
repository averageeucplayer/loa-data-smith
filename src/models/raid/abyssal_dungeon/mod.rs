use ancient_elveria::AncientElveria;
use ark_of_arrogance::ArkOfArrogance;
use gate_of_paradise::GateOfParadise;
use ivory_tower_of_chaos::IvoryTowerOfChaos;
use kayangel::Kayangel;
use orehas_well::OrehasWell;
use phantom_palace::PhantomPalace;

pub mod ancient_elveria;
pub mod phantom_palace;
pub mod ark_of_arrogance;
pub mod gate_of_paradise;
pub mod orehas_well;
pub mod kayangel;
pub mod ivory_tower_of_chaos;
use crate::wrap_npc_struct;

wrap_npc_struct!(AbyssalDungeon,
{
    ancient_elveria: AncientElveria<'a>,
    phantom_palace: PhantomPalace<'a>,
    ark_of_arrogance: ArkOfArrogance<'a>,
    gate_of_paradise: GateOfParadise<'a>,
    orehas_well: OrehasWell<'a>,
    kayangel: Kayangel<'a>,
    ivory_tower_of_chaos: IvoryTowerOfChaos<'a>
});